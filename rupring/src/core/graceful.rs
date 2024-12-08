use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use log::Level;

use crate::{application_properties, logger::print_system_log};

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

pub fn handle_graceful_shutdown(
    application_properties: &application_properties::ApplicationProperties,
    service_avaliable: Arc<AtomicBool>,
    running_task_count: Arc<AtomicU64>,
) {
    let signal_flags = SignalFlags::new();
    let shutdown_timeout_duration = application_properties.server.shutdown_timeout_duration();

    if let Err(error) = signal_flags.register_hooks() {
        print_system_log(
            Level::Error,
            format!("Error registering signal hooks: {:?}", error).as_str(),
        );
    } else {
        print_system_log(Level::Info, "Graceful shutdown enabled");

        let service_avaliable = Arc::clone(&service_avaliable);
        let running_task_count = Arc::clone(&running_task_count);
        tokio::spawn(async move {
            let sigterm = Arc::clone(&signal_flags.sigterm);
            let sigint = Arc::clone(&signal_flags.sigint);

            loop {
                if sigterm.load(std::sync::atomic::Ordering::Relaxed) {
                    print_system_log(
                        Level::Info,
                        "SIGTERM received. Try to shutdown gracefully...",
                    );
                    service_avaliable.store(false, std::sync::atomic::Ordering::Release);
                    break;
                }

                if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                    print_system_log(
                        Level::Info,
                        "SIGINT received. Try to shutdown gracefully...",
                    );
                    service_avaliable.store(false, std::sync::atomic::Ordering::Release);
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }

            let shutdown_request_time = std::time::Instant::now();

            loop {
                if running_task_count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    print_system_log(Level::Info, "All tasks are done. Shutting down...");
                    std::process::exit(0);
                }

                // timeout 지나면 강제로 종료
                let now = std::time::Instant::now();
                if now.duration_since(shutdown_request_time) >= shutdown_timeout_duration {
                    print_system_log(Level::Info, "Shutdown timeout reached. Forcing shutdown...");
                    std::process::exit(0);
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });
    }
}
