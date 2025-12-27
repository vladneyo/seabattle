---
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