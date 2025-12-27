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
- any constraints (threading, realtime safety, allocations, locks)