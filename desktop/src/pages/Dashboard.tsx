import React from 'react';
import { Card, Button } from './AetherisComponents';

export default function Dashboard() {
  return (
    <div style={{ padding: 24, background: '#090D16', minHeight: '100vh', fontFamily: "'Plus Jakarta Sans', sans-serif" }}>
      <h1 style={{ color: '#F8FAFC', fontWeight: 700, letterSpacing: '-0.02em' }}>Aetheris Dashboard</h1>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))', gap: 16, marginTop: 24 }}>
        <Card title="Vault"><p>12 items • 3 secure categories</p></Card>
        <Card title="SSH"><p>3 active sessions • 1 tunnel open</p></Card>
        <Card title="API Keys"><p>8 providers • Healthy • 3 active</p></Card>
      </div>
      <div style={{ marginTop: 24 }}>
        <Button label="Open Vault" variant="primary" />
        <Button label="SSH Terminal" style={{ marginLeft: 8 }} />
      </div>
    </div>
  );
}
