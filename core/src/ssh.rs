#![allow(dead_code)]
use std::net::TcpStream;
use std::io::{Error, ErrorKind, Write, Result as IoResult};
use std::sync::mpsc;

pub struct SshClient {
    host: String,
    port: u16,
    channel: Option<mpsc::Sender<String>>,
}

impl SshClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            channel: None,
        }
    }
    
    pub fn connect(&mut self) -> IoResult<TcpStream> {
        let addr = format!("{}:{}", self.host, self.port);
        TcpStream::connect(&addr)
    }
    
    pub fn execute(&mut self, command: &str) -> Result<String, String> {
        let mut stream = self.connect()?;
        let output = format!("Executed: {}\
Output: {}", command, command.to_uppercase());
        Ok(output)
    }
    
    pub fn spawn_channel(&mut self) -> Result<(), String> {
        let (sender, _) = mpsc::channel();
        self.channel = Some(sender);
        Ok(())
    }
    
    pub fn receive_channel(&self) -> Option<String> {
        self.channel.as_ref().map(|c| "SSH Channel Ready".to_string())
    }
}