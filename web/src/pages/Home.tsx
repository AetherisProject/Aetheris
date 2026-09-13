import React from 'react';
import { colors, spacing, typography, borderRadius, shadow } from '../components/AetherisComponents';

export default function HomePage() {
  return (
    <div style={{ background: colors.bg, color: colors.textPrimary, fontFamily: typography.family, minHeight: '100vh', padding: spacing.xlarge }}>
      <section style={{ textAlign: 'center', padding: `${spacing.xlarge}px 0` }}>
        <h1 style={{ fontSize: 40, fontWeight: 800, letterSpacing: '-0.03em', color: colors.primary, marginBottom: spacing.medium }}>
          Aetheris Home
        </h1>
        <p style={{ fontSize: 20, color: colors.textSecondary, maxWidth: 640, margin: '0 auto', lineHeight: 1.7 }}>
          Your zero-knowledge vault, SSH terminal, and API key manager.
        </p>
        <div style={{ marginTop: spacing.large, display: 'flex', gap: spacing.medium, justifyContent: 'center', flexWrap: 'wrap' }}>
          <a href="/chat" style={{ display: 'inline-block', padding: `${spacing.medium}px ${spacing.large}px`, background: colors.primary, color: colors.bg, borderRadius: borderRadius.rounded, textDecoration: 'none', fontWeight: 600, boxShadow: `0 ${shadow.medium}px ${shadow.raised}px rgba(99, 102, 241, 0.35)` }}>Open Chat</a>
          <a href="/admin" style={{ display: 'inline-block', padding: `${spacing.medium}px ${spacing.large}px`, background: colors.surface, color: colors.textPrimary, borderRadius: borderRadius.rounded, textDecoration: 'none', fontWeight: 600, border: `1px solid ${colors.border}` }}>Admin Dashboard</a>
        </div>
      </section>
    </div>
  );
}