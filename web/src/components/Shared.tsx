import React from 'react';

// Design token values exported from Rust design system
export const colors = {
  primary: '#6366F1',
  secondary: '#A855F7',
  accent: '#06B6D4',
  success: '#10B981',
  error: '#EF4444',
  warning: '#F59E0B',
  bg: '#090D16',
  surface: '#1E293B',
  textPrimary: '#F8FAFC',
  textSecondary: '#CBD5E1',
  border: '#334155',
  cyberGlow: '#6366F1',
  heraldicGold: '#F59E0B',
  midnightBlue: '#090D16',
};

export const spacing = { base: 8, small: 12, medium: 16, large: 24, xlarge: 32 };
export const typography = { weights: [400,500,600,700,800], family: "Plus Jakarta Sans", mono: "JetBrains Mono" };
export const borderRadius = { sharp: 0, slight: 4, rounded: 8, pill: 16 };
export const shadow = { none: 0, subtle: 1, medium: 2, raised: 4, high: 8, floating: 12 };

export const Card = ({ children, title }: { children: React.ReactNode; title: string }) => (
  <div style={{ background: colors.surface, borderRadius: borderRadius.rounded, padding: spacing.medium, border: `1px solid ${colors.border}`, boxShadow: `0 ${shadow.subtle}px ${shadow.medium}px rgba(0,0,0,0.1)` }}>
    <h3 style={{ color: colors.textPrimary, fontFamily: typography.family, fontWeight: 600 }}>{title}</h3>
    <div style={{ color: colors.textSecondary }}>{children}</div>
  </div>
);

export const Button = ({ label, variant = 'primary' }: { label: string; variant?: 'primary' | 'outline' }) => (
  <button style={{ background: variant === 'primary' ? colors.primary : 'transparent', color: variant === 'primary' ? colors.bg : colors.textPrimary, padding: `${spacing.small}px ${spacing.medium}px`, borderRadius: borderRadius.rounded, border: `1px solid ${variant === 'primary' ? colors.primary : colors.border}`, fontFamily: typography.family, fontWeight: 600, cursor: 'pointer' }}>{label}</button>
);
