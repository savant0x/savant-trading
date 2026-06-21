# Token Universe Diagnostic — 2026-06-24

## Hypothesis A — Volume filter too aggressive

**Status: NOT the bottleneck (testnet)**

`test-anvil.toml:90` sets `min_volume_24h_usd = 1_500_000.0`. This filter is applied to
Kraken AssetPairs in `src/data/candle_client.rs:261` (`discover_safe_usd_pairs`). On a
**live** Kraken feed this would drop most low-liquidity pairs. But on **testnet Anvil** the
engine is not calling Kraken — it's loading from `data/tokens.json` via
`extend_token_db()` at `src/engine/mod.rs:233` and `:1996`. The tokens.json path does NOT
apply the volume filter — it ingests whatever is in the file.

## Hypothesis B — Funnel top-K is the real bottleneck

**Status: CONFIRMED — this is the #1 bottleneck**

Evidence from `dev/logs/funnel-rankings.jsonl` (173 lines, cycles 1-68):

- `funnel_enabled: false` on EVERY SINGLE LINE
- `hunt_mode: true` on every line
- `hunt_mode_bypass: true` on every line
- `output_K: 0` on every line (funnel returns 0 pairs)
- `input_N` grew from 12 → 27 between cycle 1 and cycle 68

This means the Funnel v1 pre-scorer (FID-222) — the top-K selector that is supposed to
narrow 150-300 Kraken pairs down to 12 — **is not running on testnet at all**. The
`hunt_mode_bypass` flag means the engine is in "hunt mode" (no real positions yet), and
the funnel code path short-circuits when hunt mode is active.

The config that would enable it (`[trading.funnel_v1] enabled = true`) exists in
`config/default.toml:174` but is **MISSING from `config/test-anvil.toml`** — there is
no `[trading.funnel_v1]` block in the testnet config at all.

Even if funnel were enabled, `hunt_mode_bypass: true` would still force `output_K: 0`
because the open-position safety guard injects all positioned pairs when no real
positions exist. With 0 positions, the guard injects everything → output_K=0.

## Hypothesis C — Conviction gate too tight

**Status: LIKELY CONTRIBUTING, but secondary to B**

With `funnel_enabled: false`, the engine falls back to evaluating ALL `input_N` pairs
through the LLM. At cycle 68, that's 27 pairs. The LLM (Haiku via the decision parser)
runs the conviction gate at `src/agent/decision_parser.rs:62-93`:

- Trending: threshold = 0.05 (probe escape = 0.03)
- Dynamic conviction = clamp((strong*1.0 + moderate*0.7 + weak*0.4) / 3.0, 0, 1)

The `/3.0` fixed denominator is punitive. If the LLM returns weak=1, moderate=0, strong=0
for a pair, conviction = 0.4/3.0 = 0.133. This passes Trending (0.05) but FAILS
Volatile (0.15). More importantly, the `MAX_NON_PASS=15` cap at `src/api/mod.rs:286`
means only 15 of the 27 pairs can produce non-PASS verdicts per cycle. The remaining 12+
pairs are silently dropped.

On testnet with 27 illiquid Anvil-forked pairs (most with near-zero real volume), the
LLM is likely assigning weak-or-lower scores to most of them, and the 15-pair cap
truncates the rest. Net effect: 0 BUY verdicts because none of the 27 pairs clear even
the lenient Trending threshold when the LLM has no real price/volume data to work with.

## Hypothesis D — `dex_state.json` phantom residue

**Status: NOT currently active**

`data/dex_state.json` content:
```json
{
  "positions": [],
  "closed_trades": [],
  "balance": 50.0,
  "order_counter": 11
}
```

Clean. No phantom positions. The `ai-1` phantom from v0.15.7 (28.33 units, $5.25
divergence at cycle ~34) has been purged — either by the FID-225 round 2 fix
(`src/execution/reconciliation.rs:1131-1136`, phantom classification + auto-purge) or
by manual cleanup. This is NOT the current blocker.

However, the defensive purge on Anvil startup (operator-routed decision from
`dev/vera/memory/2026-06-21-v0157-release.md:179`) is still not implemented. If the
engine is restarted without manually clearing `data/dex_state.json`, phantoms could
re-appear from a previous session's residue.

## Summary — Root cause ranking

| # | Cause | Impact | Fix complexity |
|---|-------|--------|----------------|
| 1 | **Funnel disabled on testnet** (`test-anvil.toml` missing `[trading.funnel_v1]`) | top-K selection never runs; engine evaluates all 27 pairs raw | TOML edit |
| 2 | **Hunt mode bypass** (`hunt_mode=true` because 0 positions) | Even if funnel were enabled, `output_K=0` until first position exists | Code change: seed initial pairs when hunt mode + 0 positions |
| 3 | **Volume filter on Kraken path** (`min_volume_24h_usd=1.5M`) | Irrelevant on testnet (tokens.json path doesn't apply it); would block 90% of pairs on live | Already correct for live; N/A for testnet |
| 4 | **Conviction gate + MAX_NON_PASS=15 cap** | 27 pairs compete for 15 non-PASS slots; most get dropped | Raise cap or pre-filter before LLM |
| 5 | **dex_state.json phantom residue** | NOT currently active; FID-225 round 2 fix handles it | Defensive purge still recommended |

## Recommended action

**Immediate (5 min):** Add `[trading.funnel_v1]` block to `config/test-anvil.toml`:

```toml
[trading.funnel_v1]
enabled = true
top_k = 12
min_score_threshold = 0.20
```

This alone will NOT fix the problem because of hunt_mode_bypass. The funnel needs a
"seed path" for the 0-position case: when `positioned_pairs.is_empty()`, either
(a) inject the top-K by score anyway, or (b) fall back to a static seed list of
high-liquidity pairs (BTC/USD, ETH/USD, etc.) so the LLM has something to evaluate.

**Follow-up:** The larger architectural question — does testnet Anvil even need the
funnel? The funnel is designed for a 150-300 pair Kraken universe. On Anvil, the
universe is 27 illiquid micro-caps from `data/tokens.json`. The funnel's 6-signal
composite scoring (EMA, RSI, ADX, vol, VWAP, BB) is meaningless on fake data. The
right fix may be: on Anvil, disable funnel AND bypass conviction gate AND raise
MAX_NON_PASS, so the LLM evaluates all 27 pairs with no pre-filter.
