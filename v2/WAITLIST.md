# Aetheris Waitlist

> **Status:** Active
> **Last Updated:** September 2026
> **Owner:** W7-Market Workstream

---

## Overview

The Aetheris waitlist tracks interest in upcoming paid tiers and service offerings. This document serves as the canonical source of truth for waitlist entries.

---

## Current Count

| Metric | Count | Target | Status |
|--------|-------|--------|--------|
| Total Waitlist Entries | **0** | 10+ | 🟡 Not yet met |
| Sync Tier Interest | 0 | 5+ | 🟡 Not yet met |
| Desktop Pro Interest | 0 | 3+ | 🟡 Not yet met |
| Service Clients | 0 | 1 | 🟡 Not yet met |

**Acceptance Gate:** Waitlist ≥ 10 emails OR first service client booked

---

## Waitlist Tiers

### 1. Sync Tier (€5/month or €49/year)
**Description:** Multi-device sync with hosted hub. Zero-knowledge architecture.

**Target Launch:** Q4 2026

**Current Interest:** 0 entries

**Waitlist Goal:** 10+ entries

---

### 2. Desktop Pro (€89 lifetime)
**Description:** Full-featured desktop application with SSH terminal and SFTP support.

**Target Launch:** Q4 2026

**Current Interest:** 0 entries

**Waitlist Goal:** 5+ entries

---

### 3. Service Offer (€400-800 fixed)
**Description:** LLM Key Hygiene & Cost Setup - on-site/remote half-day installing vault+gateway for a small agency, migrating keys out of .env, budgets + fallback to local models, handover doc.

**Target:** 1 client/month

**Current Interest:** 0 entries

**Waitlist Goal:** 1 client

---

## Waitlist Entries

> **Note:** Entries are stored locally in the landing page. This section will be updated manually from localStorage exports.

### Format
```
- **Email:** user@example.com
- **Name:** (optional)
- **Interest:** sync | desktop | both | service
- **Message:** (optional)
- **Timestamp:** 2026-09-14T10:00:00Z
- **Source:** landing-page | direct | referral
```

### Current Entries

*No entries yet. Waitlist form is available at [https://aetheris.dev/#waitlist](https://aetheris.dev/#waitlist)*

---

## How to Join

### Method 1: Landing Page Form
1. Visit [https://aetheris.dev/#waitlist](https://aetheris.dev/#waitlist)
2. Fill out the form:
   - Email Address (required)
   - Name (optional)
   - Interest (Sync Tier, Desktop Pro, Both, or Service)
   - Message (optional)
3. Click "Join Waitlist"

### Method 2: Direct Email
Send an email to `waitlist@aetheris.dev` with:
- Subject: `Waitlist: [Your Interest]`
- Body: Your email, name (optional), and any message

### Method 3: GitHub Discussion
Open a discussion at [https://github.com/Aetheris/Aetheris/discussions](https://github.com/Aetheris/Aetheris/discussions) with:
- Title: `[Waitlist] [Your Interest]`
- Body: Your interest and contact information

---

## Waitlist Benefits

All waitlist members receive:

1. **Early Access** - Get access to paid tiers before public launch
2. **Discounted Pricing** - Special launch pricing (e.g., €4/month for Sync instead of €5)
3. **Exclusive Updates** - Early previews of new features
4. **Direct Feedback Channel** - Your input shapes the product
5. **Priority Support** - Jump the queue for support requests

---

## Service Clients

### Target: 1 client/month

**Service Offer:** "LLM Key Hygiene & Cost Setup — €400–800 fixed"

**Description:**
- On-site or remote half-day session
- Install vault+gateway for small agency
- Migrate keys out of .env files
- Set up budgets and cost controls
- Configure fallback to local models
- Provide handover documentation

**Target Clients:**
- DACH freelancing boards (Gulp, freelancermap, Malt)
- Local web agencies (10 targeted for direct outreach)

**Current Status:** 0 clients booked

---

## Outreach Plan

### Phase 1: Community (D-14 to D-0)
- [ ] Post on r/LocalLLaMA
- [ ] Post on r/selfhosted
- [ ] Post on r/csharp
- [ ] Post on r/dotnet
- [ ] Share on Twitter/LinkedIn

### Phase 2: Content (D-7 to D+30)
- [ ] Publish 3 dev.to/HN posts
- [ ] Weekly build-in-public posts
- [ ] GitHub Discussions roadmap vote

### Phase 3: Direct Outreach (D+7 onwards)
- [ ] Contact 10 local web agencies
- [ ] Post on DACH freelancing boards
- [ ] Engage with potential service clients

---

## Tracking

### Weekly Metrics

| Week | Date | New Entries | Total | Notes |
|------|------|-------------|-------|-------|
| W1 | 2026-09-14 | 0 | 0 | Landing page launched |
| W2 | 2026-09-21 | TBD | TBD | First posts published |
| W3 | 2026-09-28 | TBD | TBD | Launch week |
| W4 | 2026-10-05 | TBD | TBD | Post-launch |

### Conversion Rates

| Source | Visitors | Signups | Rate |
|--------|----------|---------|------|
| Landing Page | TBD | TBD | TBD |
| GitHub | TBD | TBD | TBD |
| Social Media | TBD | TBD | TBD |
| Referrals | TBD | TBD | TBD |

---

## Next Steps

- [ ] **Immediate:** Launch landing page with waitlist form
- [ ] **This Week:** Publish first blog post
- [ ] **Next Week:** Publish second blog post
- [ ] **D-7:** Publish third blog post
- [ ] **D-0:** Launch on r/LocalLLaMA
- [ ] **D+7:** Launch on r/selfhosted
- [ ] **D+14:** Show HN post

---

## Acceptance Criteria

The waitlist acceptance gate is met when **EITHER** of the following is true:

1. **Waitlist has ≥ 10 email entries**
   - Tracked via landing page form submissions
   - Verified by checking localStorage or backend database

2. **First service client is booked**
   - Contract signed or verbal agreement confirmed
   - Deposit received (if applicable)

**Current Status:** ❌ Not yet met

---

## Verification

### Check Waitlist Count
```bash
# If using localStorage (client-side)
# Check the landing page's localStorage for 'waitlistCount'

# If using backend
curl -s https://api.aetheris.dev/waitlist/count | jq .count
```

### Check Service Clients
```bash
# Check for any confirmed service bookings
grep -r "client" SERVICE_BOOKINGS.md || echo "No clients booked"
```

---

## Resources

- [Landing Page](https://aetheris.dev)
- [GitHub Repository](https://github.com/Aetheris/Aetheris)
- [LemonSqueezy Setup](LEMONSQUEEZY.md)
- [Metrics Dashboard](METRICS.md)

---

## Notes

- Waitlist entries are stored client-side in the landing page's localStorage
- For production, consider migrating to a backend database
- All entries should be exported and backed up regularly
- Respect GDPR/privacy regulations when storing email addresses
