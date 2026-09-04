# rustashop brand assets (`docs/brand`)

Public named copies and **resize-only** size declensions.

## Source of truth

Masters live under local `assets/` (UUID filenames). That directory is
**gitignored** and must stay on disk as the crop master set. **Do not delete or
mutate `assets/`.**

This folder holds:

1. Byte-identical renamed masters (`logo-banner.png`, …)
2. Width/size variants (`-readme`, `-desktop`, `-mobile`, `-256`, `-128`, `-64`, favicons)

Processing rule: **copy + LANCZOS resize only**. No flood-fill, no alpha rewriting, no "make transparent" scripts.

## UUID → name map

| `assets/` suffix | Master name             |
| ---------------- | ----------------------- |
| `e94`            | `logo-mascot`           |
| `e95`            | `logo-wordmark`         |
| `e97`            | `logo-wordmark-compact` |
| `e03`            | `logo-gear-stacked`     |
| `e05`            | `logo-banner`           |
| `e06`            | `logo-horizontal`       |
| `e98`            | `logo-crab-lockup`      |
| `e99`            | `logo-speed-lockup`     |
| `e01`            | `mark-speed-bag`        |
| `e96`            | `mark-speed-bag-alt`    |
| `e02`            | `mark-bags-3d`          |
| `e04`            | `mark-crab`             |
| `e07`            | `mark-speed-bag-3d`     |
| `e08`            | `icon-app-crab`         |
| `e09`            | `icon-app-gear-lock`    |
| `e10`            | `icon-app-speed-bag`    |
| `e11`            | `icon-app-cart`         |
| `e12`            | `icon-circle-bag`       |
| `e13`            | `icon-circle-gear`      |
| `e14`            | `icon-circle-crab`      |
| `e15`            | `badge-stack`           |
| `e16`            | `seal-gear`             |
| `e17`            | `seal-crab`             |

## Suggested defaults

- README header: `logo-banner-readme.png` + `icon-app-crab-256.png`
- Desktop header: `logo-banner-desktop.png`
- Mobile header: `logo-banner-mobile.png` or `icon-app-crab-128.png`
- Favicon: `favicon-32.png`

Tagline: **Modern commerce. Rust powered.**

## GitHub README wordmark

Theme-aware SVG (black/orange on light, white/orange on dark):

| File | Use |
| --- | --- |
| `wordmark-gh-light.svg` | GitHub light (`#gh-light-mode-only`) |
| `wordmark-gh-dark.svg` | GitHub dark (`#gh-dark-mode-only`) |
