//! Real-time Windows registry change monitor.
//!
//! Uses `RegNotifyChangeKeyValue` to detect when environment variables are
//! modified by other processes (e.g., the user editing System Properties,
//! another tool setting env vars). Triggers a callback when changes occur.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::error::AetherisError;

/// Callback type for env change notifications.
pub type ChangeCallback = Arc<dyn Fn() + Send + Sync>;

/// Monitors the Windows registry for environment variable changes.
pub struct EnvMonitor {
    running: Arc<AtomicBool>,
    callback: ChangeCallback,
    scope: String,
}

impl EnvMonitor {
    /// Creates a new monitor with the given callback.
    pub fn new(callback: ChangeCallback, scope: impl Into<String>) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            callback,
            scope: scope.into(),
        }
    }

    /// Starts monitoring in a background thread.
    ///
    /// The callback is invoked whenever a registry change is detected.
    pub fn start(&self) -> Result<(), AetherisError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(AetherisError::VaultError("Monitor already running".into()));
        }

        let running = self.running.clone();
        let callback = self.callback.clone();
        let scope = self.scope.clone();

        thread::spawn(move || {
            #[cfg(windows)]
            {
                use windows::Win32::System::Registry::{
                    RegOpenKeyExW, RegNotifyChangeKeyValue, HKEY, HKEY_CURRENT_USER,
                    HKEY_LOCAL_MACHINE, KEY_NOTIFY, REG_NOTIFY_CHANGE_NAME,
                    REG_NOTIFY_CHANGE_LAST_SET,
                };

                let (subkey, hkey): (&str, HKEY) = match scope.as_str() {
                    "user" | "User" => ("Environment", HKEY_CURRENT_USER),
                    "system" | "System" => (
                        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
                        HKEY_LOCAL_MACHINE,
                    ),
                    _ => return,
                };

                let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
                let mut hkey_out = HKEY::default();
                unsafe {
                    if RegOpenKeyExW(
                        hkey,
                        windows::core::PCWSTR(subkey_wide.as_ptr()),
                        0,
                        KEY_NOTIFY,
                        &mut hkey_out,
                    )
                    .is_err()
                    {
                        running.store(false, Ordering::SeqCst);
                        return;
                    }
                }

                while running.load(Ordering::SeqCst) {
                    let result = unsafe {
                        RegNotifyChangeKeyValue(
                            hkey_out,
                            true,
                            REG_NOTIFY_CHANGE_NAME | REG_NOTIFY_CHANGE_LAST_SET,
                            None,
                            false,
                        )
                    };
                    if result.is_ok() {
                        callback();
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
            #[cfg(not(windows))]
            {
                let _ = scope;
                while running.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });

        Ok(())
    }

    /// Stops the monitor.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Returns true if the monitor is currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for EnvMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_monitor_is_not_running() {
        let callback: ChangeCallback = Arc::new(|| {});
        let monitor = EnvMonitor::new(callback, "user");
        assert!(!monitor.is_running());
    }

    #[test]
    fn stop_sets_not_running() {
        let callback: ChangeCallback = Arc::new(|| {});
        let monitor = EnvMonitor::new(callback, "user");
        monitor.stop();
        assert!(!monitor.is_running());
    }
}
