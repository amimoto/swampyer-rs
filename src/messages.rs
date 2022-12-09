#![allow(unused_imports, unreachable_code)]
#![allow(unused_variables, dead_code, unused_must_use)]

// To help us build Builders
use derive_builder::Builder;

use minicbor::Decoder;
use minicbor_derive::{Encode, Decode};

// For our errors

/**************************************************/
use std::fmt;

#[derive(Debug)]
pub enum WampError {
    NotArray,
    IncorrectElementCount,
    IncorrectElementType,
    InvalidField,
}

#[derive(Debug, Clone)]
pub struct NotArray;

impl fmt::Display for NotArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message was not an array")
    }
}

#[derive(Debug, Clone)]
pub struct IncorrectElementCount;

impl fmt::Display for IncorrectElementCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect number of elements")
    }
}


#[derive(Debug, Clone)]
pub struct IncorrectElementType;

impl fmt::Display for IncorrectElementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect number of elements")
    }
}



#[derive(Debug, Clone)]
pub struct InvalidField;

impl fmt::Display for InvalidField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field does not exist")
    }
}

/**************************************************/
pub struct Message<'a> {
    pub decoder: Box<Decoder<'a>>,
    pub entries: u64,
    pub index_positions: Box<Vec<usize>>,
}

pub trait MessageTrait<'a> {
    fn code(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn realm(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    fn session_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn reason(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn auth_method(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn extra<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    fn signature(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn request_code(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn request_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn error(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    fn options<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    fn topic(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn publication_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn subscription_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn publish_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
    fn procedure(&mut self) -> Result<&str, WampError> { Err(WampError::InvalidField) }
    fn registration_id(&mut self) -> Result<u64, WampError> { Err(WampError::InvalidField) }
}


/***********************************************
 * WHello id:1
 ***********************************************/
pub struct MsgHello<'a>(pub Message<'a>);

impl <'a> MsgHello<'a> {
    pub fn new (buf:&'a [u8]) -> MsgHello<'a> {
        let decoder = Decoder::new(&buf);
        MsgHello(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgHello<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn realm (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WWelcome id:2
 ***********************************************/
pub struct MsgWelcome<'a>(pub Message<'a>);

impl <'a> MsgWelcome<'a> {
    pub fn new (buf:&'a [u8]) -> MsgWelcome<'a> {
        let decoder = Decoder::new(&buf);
        MsgWelcome(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgWelcome<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn session_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WAbort id:3
 ***********************************************/
pub struct MsgAbort<'a>(pub Message<'a>);

impl <'a> MsgAbort<'a> {
    pub fn new (buf:&'a [u8]) -> MsgAbort<'a> {
        let decoder = Decoder::new(&buf);
        MsgAbort(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgAbort<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn reason (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WChallenge id:4
 ***********************************************/
pub struct MsgChallenge<'a>(pub Message<'a>);

impl <'a> MsgChallenge<'a> {
    pub fn new (buf:&'a [u8]) -> MsgChallenge<'a> {
        let decoder = Decoder::new(&buf);
        MsgChallenge(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgChallenge<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn auth_method (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn extra<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WAuthenticate id:5
 ***********************************************/
pub struct MsgAuthenticate<'a>(pub Message<'a>);

impl <'a> MsgAuthenticate<'a> {
    pub fn new (buf:&'a [u8]) -> MsgAuthenticate<'a> {
        let decoder = Decoder::new(&buf);
        MsgAuthenticate(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgAuthenticate<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn signature (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn extra<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WGoodbye id:6
 ***********************************************/
pub struct MsgGoodbye<'a>(pub Message<'a>);

impl <'a> MsgGoodbye<'a> {
    pub fn new (buf:&'a [u8]) -> MsgGoodbye<'a> {
        let decoder = Decoder::new(&buf);
        MsgGoodbye(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgGoodbye<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn reason (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WError id:8
 ***********************************************/
pub struct MsgError<'a>(pub Message<'a>);

impl <'a> MsgError<'a> {
    pub fn new (buf:&'a [u8]) -> MsgError<'a> {
        let decoder = Decoder::new(&buf);
        MsgError(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgError<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn error (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[5]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[6]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WPublish id:16
 ***********************************************/
pub struct MsgPublish<'a>(pub Message<'a>);

impl <'a> MsgPublish<'a> {
    pub fn new (buf:&'a [u8]) -> MsgPublish<'a> {
        let decoder = Decoder::new(&buf);
        MsgPublish(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgPublish<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn options<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn topic (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[5]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WPublished id:17
 ***********************************************/
pub struct MsgPublished<'a>(pub Message<'a>);

impl <'a> MsgPublished<'a> {
    pub fn new (buf:&'a [u8]) -> MsgPublished<'a> {
        let decoder = Decoder::new(&buf);
        MsgPublished(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgPublished<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn publication_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WSubscribe id:32
 ***********************************************/
pub struct MsgSubscribe<'a>(pub Message<'a>);

impl <'a> MsgSubscribe<'a> {
    pub fn new (buf:&'a [u8]) -> MsgSubscribe<'a> {
        let decoder = Decoder::new(&buf);
        MsgSubscribe(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgSubscribe<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn options<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn topic (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WSubscribed id:33
 ***********************************************/
pub struct MsgSubscribed<'a>(pub Message<'a>);

impl <'a> MsgSubscribed<'a> {
    pub fn new (buf:&'a [u8]) -> MsgSubscribed<'a> {
        let decoder = Decoder::new(&buf);
        MsgSubscribed(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgSubscribed<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn subscription_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WUnsubscribe id:34
 ***********************************************/
pub struct MsgUnsubscribe<'a>(pub Message<'a>);

impl <'a> MsgUnsubscribe<'a> {
    pub fn new (buf:&'a [u8]) -> MsgUnsubscribe<'a> {
        let decoder = Decoder::new(&buf);
        MsgUnsubscribe(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgUnsubscribe<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn subscription_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WUnsubscribed id:35
 ***********************************************/
pub struct MsgUnsubscribed<'a>(pub Message<'a>);

impl <'a> MsgUnsubscribed<'a> {
    pub fn new (buf:&'a [u8]) -> MsgUnsubscribed<'a> {
        let decoder = Decoder::new(&buf);
        MsgUnsubscribed(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgUnsubscribed<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WEvent id:36
 ***********************************************/
pub struct MsgEvent<'a>(pub Message<'a>);

impl <'a> MsgEvent<'a> {
    pub fn new (buf:&'a [u8]) -> MsgEvent<'a> {
        let decoder = Decoder::new(&buf);
        MsgEvent(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgEvent<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn subscription_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn publish_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[5]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WCall id:48
 ***********************************************/
pub struct MsgCall<'a>(pub Message<'a>);

impl <'a> MsgCall<'a> {
    pub fn new (buf:&'a [u8]) -> MsgCall<'a> {
        let decoder = Decoder::new(&buf);
        MsgCall(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgCall<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn options<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn procedure (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[5]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WResult id:50
 ***********************************************/
pub struct MsgResult<'a>(pub Message<'a>);

impl <'a> MsgResult<'a> {
    pub fn new (buf:&'a [u8]) -> MsgResult<'a> {
        let decoder = Decoder::new(&buf);
        MsgResult(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgResult<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WRegister id:64
 ***********************************************/
pub struct MsgRegister<'a>(pub Message<'a>);

impl <'a> MsgRegister<'a> {
    pub fn new (buf:&'a [u8]) -> MsgRegister<'a> {
        let decoder = Decoder::new(&buf);
        MsgRegister(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgRegister<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn procedure (&mut self) -> Result<&str, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WRegistered id:65
 ***********************************************/
pub struct MsgRegistered<'a>(pub Message<'a>);

impl <'a> MsgRegistered<'a> {
    pub fn new (buf:&'a [u8]) -> MsgRegistered<'a> {
        let decoder = Decoder::new(&buf);
        MsgRegistered(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgRegistered<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn registration_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WUnregister id:66
 ***********************************************/
pub struct MsgUnregister<'a>(pub Message<'a>);

impl <'a> MsgUnregister<'a> {
    pub fn new (buf:&'a [u8]) -> MsgUnregister<'a> {
        let decoder = Decoder::new(&buf);
        MsgUnregister(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgUnregister<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn registration_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WUnregistered id:67
 ***********************************************/
pub struct MsgUnregistered<'a>(pub Message<'a>);

impl <'a> MsgUnregistered<'a> {
    pub fn new (buf:&'a [u8]) -> MsgUnregistered<'a> {
        let decoder = Decoder::new(&buf);
        MsgUnregistered(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgUnregistered<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WInvocation id:68
 ***********************************************/
pub struct MsgInvocation<'a>(pub Message<'a>);

impl <'a> MsgInvocation<'a> {
    pub fn new (buf:&'a [u8]) -> MsgInvocation<'a> {
        let decoder = Decoder::new(&buf);
        MsgInvocation(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgInvocation<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn registration_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn details<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[5]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

/***********************************************
 * WYield id:70
 ***********************************************/
pub struct MsgYield<'a>(pub Message<'a>);

impl <'a> MsgYield<'a> {
    pub fn new (buf:&'a [u8]) -> MsgYield<'a> {
        let decoder = Decoder::new(&buf);
        MsgYield(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for MsgYield<'a> {
    fn code (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[0]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn request_id (&mut self) -> Result<u64, WampError> {
        self.0.decoder.set_position(self.0.index_positions[1]);
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn options<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[2]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn args<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[3]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
    fn kwargs<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
        self.0.decoder.set_position(self.0.index_positions[4]);
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
    }
}

pub enum Messages {
    MsgHello,
    MsgWelcome,
    MsgAbort,
    MsgChallenge,
    MsgAuthenticate,
    MsgGoodbye,
    MsgError,
    MsgPublish,
    MsgPublished,
    MsgSubscribe,
    MsgSubscribed,
    MsgUnsubscribe,
    MsgUnsubscribed,
    MsgEvent,
    MsgCall,
    MsgResult,
    MsgRegister,
    MsgRegistered,
    MsgUnregister,
    MsgUnregistered,
    MsgInvocation,
    MsgYield,
}



