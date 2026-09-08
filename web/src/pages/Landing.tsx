import React from 'react';
import { colors, spacing, typography, borderRadius, shadow } from '../components/AetherisComponents';

export default function LandingPage() {
  return (
    <div style={{ background: colors.bg, color: colors.textPrimary, fontFamily: typography.family, minHeight: '100vh', padding: spacing.xlarge }}>
      {/* Hero */}
      <section style={{ textAlign: 'center', padding: `${spacing.xlarge}px 0 ` }}>
        <h1 style={{ fontSize: 48, fontWeight: 800, letterSpacing: '-0.03em', color: colors.primary, marginBottom: spacing.medium }}>
          Aetheris
        </h1>
        <p style={{ fontSize: 20, color: colors.textSecondary, maxWidth: 640, margin: '0 auto' }}>
          Zero-knowledge vault. SSH terminal. API key manager. One unified secrets OS.
        </p>
        <div style={{ marginTop: spacing.large }}>
          <a href="#features" style={{ display: 'inline-block', padding: `${spacing.medium}px ${spacing.large}px`, background: colors.primary, color: colors.bg, borderRadius: borderRadius.rounded, textDecoration: 'none', fontWeight: 600 }}>Explore Features</a>
        </div>
      </section>

      {/* Vision */}
      <section id="vision" style={{ maxWidth: 960, margin: '0 auto', padding:`${spacing.xlarge}px 0 ` }}>
        <h2 style={{ fontSize: 32, fontWeight: 700, color: colors.accent, marginBottom: spacing.medium }}>The Vision</h2>
        <p style={{ color: colors.textSecondary, lineHeight: 1.7, fontSize: 16 }}>
          Aetheris eliminates the "re-setup" nightmare. Your vault, SSH sessions, and API keys live in a client-side zero-knowledge architecture — master passwords never leave CPU registers, keys never touch disk unencrypted. Design tokens from the Rust core guarantee consistency across CLI (ratatui), Desktop (Tauri + React), Mobile (Flutter), Web, and Browser Extension.
        </p>
      </section>

      {/* Features */}
      <section id="features" style={{ maxWidth: 1200, margin: '0 auto', padding: `${spacing.xlarge}px 0 ` }}>
        <h2 style={{ fontSize: 32, fontWeight: 700, color: colors.secondary, marginBottom: spacing.large }}>Features</h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: spacing.large }}>
          {[
            { title: 'Vault', desc: 'Encrypted item storage with Shamir Secret Sharing support.', color: colors.primary },
            { title: 'SSH Terminal', desc: 'Dynamic injection of secrets into process memory loops.', color: colors.accent },
            { title: 'API Key Manager', desc: 'Auto-rotation for NVIDIA, OpenAI, AWS, Google Cloud.', color: colors.secondary },
            { title: 'Zero-Knowledge Sync', desc: 'S3-compatible sync (Backblaze B2, R2, MinIO).', color: colors.success },
            { title: 'Proactive Engine', desc: 'Hardware-aware AI routing and benchmark layers.', color: colors.warning },
            { title: 'Cross-Platform', desc: 'CLI, Desktop, Mobile, Web, Browser — one design system.', color: colors.cyberGlow },
          ].map((f) => (
            <div key={f.title} style={{ background: colors.surface, border: `1px solid ${colors.border}`, borderRadius: borderRadius.rounded, padding: spacing.large, boxShadow: `0 ${shadow.subtle}px ${shadow.medium}px rgba(0,0,0,0.1)` }}>
              <h3 style={{ color: f.color, fontSize: 20, fontWeight: 600, marginBottom: spacing.small }}>{f.title}</h3>
              <p style={{ color: colors.textSecondary, fontSize: 14 }}>{f.desc}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Design System */}
      <section style={{ background: colors.surface, maxWidth: 1200, margin: '0 auto', padding: spacing.xlarge, borderRadius: borderRadius.pill }}>
        <h2 style={{ fontSize: 28, fontWeight: 700, color: colors.textPrimary, marginBottom: spacing.medium }}>Design System</h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: spacing.medium }}>
          <div><h4 style={{ color: colors.primary }}>Colors</h4><p style={{ fontSize: 12, color: colors.textSecondary }}>#6366F1 · #A855F7 · #06B6D4 · #090D16 · #1E293B</p></div>
          <div><h4 style={{ color: colors.accent }}>Typography</h4><p style={{ fontSize: 12, color: colors.textSecondary }}>Plus Jakarta Sans · JetBrains Mono</p></div>
          <div><h4 style={{ color: colors.secondary }}>Spacing</h4><p style={{ fontSize: 12, color: colors.textSecondary }}>8px base · sm/md/lg/xl</p></div>
          <div><h4 style={{ color: colors.success }}>Tokens Export</h4><p style={{ fontSize: 12, color: colors.textSecondary }}>JSON · TypeScript · Dart · CSS-in-JS</p></div>
        </div>
      </section>

      <section id="auth" style={{ padding: `${spacing.xlarge}px 0`, maxWidth: 960, margin: '0 auto' }}>
    <h2 style={{ fontSize: 32, fontWeight: 700, color: colors.accent, marginBottom: spacing.medium }}>Account</h2>
    <p style={{ color: colors.textSecondary, lineHeight: 1.7 }}>
      Register, login, change password, reset — shared across CLI (ratatui), Desktop (Tauri + React), Mobile (Flutter + FFI), Web, and Browser Extension. All passwords hashed with Argon2id; master password never stored or logged; Zeroize on drop.</p>
  </section>
  <footer style={{ textAlign: 'center', padding: spacing.xlarge, color: colors.textDisabled, fontSize: 14 }}>
        Aetheris — Unified secrets management. Zero-knowledge. Cross-platform.
      </footer>
    </div>
  );
}
