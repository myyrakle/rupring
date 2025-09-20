#![allow(dead_code)]
pub struct StreamHandler {
    unbounded_tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
}

impl StreamHandler {
    // pub fn new() -> Self {
    //     Self {}
    // }

    pub async fn send(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
