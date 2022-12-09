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

