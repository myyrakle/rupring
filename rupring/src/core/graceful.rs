use std::sync::{atomic::AtomicBool, Arc};

pub struct SignalFlags {
    pub sigterm: Arc<AtomicBool>,
    pub sigint: Arc<AtomicBool>,
}

impl SignalFlags {
    pub fn new() -> Self {
        Self {
            sigterm: Arc::new(AtomicBool::new(false)),
            sigint: Arc::new(AtomicBool::new(false)),
        }
    }
}
