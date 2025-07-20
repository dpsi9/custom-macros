use std::fmt::Error;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
    fn deserialize(base: &[u8]) -> Result<Self, Error>;
}

// get the sum of all the fields of a struct
pub trait AddFields {
    fn add_fields(&self) -> i32;
}
