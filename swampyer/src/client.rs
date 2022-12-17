// To manage concurrency
use smol::Executor;
use async_channel::{unbounded, Sender, Receiver};

// Debugging
use std::mem;

use std::sync::Arc;
use async_mutex::Mutex;

use std::{ thread, time };

// To help us build Builders
use derive_builder::Builder;

// For mapping requests id back to consumers
use std::collections::HashMap;

pub mod transport;
use crate::serialization::WampData;
use crate::wdata;
use crate::{WampError, WampHash, WampArray};

const WAMP_HELLO:u64 = 1;
const WAMP_WELCOME:u64 = 2;
const WAMP_CHALLENGE:u64 = 4;
const WAMP_AUTHENTICATE:u64 = 5;
const WAMP_PUBLISH:u64 = 16;
const WAMP_PUBLISHED:u64 = 17;
const WAMP_SUBSCRIBE:u64 = 32;
const WAMP_SUBSCRIBED:u64 = 33;
const WAMP_EVENT:u64 = 36;
const WAMP_CALL:u64 = 48;
const WAMP_RESULTS:u64 = 50;


struct ConnectionInfo {
    url: String,
    realm: String,
    username: String,
    password: String,
}

type JoinFn = fn(&mut WampClient, Box<WampData>);

#[derive(Default, Builder)]
#[builder(setter(into))]
struct Tracker {
    #[builder(default = "None")]
    onjoin: Option<JoinFn>,

    #[builder(default = "0")]
    message_index: u64,

    #[builder(default = "HashMap::new()")]
    requests_pending: HashMap<u64, Sender<Box<WampData>>>,

    #[builder(default = "None")]
    message_sender: Option<Sender<Vec<u8>>>,

    #[builder(default = "None")]
    message_receiver: Option<Receiver<Vec<u8>>>,
}

impl Tracker {
    fn next_request_id(&mut self) -> u64 {
        self.message_index += 1;
        self.message_index
    }
}

#[derive(Clone)]
pub struct WampClient {
    info: Arc<ConnectionInfo>,
    transport: transport::Transport,
    tracker: Arc<Mutex<Tracker>>,
    thread_stack_size: usize,
}

impl WampClient {

    pub async fn authenticate(&mut self) {
        let details = wdata!({
                            "authid": (self.info.username.clone()),
                            "agent": "swampyer-rs",
                            "authmethods": [ "ticket" ],
                            "roles": {
                                "subscriber": {},
                                "publisher": {},
                                "caller": {},
                                "callee": {},
                            },
                      });

        let message = wdata!([
                            WAMP_HELLO,
                            (self.info.realm.clone()),
                            details
                        ]);
        self.message_send(message).await;
    }

    pub async fn next_request_id(&self) -> u64 {
        return self.tracker.lock().await.next_request_id()
    }

    pub async fn request_response(&self) -> ( u64, Receiver<Box<WampData>> )  {
        let request_id = self.next_request_id().await;
        let (s, r):(Sender<Box<WampData>>, Receiver<Box<WampData>>) = unbounded();
        self.tracker.lock().await.requests_pending.insert(request_id, s);
        ( request_id, r )
    }

    pub async fn submit_response(&self, message:Box<WampData>) -> Result<(), WampError> {
        let request_id = message.a(1)?.as_u64()?;
        match self.tracker.lock().await.requests_pending.get(&request_id) {
            Some(sender) => {
                sender.send(message).await;
                Ok(())
            },

            None => { Err(WampError::UnknownRequestID) }
        }
    }

    pub async fn handle_challenge(&mut self, message:Box<WampData>) {
        self.message_send(wdata!([
                            WAMP_AUTHENTICATE,
                            (self.info.password.clone()),
                            {}
                        ])).await;
    }

    pub async fn handle_welcome(&mut self, message:Box<WampData>) {
        // println!("GOT WELCOME: {:?}", message);
        let onjoin = self.tracker.lock().await.onjoin;
        if let Some(onjoin) = onjoin {
            onjoin(&mut self.clone(), message);
        }
    }

    pub async fn message_send(&mut self, message:WampData) {
        println!("Sending: {:?}", message);
        self.transport.message_send(message.to_vec()).await;
    }

    pub async fn message_process(&mut self, message_str:Vec<u8>) {
        // FIXME: Need to handle error properly
        println!("Parsiing data");
        let message = WampData::from_slice(message_str).unwrap();
        println!("Parsed Data");
        println!("Getting message type");
        let message_type = message.a(0).unwrap().as_u64().unwrap();
        println!("Got message type");
        match message_type {
            WAMP_CHALLENGE => {
                println!("authentication request");
                self.handle_challenge(message).await;
                println!("authentication done");
            },
            WAMP_WELCOME => {
                println!("welcome");
                self.handle_welcome(message).await;
                println!("welcome done");
            },
            WAMP_RESULTS => {
                println!("result!");
                self.submit_response(message).await;
            },
            _ => {
                println!("random");
            },
        };
        println!("<<< Leaving process");
    }


    pub async fn message_get(&mut self) -> Option<Box<WampData>> {
        if let Some(buf) = self.transport.message_get().await {
            let message = WampData::from_slice(buf).unwrap();
            return Some(message);
        }
        None
    }

    pub async fn connect(url:&str, realm:&str, username:&str, password:&str) -> WampClient {
        let info = ConnectionInfo {
                        url: url.to_string(),
                        realm: realm.to_string(),
                        username: username.to_string(),
                        password: password.to_string(),
                    };
        let transport = transport::Transport::connect(url).unwrap();
        let mut tracker = TrackerBuilder::default().build().unwrap();

        let (sender, receiver):(Sender<Vec<u8>>, Receiver<Vec<u8>>) = unbounded();
        tracker.message_sender = Some(sender);
        tracker.message_receiver = Some(receiver);

        let mut wamp = WampClient {
            info: Arc::new(info),
            transport,
            tracker: Arc::new(Mutex::new(tracker)),
            thread_stack_size: 118192,
        };

        wamp.authenticate().await;

        wamp
    }

    fn loop_process_messages(&mut self, receiver:&Receiver<Vec<u8>>) {
        println!("message occupies {} bytes on the stack", mem::size_of_val(&receiver));
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
                        sender.try_send(message.to_vec());
                    }
                }
            }
        });
    }

    pub fn additional_thread(&mut self, stack_size:usize) -> thread::JoinHandle<()> {
        // Thread to handle the incoming events
        smol::block_on(async {
            let r = self.tracker.lock().await.message_receiver.as_ref().unwrap().clone();
            let mut thread_copy = self.clone();
            thread::Builder::new().stack_size(stack_size).spawn(move || {
                thread_copy.loop_process_messages(&r);
            }).unwrap()
        })
    }

    pub async fn run(&mut self) {
        self.authenticate().await;

        let j1 = self.additional_thread(self.thread_stack_size);
        let j2 = self.additional_thread(self.thread_stack_size);

        // Separate thread receiver for events coming in from nexushost
        let s1 = self.tracker.lock().await.message_sender.as_ref().unwrap().clone();
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

    pub fn onjoin (&self, cb:JoinFn) {
        smol::block_on(async {
            self.tracker.lock().await.onjoin = Some(cb);
        });
    }

    pub async fn call(&mut self, uri:&str, args:WampData, kwargs:WampData ) {
        let message:WampData = wdata!([
                                    WAMP_CALL,
                                    123, // request id
                                    {}, // options
                                    uri, // procedure
                                    args,
                                    kwargs
                                ]);
        println!("Message: {:?}", message);
        self.message_send(message).await;
    }
}

struct Call {
    uri: String,
    args: WampData,
    kwargs: WampData,
    options: WampData,
}

impl Call {
    fn new (uri: &str) -> Call {
        Call {
            uri: uri.into(),
            args: wdata!([]),
            kwargs: wdata!({}),
            options: wdata!({}),
        }
    }

    fn uri(uri: &str) -> Call {
        Call {
            uri: uri.into(),
            args: wdata!([]),
            kwargs: wdata!({}),
            options: wdata!({}),
        }
    }

    fn with(&self, wamp: &WampClient) {
    }
}



