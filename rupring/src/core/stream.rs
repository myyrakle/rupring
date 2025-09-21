use std::{
    convert::Infallible,
    sync::{atomic::AtomicBool, Arc},
};

use bytes::Bytes;
use hyper::body::Frame;

use crate::http::sse::Event;

pub type StreamChannelType = Result<Frame<Bytes>, Infallible>;

/// Handler for managing streams.
#[derive(Debug, Clone)]
pub struct StreamHandler {
    sender: tokio::sync::mpsc::UnboundedSender<StreamChannelType>,
    closed: Arc<AtomicBool>,
}

impl StreamHandler {
    pub(crate) fn new(
        sender: tokio::sync::mpsc::UnboundedSender<StreamChannelType>,
        closed: Arc<AtomicBool>,
    ) -> Self {
        Self { sender, closed }
    }

    /// Send bytes to the stream.
    /// Returns an error if the stream is closed or if sending fails.
    pub async fn send_bytes(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.closed.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("Stream is closed".into());
        }

        let bytes = Bytes::copy_from_slice(bytes);
        let frame = Frame::data(bytes);
        self.sender.send(Ok(frame))?;

        Ok(())
    }

    /// Send an SSE event to the stream.
    /// Returns an error if the stream is closed or if sending fails.
    /// The event is built using the `Event` struct from the `sse` module.
    pub async fn send_event(&self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let sse_data = event.build();
        self.send_bytes(sse_data.as_bytes()).await
    }

    /// Check if the stream is closed.
    /// This can be used to stop sending data and stop tasks when the client disconnects.
    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
