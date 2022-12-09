#![feature(trace_macros)]
#![allow(unused_imports, unreachable_code)]
#![allow(unused_variables, dead_code, unused_must_use)]

mod serialization;
use crate::serialization::*;

mod errors;

/*************************************************
 * Test Serializable Datastructure
 ************************************************/
#[derive(Encode, Decode, Clone, Debug, Wamp)]
struct Bump {
    #[n(0)] x: u8,
    #[n(1)] y: u8,
    #[n(2)] z: u8
}

fn main() {
    let bump:Bump = Bump { x:201, y:202, z:203};

    let arf = "123";
    let data:WampData = wamp!({
                "12": "bar",
                "items": [ 1, 2, 3, 4, bump, 600, 2000 ],
                "foo": (Bump { x:101, y:103, z: 104 }),
          });
    println!("{:?}", data);

    let h:WampData = WampData::Hash(
                          Box::new(
                              [
                                  ("bar".into(), data),
                              ].iter().cloned().collect()
                          ), 0
                      );
    println!("{:?}", h);

    let mut wriex = WampWrite { buffer: vec![] };
    let mut encoder = Encoder::new(&mut wriex);
    h.serialize_with(&mut encoder);
    println!("{:?}", wriex);

    // [1,2,[3,4,5],{"arf": 34}]
    // let test_data = [0x84, 0x1, 0x2, 0x83, 0x3, 0x4, 0x5, 0xa1, 0x63, 0x61, 0x72, 0x66, 0x18, 0x22];
    let test_data = wriex.buffer;
    let mut decoder = Decoder::new(&test_data);
    let mut desered = WampData::deserialize_with(&mut decoder);
    println!("{:?}", desered);

    // How do we index in?
    /*
    let ar = desered.as_array();
    println!("{:?}", ar);
    */

    /* Options
    let b:Bump = desered.h("bar")?.h("items")?.a(4)?.decode()?;
    let b:Bump = wamp_decode!(desered["bar"]["items"][4])?;
    */

    let hs = desered.h("bar").unwrap();
    println!("{:?}", hs);
    let hs2 = hs.h("items").unwrap();
    println!("{:?}", hs2);
    let ar = hs2.a(4).unwrap();
    println!("{:?}", ar);

    /*
    let hs = desered.as_hash().unwrap();
    let hs2 = hs["bar"].as_hash().unwrap();
    println!("{:?}", hs2);
    let hs3 = hs2["items"].as_array().unwrap();
    println!("{:?}", hs3[4]);
    // let b:Bump = hs3[4].decode_with(&mut decoder).unwrap();
    let b:Bump = hs3[4].decode_with(&mut decoder).unwrap();
    println!("{:?}", b);
    */
}




