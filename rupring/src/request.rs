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

pub trait BindFromRequest {
    fn bind(&mut self, request: Request) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl UnwindSafe for Request {}

impl Request {
    pub fn get_provider<T: 'static>(&self) -> Option<&T> {
        return self.di_context.get::<T>();
    }
}

#[derive(Debug, Clone)]
pub struct QueryString(pub Vec<String>);

pub trait QueryStringDeserializer<T>: Sized {
    type Error;

    fn deserialize(&self) -> Result<T, Self::Error>;
}

impl<T> QueryStringDeserializer<Option<T>> for QueryString
where
    QueryString: QueryStringDeserializer<T>,
{
    type Error = ();

    fn deserialize(&self) -> Result<Option<T>, Self::Error> {
        let result = Self::deserialize(self);
        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

impl QueryStringDeserializer<i8> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<i8, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<i8>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i16> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<i16, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<i16>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i32> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<i32, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<i32>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i64> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<i64, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<i64>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i128> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<i128, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<i128>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<isize> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<isize, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<isize>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u8> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<u8, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<u8>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u16> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<u16, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<u16>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u32> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<u32, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<u32>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u64> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<u64, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<u64>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u128> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<u128, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<u128>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<usize> for QueryString {
    type Error = ();

    fn deserialize(&self) -> Result<usize, Self::Error> {
        if let Some(e) = self.0.get(0) {
            e.parse::<usize>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParamString(pub String);

pub trait ParamStringDeserializer<T>: Sized {
    type Error;

    fn deserialize(&self) -> Result<T, Self::Error>;
}

impl<T> ParamStringDeserializer<Option<T>> for ParamString
where
    ParamString: ParamStringDeserializer<T>,
{
    type Error = ();

    fn deserialize(&self) -> Result<Option<T>, Self::Error> {
        let result = Self::deserialize(self);
        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

impl ParamStringDeserializer<i8> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i8, Self::Error> {
        self.0.parse::<i8>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i16> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i16, Self::Error> {
        self.0.parse::<i16>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i32, Self::Error> {
        self.0.parse::<i32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i64, Self::Error> {
        self.0.parse::<i64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i128> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i128, Self::Error> {
        self.0.parse::<i128>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<isize> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<isize, Self::Error> {
        self.0.parse::<isize>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u8> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u8, Self::Error> {
        self.0.parse::<u8>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u16> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u16, Self::Error> {
        self.0.parse::<u16>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u32, Self::Error> {
        self.0.parse::<u32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u64, Self::Error> {
        self.0.parse::<u64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u128> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u128, Self::Error> {
        self.0.parse::<u128>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<usize> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<usize, Self::Error> {
        self.0.parse::<usize>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<f32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<f32, Self::Error> {
        self.0.parse::<f32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<f64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<f64, Self::Error> {
        self.0.parse::<f64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<bool> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<bool, Self::Error> {
        self.0.parse::<bool>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<String> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<String, Self::Error> {
        Ok(self.0.clone())
    }
}
