# FID-227: Anvil-Native Token Universe Unblock

| Field | Value |
|-------|-------|
| **FID** | 227 |
| **Status** | implemented — pending merge (commit by operator) |
| **Severity** | high |
| **Author** | Vera |
| **Operator** | Spencer |
| **Created** | 2026-06-24 13:30 local |
| **Sibling FIDs** | FID-226 (conviction-gate probe escape + scan_all_pairs, shipped v0.15.7-a.1), FID-222 (funnel v1 pre-scorer), FID-225 (phantom-position halt, shipped v0.15.7-a.1) |
| **Diagnostic log** | `dev/logs/token-universe-diagnostic-2026-06-24.md` |

## Summary

FID-226 shipped `scan_all_pairs=true` and the probe-escape hatch. But the Anvil testnet bot still produces **0 trades** after 68+ cycles. The diagnostic in `dev/logs/token-universe-diagnostic-2026-06-24.md` confirms FID-226's universe expansion is necessary but not sufficient on Anvil — three additional bottlenecks stack on top of each other to keep the bot dead:

1. **Funnel v1 is disabled on testnet** — `test-anvil.toml` has no `[trading.funnel_v1]` block, so the top-K pre-scorer never runs. Funnel telemetry (`dev/logs/funnel-rankings.jsonl`, 173 lines) shows `funnel_enabled: false` and `output_K: 0` on every cycle.
2. **Hunt mode bypass** — even with funnel enabled, `hunt_mode=true` (0 positions, <$500 equity) forces `hunt_mode_bypass: true` → `output_K: 0`. The funnel has no "seed path" for the 0-position case.
3. **Conviction gate + MAX_NON_PASS=15 cap** — 27 raw Anvil pairs compete for 15 non-PASS slots per cycle. The LLM assigns weak scores to illiquid micro-caps with no real data, the gate downgrades them to PASS, the cap truncates the rest. Net: 0 BUY verdicts.

`data/dex_state.json` is clean (no phantom residue). FID-225 round 2 fix holds.

## Evidence

### Funnel telemetry (cycles 1-68, 173 log lines)

```
funnel_enabled: false   ← every line
hunt_mode: true         ← every line
hunt_mode_bypass: true  ← every line
output_K: 0             ← every line
input_N: 12 → 27        ← grew as tokens.json pairs got discovered
```

### Config audit

| Config | `[trading.funnel_v1]` block | `scan_all_pairs` |
|--------|----------------------------|------------------|
| `config/default.toml` | present (`enabled=true, top_k=12`) | `true` (FID-226) |
| `config/test-anvil.toml` | **MISSING** | `true` (FID-226) |

### `data/dex_state.json`

```json
{"positions": [], "closed_trades": [], "balance": 50.0, "order_counter": 11}
```

Clean. No phantom residue.

### Code locations

| Location | Role |
|----------|------|
| `src/engine/mod.rs:2052-2120` | Funnel wiring — calls `pre_scorer::score_pairs()` when `funnel_enabled && !hunt_mode_bypass` |
| `src/strategy/pre_scorer.rs:235+` | Funnel scoring logic — returns `PassThrough` (empty) when `positioned_pairs.is_empty()` |
| `src/api/mod.rs:284-286` | `MAX_DECISIONS=50`, `MAX_NON_PASS=15` (raised from 20/10 for scan_all_pairs) |
| `src/agent/decision_parser.rs:496-510` | Conviction gate — `conviction_score < regime_threshold` → Pass |
| `src/agent/decision_parser.rs:62-93` | `RegimeLabel::conviction_threshold()` — Trending 0.05, Volatile 0.15, Ranging 0.10, GreyZone 0.20 |
| `src/engine/mod.rs:2245-2246` | scan_all_pairs exception comment — "$0 LLM cost, let agent evaluate everything" |

## Impact Assessment

- [ ] Critical: System crash, data loss, or security vulnerability
- [x] High: Major feature broken (trade-flow generation dead on testnet), no workaround
- [ ] Medium: Feature degraded, workaround exists
- [ ] Low: Minor issue, cosmetic, or edge case

Without this fix, the Anvil testnet bot produces 0 trades regardless of runtime. The $50 testnet thesis remains untestable.

## Proposed Solution

Two independent changes, both required:

### Change 1 — Add `[trading.funnel_v1]` to `config/test-anvil.toml`

```toml
[trading.funnel_v1]
enabled = true
top_k = 12
min_score_threshold = 0.20
```

This alone will NOT fix the problem because of hunt_mode_bypass (Change 2). But it's a prerequisite — without it, the funnel code path is dead regardless of hunt mode.

### Change 2 — Anvil-native bypass in `src/engine/mod.rs`

When ALL three conditions hold:
- `config.trading.scan_all_pairs == true`
- `hunt_mode == true`
- `portfolio.positions().is_empty()`

Then set `SAVANT_GATE_DISABLED=1` (env var) which `decision_parser.rs:496` already honors to skip both the conviction gate and confidence floor. This reuses the existing bypass mechanism without adding a new code path.

**Why this is the right approach for Anvil:**
- The funnel's 6-signal composite scoring (EMA, RSI, ADX, vol, VWAP, BB) is meaningless on Anvil's fake data. Pre-scoring 27 illiquid micro-caps with no real price history produces noise, not signal.
- The conviction gate is similarly counterproductive — the LLM correctly assigns low conviction to junk data, and the gate correctly downgrades them to PASS. The gate is doing its job; the data is the problem.
- The right fix is to bypass both gates and let execution sizing (probe 0.5× at `engine/mod.rs:4322`) handle risk.

**What this does NOT change:**
- Live Kraken behavior is unaffected (the bypass only fires when `scan_all_pairs && hunt_mode && 0 positions`, which is impossible on live — live always has positions or exits hunt mode within seconds of starting).
- The probe-escape hatch (FID-198) still works — it lives inside `parse_decision()` and only fires in the `[probe_threshold, regime_threshold)` zone, which is the correct behavior for both real and fake data.
- `MAX_NON_PASS=15` cap is unchanged — execution sizing and portfolio-level risk limits still apply.

### Alternative considered but rejected

**"Seed the funnel for 0-position case"** — add a code path in `pre_scorer.rs` that returns top-K by score even when `positioned_pairs.is_empty()`. Rejected because:
- The funnel's scoring is meaningless on Anvil fake data
- Would add complexity to the funnel (already the most complex module in the codebase) for a testnet-only edge case
- The env-var bypass is a 1-line change that reuses an existing mechanism

## Files Changed

| File | Change |
|------|--------|
| `config/test-anvil.toml` | Add `[trading.funnel_v1]` block (6 lines) |
| `src/engine/mod.rs` | Add 8-line bypass block after hunt_mode sync (~line 2062) |

Total: 2 files, ~14 lines added. No deletions.

## Verification Plan

1. `cargo check --lib` — must pass
2. `cargo clippy --lib -- -D warnings` — must pass
3. `cargo test --lib` — must pass (514+ tests)
4. Run Anvil testnet for ≥10 cycles, verify:
   - Funnel telemetry shows `funnel_enabled: true`
   - Log line `FID-226.1: Anvil-native bypass` appears
   - At least 1 non-PASS verdict appears in decision logs
   - 0 phantom positions (FID-225 round 2 fix still holds)

## Open Questions

1. **Should the bypass also raise `MAX_NON_PASS` from 15 to 50 (matching `MAX_DECISIONS`)?** Currently the cap is 15 non-PASS per cycle. With 27 Anvil pairs, 12 are still silently dropped. Raising to 50 would let all pairs produce verdicts, but execution sizing and portfolio limits already cap the actual risk. Spencer's call.
2. **Should `data/dex_state.json` be defensively purged on Anvil startup?** Currently the file persists across restarts. FID-225 round 2 handles phantom classification at runtime, but a stale file from a previous session could re-introduce phantoms. Spencer's call.
3. **Does this FID supersede FID-226, or is it a follow-on?** FID-226's probe-escape and universe-expansion fixes are correct and shipped. This FID addresses the remaining bottlenecks that FID-226's changes exposed. Recommend treating as follow-on (227) rather than amendment.
