#![allow(unused_imports)]

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_svc::eth::EspEth;

use embedded_svc::eth;

use embedded_svc::eth::{Eth, TransitionalState, Configuration};
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::eth::*;

use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;

use esp_idf_hal::prelude::Peripherals;

use smol::{ prelude::*, Executor, Task, Unblock };
use async_channel::{unbounded, Sender, Receiver};

use serde_json::{ Value, to_vec, from_slice, json };

pub mod secrets;

/******************************************************************************/
/* Typedefs */
/******************************************************************************/

type BoxValue = Box<Value>;


/******************************************************************************/
/******************************************************************************/

#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

#[cfg(not(feature = "qemu"))]
#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>, esp_idf_sys::EspError> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    println!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&embedded_svc::wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(std::time::Duration::from_secs(20), |status| !status.is_transitional());

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        println!("Wifi connected");
    } else {
        println!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

#[cfg(any(feature = "qemu"))]
fn eth_configure<HW>(mut eth: Box<EspEth<HW>>) -> Result<Box<EspEth<HW>>, std::fmt::Error> {
    println!("Eth created");

    eth.set_configuration(&eth::Configuration::Client(Default::default())).unwrap();

    println!("Eth configuration set, about to get status");

    eth.wait_status_with_timeout(std::time::Duration::from_secs(10), |status| !status.is_transitional());

    let status = eth.get_status();

    if let eth::Status::Started(eth::ConnectionStatus::Connected(eth::IpStatus::Done(Some(
        ip_settings,
    )))) = status
    {
        println!("Eth connected");

    } else {
        println!("Unexpected Eth status: {:?}", status);
    }

    Ok(eth)
}

/******************************************************************************/
/* Transport */
/******************************************************************************/
use smol::{net, prelude::*};
use bytes::{BytesMut, BufMut};

const MAGIC:u8 = 0x7f;
//const MAX_BLOCK_SIZE:u8 = 0x30; // 2^12 bytes = 4k
const MAX_BLOCK_SIZE:u8 = 0x20; // 2^11 bytes = 2k
const SERIALIZER:u8 = 0x01; // JSON serializer

const RAWSOCKET_MESSAGE_TYPE_REGULAR:u8 = 0;

// Connect and say hello to the server. We need to do the
// handshake here.
const HANDSHAKE:[u8;4] = [
                            MAGIC, // Flags to crossbar that we're speaking the same language
                            MAX_BLOCK_SIZE | SERIALIZER,
                            0, 0,
                        ]; 

#[derive(Clone)]
struct Transport {
    stream: net::TcpStream,
}

impl Transport {

    pub async fn message_send(&mut self, buf:Vec<u8>) {
        let mut message_length_buf = BytesMut::with_capacity(10);
        message_length_buf.put_u8(RAWSOCKET_MESSAGE_TYPE_REGULAR);
        message_length_buf.put_u8(0);
        message_length_buf.put_u16(buf.len().try_into().unwrap());

        println!("stream.write header : {:?}", message_length_buf);
        self.stream.write(&message_length_buf.to_vec()).await;
        println!("stream.write data: {:?}", buf);
        self.stream.write(&buf).await;
    }

    pub async fn message_get(&mut self) -> Option<Vec<u8>> {
        let mut buf = vec![0u8; 2048].into_boxed_slice();

        println!("WAITING FOR DATA");
        let read_bytes = self.stream.read(&mut buf).await;
        println!("GOT {:?} bytes!", read_bytes);

        // FIXME: need to keep collecting data or dealing with chunked
        // data or a burst with multiple packets at the same time?
        if buf.len() < 4 {
            return None;
        }

        Some(
            buf[4..read_bytes.unwrap()].to_vec()
        )
    }

    pub async fn negotiate(&mut self) {
        let mut buf = vec![0u8; 32];

        println!("Attempting handshake");

        // Perform the handshake

        // We start things off by doing the raw socket handshake with nexus
        // which determines if this is a nexus server, the protocol to use
        // and so on
        self.stream.write(&HANDSHAKE).await;

        // Let's get the server's response
        // FIXME: handle errors properly
        let read_bytes = self.stream.read(&mut buf).await.unwrap();
        if read_bytes != 4 {
            panic!("Did not get 4 bytes!")
        }
        if buf[0] != MAGIC {
            panic!("Did not get MAGIC")
        }

        let server_serializer = buf[1] & 0x0f;
        if server_serializer != SERIALIZER {
            panic!("Server did not agree to use JSON")
        }
        let server_buffer_size = buf[1] >> 4;
        println!("Server buffer size is: {}", server_buffer_size);
    }

    pub fn connect( url:&str ) -> Transport {
        println!("!!! CONNECTING to {}", url);
        let stream = smol::block_on(async {
                            net::TcpStream::connect(url).await.unwrap()
                        });
        let mut transport = Transport { stream };

        smol::block_on(async {
            transport.negotiate().await;
        });

        transport
    }
}

/******************************************************************************/
/* Wamp Client  */
/******************************************************************************/
use std::sync::Arc;
use async_mutex::Mutex;

// To help us build Builders
use derive_builder::Builder;

use std::{ thread, time };

// For mapping requests id back to consumers
use std::collections::HashMap;

const WAMP_HELLO:u64 = 1;
const WAMP_WELCOME:u64 = 2;
const WAMP_CHALLENGE:u64 = 4;
const WAMP_AUTHENTICATE:u64 = 5;
const WAMP_CALL:u64 = 48;

struct ConnectionInfo {
    url: String,
    realm: String,
    username: String,
    password: String,
}

type JoinFn = fn(&mut Wamp, &Value);

#[derive(Default, Builder)]
#[builder(setter(into))]
struct Tracker {
    #[builder(default = "None")]
    onjoin: Option<JoinFn>,
    #[builder(default = "0")]
    message_index: u64,
    #[builder(default = "HashMap::new()")]
    requests_pending: HashMap<u64, Sender<Value>>,
}


impl Tracker {
    fn next_request_id(&mut self) -> u64 {
        self.message_index += 1;
        self.message_index
    }
}

#[derive(Clone)]
pub struct Wamp {
    info: Arc<ConnectionInfo>,
    transport: Box<Transport>,
    tracker: Arc<Mutex<Tracker>>,
}

impl Wamp {

    pub async fn connect(url:&str, realm:&str, username:&str, password:&str) -> Wamp {
        let info = ConnectionInfo {
                        url: url.to_string(),
                        realm: realm.to_string(),
                        username: username.to_string(),
                        password: password.to_string(),
                    };
        let transport = Box::new(Transport::connect(url));
        let tracker = TrackerBuilder::default().build().unwrap();
        let wamp = Wamp {
            info: Arc::new(info),
            tracker: Arc::new(Mutex::new(tracker)),
            transport,
        };
        wamp
    }

    pub fn onjoin (&self, cb:JoinFn) {
        smol::block_on(async {
            self.tracker.lock().await.onjoin = Some(cb);
        });
    }

    pub async fn message_send(&mut self, message:Vec<u8>) {
        self.transport.message_send(message).await;
    }

    pub async fn message_get(&mut self) -> Option<Vec<u8>> {
        if let Some(buf) = self.transport.message_get().await {
            println!("!!!!!!!!!! GOT???? [[{}]]", std::str::from_utf8(&buf[..]).unwrap() );
            return Some(buf);
        }
        None
    }

    pub async fn authenticate(&mut self) {
        let message = Box::new(json!([
                            WAMP_HELLO,
                            self.info.realm,
                            {
                                "agent": "swampyer-rs",
                                "authid": self.info.username,
                                "authmethods": [ "ticket" ],
                                "roles": {
                                    "subscriber": {},
                                    "publisher": {},
                                    "caller": {},
                                    "callee": {},
                                }
                            }
                        ]));
        self.message_send(to_vec(&message).unwrap()).await;
    }

    pub async fn next_request_id(&self) -> u64 {
        return self.tracker.lock().await.next_request_id()
    }

    pub async fn request_response(&self) -> ( u64, Receiver<Value> )  {
        let request_id = self.next_request_id().await;
        let (s, r):(Sender<Value>, Receiver<Value>) = unbounded();
        self.tracker.lock().await.requests_pending.insert(request_id, s);
        ( request_id, r )
    }


    pub async fn handle_challenge(&mut self, message:&Value) {
        let json_message = json!([
                            WAMP_AUTHENTICATE,
                            self.info.password,
                            {}
                        ]);
        self.message_send(to_vec(&json_message).unwrap()).await;
    }

    pub async fn handle_welcome(&mut self, message:&Value) {
        let onjoin = self.tracker.lock().await.onjoin;
        if let Some(onjoin) = onjoin {
            onjoin(&mut self.clone(), message);
        }
    }

    pub async fn message_process(&mut self, message_str:Vec<u8>) {
        let message:Value = from_slice(&message_str).unwrap();
        let message_type = message[0].as_u64().unwrap();
        match message_type {
            WAMP_CHALLENGE => {
                println!("authentication request");
                self.handle_challenge(&message).await;
                println!("authentication done");
            },
            WAMP_WELCOME => {
                println!("welcome");
                self.handle_welcome(&message).await;
                println!("welcome done");
            },
            _ => {
                println!("random");
            },
        };
        println!("<<< Leaving process");
    }

    fn loop_process_messages(&mut self, receiver:&Receiver<Vec<u8>>) {
        smol::block_on(async move {
            loop {
                match receiver.try_recv() {
                    Ok(message) => {
                        println!("loop_process_messages.Ok.message");
                        self.message_process(message).await;
                    },
                    Err(e) => {
                        smol::Timer::after(std::time::Duration::from_secs(1)).await;
                        smol::future::yield_now().await;
                    },
                };
            };
        });
    }

    fn loop_incoming_dispatch(&mut self, sender:&Sender<Vec<u8>>) {
        smol::block_on(async move {
            loop {
                match self.message_get().await {
                    None => { println!("Things exploded." ) },
                    Some(message) => {
                        sender.try_send(message);
                    }
                }
            }
        });
    }

    pub async fn run(&mut self) {
        self.authenticate().await;

        let (sender, receiver):(Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();

        // Thread to handle the incoming events
        let r1 = receiver.clone();
        let mut thread_copy2 = self.clone();
        let handler2 = thread::Builder::new().stack_size(8192).spawn(move || {
            thread_copy2.loop_process_messages(&r1);
        }).unwrap();

        // Thread to handle the incoming events
        let r2 = receiver.clone();
        let mut thread_copy3 = self.clone();
        let handler3 = thread::Builder::new().stack_size(8192).spawn(move || {
            thread_copy3.loop_process_messages(&r2);
        }).unwrap();

        // Separate thread receiver for events coming in from nexushost
        let s1 = sender.clone();
        let mut thread_copy = self.clone();
        let handler = thread::Builder::new().spawn(move || {
            thread_copy.loop_incoming_dispatch(&s1);
        }).unwrap();

        // Permablock for now
        smol::block_on(async move {
            loop {
                smol::Timer::after(std::time::Duration::from_secs(1)).await;
            }
        });

    }
}

/******************************************************************************/
/* Calls */
/******************************************************************************/


#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct Call {
    uri: String,
    #[builder(default = "0")]
    request_id: u64,
    #[builder(default = "json!([])")]
    args: Value,
    #[builder(default = "json!({})")]
    kwargs: Value,
    #[builder(default = "json!({})")]
    options: Value,
}

impl Call {
    pub fn with(&self, wamp: &mut Wamp) {
        smol::block_on(async {
            let ( request_id, receiver ) = wamp.request_response().await;
            let message = json!([
                                WAMP_CALL,
                                request_id,
                                self.options,
                                self.uri,
                                self.args,
                                self.kwargs,
                            ]);

            println!("Sending: {:?}", message);
            wamp.message_send(to_vec(&message).unwrap()).await;
            println!("Sent Message");

            loop {
                match receiver.try_recv() {
                    Ok(t) => {
                        return t
                    },
                    Err(e) => {
                        println!("tick");
                        smol::Timer::after(std::time::Duration::from_secs(1)).await;
                        smol::future::yield_now().await;
                    },
                };
            };
        });
    }
}



/******************************************************************************/
/******************************************************************************/

fn onjoin ( wamp: &mut Wamp , message:&Value ) {
    println!("ONJOINCALLED! {:?}", message);

    let call = CallBuilder::default()
                      .uri("com.izaber.wamp.auth.whoami")
                      .build()
                      .unwrap();
    call.with(wamp);
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Handle network creation
    let peripherals = Peripherals::take().unwrap();

    // Initialize the network stack
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    #[cfg(any(feature = "qemu"))]
    let eth = eth_configure(Box::new(EspEth::new_openeth(
        netif_stack.clone(),
        sys_loop_stack.clone(),
    ).unwrap())).unwrap();

    #[cfg(not(feature = "qemu"))]
    #[allow(unused_mut)]
    let mut wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    ).unwrap();

    // This seems to be required for SMOL to be happy
    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::esp_vfs_eventfd_register(&esp_idf_sys::esp_vfs_eventfd_config_t {
            max_fds: 10,
            ..Default::default()
        })
    });

    // Now start talking?
    smol::block_on(async {
        let mut client = Box::new(Wamp::connect(
                                            secrets::URL,
                                            secrets::REALM,
                                            secrets::USERNAME,
                                            secrets::PASSWORD,
                                        ).await);
        client.onjoin(onjoin);
        client.run().await;
    });

}
