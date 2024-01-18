use std::{collections::HashMap, panic::UnwindSafe, sync::Arc};

use crate::Method;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
    pub(crate) di_context: Arc<crate::DIContext>,
}

impl UnwindSafe for Request {}

impl Request {
    pub fn get_provider<T: 'static>(&self) -> Option<&T> {
        return self.di_context.get::<T>();
    }
}

#[derive(Debug, Clone)]
pub struct ParamString(String);

impl TryFrom<ParamString> for i8 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<i8>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for i16 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<i16>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for i32 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<i32>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for i64 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<i64>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for i128 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<i128>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for isize {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<isize>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for u8 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<u8>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for u16 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<u16>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for u32 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<u32>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for u64 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<u64>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for u128 {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<u128>().map_err(|_| ())
    }
}

impl TryFrom<ParamString> for usize {
    type Error = ();

    fn try_from(value: ParamString) -> Result<Self, Self::Error> {
        value.0.parse::<usize>().map_err(|_| ())
    }
}
