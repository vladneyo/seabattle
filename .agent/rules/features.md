---
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
- Follow tests.md, refactoring.md and docs.md rule-sets.