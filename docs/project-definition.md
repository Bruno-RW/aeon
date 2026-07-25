# Aeon Project Definition

## Status

This repository is currently a **product shell**, not yet a defined product.

Verified from the codebase:

- Desktop app built with **Tauri + React + TypeScript**
- Basic Tauri starter UI still present
- Tauri config also suggests possible **Android** development later
- No domain model, feature set, or user workflow is defined in the repo yet

That means the most valuable next step is to define **what Aeon is for** before shaping modules, screens, or persistence.

---

## Draft product definition

Use this as a working draft, not as settled truth.

### Working product statement

**Aeon is a system-performance monitoring application for users who want to understand how application usage affects their computer.**

More specifically, the current working definition is:

> Aeon is for users who want to monitor their computer usage. They use it to monitor application usage in order to improve system performance.

### Product shape already implied by the repo

Because this is a Tauri app, Aeon is best defined as one of these:

1. a **desktop-first local tool**
2. a **desktop companion** for an external system
3. a **cross-platform utility** that may later extend to Android

If that is wrong, the current stack should be reconsidered early.

---

## Decisions we need to make

These are ordered so early answers make later ones easier.

### 1. Primary user

Who is Aeon for?

Examples:

- an individual user
- a small team
- an operator or analyst
- a creator or developer
- a domain specialist

**Why this matters:** user identity shapes language, workflows, permissions, and what “simple” means.

### 2. Core job-to-be-done

What is the one thing Aeon must help that user do exceptionally well?

Current working answer:

- observe application usage
- connect it to machine behavior
- identify what is hurting system performance
- focus the MVP on CPU, RAM, and GPU monitoring

**Recommendation:** keep v1 focused on diagnosis, not remediation. Monitoring and explanation first; optimization actions later.

### 3. Source of truth

Where does the important data live?

Possible answers:

- local files on disk
- an embedded database
- a remote backend
- a third-party API
- a hybrid local-first sync model

**Why this matters:** this is an architectural seam with long-term consequences.

### 4. Platform promise

What platform experience are we actually promising?

Current working answer:

- Windows only for v1/MVP
- other desktop platforms are future scope
- Android is not part of the MVP

**Recommendation:** treat Windows support as an explicit product constraint, not just an implementation accident.

### 5. Core workflow

What does success look like in one session?

A strong workflow can be written as:

1. user opens Aeon
2. user performs action A
3. Aeon computes or stores B
4. user reviews or exports C
5. user leaves with outcome D

### 6. Non-goals

What should Aeon explicitly not try to do in version 1?

This keeps the module interfaces small and the product deep instead of shallow.

---

## Recommended v0 definition template

If you want a crisp first version, fill in this template:

### Audience

Aeon is for **[specific user]**.

### Job

They use it to **[single recurring high-value task]**.

### Trigger

They open Aeon when **[situation/event]** happens.

### Outcome

They are done when **[observable result]** is true.

### Data model

The core things in the system are:

- **[entity 1]**
- **[entity 2]**
- **[entity 3]**

### Version 1 scope

Aeon v1 must support:

- **[capability A]**
- **[capability B]**
- **[capability C]**

Aeon v1 must not include:

- **[non-goal A]**
- **[non-goal B]**

### Platform

Aeon v1 targets **[desktop only / desktop + Android]**.

### Storage

Aeon stores data in **[local files / sqlite / remote api / hybrid]**.

---

## What to define next after this

Once the above is answered, the repo should gain:

1. `CONTEXT.md` — glossary of domain terms
2. `docs/adr/0001-*.md` — only for hard-to-reverse architectural choices
3. a feature slice plan for the first usable workflow
4. a deeper module design around the first real seam

---

## Current definition checkpoint

Resolved so far:

- Aeon is a **system performance** product, not a productivity tracker
- The current core object of interest is **application usage**
- The likely v1 direction is **diagnosis through monitoring**
- Aeon MVP should monitor **CPU, RAM, and GPU**
- Aeon MVP targets **Windows only**

## Emerging v1 statement

> Aeon is a Windows desktop application for users who want to monitor how running applications affect CPU, RAM, and GPU usage so they can diagnose system performance problems.

Current v1 monitoring scope:

- **Resources:** CPU, RAM, GPU
- **Platform:** Windows only
- **Views:** both system-wide and per-application
- **Time model:** live monitoring plus short-term history
- **History retention:** in-memory session history only
- **Primary UX goal:** quickly identify top resource-hungry applications
- **Application model:** group related processes into one user-facing application when possible
- **Interaction model:** read-only monitoring only
- **Refresh model:** user-configurable live refresh interval
- **Main list detail:** current values plus mini history charts per application
- **Top-level layout:** one dashboard with system summary and app list
- **List ranking:** user-selectable CPU, RAM, or GPU sort with a sensible default
- **Drill-down:** side panel with current metrics and a larger recent chart
- **Alerts:** none in v1; dashboard-driven diagnosis only

## Emerging MVP statement

> Aeon is a Windows desktop monitor that shows live and recent CPU, RAM, and GPU usage at both the system and per-application level so users can diagnose performance problems during the current session.

## MVP definition summary

Aeon v1 is a **Windows-only desktop monitoring application** that helps users diagnose performance problems by showing **live** and **recent session-local** CPU, RAM, and GPU usage at both the **system-wide** and **per-application** level.

The product is optimized to help users **quickly identify the top resource-hungry applications**.

### Confirmed MVP characteristics

- one dashboard
- system summary plus application list
- grouped user-facing applications rather than raw process rows where possible
- sortable by CPU, RAM, or GPU
- refresh uses fixed presets plus a paused state
- mini history charts in the main list
- side-panel drill-down for a selected application
- application panel shows current metrics, a larger recent chart, and basic metadata
- read-only monitoring only
- no alerts, notifications, or automated recommendations in v1
- GPU scope includes usage and GPU memory
- temperature may be shown when available from the underlying hardware/OS
- system summary shows current machine-wide values plus mini history charts

## Remaining decisions to define later

- what exact preset refresh intervals should be available
- how application grouping should work on Windows when process relationships are ambiguous
- whether temperature can be shown reliably enough to treat as a first-class metric

## Related documents

- [Architecture](./architecture.md) — proposed module structure, seams, and runtime shape for the MVP
