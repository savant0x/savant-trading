# FID-228: Jury Dashboard + Config Hardcode "M3" References — Decouple from Specific Model

| Field | Value |
|-------|-------|
| **FID** | 228 |
| **Status** | implemented — pending merge |
| **Severity** | medium |
| **Author** | Vera |
| **Operator** | Spencer |
| **Created** | 2026-06-24 14:30 local |
| **Sibling FIDs** | FID-227 (Anvil universe unblock), FID-226 (conviction gate) |

## Summary

The jury system's telemetry snapshot and dashboard frontend contain hardcoded "M3" labels (`"M3 ctrl"`, `"M3 calls"`, `m3_control_active`, `estimated_m3_calls`) that assume the control/influencer model is always MiniMax-M3. Since FID-227 switched the agent model to `openrouter/owl-alpha` (1M context), the labels are misleading — the dashboard shows "M3 ctrl: active" even though the agent is owl-alpha. The jury's "control" slot (juror 9) is also hardcoded to `minimax/minimax-m3` in `default_jury_models()`. These references must be decoupled from any specific model and made dynamic.

## Environment

- **OS:** Windows 11 Home 10.0.26200
- **Rust:** 1.x (latest stable)
- **Model:** `openrouter/owl-alpha` ( transitioned from `minimax/minimax-m3`)
- **Commit/State:** post-v0.15.7-a.1 (FID-227 implemented but uncommitted)

## Detailed Description

### Problem

Three layers of "M3" hardcode couple the jury system to a model that is no longer in use:

**Layer 1 — Telemetry struct (`src/core/shared.rs:129-144`):**
```rust
pub struct JuryStateSnapshot {
    pub m3_control_active: bool,        // ← hardcoded name
    pub estimated_m3_calls: u64,         // ← hardcoded name
    ...
}
```

**Layer 2 — Engine snapshot population (`src/engine/mod.rs:2982-3001`):**
```rust
*shared.jury_state.write().await = savant_trading::core::shared::JuryStateSnapshot {
    m3_control_active: true,             // ← hardcoded field
    estimated_m3_calls: jp.m3_calls(), // ← hardcoded field
    ...
};
```

**Layer 3 — Dashboard frontend (`dashboard/src/app/page.tsx:905-920`):**
```tsx
<MetricRow icon="fa-microchip" label="M3 ctrl" ... />
<MetricRow icon="fa-brain" label="M3 calls" ... />
<MetricRow icon="fa-list-ol" label="Free models" ... />
<MetricRow icon="fa-coins" label="Free calls" ... />
```

**Layer 4 — Config default (`src/core/config.rs:445`):**
```rust
fn default_jury_models() -> Vec<String> {
    vec![
        ...
        "minimax/minimax-m3".into(),  // ← juror 9 is hardcoded to M3
    ]
}
```

### Expected Behavior

The jury system should be model-agnostic. The control model (juror 0 / "influencer") should be whatever model the agent is actually using, as specified in `config.ai.model` or `[ai].model`. The telemetry field names, API response keys, and dashboard labels should use generic terms like "control", "ctrl", "primary", or dynamically inject the actual model name.

### Root Cause

FID-143 (MS-3) introduced multi-model jury diversity with M3 as the "control group" member. When FID-227 migrated away from M3 to owl-alpha, it updated `config.{default,test-anvil}.toml` model fields but did not sweep the jury-specific references. The jury was conceptually decoupled from the main agent model but the naming wasn't updated to reflect that.

### Evidence

**Log (from Spencer's terminal, 2026-06-24 ~11:00 PM):**
```
[jury] Control: MiniMax-M3  ← stale label
[jury] Influencer: MiniMax-M3  ← stale label
```

API_response (GET /api/jury/status):
```json
{
  "m3_control_active": true,
  "estimated_m3_calls": 142,
  ...
}
```

Dashboard rendering:
```
Jury
  Size: 10 (T:3 R:4 V:3)
  M3 ctrl: active       ← hardcoded
  M3 calls: 142         ← hardcoded
  Free models: 7
  Free calls: 28
```

**Code locations:**

| File | Line(s) | Issue |
|------|---------|-------|
| `src/core/shared.rs` | 132, 139 | `m3_control_active`, `estimated_m3_calls` field names |
| `src/engine/mod.rs` | 2985, 2996 | Populates hardcoded fields |
| `src/agent/jury/pool.rs` | comments | "M3 control group" comments |
| `src/core/config.rs` | 445 | `minimax/minimax-m3` as default juror 9 |
| `src/api/mod.rs` | ~300+ | Serializes `m3_control_active` to API response (field name in JSON) |
| `dashboard/src/app/page.tsx` | 905, 912 | "M3 ctrl" and "M3 calls" labels hardcoded in JSX |
| `tests/` (various) | — | Any test asserting on `m3_control_active` or `estimated_m3_calls` |

## Impact Assessment

### Affected Components

- `src/core/shared.rs` — `JuryStateSnapshot` struct
- `src/engine/mod.rs` — jury state snapshot population
- `src/agent/jury/pool.rs` — jury pool internals (comments + field names)
- `src/core/config.rs` — `default_jury_models()`
- `src/api/mod.rs` — JSON serialization
- `dashboard/src/app/page.tsx` — UI labels
- Tests referencing `m3_*` fields

### Risk Level

- [ ] Critical: System crash, data loss, or security vulnerability
- [ ] High: Major feature broken, no workaround
- [x] Feature degraded: Dashboard shows misleading info, jury default config stale
- [ ] Low: Minor issue, cosmetic, or edge case

## Proposed Solution

### Approach

**Rename + make dynamic.** Two parallel changes:

1. **Engine + API layer** — Rename `m3_control_active` → `control_active`, `estimated_m3_calls` → `estimated_control_calls`, add new `control_model: String` field that captures the actual model name from config.
2. **Dashboard** — Replace "M3 ctrl" with dynamic label using `control_model` (or generic "Ctrl"), replace "M3 calls" → "Ctrl calls".
3. **Config** — Remove `minimax/minimax-m3` from `default_jury_models()`, replace with `openrouter/owl-alpha:free` (or the actual real model).

### Steps

#### Step 1 — `src/core/shared.rs`

Replace:
```rust
pub m3_control_active: bool,
pub estimated_m3_calls: u64,
```
With:
```rust
pub control_active: bool,
pub control_model: String,
pub estimated_control_calls: u64,
```

Update `Default` impl accordingly.

#### Step 2 — `src/engine/mod.rs`

Update the snapshot population to use new field names and populate `control_model`:
```rust
control_active: config.ai.jury.models.first().map(|_| true).unwrap_or(false),
control_model: config.ai.model.clone(),  // the actual agent model
estimated_control_calls: jp.m3_calls(),  // internal counter stays; just rename field
```

Or better: add a `control_model` field to the jury pool config that snapshots which model is the influencer.

#### Step 3 — `src/agent/jury/pool.rs`

Update comments + any field references. If there's a `provider_config_m3`, rename to `provider_config_control` or make it dynamic.

#### Step 4 — `src/core/config.rs`

Remove M3 from `default_jury_models()`:
```rust
fn default_jury_models() -> Vec<String> {
    vec![
        "google/gemma-4-26b-a4b-it:free".into(),
        "google/gemma-4-31b-it:free".into(),
        "meta-llama/llama-3.2-3b-instruct:free".into(),
        "meta-llama/llama-3.3-70b-instruct:free".into(),
        "nvidia/nemotron-3-super-120b-a12b:free".into(),
        "nvidia/nemotron-3-ultra-550b-a55b:free".into(),
        "qwen/qwen3-coder:free".into(),
        "qwen/qwen3-next-80b-a3b-instruct:free".into(),
        "openai/gpt-oss-120b:free".into(),
        "openrouter/owl-alpha:free".into(),  // ← was minimax-m3
    ]
}
```

#### Step 5 — `src/api/mod.rs`

Rename JSON keys in serialization:
- `m3_control_active` → `control_active`
- `estimated_m3_calls` → `estimated_control_calls`
- Add `control_model` field.

Add a `#[serde(rename = "control_active")]` or handle the rename in the API serializer.

#### Step 6 — `dashboard/src/app/page.tsx`

Replace:
```tsx
<MetricRow icon="fa-microchip" label="M3 ctrl" value={status?.control_active ? "active" : "off"} ... />
<MetricRow icon="fa-brain" label="M3 calls" value={status?.estimated_control_calls} ... />
```
With:
```tsx
<MetricRow icon="fa-microchip" label={status?.control_model ? `${status.control_model} ctrl` : "Ctrl"} value={status?.control_active ? "active" : "off"} ... />
<MetricRow icon="fa-brain" label="Ctrl calls" value={status?.estimated_control_calls} ... />
```

Or keep it simpler:
```tsx
<MetricRow icon="fa-microchip" label={`${status?.control_model ?? "Agent"} ctrl`} ... />
<MetricRow icon="fa-brain" label="Ctrl calls" ... />
```

#### Step 7 — Tests

Search for `m3_control_active`, `estimated_m3_calls`, `m3_api_key`, `m3 ctrl`, etc. across all test files. Update assertions to use new field names.

#### Step 8 — Docs

Update any FID updates or comments in `src/agent/jury/*.rs` that reference "M3" as the control model.

### Verification

1. `cargo check --workspace --all-targets` — must pass
2. `cargo clippy --all-targets -- -D warnings` — must pass
3. `cargo test --workspace --all-targets` — must pass (no broken field name references)
4. `cargo fmt --check` — must pass
5. GET `/api/jury/status` returns `control_active`, `control_model`, `estimated_control_calls` (no `m3_*` keys)
6. Dashboard displays dynamic model name or generic "Ctrl" label

## Files Changed (estimated)

| File | Change |
|------|--------|
| `src/core/shared.rs` | Rename 2 fields, add `control_model` |
| `src/engine/mod.rs` | Update population |
| `src/agent/jury/pool.rs` | Rename fields, update comments |
| `src/core/config.rs` | Replace M3 in `default_jury_models()` |
| `src/api/mod.rs` | Update JSON key names |
| `dashboard/src/app/page.tsx` | 2 label changes |
| Tests | Update assertions referencing old field names |

Estimated: ~7 files, ~30-40 lines changed.

## Open Questions

1. **Should `control_model` be the main agent model (`config.ai.model`) or the jury influencer model (juror 0 from `jury.models`)?** Conceptually these should be the same — the influencer is the agent's twin. But the jury config has its own `models` vec. Recommend: use `config.ai.model` (the agent model) as the label, and verify the jury's influencer slot is the same model (which it should be since FID-143 designed it that way).
2. **Should we keep `minimax/minimax-m3` in `default_jury_models()` as a fallback for backward compat, or fully remove?** Spencer's call, but recommend removing — M3 is no longer used anywhere.
3. **Does this affect the jury's actual behavior or only telemetry/labels?** Only telemetry + default config. The jury's runtime logic is model-agnostic already. But the default juror 9 (`minimax/minimax-m3`) won't work if the M3 key is gone from config — it must be updated.
4. **Should the API field rename be a breaking change?** If any external tool reads `/api/jury/status` and expects `m3_*` keys, it will break. Recommend keeping `#[serde(alias)]` for backward compat during transition, or ask Spencer if any consumers exist.

## Lessons Learned (anticipated)

When migrating between models (e.g., M3 → owl-alpha), a **config grep sweep** is mandatory. Config field names are the easy part — struct field names, API keys, dashboard labels, and comment references are the silent breakers. ECHO Law 1 ("Read before act") applies before a model migration FID ships: grep for the old model's name across `src/`, `tests/`, `dashboard/`, and `config/` directories, not just the TOML files.

---

*FID-228 draft — awaiting approval. Vera, 2026-06-24.*
