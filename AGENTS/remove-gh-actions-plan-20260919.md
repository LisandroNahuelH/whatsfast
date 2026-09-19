# Remove GitHub Actions — plan (2026-09-19)

## Goal

Zero CI/workflows on GitHub; same app behavior; local *Definition of done* only.

## Steps (applied)

1. Backup `.github/workflows/` → `Backups/remove-gh-actions-*`.
2. Delete `ci.yml` and `release.yml`; remove empty `.github/`.
3. DELETE branch protection on `main`; PATCH repo `allow_actions=disabled`.
4. Update owners: `AGENTS.md`, `ARCHITECTURE.md`, `DECISIONS.md`, runbooks, `README.md`.
5. Local full checks; atomic commit; push.

## Verification

- No files under `.github/workflows/`.
- `gh api .../protection` → 404.
- Product binaries unchanged (no Rust edits).

## Veredicto

OK when build passes and remote has no workflow files.
