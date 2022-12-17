#![allow(dead_code)]

use smol::{Executor};

pub mod secrets;

use swampyer::{WampClient, WampData, wdata, WampArray, WampHash, ALLOCATOR};
// use swampyer::{wdata, WampData};

fn onjoin (wamp: &mut WampClient, message:Box<WampData>) {
    // println!("ONJOINCALLED! {:?}", message);
    println!("ONJOINCALLED!");
    smol::block_on(async {
        wamp.call("auth.whoami", wdata!([]), wdata!({})).await;
    });
}

fn main() {
    ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();

    println!("Connecting to {}", secrets::URL);
    smol::block_on(async {
        let mut client = WampClient::connect(
                              secrets::URL,
                              secrets::REALM,
                              secrets::USERNAME,
                              secrets::PASSWORD,
                          ).await;
        let ex = Executor::new();
            ex.run(async {
            client.onjoin(onjoin);
            client.run().await;
        }).await;
    });
}

