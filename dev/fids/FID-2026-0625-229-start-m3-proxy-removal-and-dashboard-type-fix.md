# FID-229: start.bat M3 Proxy Hardcode — Remove Dead Proxy Launch + Dashboard Type Staleness

| Field | Value |
|-------|-------|
| **FID** | 229 |
| **Status** | implemented — pending merge |
| **Severity** | high (launch blocker) |
| **Author** | Vera |
| **Operator** | Spencer |
| **Created** | 2026-06-25 10:15 local |
| **Sibling FIDs** | FID-228 (jury M3 hardcode decouple — in progress), FID-227 (Anvil unblock) |

## Summary

`start.bat` unconditionally launches `m3-proxy.bat` (line 85), which reads `TOKENROUTER_API_KEY` from the environment and starts a local proxy on :4000. Since FID-227 removed tokenrouter/M3 from the provider stack (replaced with `openrouter/owl-alpha`), the proxy has no valid backend to route to — it logs `Using TOKENROUTER_API_KEY=sk-YWPfP...` (stale key from old .env) and the proxy either fails to start or serves stale credentials. Simultaneously, the Next.js dashboard fails to TypeScript-compile because FID-228 renamed `JuryStateSnapshot` fields in the Rust struct but the dashboard's mirror interface in `src/lib/api.ts` still references `m3_control_active` and `estimated_m3_calls`.

## Environment

- **OS:** Windows 11 Home 10.0.26200
- **Shell:** cmd.exe (start.bat), PowerShell (cleanup scripts)
- **Model:** `openrouter/owl-alpha` (post-FID-227)
- **Commit/State:** FID-228 partially applied (struct rename done, dashboard type not yet updated)

## Detailed Description

### Problem 1: M3 Proxy Launch in start.bat

`start.bat:85` calls `m3-proxy.bat` which forwards to `scripts/m3-proxy-controller.bat`. The controller:

1. Reads `TOKENROUTER_API_KEY` from env (line 172-181)
2. Fails with `ERROR: TOKENROUTER_API_KEY missing or empty` if absent
3. Launches `scripts/m3-proxy.js` on :4000 if key is present

The proxy was a "Thinking Killer" proxy for MiniMax M3 — it intercepted LLM calls to inject thinking tokens. With M3 removed from the provider stack, this proxy is dead code. The engine's LLM client now calls `https://openrouter.ai/api/v1` directly (per FID-227 config).

**Symptom:** Boot log shows:
```
[start] Using TOKENROUTER_API_KEY=sk-YWPfP...
[start] UP: proxy listening on :4000.
```
But the proxy serves no purpose and may interfere with LLM routing.

### Problem 2: Dashboard TypeScript Type Error

`dashboard/src/lib/api.ts:181-202` defines a `JuryStateSnapshot` interface that mirrors the Rust struct. FID-228 renamed:
- `m3_control_active` → `control_active`
- `estimated_m3_calls` → `estimated_control_calls`
- Added `control_model: String`

But the dashboard interface was not updated. `next build` fails with:
```
Type error: Property 'control_model' does not exist on type 'JuryStateSnapshot'.
```

**Symptom:** `start.bat` aborts after `DASHBOARD BUILD FAILED. Fix errors and restart.`

### Problem 3: copy.ts References Old Field Name

`dashboard/src/lib/copy.ts:94` uses `j.m3_control_active` which no longer exists after FID-228 rename.

## Impact Assessment

- [x] **Critical: Launch blocker** — `start.bat` cannot build the dashboard or start the engine
- [x] **High: Dead code in startup path** — M3 proxy runs every boot, wastes resources, may interfere
- [ ] Medium: Misleading log output (proxy "UP" but serves no purpose)
- [ ] Low: Stale comments in batch files reference M3

## Proposed Solution

### Approach

1. **Remove the M3 proxy launch block from `start.bat`** — delete lines 82-91 (the `:: Start M3 Thinking Killer Proxy` section and the `call m3-proxy.bat` invocation)
2. **Update `dashboard/src/lib/api.ts`** — rename `m3_control_active` → `control_active`, `estimated_m3_calls` → `estimated_control_calls`, add `control_model: string`
3. **Update `dashboard/src/lib/copy.ts`** — replace `j.m3_control_active` → `j.control_active`, update label from "M3 control" → "Ctrl"
4. **Clean up M3 proxy files** (optional, follow-up) — `m3-proxy.bat`, `scripts/m3-proxy-controller.bat`, `scripts/m3-proxy.js`, `scripts/m3-proxy.log` can be deleted in a separate FID if disk cleanup is desired. Do NOT delete in this FID — just stop launching.

### Steps

#### Step 1 — `start.bat` (lines 82-91)

Delete:
```bat
:: ============================================================
:: Start M3 Thinking Killer Proxy (required for MiniMax M3 in Kilo)
:: ============================================================
call "%~dp0m3-proxy.bat"
if errorlevel 1 (
    echo  WARNING: M3 proxy failed to start. Kilo will get think tags.
) else (
    echo  M3 proxy running on :4000.
)
echo.
```

Also update the pre-build cleanup section (lines 61-67) to remove the `m3-proxy` cmdline filter since the proxy no longer runs:
```bat
:: Before: if ($cmd -like '*savant-trading*' -or $cmd -like '*m3-proxy*')
:: After:  if ($cmd -like '*savant-trading*')
```

#### Step 2 — `dashboard/src/lib/api.ts` (lines 181-202)

Replace:
```ts
export interface JuryStateSnapshot {
  enabled: boolean;
  jury_size: number;
  m3_control_active: boolean;        // → control_active
  free_models_used: string[];
  veto_enabled: boolean;
  veto_threshold: number;
  regime_sizes: { trending: number; ranging: number; volatile: number };
  cumulative: { ... };
  key_health: JuryKeyHealth;
  estimated_m3_calls: number;       // → estimated_control_calls
  estimated_free_model_calls: number;
  veto_flag_active_now: boolean;
  last_cycle_at: string | null;
  source: "live" | "stale" | "never_ran" | "engine_off" | "disabled";
}
```
With:
```ts
export interface JuryStateSnapshot {
  enabled: boolean;
  jury_size: number;
  control_active: boolean;
  control_model: string;
  free_models_used: string[];
  veto_enabled: boolean;
  veto_threshold: number;
  regime_sizes: { trending: number; ranging: number; volatile: number };
  cumulative: { ... };
  key_health: JuryKeyHealth;
  estimated_control_calls: number;
  estimated_free_model_calls: number;
  veto_flag_active_now: boolean;
  last_cycle_at: string | null;
  source: "live" | "stale" | "never_ran" | "engine_off" | "disabled";
}
```

#### Step 3 — `dashboard/src/lib/copy.ts` (line 94)

Replace:
```ts
line("M3 control", j.m3_control_active ? "active" : "off"),
```
With:
```ts
line("Ctrl", j.control_active ? "active" : "off"),
```

#### Step 4 — `dashboard/src/app/page.tsx` (already done in FID-228)

Already updated in FID-228 edit pass. Verify the compiled output uses `control_model` and `control_active`.

### Verification

1. `cargo check --workspace --all-targets` — must pass
2. `cargo test --workspace --all-targets` — must pass
3. `next build` (in `dashboard/`) — must pass TypeScript type check
4. `start.bat` — must boot without `m3-proxy.bat` invocation, no "M3 proxy running on :4000" log line
5. Dashboard jury section shows dynamic `control_model` or "Ctrl" label, not "M3 ctrl"

## Files Changed

| File | Change |
|------|--------|
| `start.bat` | Remove M3 proxy launch block + cleanup filter |
| `dashboard/src/lib/api.ts` | Rename 2 fields, add `control_model` |
| `dashboard/src/lib/copy.ts` | Update field reference + label |
| `dashboard/src/app/page.tsx` | Already done in FID-228 |

Estimated: 3 files, ~20 lines changed.

## Open Questions

1. **Should we also remove the `m3-proxy` cmdline filter from the pre-build cleanup section (line 67)?** If the proxy is never launched, no `m3-proxy` node process will exist. But leaving the filter is harmless (it just never matches). Recommend: remove for cleanliness.
2. **Should we delete the `m3-proxy*.bat` and `scripts/m3-proxy*` files in a follow-up FID?** They're dead code now. Recommend: separate FID for cleanup to keep this one focused on the launch blocker.
3. **Does the engine code have any remaining references to port 4000 or the proxy?** `grep -r "4000" src/` should return zero hits. If the engine still tries to route LLM calls through :4000, that's a separate critical bug.

## Lessons Learned

When removing a provider/model (FID-227), the **startup orchestration scripts** must be audited alongside config files. `start.bat` is the single entry point for the entire system — any service it launches must be validated as still needed. ECHO Law 1 ("Read before act") applies to infrastructure scripts just as strictly as source code.

---

*FID-229 draft — awaiting approval. Vera, 2026-06-25.*
