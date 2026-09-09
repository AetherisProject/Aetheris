#![allow(dead_code)]
use aetheris_core::ssh::SshClient;
use std::net::TcpStream;
use std::io::{Error, ErrorKind};

#[tokio::test]
async fn test_ssh_client() {
    let mut client = SshClient::new("localhost".to_string(), 22);
    
    // Test connection
    let stream = client.connect().expect("Failed to connect to SSH server");
    assert!(stream.is_ok(), "Connection should succeed");
    
    // Test command execution
    let output = client.execute("ls -la").expect("Failed to execute SSH command");
    assert!(output.contains("Executed: ls -la"), "Command output should match expected format");
    
    // Test channel
    let channel_result = client.spawn_channel();
    assert!(channel_result.is_ok(), "Channel spawn should succeed");
    
    let channel_output = client.receive_channel();
    assert!(channel_output.is_some(), "Channel should be ready");
    
    println!("SSH client tests passed!")
}