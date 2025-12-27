---
trigger: always_on
---

## Docs

### 2.1 Always clarify scope before writing docs
Before creating/updating documentation, ask:
1) **Where should the markdown live?**
   - `README.md` (top-level overview)
   - `docs/` (detailed architecture / design notes)
   - module-level docs (`src/**/mod.rs` or `lib.rs` rustdoc)
2) **How deep should it go?**
   - quick overview / onboarding
   - developer-level architecture
   - algorithm-level notes and tradeoffs

### 2.2 Minimum documentation requirements (for any new/changed feature)
Docs must be sufficient to understand, at least superficially:
- the processing flow (inputs → steps → outputs)
- dependencies and why they exist
- key data structures and ownership/lifetimes at a high level
- algorithm overview and complexity notes (where relevant)
- any constraints (threading, realtime safety, allocations, locks)---
trigger: always_on
---

## New features (engineering priorities)

### 4.1 Core priorities
When implementing new features, always consider:
- memory safety (no unsafe unless explicitly requested)
- memory consumption (avoid unnecessary allocations, especially in hot paths)
- algorithmic complexity (choose scalable approaches)
- code reuse and clarity

### 4.2 Avoid tech debt
- Prefer small, targeted refactoring to reuse existing structures and utilities.
- Avoid duplicating logic or introducing “temporary” copy-paste implementations.
- Avoid code smells (deep nesting, massive functions, unclear ownership, hidden side effects).
- If new feature reveals an architectural gap, address it cleanly rather than layering hacks.
- Follow tests.md, refactoring.md and docs.md rule-sets.---
trigger: always_on
---

## Refactoring

### 3.1 Testing gate before refactor
Before refactoring any module:
- Validate the module has sufficient tests to prevent regressions.
- If tests are insufficient:
  - write/expand tests **first**
  - ensure they are deterministic and precise

### 3.2 Mandatory validation step
For any refactor:
- Run the relevant test suite **before** refactoring.
- Run the same tests **after** refactoring.
- Treat failing tests as a hard stop; fix before proceeding.---
trigger: always_on
---

## Tests

### 1.1 When to write tests
- Write tests for all crucial functionality, especially:
  - parsing/serialization
  - boundary conditions (min/max, empty, overflow/underflow)
  - deterministic DSP/math transforms and helpers
  - any code that can regress silently (formatting, indexing, windowing, coordinate transforms, etc.)
- If a change touches behavior or public API: add or update tests.

### 1.2 Test style and quality
- Follow AAA pattern: **Arrange → Act → Assert** (clearly separated).
- Assertions must check **precise results** when possible (exact values, exact vectors, exact errors).
- If floating-point is involved:
  - Prefer deterministic fixtures.
  - Use a clear tolerance, and justify it (e.g. `abs_err < 1e-6`).
- Prefer table-driven tests for multiple inputs/outputs.
- Tests should be readable and explain intent.

### 1.3 Determinism
- Avoid randomness in tests; if randomness is unavoidable, seed it and document the seed.
- Avoid relying on wall-clock timing for tests.