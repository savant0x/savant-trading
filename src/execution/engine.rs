use async_trait::async_trait;

use crate::core::types::{Order, Position, Side};

/// FID-231: Stats returned by `process_retry_queue` for engine telemetry.
#[derive(Debug, Clone, Default)]
pub struct RetryQueueStats {
    /// Total items drained from queue at cycle start.
    pub attempted: usize,
    /// Pairs that succeeded on retry (cleared from queue permanently).
    pub succeeded: Vec<String>,
    /// Items pushed back with attempts incremented (still recoverable).
    pub requeued: Vec<String>,
    /// Items dropped because attempts >= max_retries (logged WARN).
    pub dropped: Vec<(String, String)>,
}

#[async_trait]
pub trait ExecutionEngine: Send + Sync {
    async fn place_order(
        &mut self,
        pair: &str,
        side: Side,
        quantity: f64,
        price: Option<f64>,
    ) -> Result<Order, crate::core::error::ExecutionError>;

    async fn close_position(
        &mut self,
        position_id: &str,
    ) -> Result<Order, crate::core::error::ExecutionError>;

    /// Close a specific quantity of a position (for TP scale-outs).
    /// Default: delegates to `close_position()` (full close).
    async fn close_position_partial(
        &mut self,
        position_id: &str,
        quantity: f64,
    ) -> Result<Order, crate::core::error::ExecutionError> {
        let _ = quantity;
        self.close_position(position_id).await
    }

    fn open_positions(&self) -> Vec<&Position>;

    fn balance(&self) -> f64;

    // ---- Optional lifecycle methods with safe defaults ----

    /// Kill switch — cancel all orders and close all positions.
    /// Default: no-op (safe for backends that don't support it).
    async fn kill(&mut self) -> Result<(), crate::core::error::ExecutionError> {
        Ok(())
    }

    /// Sync balance from the exchange (e.g. from exchange API).
    /// Default: no-op (uses locally tracked balance).
    async fn sync_balance(&mut self) -> Result<(), crate::core::error::ExecutionError> {
        Ok(())
    }

    /// Place a stop-loss order on the exchange for a tracked position.
    /// Default: no-op (stops are monitored client-side).
    async fn place_stop_loss(
        &mut self,
        _position_id: &str,
    ) -> Result<(), crate::core::error::ExecutionError> {
        Ok(())
    }

    /// Check if liquidity is available for a pair (read-only, no gas).
    /// Returns rich data: availability, tax, balance issues.
    /// Default: always available (paper trading).
    async fn check_liquidity(
        &self,
        _pair: &str,
        _side: Side,
        _amount_usd: f64,
    ) -> Result<crate::execution::dex::LiquidityCheck, crate::core::error::ExecutionError> {
        Ok(crate::execution::dex::LiquidityCheck {
            available: true,
            buy_tax_bps: 0,
            sell_tax_bps: 0,
            buy_amount: "0".to_string(),
            balance_ok: true,
            allowance_ok: true,
            price: "0".to_string(),
            buy_token_price_usd: None,
        })
    }

    /// Reconcile on-chain token balances with tracked positions.
    /// Returns list of (pair, on_chain_qty, tracked_qty) for discrepancies.
    /// Default: empty list (paper trading has no on-chain state).
    async fn sync_wallet_positions(&self, _curated_pairs: &[String]) -> Vec<(String, f64, f64)> {
        Vec::new()
    }

    /// Register a wallet-recovered position so close_position() can find it.
    /// Called during wallet sync for positions discovered on-chain but not in the executor.
    /// Default: no-op (paper trading doesn't need this).
    fn register_position(&mut self, _id: String, _pos: Position) {}

    /// Query on-chain ERC-20 token balance for a specific token.
    /// Returns Some(balance) or None if query fails.
    /// Default: None (paper trading has no on-chain state).
    async fn query_token_balance(&self, _token_address: &str, _decimals: u8) -> Option<f64> {
        None
    }

    /// Get the chain ID for this executor.
    fn chain_id(&self) -> u64 {
        0
    }

    /// FID-231: Drain the retry queue and re-attempt failed swaps.
    /// Default implementation returns empty stats (no retry queue).
    async fn process_retry_queue(&mut self) -> RetryQueueStats {
        RetryQueueStats::default()
    }

    /// FID-232: Set an external reference price for the spread filter.
    /// When set, the spread filter compares against this price instead of
    /// the DEX API's self-reported `quote.price`. Default: no-op.
    fn set_reference_price_override(&mut self, _price: Option<f64>) {}
}
