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
{%- for am, col in all_mfields.items() %}
    {%- if col.col_type == "dict" %}
    fn {{col.name}}<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    {%-elif col.col_type == "list" %}
    fn {{col.name}}<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> { Err(WampError::InvalidField) }
    {%-else %}
    fn {{ am }}(&mut self) -> Result<{{col.col_type}}, WampError> { Err(WampError::InvalidField) }
    {%- endif %}
{%- endfor %}
}

{%for m in message_entries%}
/***********************************************
 * W{{m.name}} id:{{m.code.default}}
 ***********************************************/
pub struct Msg{{m.name}}<'a>(pub Message<'a>);

impl <'a> Msg{{m.name}}<'a> {
    pub fn new (buf:&'a [u8]) -> Msg{{m.name}}<'a> {
        let decoder = Decoder::new(&buf);
        Msg{{m.name}}(Message {
            decoder: Box::new(decoder),
            entries: 3,
            index_positions: Box::new(vec![1,2,3]),
        })
    }
}

impl <'a> MessageTrait<'a> for Msg{{m.name}}<'a> {
    {%- for col in m.mfields -%}
    {% if col.col_type == "dict" %}
    fn {{col.name}}<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
    {%- elif col.col_type == "list" %}
    fn {{col.name}}<T:minicbor::Decode<'a, ()>>(&mut self) -> Result<T, WampError> {
    {%- else %}
    fn {{col.name}} (&mut self) -> Result<{{col.col_type}}, WampError> {
    {%- endif %}
        self.0.decoder.set_position(self.0.index_positions[{{col.index}}]);
        {%- if col.col_type == "u64" %}
        self.0.decoder.u64().or_else(|_| Err(WampError::IncorrectElementType))
        {%- elif col.col_type == "&str" %}
        self.0.decoder.str().or_else(|_| Err(WampError::IncorrectElementType))
        {%- else %}
        self.0.decoder.clone().decode().or_else(|_| Err(WampError::IncorrectElementType))
        {%- endif %}
    }
    {%- endfor %}
}
{% endfor %}
pub enum Messages {
{%-for m in message_entries%}
    Msg{{m.name}},
{%-endfor%}
}




