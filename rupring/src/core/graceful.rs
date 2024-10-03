use std::sync::{atomic::AtomicBool, Arc};

#[derive(Debug, Clone)]
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

    pub fn register_hooks(&self) -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        {
            signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&self.sigterm))?;
            signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&self.sigint))?;
        }

        Ok(())
    }
}
