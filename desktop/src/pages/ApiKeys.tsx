import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface KeyFinding {
  var_name: string;
  provider: string;
  confidence: string;
  scope: string;
}

interface VaultItem {
  name: string;
  value: string;
  scope: string;
  created_at: number;
  updated_at: number;
}

interface Snapshot {
  id: string;
  label: string;
  created_at: number;
  item_count: number;
}

const PROVIDERS = [
  { id: 'openai', name: 'OpenAI', color: '#10B981', icon: '⚡' },
  { id: 'anthropic', name: 'Anthropic', color: '#A855F7', icon: '🧠' },
  { id: 'github', name: 'GitHub', color: '#6366F1', icon: '🐙' },
  { id: 'aws', name: 'AWS', color: '#F59E0B', icon: '☁️' },
  { id: 'stripe', name: 'Stripe', color: '#06B6D4', icon: '💳' },
  { id: 'google', name: 'Google', color: '#EF4444', icon: '🔍' },
  { id: 'azure', name: 'Azure', color: '#0EA5E9', icon: '🔷' },
  { id: 'custom', name: 'Custom', color: '#94A3B8', icon: '🔑' },
];

const styles: Record<string, React.CSSProperties> = {
  container: { padding: 24, background: '#090D16', minHeight: '100vh', fontFamily: "'Inter', 'Plus Jakarta Sans', sans-serif", color: '#E2E8F0' },
  header: { display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 24 },
  title: { fontWeight: 700, fontSize: 22, letterSpacing: '-0.02em', margin: 0 },
  subtitle: { color: '#94A3B8', fontSize: 13, marginTop: 4 },
  grid: { display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))', gap: 16 },
  card: { background: 'linear-gradient(180deg, rgba(30,41,59,.82), rgba(15,23,42,.95))', border: '1px solid rgba(148,163,184,0.16)', borderRadius: 14, padding: 16 },
  providerHeader: { display: 'flex', alignItems: 'center', gap: 10, marginBottom: 12 },
  providerIcon: { width: 32, height: 32, borderRadius: 9, display: 'grid', placeItems: 'center', fontSize: 14, fontWeight: 700, color: '#fff' },
  providerName: { fontWeight: 600, fontSize: 14 },
  keyItem: { display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '8px 0', borderBottom: '1px solid rgba(148,163,184,0.08)' },
  keyName: { fontSize: 13, fontFamily: "'JetBrains Mono', monospace", color: '#CBD5E1' },
  keyActions: { display: 'flex', gap: 8 },
  empty: { color: '#94A3B8', fontSize: 13, textAlign: 'center', padding: 24 },
  btn: { border: '1px solid rgba(148,163,184,0.16)', background: '#1E293B', color: '#E2E8F0', padding: '6px 12px', borderRadius: 8, cursor: 'pointer', fontSize: 12, fontFamily: 'inherit' },
  btnPrimary: { background: 'linear-gradient(135deg, #6366F1, #A855F7)', border: 'none', fontWeight: 600 },
  btnGhost: { background: 'transparent' },
  btnDanger: { borderColor: '#EF4444', color: '#EF4444' },
  sectionTitle: { fontSize: 16, fontWeight: 600, marginBottom: 16 },
  toggleSwitch: { position: 'relative', width: 36, height: 20, background: '#334155', borderRadius: 10, cursor: 'pointer', transition: 'background 0.2s' },
  toggleActive: { background: '#10B981' },
  toggleKnob: { position: 'absolute', top: 2, left: 2, width: 16, height: 16, background: '#fff', borderRadius: 50, transition: 'transform 0.2s' },
  badge: { display: 'inline-block', fontSize: 10, padding: '2px 6px', borderRadius: 99, fontWeight: 600 },
  badgeHigh: { background: 'rgba(239,68,68,.15)', color: '#EF4444', border: '1px solid rgba(239,68,68,.35)' },
  badgeMedium: { background: 'rgba(245,158,11,.12)', color: '#F59E0B', border: '1px solid rgba(245,158,11,.35)' },
  badgeLow: { background: 'rgba(6,182,212,.12)', color: '#06B6D4', border: '1px solid rgba(6,182,212,.35)' },
  monitoringBanner: { background: 'rgba(99,102,241,.1)', border: '1px solid rgba(99,102,241,.3)', borderRadius: 10, padding: 12, marginBottom: 24, display: 'flex', alignItems: 'center', gap: 12 },
  monitoringDot: { width: 8, height: 8, borderRadius: 50, background: '#10B981', animation: 'pulse 2s infinite' },
  snapshotBar: { display: 'flex', alignItems: 'center', gap: 8, marginTop: 16, paddingTop: 16, borderTop: '1px solid rgba(148,163,184,0.08)' },
  input: { width: '100%', padding: '8px 12px', borderRadius: 8, fontFamily: 'inherit', fontSize: 13, border: '1px solid rgba(148,163,184,0.16)', background: '#0F172A', color: '#E2E8F0' },
  modalOverlay: { position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.7)', display: 'grid', placeItems: 'center', zIndex: 100 },
  modal: { background: '#1E293B', border: '1px solid rgba(148,163,184,0.16)', borderRadius: 14, padding: 24, width: 400, maxWidth: '90vw' },
};

export default function ApiKeysScreen() {
  const [password, setPassword] = useState('');
  const [authenticated, setAuthenticated] = useState(false);
  const [findings, setFindings] = useState<KeyFinding[]>([]);
  const [vaultItems, setVaultItems] = useState<VaultItem[]>([]);
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loading, setLoading] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [monitoring, setMonitoring] = useState(false);
  const [toast, setToast] = useState('');
  const [showAddModal, setShowAddModal] = useState(false);
  const [showSnapshotModal, setShowSnapshotModal] = useState(false);
  const [newKeyName, setNewKeyName] = useState('');
  const [newKeyValue, setNewKeyValue] = useState('');
  const [snapshotLabel, setSnapshotLabel] = useState('');
  const [activeProviders, setActiveProviders] = useState<Record<string, boolean>>({});

  const showToast = useCallback((msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(''), 3000);
  }, []);

  const loadVault = useCallback(async () => {
    if (!password) return;
    setLoading(true);
    try {
      const items = await invoke<VaultItem[]>('vault_list', { password });
      setVaultItems(items || []);
      const snaps = await invoke<Snapshot[]>('vault_snapshots', { password });
      setSnapshots(snaps || []);
    } catch (e) {
      showToast(`Vault error: ${e}`);
    }
    setLoading(false);
  }, [password, showToast]);

  const handleLogin = async () => {
    if (!password) { showToast('Enter master password'); return; }
    setAuthenticated(true);
    await loadVault();
  };

  const handleScan = async () => {
    setScanning(true);
    try {
      const result = await invoke<KeyFinding[]>('env_scan', { password });
      setFindings(result || []);
      showToast(`Found ${(result || []).length} potential API keys`);
    } catch (e) {
      showToast(`Scan error: ${e}`);
    }
    setScanning(false);
  };

  const handleSync = async () => {
    setLoading(true);
    try {
      const count = await invoke<number>('env_sync', { password });
      showToast(`Synced ${count} keys to vault`);
      await loadVault();
    } catch (e) {
      showToast(`Sync error: ${e}`);
    }
    setLoading(false);
  };

  const handleAddKey = async () => {
    if (!newKeyName || !newKeyValue) return;
    try {
      await invoke('vault_add', { password, name: newKeyName, value: newKeyValue });
      showToast(`Added ${newKeyName}`);
      setShowAddModal(false);
      setNewKeyName('');
      setNewKeyValue('');
      await loadVault();
    } catch (e) {
      showToast(`Add error: ${e}`);
    }
  };

  const handleDeleteKey = async (name: string) => {
    if (!confirm(`Delete ${name}?`)) return;
    try {
      await invoke('vault_delete', { password, name });
      showToast(`Deleted ${name}`);
      await loadVault();
    } catch (e) {
      showToast(`Delete error: ${e}`);
    }
  };

  const handleSnapshot = async () => {
    if (!snapshotLabel) return;
    try {
      await invoke('vault_snapshot', { password, label: snapshotLabel });
      showToast(`Snapshot "${snapshotLabel}" created`);
      setSnapshotLabel('');
      setShowSnapshotModal(false);
      await loadVault();
    } catch (e) {
      showToast(`Snapshot error: ${e}`);
    }
  };

  const handleRollback = async (id: string) => {
    if (!confirm('Rollback? Current state will be replaced.')) return;
    try {
      await invoke('vault_rollback', { password, snapshotId: id });
      showToast('Rolled back');
      await loadVault();
    } catch (e) {
      showToast(`Rollback error: ${e}`);
    }
  };

  const toggleProvider = (id: string) => {
    setActiveProviders(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const findingsByProvider = PROVIDERS.map(p => ({
    ...p,
    keys: findings.filter(f => f.provider === p.id),
  }));

  if (!authenticated) {
    return (
      <div style={styles.container}>
        <div style={styles.modalOverlay}>
          <div style={styles.modal}>
            <h2 style={{ margin: 0, marginBottom: 8, color: '#F8FAFC' }}>🔑 API Keys Vault</h2>
            <p style={{ color: '#94A3B8', fontSize: 13, marginBottom: 16 }}>Enter master password to unlock</p>
            <input
              style={styles.input}
              type="password"
              placeholder="Master password"
              value={password}
              onChange={e => setPassword(e.target.value)}
              onKeyDown={e => e.key === 'Enter' && handleLogin()}
            />
            <button style={{ ...styles.btn, ...styles.btnPrimary, width: '100%', marginTop: 12, padding: 10 }} onClick={handleLogin}>
              Unlock
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div style={styles.container}>
      <style>{`
        @keyframes pulse { 0%,100% { opacity: 1; } 50% { opacity: 0.4; } }
      `}</style>
      
      {/* Header */}
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>🔑 API Keys Manager</h1>
          <p style={styles.subtitle}>8 providers • EnvStudio-style control</p>
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          <button style={{ ...styles.btn, ...(scanning ? styles.btnPrimary : {}) }} onClick={handleScan} disabled={scanning}>
            {scanning ? 'Scanning...' : '🔍 Scan'}
          </button>
          <button style={{ ...styles.btn, ...styles.btnPrimary }} onClick={handleSync} disabled={loading}>
            {loading ? '...' : '⬆️ Sync to Vault'}
          </button>
          <button style={styles.btn} onClick={() => setShowAddModal(true)}>➕ Add</button>
          <button style={styles.btn} onClick={() => setShowSnapshotModal(true)}>📸 Snapshot</button>
        </div>
      </div>

      {/* Monitoring banner */}
      <div style={styles.monitoringBanner}>
        <div style={styles.monitoringDot} />
        <span style={{ fontSize: 13, fontWeight: 600 }}>
          {monitoring ? 'Live monitoring active' : 'Real-time env monitoring'}
        </span>
        <button style={{ ...styles.btn, marginLeft: 'auto' }} onClick={() => setMonitoring(!monitoring)}>
          {monitoring ? '⏸ Pause' : '▶ Start'}
        </button>
      </div>

      {/* Provider cards grid */}
      <div style={styles.grid}>
        {findingsByProvider.map(provider => (
          <div key={provider.id} style={styles.card}>
            <div style={styles.providerHeader}>
              <div style={{ ...styles.providerIcon, background: `linear-gradient(135deg, ${provider.color}, ${provider.color}88)` }}>
                {provider.icon}
              </div>
              <div style={{ flex: 1 }}>
                <div style={styles.providerName}>{provider.name}</div>
                <div style={{ fontSize: 11, color: '#94A3B8' }}>{provider.keys.length} key(s) detected</div>
              </div>
              <div
                style={{ ...styles.toggleSwitch, ...(activeProviders[provider.id] ? styles.toggleActive : {}) }}
                onClick={() => toggleProvider(provider.id)}
              >
                <div style={{ ...styles.toggleKnob, transform: activeProviders[provider.id] ? 'translateX(16px)' : 'translateX(0)' }} />
              </div>
            </div>

            {provider.keys.length > 0 ? (
              provider.keys.map(key => (
                <div key={key.var_name} style={styles.keyItem}>
                  <div>
                    <div style={styles.keyName}>{key.var_name}</div>
                    <span style={{ ...styles.badge, ...(key.confidence === 'High' ? styles.badgeHigh : key.confidence === 'Medium' ? styles.badgeMedium : styles.badgeLow) }}>
                      {key.confidence}
                    </span>
                  </div>
                  <div style={styles.keyActions}>
                    <button style={{ ...styles.btn, ...styles.btnPrimary }} onClick={() => handleSync}>Sync</button>
                    <button style={{ ...styles.btn, ...styles.btnGhost }}>Hide</button>
                  </div>
                </div>
              ))
            ) : (
              <div style={styles.empty}>No keys detected</div>
            )}
          </div>
        ))}
      </div>

      {/* Vault items section */}
      <div style={{ ...styles.card, marginTop: 24 }}>
        <h3 style={{ ...styles.sectionTitle, marginTop: 0 }}>Vault ({vaultItems.length} items)</h3>
        {vaultItems.length > 0 ? (
          vaultItems.map(item => (
            <div key={item.name} style={styles.keyItem}>
              <div>
                <div style={styles.keyName}>{item.name}</div>
                <div style={{ fontSize: 11, color: '#94A3B8' }}>{'•'.repeat(12)} • {item.scope}</div>
              </div>
              <div style={styles.keyActions}>
                <button style={{ ...styles.btn, ...styles.btnDanger }} onClick={() => handleDeleteKey(item.name)}>Delete</button>
              </div>
            </div>
          ))
        ) : (
          <div style={styles.empty}>Vault is empty. Scan and sync to add keys.</div>
        )}

        {/* Snapshots */}
        <div style={styles.snapshotBar}>
          <span style={{ fontSize: 12, color: '#94A3B8' }}>📸 {snapshots.length} snapshot(s)</span>
        </div>
        {snapshots.map(snap => (
          <div key={snap.id} style={styles.keyItem}>
            <div>
              <div style={styles.keyName}>{snap.label}</div>
              <div style={{ fontSize: 11, color: '#94A3B8' }}>{snap.item_count} items • {new Date(snap.created_at * 1000).toLocaleString()}</div>
            </div>
            <button style={{ ...styles.btn, ...styles.btnGhost }} onClick={() => handleRollback(snap.id)}>Rollback</button>
          </div>
        ))}
      </div>

      {/* Add key modal */}
      {showAddModal && (
        <div style={styles.modalOverlay} onClick={() => setShowAddModal(false)}>
          <div style={styles.modal} onClick={e => e.stopPropagation()}>
            <h3 style={{ margin: 0, marginBottom: 16, color: '#F8FAFC' }}>Add API Key</h3>
            <input style={styles.input} placeholder="Key name (e.g. OPENAI_API_KEY)" value={newKeyName} onChange={e => setNewKeyName(e.target.value)} />
            <input style={{ ...styles.input, marginTop: 8 }} type="password" placeholder="Key value" value={newKeyValue} onChange={e => setNewKeyValue(e.target.value)} />
            <div style={{ display: 'flex', gap: 8, marginTop: 16, justifyContent: 'flex-end' }}>
              <button style={{ ...styles.btn, ...styles.btnGhost }} onClick={() => setShowAddModal(false)}>Cancel</button>
              <button style={{ ...styles.btn, ...styles.btnPrimary }} onClick={handleAddKey}>Save</button>
            </div>
          </div>
        </div>
      )}

      {/* Snapshot modal */}
      {showSnapshotModal && (
        <div style={styles.modalOverlay} onClick={() => setShowSnapshotModal(false)}>
          <div style={styles.modal} onClick={e => e.stopPropagation()}>
            <h3 style={{ margin: 0, marginBottom: 16, color: '#F8FAFC' }}>Create Snapshot</h3>
            <input style={styles.input} placeholder="Snapshot label (e.g. before-changes)" value={snapshotLabel} onChange={e => setSnapshotLabel(e.target.value)} />
            <div style={{ display: 'flex', gap: 8, marginTop: 16, justifyContent: 'flex-end' }}>
              <button style={{ ...styles.btn, ...styles.btnGhost }} onClick={() => setShowSnapshotModal(false)}>Cancel</button>
              <button style={{ ...styles.btn, ...styles.btnPrimary }} onClick={handleSnapshot}>Create</button>
            </div>
          </div>
        </div>
      )}

      {/* Toast */}
      {toast && (
        <div style={{ position: 'fixed', bottom: 24, left: '50%', transform: 'translateX(-50%)', background: '#1E293B', border: '1px solid #10B981', color: '#10B981', padding: '9px 18px', borderRadius: 99, fontSize: 13, zIndex: 99 }}>
          {toast}
        </div>
      )}
    </div>
  );
}
