# README visual restyle (no product change)

Date: 2026-09-19

Restyle `README.md` so GitHub looks premium (hero, badges, screenshot, feature cards). Keep every current fact. Do not change app behavior, APIs, or claims.

## Findings

| id | sev | evidence | why | risk if skipped |
|----|-----|----------|-----|-----------------|
| F1 | media | `README.md` is plain text, no logo, badges, or screenshot | Visitors bounce | Low product risk, high first-impression gap |
| F2 | baja | `docs/assets/benchmarks` are ZapFast/Linux | Using them here would misstate WhatsFast Windows | Do not embed those charts |

## Do not touch

- Product source, installer, protocol claims.
- ZapFast Linux benchmark SVGs as WhatsFast numbers.
- Real chat data. Screenshot only via `--demo-shot` (synthetic demo).
- Duplicate facts into `AGENTS.md` or `ARCHITECTURE.md`.

## P0 assets

### 1.1 Demo screenshot

- Command: `cargo run --locked --features demo -- --demo-page chat --demo-size 1280x800 --demo-shot docs/assets/images/demo-chat.png --demo-shot-delay 3000`
- Synthetic demo only.
- Verify: PNG exists, no real phone numbers from the user archive.

### 1.2 Banner SVG

- New `docs/assets/images/readme-banner.svg` (1280x320): dark green field, existing WhatsFast mark, title, Windows-only line.
- No web font dependency. System sans-serif.

## P1 README

### 2.1 Hero

- Centered logo (`packaging/icons/whatsfast.svg` or `docs/assets/images/logo.svg`), title, tagline from current paragraph 1.
- Shields: Windows 10/11, Rust/egui, MIT, latest release, no browser engine.
- CTA to GitHub Releases (same Install URL).

### 2.2 Screenshot

- `demo-chat.png` under the hero. Alt text: synthetic demo chat.

### 2.3 Improvements

- Same 11 feature titles and Technical / Daily use text.
- Visual cards (2-col HTML table) with emoji or tinted icons.
- Do not drop or rewrite claims.

### 2.4 Remaining sections

- Keep: Upgrading from ZapFast, Install, Build, License, Attributions, trademark line, maintainer, not-upstream.

## Verify

- Diff is README + image assets + DECISIONS row (visual README choice).
- Every old fact still present (grep titles).
- No em dash in user-facing English.
