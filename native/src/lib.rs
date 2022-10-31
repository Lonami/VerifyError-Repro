#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::fmt;
use std::time::SystemTime;

include!(concat!(env!("OUT_DIR"), "/verifyerror.uniffi.rs"));

#[derive(Debug, Clone, Copy)]
pub enum NativeError {
    Initialization,
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialization => write!(f, "a resource could not be initialized correctly"),
        }
    }
}

impl std::error::Error for NativeError {}

#[derive(Debug, Clone, Copy)]
pub enum Formatting {
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TextFormat {
    format: Formatting,
    offset: i32,
    length: i32,
    extra: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    id: i32,
    sender: String,
    text: String,
    date: SystemTime,
    edit_date: Option<SystemTime>,
    formatting: Vec<TextFormat>,
}

pub fn get_message() -> Result<Message, NativeError> {
    todo!()
}
