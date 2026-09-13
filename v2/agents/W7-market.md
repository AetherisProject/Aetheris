# W7 — Market: monetization & launch spec

- **Owns:** positioning, pricing, launch artifacts (landing page, posts, demos)
- **Depends on:** W1–W3 minimum (gateway must really work); **W4/W5 for Momentum**
- **Effort:** 2–3 h/day alongside W-cards · **Rule:** no marketing claims CI can't prove

## Mission

Turn the working base into first revenue and reputation. The story is fixed:
**"Your LLM keys never live in .env again — one encrypted vault, one local
gateway, on every device."** Every artifact repeats exactly that.

## 1. Positioning & ICPs

| ICP | Pain | First offer |
|---|---|---|
| Self-hosters / homelab (r/selfhosted) | keys scattered across `docker-compose.yml` & `.env` | free OSS gateway + vault (stars, future upsell) |
| LLM-heavy devs & small agencies | cost overruns, no budgets, leaked keys | gateway + budgets; later **hosted sync €5/mo** |
| EU compliance-minded SMEs (later) | DSGVO/GDPR: keys in US SaaS dashboards | audit logs + local vaulting = **Compliance tier** (post-launch) |

## 2. Pricing model (locked)

| Tier | Price | Contents |
|---|---|---|
| Core (MIT) | €0 | vault, gateway, hub, CLI — OSS forever |
| Sync | €5/mo or €49/yr | hosted hub (owner-run), multi-device, backups |
| Desktop Pro | €89 lifetime | Avalonia app + SSH terminal + SFTP (W6) |
| Compliance (later) | custom | audit exports, DPA, SSO — only after demand is proven |

Payments: **Lemon Squeezy** (merchant of record → handles EU VAT; no Steuerberater
nightmare for a solo seller). Keys via their license API; verification in-app.

## 3. Launch plan (D-14 → D+30)

**D-14 — repo polish:** README hero GIF (≤90 s: add key → curl → budgets move),
badges (CI green, license), `docker-compose up` single-command demo, SECURITY.md
honest about the trusted-gateway boundary.

**D-7 — content:** three dev.to/HN-post drafts:
1. "I built a local LLM gateway that keeps keys out of .env (C#/.NET 8, OSS)"
2. "Designing an encrypted vault over BouncyCastle: Argon2id + XChaCha20"
3. "One PWA, every platform: how the vault sync stays zero-knowledge"

**D-0 — launches (one per week, not all at once):**
r/LocalLLaMA → r/selfhosted → Show HN ("Show HN: Local LLM gateway that keeps
API keys in an encrypted vault"). Rules: no vote begging, answer every comment
within 2 h, post 9–11 am ET.

**D+7 → D+30:** weekly build-in-public posts (screenshots, honest metrics,
failures included). Open `GH Discussions` roadmap vote: what ships next.

## 4. Honest metric targets

| Metric | D+30 | D+90 | D+365 |
|---|---|---|---|
| GitHub stars | 150–500 | 1–3k (if a launch hits) | 5–15k |
| Gateway first-calls tracked (opt-in ping) | 20 | 200 | 2k+ |
| Paying sync users | 0–2 | 5–20 | 100–400 (→ €500–2,000 MRR) |

Base rate truth: most launches land at the low end; ONE front-page post moves
you a full row up. Consistency (weekly shipping + writing) is the only lever
fully under your control.

## 5. Service bridge (money NOW, product LATER)

Productized freelance offer, using this exact codebase as the demo:
**"LLM Key Hygiene & Cost Setup — €400–800 fixed"**: on-site/remote half-day
installing vault+gateway for a small agency, migrating keys out of `.env`,
budgets + fallback to local models, handover doc. Sell via DACH freelancing
boards (Gulp, freelancermap, Malt) and direct outreach to 10 local web
agencies. Target: **1 client/month** while the product grows. Same code,
immediate cash flow, references, and screenshots for the launch posts.

## 6. Acceptance gate

- [ ] Landing page live (GitHub Pages from this repo, or the
      `aetheris.merlin-tribukait.com` brand site) — embed
      `v2/mockups/index.html` as the interactive "try the product" preview
- [ ] LemonSqueezy checkout link wired for Sync tier (even if "coming soon" waitlist)
- [ ] 3 posts published, links recorded in this file
- [ ] Waitlist ≥ 10 emails OR first service client booked
- [ ] Honest metrics block updated monthly at the bottom of this file

## Not yours (ever)

Paid ads, cold enterprise sales, app-store campaigns, influencer deals.
Indie distribution = community + content + search.
