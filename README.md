# Aeon

Aeon is a Windows desktop app for system-performance monitoring.
It helps users understand how running applications affect CPU, RAM, and GPU usage during the current session.

## MVP

Aeon v1 focuses on:

- Windows-only, read-only monitoring
- live system-wide usage
- per-application usage when possible
- short-term, in-memory session history
- one dashboard with system summary, ranked app list, and app detail panel
- CPU, RAM, GPU, and GPU memory
- optional temperature when available
- refresh presets plus paused monitoring

## Source of truth

This repo is defined by the docs in `docs/`:

- `docs/project-definition.md` - product scope and non-goals
- `docs/CONTEXT.md` - domain language
- `docs/architecture.md` - module shape and runtime model

## Scripts

From the project root:

- `npm run dev` - start Vite dev server
- `npm run build` - type-check and build frontend
- `npm run preview` - preview built frontend
- `npm run tauri` - run Tauri CLI
- `npm run lint` - run ESLint

## Tech stack

- Tauri 2
- React 19
- TypeScript
- Rust
- `sysinfo` for telemetry work in progress

## Status

Aeon is still early.
The backend telemetry and dashboard product shape are being built toward the MVP described in `docs/`.
