<!-- markdownlint-disable MD024 -->

# FID-226: Conviction-Gate Probe Escape + Universe Expansion via scan_all_pairs

| Field | Value |
|-------|-------|
| **FID** | 226 |
| **Status** | verified (build clean; 9/9 is_probe tests green; clippy 0/0; workspace tests green). v1 + v2 normalization amendment applied 2026-06-21. |
| **Severity** | high |
| **Author** | Buffy (Codebuff CLI substrate, minimax-m3) |
| **Operator** | Spencer |
| **Created** | 2026-06-21 ~04:00 UTC |
| **Sibling FIDs (linked)** | FID-225 (phantom-position-halt, v0.15.7-a.1 ship), FID-222.5 (regime-conviction-threshold audit), FID-184 (probe path spec), FID-198 (probe exit hatch doctrine), FID-126 (conviction-weighted entry) |
| **Engine call site coincident** | engine/mod.rs:173-196 + engine/mod.rs:4322 + engine/mod.rs:2052-2120 (pre-scoring) |

## Summary

Savant's Anvil-fork bot run on 2026-06-21 from 06:32 AM to 04:01 PM (9.5h, 206 cycles) produced **0 entries, 851 PASS lines, and conviction=0.000 on all but one pair** — even after FID-225's v0.15.7-a.1 hotfix cleared the wallet-reconciliation crash. Two independent root causes decompose the dead state, and each gets its own surgical fix:

1. **Universe collapse**: `config/{default,test-anvil}.toml:scan_all_pairs = false` makes `engine/mod.rs:173` route to a static 18-pair TOML list. Hygiene filters drop that to ~12. With `scan_all_pairs = true`, the engine calls `candle_api.discover_safe_usd_pairs()` (Kraken REST AssetPairs + Ticker, defined at `src/data/candle_client.rs:261`) and surfaces ~150-300 tradable USD pairs.
2. **Probe-blindness in conviction gate**: `RegimeLabel::probe_threshold()` does NOT exist on `decision_parser.rs` (zero hits in src/ pre-fix) and the verdict gate at line 496 has no probe exempt branch, so any `is_probe=true` Buy/Sell made it into the [probe_threshold, main_threshold] conviction zone gets force-downgraded to Pass — silently killing FID-184/FID-198's probe path even though downstream sizing at `engine/mod.rs:4322` is already wired.

Both fixes ship together as a single change because a single-cause fix would still leave the other dead end. The probe-escape specifically EXPECTS a larger universe — without it, the larger universe would just produce more PASS lines, not more entries.

## Environment

- **OS:** Windows 11
- **Language/Runtime:** Rust 1.85+
- **Tool versions:** cargo 1.85, pre-push hook enforces `cargo fmt --check` + clippy (-D warnings) + test gates
- **Base commit:** v0.15.7-a.1 (FID-225 ship, 2026-06-21 ~03:00 UTC)
- **Observational evidence:** `logs/terminal/next-server (v16.2.7).txt` (1.0 MB, 8,939 lines, 06:32 AM → 04:01 PM)
- **Affected files (this FID ONLY):**
  - `src/agent/soul.md` — line 83 + line 120 (regime doctrine + probe escape path)
  - `src/agent/decision_parser.rs` — `RegimeLabel::probe_threshold()` method; verdict gate at ~496; 3 unit tests in `#[cfg(test)] mod tests`
  - `config/test-anvil.toml` — `scan_all_pairs = false` → `true`
  - `config/default.toml` — `scan_all_pairs = false` → `true` (replaces stale FID-189 comment)
  - `dev/fids/FID-2026-0621-226-...` — this doc (NEW)

## Detailed Description

### Problem

File-cited log output from the multi-hour dead-state session:

```text
[INFO] [PHASE2] 12 pairs queued for LLM evaluation
[INFO] [Judge] Judge: Pass (BUY:1, SELL:0, HOLD:9, consensus: 90%)
[INFO] PARSE_DECISION_OUT action=Pass pair=BTC/USD conviction=0.000 confidence=0.000 regime=Ranging gate_disabled=true is_entry=false entry=0.0000 stop=0.0000 tp1=0.0000
[INFO] PARSE_DECISION_OUT action=Pass pair=ETH/USD conviction=0.000 confidence=0.000 regime=Ranging gate_disabled=true is_entry=false entry=0.0000 stop=0.0000 tp1=0.0000
... (repeated ~850 times across 206 cycles)
```

Across 206 cycles the verdict distribution was **90% HOLD** and the only non-zero conviction signal (FET/USD at 0.130) was vetoed by FID-206 (Pass-with-conviction > 0.10 logs a WARN, per `decision_parser.rs:~520`).

### Expected Behavior

A scalping bot on a quiet weekend with $50 starting capital should:

1. Scan a broad USD-pair universe (~150-300, not 12) sourced from Kraken AssetPairs
2. Let marginal conviction signals (0.03-0.08 zone, well below the main threshold) surface as **probes** — 0.5× sizing, auto-TP at 0.6%, 10-minute timeout — to generate trade-flow data for later strategy validation
3. Stop emitting PASS for everything; reach at least one entry per hour of running on a non-zero-volatility session
4. Produce a meaningful BUY/SELL/HOLD verdict distribution that an operator can debug

### Root Cause A — Universe Collapse

`src/engine/mod.rs:173-196` (verbatim):

```rust
let active_pairs = if config.trading.scan_all_pairs {
    match candle_api.discover_safe_usd_pairs(
        config.trading.min_volume_24h_usd,
        config.trading.min_price_usd,
        &config.trading.blacklisted_symbols,
    ).await { /* curated_pairs extends */ }
} else {
    config.trading.pairs.clone()  // ← static 18 from TOML
};
```

`config/default.toml:107-109` (pre-fix):

```toml
# FID-189: scan_all_pairs = false activates the existing smart pre-scoring
# at engine/mod.rs:2052-2120. ...
scan_all_pairs = false
```

Plus `src/engine/mod.rs:202-211` actively curates `curated_pairs` — but `curated_pairs` is a Union (not intersection) with `active_pairs`, so the larger scan_all_pairs universe correctly expands `curated_pairs` rather than collapsing it.

FID-189's rationale (avoid LLM default-to-hold bias by pre-scoring only profitable pairs) is now superseded by FID-222's funnel (top-K selector via 6-signal composite scoring on a 200-250 candidate set). The pre-scoring rationale for `scan_all_pairs=false` no longer applies.

### Root Cause B — Probe-Blindness in Gate

`src/agent/decision_parser.rs:496` (PRE-FIX, verbatim):

```rust
let regime_threshold = decision.regime_label.conviction_threshold();
if !bypass_gates && is_entry && decision.conviction_score < regime_threshold {
    tracing::info!("FID-126 Conviction gate: ...");
    decision.action = TradeAction::Pass;
    decision.override_source = Some("conviction_gate".to_string());
}
```

There is no `probe_threshold()` method on `RegimeLabel` (verified: `grep -rn 'probe_threshold' src/` returns zero hits pre-fix). The verdict gate has no awareness of `is_probe`. So:

- LLM outputs `is_probe=true, conviction=0.04, regime=Trending` (per `src/agent/prompts/strategy_knowledge.md:19,45,152` and `src/agent/prompts/output_format.md:118,122`)
- Engine compares `0.04 < 0.05` (Trending threshold) → true → downgrades to `Pass`
- Downstream code at `engine/mod.rs:4322, 4436, 4582, 4620` (which DOES `if decision.is_probe { ... 0.5x sizing ... }`) is unreachable

The **probe path was always intended but never wired at the gate** — FID-184 specified it, FID-198 dialect-converged the prompt around it, but the convoy never made it to the verdict gate.

### Combined Effect

- 12 candidates × near-zero conviction signals → 0 entries (today)
- 150-300 candidates × conviction=0.04 marginal signals × probe-blind gate → **still** 0 entries
- 150-300 candidates × conviction=0.04 marginal signals × probe-aware gate → marginal entries become 0.5× probes (FID-184's intended flow)

The two fixes are coupled. Either alone leaves the other dead end.

### Evidence

```text
src/agent/decision_parser.rs:63-82     ← conviction_threshold() (4 regime arms)
src/agent/decision_parser.rs:496-510   ← verdict gate (no probe branch PRE-FIX)
src/engine/mod.rs:173-196              ← scan_all_pairs branch
src/engine/mod.rs:4322                 ← existing is_probe sizing (unreachable pre-fix)
config/default.toml:107-109            ← FID-189 stale rationale comment + scan_all_pairs=false
config/test-anvil.toml:82              ← scan_all_pairs=false
src/agent/soul.md:83,120               ← old 0.30/0.40 doctrine (predates FID-184; trilemma noted in FID-222.5)
src/agent/prompts/strategy_knowledge.md:19,152 ← "Probe Threshold" + "engine treats probes as 0.5x sizing"
src/agent/prompts/output_format.md:118,122     ← regime probe values
logs/terminal/next-server (v16.2.7).txt        ← 851 PASS lines / 206 cycles / 0 entries
```

## Impact Assessment

### Affected Components

- Engine cycle dispatch (active_pairs source)
- Decision-parser gate (probe escape branch — new logic)
- LLM reachable conviction zone (0.03–0.05 Trending band, 0.05–0.10 Ranging, 0.08–0.15 Volatile now feed the probe path)
- Trade-flow generation (zero since 2026-06-21 morning)
- Operator compounding thesis ($50 → $103 in 5 days, per soul.md §2.1) — dead

### Risk Level

- [ ] Critical: System crash, data loss, or security vulnerability
- [x] High: Major feature broken (BUY-signal pipeline dead), no workaround
- [ ] Medium: Feature degraded, workaround exists
- [ ] Low: Minor issue, cosmetic, or edge case

A bot that never buys never loses; it also never produces empirical data. Spencer's Anvil testnet run stuck on the "$50 → ?" thesis has no testable signal either way.

## Proposed Solution

### Approach

Two surgical, additive changes:

1. **Universe expand**: flip `scan_all_pairs = true` in both TOML configs.
2. **Probe escape**: add `RegimeLabel::probe_threshold(self) -> f64` returning regime-specific probe thresholds (Trending 0.03 / Volatile 0.08 / Ranging 0.05 / GreyZone 0.10). Update verdict gate to honor `is_probe=true` between probe_threshold and conviction_threshold.

Both changes align with the prompt contract already documented in `output_format.md` and `strategy_knowledge.md`. No new prompt text needed.

### Steps

#### File 1: `src/agent/soul.md` — ✅ DONE

- Line 83 updated to regime thresholds (Trending 0.05, Volatile 0.15, Ranging 0.10, GreyZone 0.20), plus new paragraph on probe path with explicit per-regime probe thresholds.
- Line 120 updated with FID-126 / FID-184 / FID-198 traceability + probe-escape paragraph.

#### File 2: `src/agent/decision_parser.rs` — ✅ EDITS MADE

- Added `pub fn probe_threshold(self) -> f64` to `impl RegimeLabel` after `conviction_threshold()`. Returns Trending 0.03 / Volatile 0.08 / Ranging 0.05 / GreyZone 0.10 per `output_format.md:118`.
- Updated verdict gate at ~496: when `is_probe=true && conviction_score >= probe_threshold_val`, log `FID-198 PROBE EXIT` and ALLOW BUY/SELL through (the engine's `if decision.is_probe` sizing at `engine/mod.rs:4322` takes over). Below probe_threshold with is_probe=true: still downgrade to Pass + override_source = "conviction_gate".
- Added 3 unit tests in `mod tests`:
  - `is_probe_conviction_between_probe_and_main_thr_passes` (Trending 0.04 + is_probe → Buy, no override_source)
  - `is_probe_conviction_below_probe_thr_downgrades` (Trending 0.02 + is_probe → Pass + override_source = "conviction_gate")
  - `is_probe_disabled_with_low_conviction_still_downgrades` (Trending 0.04 + is_probe=false → Pass + override_source = "conviction_gate")

#### File 3: `config/test-anvil.toml` — ✅ DONE

`scan_all_pairs = false` → `scan_all_pairs = true`. Comment block extends FID-189 stub to call out the FID-226 + FID-225 lineage so future readers know the rollback intent.

#### File 4: `config/default.toml` — ✅ DONE

Same flag flip, longer comment block since the existing FID-189 rationale comment is now stale and would mislead any operator troubleshooting in-flight.

### Verification

Perfection loop (run in order):

```bash
# 1. Format gate (pre-push hook expects clean)
cargo fmt --all -- --check

# 2. Clippy gate (pre-push hook expects zero)
cargo clippy --all-targets -- -D warnings

# 3. Specific module tests (new tests + existing is_probe tests must pass)
cargo test --lib agent::decision_parser -- --nocapture

# 4. Full workspace regression
cargo test --workspace --all-targets
```

Specific regression matrix (must remain green):

| Test | Existing expectation | After fix |
|------|---------------------|-----------|
| `conviction_gate_blocks_low_conviction` (line 1367) | conviction=0.03 + is_probe=false → Pass | ✅ unchanged |
| `is_probe_field_parses_when_true` (line 1561) | is_probe=true (only assertion) | ✅ passes either way (action now Buy after probe-exit fires; old test asserted only is_probe=true) |
| `is_probe_field_defaults_to_false_when_missing` (line 1584) | is_probe=false | ✅ unchanged |
| `is_probe_field_defaults_to_false_when_false` (line 1606) | is_probe=false | ✅ unchanged |
| NEW `is_probe_conviction_between_probe_and_main_thr_passes` | — | NEW: Trending 0.04 + is_probe → Buy |
| NEW `is_probe_conviction_below_probe_thr_downgrades` | — | NEW: Trending 0.02 + is_probe → Pass |
| NEW `is_probe_disabled_with_low_conviction_still_downgrades` | — | NEW: Trending 0.04 + is_probe=false → Pass |

Anvil manual smoke test by operator (out-of-band verification):

1. Restart engine with `config/test-anvil.toml` (where `live_execution=true, is_anvil=true`)
2. Observe `[PHASE1]` log: `discover_safe_usd_pairs(...) → 153 active pairs` (count varies per Kraken live data) — was `[PHASE2] 12 pairs queued` pre-fix
3. Observe `[DECISION]` log line within first 5 cycles: at least one `is_entry=true` entry. Could be:
   - Full conviction (≥ regime_threshold): normal Buy/Sell
   - Probe escape (probe_threshold ≤ conviction < regime_threshold AND is_probe=true): PROBE EXIT log + Buy/Sell with 0.5x sizing
4. Verify in `data/positions.json` (or equivalent) that the probe-sized position closes at 0.6% target OR 10-min timeout

## Perfection Loop

### Loop 1 (this doc — STATUS: open)

- **RED:** 0 BUY signals over 206 cycles despite FID-225 hotfix
- **GREEN:** soul.md update + probe_threshold method + verdict gate escape + scan_all_pairs=true + 3 unit tests
- **AUDIT:** cargo fmt + clippy + decision_parser unit tests + workspace regression + Anvil smoke
- **CHANGE DELTA:** ~25 LOC new (probe_threshold 12 LOC + verdict gate refactor 8 LOC + 3 unit tests 35 LOC + soul.md 12 LOC + 2 config flips)

### Loop 2 (if needed) — pending empirical data

Per FID-222.5 §Recommendation (Option D): after ≥3 post-ship cycles, review conviction distribution + verdict distribution + ProbeExitHit-Rate:

- If ProbeExit fires >50% of Buy signals → probe threshold too permissive; tighten to 0.04 minimum (Trending)
- If Buy rate <5% across 30 cycles despite universe expansion → engine-side filters binding (FID-126-R6 not shipped; refile as separate FID-126-R6 block)
- If jury-failure rate climbs above 80% post-expansion → jury calibration drift; separate FID

## Resolution (v1 verified locally; v2 normalization amendment applied; awaiting operator commit + push)

- **Fixed By:** Buffy (Codebuff CLI substrate, minimax-m3)
- **Fixed Date:** 2026-06-21 ~04:00 UTC (v1 start) / 2026-06-21 ~04:30 UTC (v1 verified) / 2026-06-21 ~04:50 UTC (v2 amendment: is_probe normalization block + extra tests + LIVE-EXEC WARNING)
- **Tests Added (cumulative):**
  - v1 (3 NEW): probe-exit, probe-below, probe-disabled
  - v2 (3 NEW): `is_probe_normalized_false_above_main_threshold`, `is_probe_normalized_false_below_probe_threshold`, `is_probe_stays_in_design_window_all_four_regimes`
  - v2 (1 MUTATION): `is_probe_conviction_below_probe_thr_downgrades` - assertion flipped from `assert!(decision.is_probe)` to `assert!(!decision.is_probe)` (post-normalization behavior)
- **Verified By:** `cargo fmt --all -- --check` clean / `cargo build --lib --tests` clean / 9/9 `is_probe*` tests green / `cargo clippy --all-targets -- -D warnings` 0 errors / 0 warnings / `cargo test --workspace --all-targets` all green
- **Commit/PR:** pending Spencer review + push
- **CHANGELOG / version bump:** pending - v0.15.8-minor recommended (semver: minor for is_probe escape hatch feature surface)
- **Archived:** pending - doc moves to `dev/fids/archive/` on operator commit per FID-TEMPLATE convention

## Lessons Learned (preliminary)

### L1: Two-cause decomposition prevents scope creep

The user reported "0 BUYs over 12 hours" which on first blush looks like one problem (LLM HOLD bias). File-cited evidence resolved to TWO independent root causes (universe + probe-gate). Without decomposition, a single-fix proposal ("further lower conviction thresholds") would only have addressed Cause B partially. With decomposition, both fixes land clean and the operator gets a coherent fix.

### L2: is_probe path was always intended (FID-184) but never wired at the gate

The engine HAS `is_probe` sizing logic at `engine/mod.rs:4322, 4436, 4582, 4620` — it was written assuming the conviction gate would honor probe. The unsurfaced bug was: the gate didn't honor probe. Pure-feature, no-prompt-change fix. **Common class bug**: "library exists, gate blocks, sits dormant." Lesson: when adding new structured protocol (is_probe, is_hedge, etc.), trace end-to-end from prompt → parser → engine gate → sizing → execution. If any link is missing, the protocol is dead and future operators will misdiagnose it as "LLM broken."

### L3: Threshold trilemma (FID-222.5) was a real bug in our doc/code consistency

`src/agent/soul.md` v3.0 still had 0.30/0.40 thresholds from the FID-126 original spec while the parser at 0.05/0.10/0.15/0.20 had been lowered per FID-184. FID-222.5 surfaced the trilemma. Had we not done that audit first, today's FID-226 fix would still have worked, but the doctrinal text would mislead the LLM through its next prompt-cache cycle (or operator eye-balling). Updating soul.md FIRST, then engine, means downstream FID-226 fixes ship with doctrine alignment.

### L4: scan_all_pairs was disabled by FID-189 with rationale — but that rationale no longer applies post-FID-222

FID-189's `scan_all_pairs=false` rationale (avoid LLM default-to-hold bias via pre-scoring) was sound WITHOUT the funnel. With FID-222 live, the funnel replaces the pre-scoring as the top-K selector. The rationale is structurally obsolete but the flag stayed at `false`. **Lesson**: when adding a new feature that supersedes an old one's rationale, audit prior config flags + their FID-annotated rationales for staleness.

### L5: Pure `RegimeLabel::probe_threshold()` self-testifies the contract

Adding a method that returns values already documented in prompts makes the contract grep-able, source-cited, and testable. Future FID touching thresholds can compare their values against the prompt + this method in a single grep. **Lesson**: don't bury threshold constants behind scattered match arms; expose them as methods on the type so the prompt → code linecount gap is single-grep.

## Open Questions / Threads

1. **Empirical tuning after 3 cycles** — if ProbeExit fires too often, tighten probe minimums.
2. **Default.toml scan_all_pairs risk** — flipping the production flag (`live_execution = false` currently: safe-as-paper-trade; risky if a future run flips live_execution without re-reading this commentary).
3. **Pre-existing baseline conviction distribution** — record baseline *before* operator commits so post-ship comparison is meaningful.
4. **FID-126-R6 (engine-side filter trace)** — this fix unblocks funnel-effectiveness measurement, but engine-side filters still aren't traced.
5. **Probe-position 10-min timeout wiring** — confirmed at engine/mod.rs (lines 4322, 4582) that 0.5x sizing + auto-TP + 10-min timeout path exists, but the cleanup task to **remove the `SAVANT_GATE_DISABLED=1` env var bypass** after FID-126-R3 ships (which FID-126-R3 hasn't shipped) is now extra-relevant.

---

## v2 Amendment - Is-Probe Normalization + Tests + LIVE-EXEC WARNING (2026-06-21 ~04:50 UTC)

After v1 landed, a thinker audit (model: gemini-thinking, source-cited) surfaced **two latent failure modes** that v1's gate fix alone would have exposed:

### Failure-mode 1 - high-convicted trade tagged as probe

LLM emits `conviction_score >= regime_threshold` AND `is_probe=true` (per `src/agent/prompts/output_format.md:122` - prompt does not forbid setting is_probe=true above the main threshold). v1's verdict-gate outer `if !bypass_gates && is_entry && decision.conviction_score < regime_threshold` block is bypassed when conviction is high. Action stays Buy/Sell - but `is_probe=true` remains intact. Downstream code at `src/engine/mod.rs:4322, 4436, 4582, 4620` (`if decision.is_probe { ... 0.5x sizing + auto-TP at 0.6% + 10-min timeout ... }`) would have executed the high-conviction trade at 0.5x sizing and auto-exit in 10 minutes - a premature-exit pattern.

### Failure-mode 2 - same concern under `bypass_gates=true` (FID-126-R3 env-var bypass)

Under `SAVANT_GATE_DISABLED=1` (FID-126-R3 diagnostic-debug mode), the verdict-gate downcast branch is skipped. `is_probe=true` with below-probe conviction executes as a 0.5x probe - also premature.

### Fix

Inserted a `FID-226 v2 normalization` block in `src/agent/decision_parser.rs` immediately before the verdict gate's outer `if !bypass_gates ...`. The block clears `decision.is_probe` to `false` when conviction is outside the `[probe_threshold, regime_threshold)` design window - regardless of the LLM's intent. After normalization, downstream gate logic sees a clean `is_probe` that's only `true` within the design window.

```rust
{
    let regime_t = decision.regime_label.conviction_threshold();
    let probe_t = decision.regime_label.probe_threshold();
    if decision.is_probe
        && !(decision.conviction_score >= probe_t
            && decision.conviction_score < regime_t)
    {
        tracing::debug!(
            "FID-226 normalize: pair={} conviction={:.3} outside [probe {:.3}, main {:.3}) -- clearing is_probe",
            decision.pair, decision.conviction_score, probe_t, regime_t
        );
        decision.is_probe = false;
    }
}
```

### Truth table (post-v2)

| conviction | regime | is_probe (LLM) | is_probe (post-norm) | Verdict gate | Final action | is_probe for sizing |
| --- | --- | --- | --- | --- | --- | --- |
| 0.02 | Trending | true | false (below probe) | downcast | Pass + override=cg | n/a |
| 0.04 | Trending | true | true (in window) | probe exit | Buy | true (0.5x + 10-min) |
| 0.10 | Trending | true | false (above main) | passes outer | Buy | false |
| 0.20 | Trending | false | false | passes outer | Buy | false |
| 0.04 | Trending | false | false (already) | downcast | Pass + override=cg | n/a |

### Tests (v2)

- `is_probe_normalized_false_above_main_threshold` - Trending conviction=0.10 + is_probe=true -> action=Buy, is_probe=false (proves no auto-shrink on high-conv)
- `is_probe_normalized_false_below_probe_threshold` - Trending conviction=0.02 + is_probe=true -> action=Pass, is_probe=false, override_source=conviction_gate
- `is_probe_stays_in_design_window_all_four_regimes` - parametric across Trending / Volatile / Ranging / GreyZone at mid-zone conviction -> action=Buy, is_probe=true (proves per-regime `probe_threshold()` values match prompt spec)
- **MUTATED v1 test**: `is_probe_conviction_below_probe_thr_downgrades` - assertion flipped to `assert!(!decision.is_probe)` (post-normalization behavior)

### Config hardening (LIVE-EXEC WARNING block in `config/default.toml`)

`config/default.toml:live_execution = false` (paper-mode) so v2 added a comment block right after `scan_all_pairs = true`:

> **FID-226 LIVE-EXEC WARNING**: with `scan_all_pairs=true` AND `live_execution=true` (the production switch), the engine dispatches ~150-300 USD-pair candidates per cycle instead of 18. Capital exposure breadth increases 10-17x. The conviction gate (probe escape hatch) still filters weak signals, but the *breadth* of attempted trades climbs. Tune `risk_pct` and `max_positions` accordingly BEFORE flipping `live_execution = false -> true`.

This is a "do-not-trip" warning for operators who toggle `live_execution` without re-reading FID-226. The operator is responsible for the *after* aspect (risk tuning); this warning is the *before* aspect.

### Validation (v2 final pass)

```text
cargo fmt --all -- --check                  # clean
cargo build --lib --tests                  # clean
cargo test --lib agent::decision_parser    # 9/9 is_probe tests pass
cargo clippy --all-targets -- -D warnings  # 0 errors / 0 warnings
cargo test --workspace --all-targets       # all green
```

### Lesson L6 (v2 catch): always normalize boolean sentinel fields against the documented design window

A boolean sentinel field like `is_probe` that the LLM is allowed to set with multiple intent states (yes / no / maybe) needs explicit defensive normalization when the gate reaches it. Without normalization, type-confusion between "high-conv tagged as probe" and "real probe in middle band" produces silent mis-execution in downstream sizing (premature-exit, undersizing).

**Lesson**: treat any LLM-emitted boolean flag with semantic state ("between thresholds", "above main", "below probe") as a prompt contract - wrap it in a normalize-pass at the gate, document the window, and ship a guard test per edge.

### Lesson L7 (v2 catch): missing-angle audits surface dormant bugs

The "include the missed angels" directive in the operator feedback was the catalyst for surfacing these two failure modes - both of which would have shipped silently if the audit had been skipped. The thinker audit played the role of "second pair of eyes" that the FID-225 reviewer's APPROVE-with-nits missed.

**Lesson**: when the operator flags "AI slop" risk, run a structural second audit (thinker gemini or code-reviewer) BEFORE claiming fix complete. Don't trust v1 shipping without explicit v2 review.

---

*End of FID-226 active doc (v1 verified, v2 normalization amendment applied). Awaiting operator commit + archive to dev/fids/archive/.*

— Buffy (Codebuff CLI substrate, minimax-m3), 2026-06-21 ~04:00 UTC (v1 start) / 2026-06-21 ~04:30 UTC (v1 verified) / 2026-06-21 ~04:50 UTC (v2 amendment applied)
