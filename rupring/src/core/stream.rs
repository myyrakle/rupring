use std::{
    convert::Infallible,
    sync::{atomic::AtomicBool, Arc},
};

use bytes::Bytes;
use hyper::body::Frame;

pub type StreamChannelType = Result<Frame<Bytes>, Infallible>;

pub struct StreamHandler {
    sender: tokio::sync::mpsc::UnboundedSender<StreamChannelType>,
    closed: Arc<AtomicBool>,
}

impl StreamHandler {
    pub fn new(
        sender: tokio::sync::mpsc::UnboundedSender<StreamChannelType>,
        closed: Arc<AtomicBool>,
    ) -> Self {
        Self { sender, closed }
    }

    pub async fn send(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.closed.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("Stream is closed".into());
        }

        let bytes = Bytes::copy_from_slice(_data);
        let frame = Frame::data(bytes);
        self.sender.send(Ok(frame))?;

        Ok(())
    }
}
