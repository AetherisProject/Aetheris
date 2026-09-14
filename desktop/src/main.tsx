import React, { useState } from 'react';
import ReactDOM from 'react-dom/client';
import Dashboard from './pages/Dashboard';
import ApiKeys from './pages/ApiKeys';
import SSH from './pages/SSH';
import Vault from './pages/Vault';

type Page = 'dashboard' | 'apikeys' | 'ssh' | 'vault';

const navItems: { id: Page; label: string; icon: string }[] = [
  { id: 'dashboard', label: 'Dashboard', icon: '🏠' },
  { id: 'apikeys', label: 'API Keys', icon: '🔑' },
  { id: 'ssh', label: 'SSH', icon: '💻' },
  { id: 'vault', label: 'Vault', icon: '🔒' },
];

export default function App() {
  const [page, setPage] = useState<Page>('dashboard');

  return (
    <div style={{ display: 'flex', height: '100vh', fontFamily: "'Inter', 'Plus Jakarta Sans', sans-serif", background: '#090D16', color: '#E2E8F0' }}>
      {/* Sidebar */}
      <div style={{ width: 200, background: '#0F172A', borderRight: '1px solid rgba(148,163,184,0.1)', padding: 16, display: 'flex', flexDirection: 'column', gap: 4 }}>
        <div style={{ padding: '8px 12px', marginBottom: 16 }}>
          <h1 style={{ margin: 0, fontSize: 18, fontWeight: 700, letterSpacing: '-0.02em' }}>Aetheris</h1>
          <div style={{ fontSize: 11, color: '#94A3B8' }}>Secrets Manager</div>
        </div>
        {navItems.map(item => (
          <button
            key={item.id}
            onClick={() => setPage(item.id)}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 10,
              padding: '10px 12px',
              borderRadius: 8,
              border: 'none',
              background: page === item.id ? 'rgba(99,102,241,0.15)' : 'transparent',
              color: page === item.id ? '#6366F1' : '#CBD5E1',
              cursor: 'pointer',
              fontSize: 13,
              fontFamily: 'inherit',
              fontWeight: page === item.id ? 600 : 400,
              textAlign: 'left',
            }}
          >
            <span>{item.icon}</span>
            <span>{item.label}</span>
          </button>
        ))}
      </div>

      {/* Main content */}
      <div style={{ flex: 1, overflow: 'auto' }}>
        {page === 'dashboard' && <Dashboard />}
        {page === 'apikeys' && <ApiKeys />}
        {page === 'ssh' && <SSH />}
        {page === 'vault' && <Vault />}
      </div>
    </div>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root')!);
root.render(<App />);
