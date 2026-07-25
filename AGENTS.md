# Aeon Agent Guide

This repo's source of truth lives in `docs/`.

Read these first:

1. `docs/project-definition.md` - what Aeon is, MVP scope, and non-goals
2. `docs/CONTEXT.md` - product language and domain vocabulary
3. `docs/architecture.md` - target module shape, seams, and runtime model

## Rules

- Use the domain terms in `docs/CONTEXT.md`.
- Keep changes aligned with the MVP in `docs/project-definition.md`.
- Prefer the architecture and deep-module boundaries in `docs/architecture.md`.
- If docs disagree, treat `docs/project-definition.md` as product scope, `docs/CONTEXT.md` as language, and `docs/architecture.md` as implementation guidance.

## Working style

- Make focused changes.
- Keep backend telemetry logic behind a small seam.
- Keep frontend focused on dashboard presentation.
- Do not introduce persistence unless the docs or task ask for it.
