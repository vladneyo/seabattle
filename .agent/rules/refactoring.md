---
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
- Treat failing tests as a hard stop; fix before proceeding.