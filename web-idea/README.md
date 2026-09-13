# web-idea — unified brand system: one person, two directions

**The architecture:** `merlin-tribukait.com` is the **trunk** (the person, CV,
languages, freelance availability) → two **directions**:

| Direction | Domain | Token accent | Content |
|---|---|---|---|
| 🛡️ **Engineering** | `aetheris.merlin-tribukait.com` | indigo `#6366F1` → violet → cyan | Aetheris (vault + LLM gateway), KEY-BITCHER, full-stack/security work |
| 🎮 **Games** | `games-reborn.com` | laser crimson `#FF2D55` → phoenix gold `#FFB703` | 20 years of game-server engineering, the multi-realm portal, launcher |

They stay **hand-in-hand** through ONE shared stylesheet (`brand/brand-core.css`)
with per-direction accent overrides + an identical page skeleton with a tri-nav
(`Merlin ⌂ · Engineering · Games`) and cross-linked footers on every page.

## Files

```
brand/brand-core.css            # shared tokens + components (the glue)
brand/direction-engineering.css # indigo/cyan accent override
brand/direction-games.css       # crimson/gold accent override
brand/merlin.svg                # personal MT monogram
brand/aetheris.svg              # engineering shield glyph
brand/games-reborn.svg          # games phoenix hex mark
pages/hub.html                  # merlin-tribukait.com — the person + direction router
pages/engineering.html          # aetheris.merlin-tribukait.com — engineering home
pages/games.html                # games-reborn.com — games home
index.html                      # preview entry (opens the hub)
STRATEGY.md                     # domain map, cleanup list, copy rules, i18n, deployment
```

**Preview:** open `web-idea/index.html` in a browser. **Deploy:** copy each
`pages/*.html` to the matching site repo (see STRATEGY.md §5) — no build step.
