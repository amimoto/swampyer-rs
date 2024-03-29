#![allow(unused_imports, unreachable_code)]
#![allow(unused_variables, dead_code, unused_must_use)]

use derive_builder::Builder;
pub use minicbor::{Decoder, Encoder, encode, data::Type};
pub use minicbor_derive::{Encode, Decode};
use std::collections::HashMap;
use std::sync::Arc;

use crate::errors::*;

use std::mem;

/*
 * Trait for allowing encode and decode
 */
pub trait WampSerializable {
    fn encode(&self, encoder:&mut Encoder<&mut WampWrite> );
    // fn decode<T:minicbor::Decode<'a, ()>>(&self, decoder:&mut Decoder) ->  T;
    fn debug_name(&self) -> &str;
}

impl core::fmt::Debug for dyn WampSerializable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.debug_name())
    }
}


impl core::fmt::Debug for dyn WampSerializable + Send + Sync {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.debug_name())
    }
}

/*
 * Our custom Write trait handler
 */
#[derive(Debug)]
pub struct WampWrite {
    pub buffer: Vec<u8>,
}

impl encode::Write for WampWrite {
    type Error = core::convert::Infallible;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.buffer.extend_from_slice(buf);
        Ok(())
    }
}

/*
 * Recursive data structure for WAMP calls
 */
pub type WampHash = HashMap<String, Box<WampData>>;
pub type WampArray = Vec<WampData>;

#[derive(Debug, Clone)]
pub enum WampData {
    Float(f64),
    Int(i64),
    UInt(u64),
    Bool(bool),
    Str(String),
    Array(Box<WampArray>, usize),
    Hash(Box<WampHash>, usize),
    Serializable(Arc<dyn WampSerializable + Send + Sync>),
    None,
}

impl <'a> WampData {
    pub fn as_u64 (&self) -> Result<u64, WampError> {
        match self {
            WampData::UInt(v) => Ok(*v),
            _ => Err(WampError::IncorrectElementType),
        }
    }

    pub fn from_slice(data:Vec<u8>) -> Result<Box<WampData>, WampError> {
        let mut decoder = Box::new(Decoder::new(&data));
        let desered = Box::new(WampData::deserialize_with(&mut decoder));
        Ok(desered)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut wriex = WampWrite { buffer: vec![] };
        let mut encoder = Encoder::new(&mut wriex);
        self.serialize_with(&mut encoder);

        wriex.buffer
    }

    pub fn serialize_with(&self, encoder:&mut Encoder<&mut WampWrite>) {
        match self {
            WampData::Float(f) => { encoder.f64(*f); },
            WampData::Int(i) => { encoder.i64(*i); },
            WampData::UInt(u) => { encoder.u64(*u); },
            WampData::Bool(b) => { encoder.bool(*b); },
            WampData::Str(s) => { encoder.str(s); },
            // WampData::Str(s) => { encoder.bytes(s.as_bytes()); },
            WampData::Array(a, _) => {
                encoder.begin_array();
                for v in a.iter() {
                    v.serialize_with(encoder);
                };
                encoder.end();
            },
            WampData::Hash(h, _) => {
                encoder.begin_map();
                for ( k, v ) in h.iter() {
                    encoder.str(k);
                    v.serialize_with(encoder);
                }
                encoder.end();
            },
            WampData::None => { encoder.null(); },
            WampData::Serializable(data) => {
                data.encode(encoder);
            },
        };
    }

    pub fn deserialize_with(decoder:&mut Box<Decoder>) -> Self {
        match decoder.datatype() {
            Ok(dt) => {
                match dt {
                    Type::Bool => { WampData::Bool(decoder.bool().unwrap().into()) },
                    Type::U8 => { WampData::UInt(decoder.u8().unwrap().into()) },
                    Type::U16 => { WampData::UInt(decoder.u16().unwrap().into()) },
                    Type::U32 => { WampData::UInt(decoder.u32().unwrap().into()) },
                    Type::U64 => { WampData::UInt(decoder.u64().unwrap().into()) },
                    Type::I16 => { WampData::Int(decoder.i16().unwrap().into()) },
                    Type::I32 => { WampData::Int(decoder.i32().unwrap().into()) },
                    Type::I64 => { WampData::Int(decoder.i64().unwrap().into()) },
                    Type::F16 => { WampData::Float(decoder.f32().unwrap().into()) },
                    Type::F32 => { WampData::Float(decoder.f32().unwrap().into()) },
                    Type::F64 => { WampData::Float(decoder.f64().unwrap().into()) },
                    Type::String => {
                        WampData::Str(decoder.str().unwrap().into())
                    },
                    Type::Array => {
                        let position = decoder.position();
                        match decoder.array() {
                            Ok(s) => {
                                match s {
                                    Some(v) => {
                                        let mut ar = Box::new(WampArray::new());
                                        for i in 0..v {
                                            let ar_element = WampData::deserialize_with(decoder);
                                            ar.push(ar_element);
                                        }
                                        WampData::Array(ar, position)
                                    },
                                    _ => WampData::None
                                }
                            },
                            Err(e) => {
                                WampData::None
                            }
                        }
                    },
                    Type::ArrayIndef => {
                        let position = decoder.position();
                        let mut ar = Box::new(WampArray::new());
                        decoder.array();
                        loop {
                            if decoder.datatype().unwrap() == Type::Break {
                                break;
                            };
                            let w = WampData::deserialize_with(decoder);
                            ar.push(w);
                        }
                        WampData::Array(ar, position)
                    }

                    Type::MapIndef => {
                        let position = decoder.position();
                        let mut hs = Box::new(WampHash::new());
                        decoder.map();
                        loop {
                            if decoder.datatype().unwrap() == Type::Break {
                                break;
                            };
                            hs.insert(
                                decoder.str().unwrap().into(),
                                Box::new(WampData::deserialize_with(decoder))
                            );
                        }
                        WampData::Hash(hs, position)
                    },

                    Type::Map => {
                        let position = decoder.position();
                        match decoder.map() {
                            Ok(s) => {
                                match s {
                                    Some(v) => {
                                        let mut hs = Box::new(WampHash::new());
                                        for i in 0..v {
                                            hs.insert(
                                                decoder.str().unwrap().into(),
                                                Box::new(WampData::deserialize_with(decoder))
                                            );
                                        }
                                        WampData::Hash(hs, position)
                                    }
                                    _ => WampData::None
                                }
                            }
                            _ => WampData::None
                        }
                    },
                    _ => {
                        println!("SKIPPING {:?}", dt);
                        decoder.skip();
                        WampData::None
                    }
                }
            },
            // Should be an error
            Err(err) => WampData::None
        }
    }

    pub fn h(&self, i:&str) -> Result<&WampData, WampError> {
        match self {
            WampData::Hash(h, _) => Ok(&h[i]),
            _ => Err(WampError::InvalidField),
        }
    }

    pub fn a(&self, i:usize) -> Result<&WampData, WampError> {
        match self {
            WampData::Array(a, _) => Ok(&a[i]),
            _ => Err(WampError::NotArray),
        }
    }

    pub fn decode_with<T:minicbor::Decode<'a, ()>>(&self, decoder:&mut Decoder<'a>) -> Result<T, WampError> {
        match self {
            WampData::Array(_, offset) => {
                decoder.set_position(*offset);
                match decoder.decode() {
                    Ok(v) => Ok(v),
                    Err(t) => Err(WampError::InvalidField),
                }
            },
            _ => Err(WampError::InvalidField),
        }
    }
}

/*
 * Add traits to various primitives so that we can convert into
 * WampData values easily
 */

impl From<u64> for WampData {
    fn from(i:u64) -> Self {
        WampData::UInt(i.into())
    }
}

impl From<i64> for WampData {
    fn from(i:i64) -> Self {
        WampData::Int(i.into())
    }
}

impl From<u32> for WampData {
    fn from(i:u32) -> Self {
        WampData::UInt(i.into())
    }
}

impl From<i32> for WampData {
    fn from(i:i32) -> Self {
        WampData::Int(i.into())
    }
}

impl From<u16> for WampData {
    fn from(i:u16) -> Self {
        WampData::UInt(i.into())
    }
}

impl From<i16> for WampData {
    fn from(i:i16) -> Self {
        WampData::Int(i.into())
    }
}

impl From<u8> for WampData {
    fn from(i:u8) -> Self {
        WampData::UInt(i.into())
    }
}

impl From<i8> for WampData {
    fn from(i:i8) -> Self {
        WampData::Int(i.into())
    }
}

impl From<f32> for WampData {
    fn from(i:f32) -> Self {
        WampData::Float(i.into())
    }
}

impl From<f64> for WampData {
    fn from(i:f64) -> Self {
        WampData::Float(i.into())
    }
}

impl From<bool> for WampData {
    fn from(i:bool) -> Self {
        WampData::Bool(i.into())
    }
}

impl From<&str> for WampData {
    fn from(i:&str) -> Self {
        WampData::Str(i.into())
    }
}

impl From<String> for WampData {
    fn from(i:String) -> Self {
        WampData::Str(i.into())
    }
}

#[macro_export]
macro_rules! wdata {

    // ARRAY
    ( [] ) => {
        {
        let array = Box::new(WampArray::new());
        WampData::Array(array, 0)
        }
    };

    ( [  $( $v:tt ),* ] ) => {
        {
        let mut array = Box::new(WampArray::with_capacity(10));
        $(
            let data = wdata!($v);
            array.push(data);
        )*
        WampData::Array(array, 0)
        }
    };

    ( [  $( $v:tt ),* , ] ) => {
        {
        let mut array = Box::new(WampArray::new());
        $(
            array.push(wdata!($v));
        )*
        WampData::Array(array, 0)
        }
    };

    // DICT
    ( { $( $k:tt: $v:tt ),* , } ) => {
        {
        let mut hash = Box::new(WampHash::new());
        $(
            let hash_key = wdata!(@dict_key $k);
            hash.insert(
                hash_key,
                Box::new(wdata!($v))
            );
        )*
        WampData::Hash(hash, 0)
        }
    };

    ( { $( $k:tt: $v:tt ),* } ) => {
        {
        let hash = Box::new(WampHash::new());
        $(
            let hash_key = wdata!(@dict_key $k);
            hash.insert(
                hash_key,
                Box::new(wdata!($v))
            );
        )*
        WampData::Hash(hash, 0)
        }
    };

    // DICT KEY
    ( @dict_key $x:expr ) => {
        $x.into()
    };

    ( $x:expr ) => { $x.into() };

    () => {};
}
