//! SSH client (minimal stub — real implementation needed for SSH terminal).

pub struct SshClient {
    host: String,
    port: u16,
}

impl SshClient {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        Err("SSH not implemented in this build".into())
    }

    pub fn spawn_channel(&mut self) -> Result<(), String> {
        Err("SSH not implemented in this build".into())
    }

    pub fn execute(&mut self, command: &str) -> Result<String, String> {
        let _ = command;
        Err("SSH not implemented in this build".into())
    }
}
