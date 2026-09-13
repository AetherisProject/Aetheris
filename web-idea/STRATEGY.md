# Brand Strategy — one person, two directions

## 1. The map

```
                    merlin-tribukait.com
                    THE PERSON (trunk)
                    story · CV · 7 languages · freelance CTA
                    links OUT to both directions
                       ┌───────────────┴───────────────┐
                       ▼                               ▼
        aetheris.merlin-tribukait.com        games-reborn.com
        ENGINEERING (direction 1)             GAMES (direction 2)
        Aetheris vault + LLM gateway          multi-realm game network
        KEY-BITCHER · security tools          portal · launcher · community
        indigo/violet/cyan                    crimson/gold
                       │                               │
                       └──── every page links back to the trunk ────┘
```

Rule that makes it work: **the brands never merge in copy** — gamers get no
key-vault talk, engineering visitors get games framed as *infrastructure
experience* (§4). Only the trunk page speaks both languages, because the trunk
IS the person who does both.

## 2. Priority cleanup (before adding anything new)

- [ ] **Reclaim `aetheris.merlin-tribukait.com`** — it currently serves an
      unrelated "Puter AI models" toy. A product name pointing at a demo toy
      kills trust. Replace with `pages/engineering.html`.
- [ ] **Consolidate brand-asset sprawl** — assets live in ≥4 places
      (merlin-tribukait.com, games-reborn.com, 3+ GitHub Pages microsites).
      Rule: games assets live at games-reborn.com ONLY; personal at the trunk;
      engineering ships with the Aetheris repo (`docs/` → its Pages site).
      Redirect or delete the duplicate Pages microsites.
- [ ] **Aetheris brand kit** — the repo has tokens (`docs/design-system.md`)
      but no logo/OG assets; generate from `brand/aetheris.svg` (favicon,
      og-image 1200×630) with any SVG→PNG tool.

## 3. The hand-in-hand mechanics

- **One stylesheet** (`brand-core.css`) = layout, nav, cards, buttons, footer.
  Directions differ ONLY by 3 variables: `--accent`, `--accent-2`, `--glow`.
- **Identical skeleton** on all three pages: tri-nav top (`Merlin ⌂` ·
  `Engineering` · `Games`, current one highlighted), hero, content, footer
  with cross-links. A visitor learns in 3 seconds that these are one person's
  two worlds.
- **Logo trinity** shares geometry language (hex/shield/monogram, titanium
  strokes) — `brand/*.svg`. Reuse existing 3D crests as social/OG art.
- **Favicons/OG**: reuse the existing kits where present (GR kit exists);
  engineering kit = export from `brand/aetheris.svg`.

## 4. Copy rules (important — this is the CV)

- On the **trunk + engineering** pages, the games direction is phrased as:
  *"20 years building and operating high-concurrency, real-time server
  infrastructure (MMO backends, launchers, portals)"* — true, impressive,
  employer-safe. Do not brandish the private-server scene on the CV track.
- On the **games** page, engineering is one respectful footer line
  ("built by a full-stack engineer →"). Gamers don't care about crypto vaults.
- **7 languages is a hiring asset** — put the language strip prominently on
  the trunk. Start EN + DE as real pages (`/en`, `/de` + `hreflang`) later;
  v1 lists the badges. Edit the badge list in `pages/hub.html` (marked comment).

## 5. Domain/deployment map

| Page | Lives at | Served from |
|---|---|---|
| `pages/hub.html` → `index.html` | merlin-tribukait.com | the `merlin-tribukait.com` repo |
| `pages/engineering.html` → `index.html` | aetheris.merlin-tribukait.com | Aetheris repo `docs/` pipeline or its own Pages branch |
| `pages/games.html` → `index.html` | games-reborn.com | the web-portal repo (`config` already drives domain links) |
| Aetheris docs/mocks | aetheris.merlin-tribukait.com/docs | `docs/_build.py` output + `v2/mockups/` as "live preview" |

## 6. Don'ts

- ❌ No third direction/brands (horror/island/other pages → archive or fold
  into the two directions).
- ❌ No shared login/community between directions — they're separate audiences.
- ❌ No new logo styles per page — the three SVGs are the whole system.
