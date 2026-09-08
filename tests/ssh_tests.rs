//! Comprehensive SSH Subsystem Integration Tests for Aetheris.

use aetheris::ssh::{
    probe_ssh_endpoint, ForwardType, HealthStatus, KeyType, PortForward, SshClient, SshConfig,
    SshKeypair,
};
use std::time::Duration;

#[test]
fn test_ed25519_keypair_generation_and_fingerprint() {
    let keypair = SshKeypair::generate_ed25519(Some("operator@aetheris-mesh"));
    assert_eq!(keypair.key_type, KeyType::Ed25519);
    assert_eq!(keypair.private_key_bytes.len(), 32);
    assert_eq!(keypair.public_key_bytes.len(), 32);

    let pubkey_str = keypair.to_openssh_public_key().expect("export OpenSSH key");
    assert!(pubkey_str.starts_with("ssh-ed25519 "));
    assert!(pubkey_str.ends_with(" operator@aetheris-mesh"));

    let fp = keypair.fingerprint_sha256();
    assert!(fp.starts_with("SHA256:"));
}

#[tokio::test]
async fn test_ssh_client_authenticated_session() {
    let config = SshConfig {
        host: "prod.infra.internal".into(),
        port: 2222,
        username: "root".into(),
        password: Some("super-secret-vault-pass".into()),
        ..Default::default()
    };

    let client = SshClient::with_config(config);
    let mut session = client.connect().await.expect("connect succeeds");

    assert!(session.is_connected());
    assert_eq!(session.host, "prod.infra.internal");
    assert_eq!(session.port, 2222);
    assert_eq!(session.username, "root");

    // Verify secret env injection in memory
    session.inject_secret_env("OPENAI_API_KEY", "sk-proj-test-12345");
    assert_eq!(
        session
            .injected_env
            .get("OPENAI_API_KEY")
            .map(String::as_str),
        Some("sk-proj-test-12345")
    );

    // Verify telemetry
    session.record_transfer(4096, 8192);
    let metrics = session.metrics();
    assert_eq!(metrics.bytes_sent, 4096);
    assert_eq!(metrics.bytes_received, 8192);

    // Verify close
    session.close().expect("clean close");
    assert!(!session.is_connected());
    assert!(session.injected_env.is_empty());
}

#[test]
fn test_port_forwarding_lifecycle() {
    let mut fwd = PortForward::local(5432, "postgres.internal", 5432);
    assert_eq!(fwd.forward_type, ForwardType::Local);
    assert_eq!(fwd.bind_port, 5432);
    assert_eq!(fwd.target_host.as_deref(), Some("postgres.internal"));
    assert!(!fwd.is_active);

    fwd.activate();
    assert!(fwd.is_active);
    fwd.deactivate();
    assert!(!fwd.is_active);
}

#[tokio::test]
async fn test_ssh_health_prober() {
    // Probe localhost on closed port with short timeout
    let report = probe_ssh_endpoint("127.0.0.1", 54321, Duration::from_millis(50)).await;
    assert_eq!(report.host, "127.0.0.1");
    assert_eq!(report.port, 54321);
    assert_eq!(report.status, HealthStatus::Unreachable);
}
