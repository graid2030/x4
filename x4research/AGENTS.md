## Project Overview

This project is a Research and Knowledge system for the game X4: Foundations

---

## Architecture Standards

### File Size & Structure

* Max code file size: 200–250 lines
* Break any file exceeding the limit into smaller modules
* Organize modules into directories by feature/domain
* Favor composition and modular design

### Design Philosophy

> Build simple, maintainable systems that solve current needs

#### Required Principles

* KISS — Keep It Simple
* YAGNI — Only build what is currently required
* SOLID principles:

  * Single Responsibility
  * Open/Closed
  * Liskov Substitution
  * Interface Segregation
  * Dependency Inversion

---

## Expected Output Style

* Clean, readable, documented code
* Clear separation of data, logic, and configuration
* Self-explanatory names
* Single-responsibility functions
* Minimalistic, focused architecture

---

## Avoid

* Over-engineering or futuristic abstractions
* Monolithic / god scripts
* Added features without request
* Deep inheritance — prefer composition

---

## Goal

Create a simple, modular, scalable trading system for X4 that follows modern software engineering practices.

---

## Research Standards

* Always decompose tasks into smaller, focused steps.
* Primary objective: build knowledge about X4 and reliable methods to extract it from saves and game resources.
* Knowledge placement (single source of truth):
  * `SaveStructure.md` — save-game paths and schema.
  * `GameDataStructure.md` — where data lives in game resources (catalog/dat) and how to resolve references.
  * `BasicInfo.md` — global conventions and cross-cutting concepts.
* Keep knowledge consistent and deduplicated:
  * When you learn something important (even if not tied to the current hypothesis), check if it already exists in the knowledge base and add/update as needed.
  * If rules repeat across docs (e.g., localization), extract into `BasicInfo.md` and link back.
* Documentation hygiene:
  * Use English, concise examples, and exact paths/IDs.
* Prefer a single canonical location for each concept; reference elsewhere rather than copy.
* Capture knowledge about game data structures themselves (save XML, catalog assets, identifiers). UI walkthroughs and tooling hints (e.g., from X4SaveGameAnalysis.R) may guide research but should not become canonical knowledge entries.

---

## Validation Questions (Ask/Requests)

* For each hypothesis (or sub-point) that requires user/game validation, add a concise “Ask:” line directly under it in `/hypotheses/*.md`.
* Prefer in-game observations the player can provide (map legend, logbook entries, station UI) rather than references to CSVs.
* Keep questions specific and minimal, enabling quick verification.

---

## Default Behaviors (Must-Do)

* When research reveals unrelated but important knowledge, proactively update the canonical docs (`BasicInfo.md`, `SaveStructure.md`, `GameDataStructure.md`, `SectorInfo.md`, etc.) after checking for duplicates.
* When adding or editing hypotheses, always include “Ask:” validation prompts under each point where in-game confirmation is useful.
* After verification, promote facts to knowledge files and trim exploratory notes.

---

## Hypothesis Testing Tools

* Build small utilities to validate hypotheses quickly.
* Examples to support:
  * Find coordinates of all stations in a star system.
  * Get the full name of the player’s current ship (e.g., `Discoverer Vanguard (FRA-696)`).
  * Read a ship’s filled cargo size/capacity.
* Allow an optional expected value and report pass/fail clearly.
* Place scripts under `tools/` with single-responsibility modules and a brief usage note.

---

## Ephemeral Self-Checks (Policy)

- Prefer a single, parameterless self-check script per hypothesis (e.g., `tools/selfcheck_h1.py`).
- Use in-memory fixtures or short-lived temp files; do not require user inputs or environment params.
- Run autonomously and report PASS/FAIL in the task output.
- Immediately clean up any temporary files; remove the self-check script after execution.
- Keep checks minimal and focused; reuse services in `services/`.

---

## Knowledge Workflow

- Hypothesize: Generate small, testable hypotheses from real scripts/code (e.g., X4SaveGameAnalysis.R) or by request. Keep each hypothesis 5–10 words of data flow, not tool-specific.
- Verify: For selected hypotheses, create ephemeral, parameterless self-checks. Run them autonomously, report PASS/FAIL, then remove the check script.
- Curate Knowledge: When a hypothesis is verified, update the relevant knowledge files instead of leaving notes in tools.
  - Sector ownership/naming: `SectorInfo.md`.
  - Cross-cutting definitions/conventions: `BasicInfo.md`.
  - Broader resource/save schemas: `SaveStructure.md`, `GameDataStructure.md`.
- Deduplicate: If the same rule appears in multiple docs, consolidate into a single canonical location and link back.
- Keep it lean: Remove outdated or exploratory notes; prefer short data-flow bullets and precise paths/ids.

### Status & Cleanup
- Mark lifecycle in `/hypotheses/*.md` using: [PROPOSED], [VERIFIED], [PARTIAL], [REJECTED], [SUPERSEDED].
- After promoting to knowledge, either keep a short stub pointing to the knowledge file, or remove the entry in the next cleanup pass.
- For failed hypotheses, keep a one-line reason (what we learned).
- Keep it lean: Remove outdated or exploratory notes; prefer short data-flow bullets and precise paths/ids.
