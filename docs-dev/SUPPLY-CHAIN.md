# Supply-chain checks

Local and CI gates for dependencies.

## Cargo

Requires once: `cargo install cargo-audit cargo-deny`

```bash
make audit   # cargo audit
make deny    # cargo deny check (deny.toml)
```

Add a documented ignore in `deny.toml` or `cargo audit --ignore RUSTSEC-…` only when an advisory is accepted with a written reason.

Current documented ignore: `RUSTSEC-2026-0258` (h2 0.3 via actix-http 3.x; patched releases are on h2 0.4.x only).

## npm (shop, admin, install)

Each app has `.npmrc` (`save-exact`, `min-release-age=3`) and `allowScripts` for packages that must run install scripts.

```bash
make audit-npm   # npm audit (high) + cve-lite-cli + malware IoC scan
make audit-all   # cargo audit + deny + audit-npm
```

Pinned CVE scanner: `cve-lite-cli@1.33.0` (Make/CI set `NPM_CONFIG_MIN_RELEASE_AGE=0` for that one-off fetch so app `min-release-age` still applies to project installs).

Malware IoC: `node scripts/npm-malware-scan.mjs .`

## Approving a new install script

From the app directory:

```bash
npm approve-scripts --allow-scripts-pending
npm approve-scripts <package>
```

Commit the updated `allowScripts` in `package.json`.

## GitHub Dependabot

Public-repo free settings (org): Dependabot alerts, Dependabot security updates, secret scanning, and push protection.

Version bumps: `.github/dependabot.yml` (weekly Cargo, npm apps, and Actions). Review those PRs like any other dependency change; local `make audit` / `make deny` / `make audit-npm` remain the merge gates.

Dependabot version updates ignore semver-major bumps; majors are intentional upgrades, not weekly noise.
