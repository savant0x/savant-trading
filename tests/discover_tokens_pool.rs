// FID: count the actual post-discovery token pool across enabled chains.
// Prints counts — does not assert so we always see real numbers.

use savant_trading::data::token_discovery::{
    discover_tokens, load_token_store, save_token_store, seed_token_store_from_static,
    TokenStoreEntry,
};
use savant_trading::execution::dex::ARBITRUM_TOKENS;
use std::collections::{BTreeMap, HashMap};

#[test]
fn pool_static_arbitrum_seed() {
    let n = ARBITRUM_TOKENS.len();
    println!(
        "\n[POOL] static ARBITRUM_TOKENS (Arbitrum-only seed): {} entries",
        n
    );
}

#[test]
fn pool_persist_static_seed_to_tokens_json() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    std::fs::create_dir_all(format!("{}/data", dir)).ok();
    let path = format!("{}/data/tokens.json", dir);
    let entries: Vec<TokenStoreEntry> = seed_token_store_from_static(&path, ARBITRUM_TOKENS, 42161);
    save_token_store(&path, &entries).expect("save_token_store failed");
    let loaded = load_token_store(&path);
    println!(
        "\n[PERSIST] wrote {} entries to {} ; re-loaded {}",
        entries.len(),
        path,
        loaded.len()
    );
}

#[tokio::test]
async fn pool_discover_active_tokens() {
    println!(
        "\n[DISCOVER] calling discover_tokens(min_volume=50_000, min_holders=50, limit=200)..."
    );
    let started = std::time::Instant::now();
    let result = discover_tokens(50_000.0, 50, 200).await;
    let elapsed_secs = started.elapsed().as_secs();
    println!("[DISCOVER] elapsed: {}s", elapsed_secs);
    let tokens: Vec<savant_trading::data::token_discovery::DiscoveredToken> = match result {
        Ok(t) => t,
        Err(e) => {
            println!("[DISCOVER] ERROR: {:?}", e);
            return;
        }
    };

    println!("[DISCOVER] total returned: {}", tokens.len());

    // Group by symbol to spot duplicates
    let mut by_symbol: HashMap<String, u32> = HashMap::new();
    for t in &tokens {
        *by_symbol.entry(t.symbol.clone()).or_insert(0) += 1;
    }
    let dup_count = by_symbol.values().filter(|c| **c > 1).count();
    println!(
        "[DISCOVER] distinct symbols: {} ; symbol-dup groups: {}",
        by_symbol.len(),
        dup_count
    );

    // Volume histogram (USDT-quoted 24h)
    let mut buckets: BTreeMap<&str, u32> = BTreeMap::new();
    for t in &tokens {
        let b = match t.volume_24h {
            v if v < 100_000.0 => "< $100k",
            v if v < 500_000.0 => "$100k-$500k",
            v if v < 1_500_000.0 => "$500k-$1.5M",
            v if v < 10_000_000.0 => "$1.5M-$10M",
            v if v < 100_000_000.0 => "$10M-$100M",
            _ => "> $100M",
        };
        *buckets.entry(b).or_insert(0) += 1;
    }
    println!("[DISCOVER] volume histogram:");
    for (b, c) in &buckets {
        println!("[DISCOVER]   {:<14} {}", b, c);
    }

    // Holder buckets
    let mut hbuckets: BTreeMap<&str, u32> = BTreeMap::new();
    for t in &tokens {
        let b = match t.holders {
            h if h < 100 => "< 100",
            h if h < 1_000 => "100-1k",
            h if h < 10_000 => "1k-10k",
            h if h < 100_000 => "10k-100k",
            _ => "> 100k",
        };
        *hbuckets.entry(b).or_insert(0) += 1;
    }
    println!("[DISCOVER] holder histogram:");
    for (b, c) in &hbuckets {
        println!("[DISCOVER]   {:<10} {}", b, c);
    }

    // Top 25 by volume
    let mut sorted = tokens.clone();
    sorted.sort_by(|a, b| {
        b.volume_24h
            .partial_cmp(&a.volume_24h)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    println!("[DISCOVER] top 25 by 24h volume:");
    for t in sorted.iter().take(25) {
        println!(
            "[DISCOVER]   {:8}  addr={:44}  vol=${:>14.0}  holders={:>8}",
            t.symbol, t.address, t.volume_24h, t.holders
        );
    }

    // Count passing realistic filter: $1.5M 24h AND >=100 holders
    let pass_1_5m = tokens
        .iter()
        .filter(|t| t.volume_24h >= 1_500_000.0 && t.holders >= 100)
        .count();
    let pass_500k = tokens
        .iter()
        .filter(|t| t.volume_24h >= 500_000.0 && t.holders >= 100)
        .count();
    println!(
        "[DISCOVER] tokens with vol>=500k AND holders>=100: {}",
        pass_500k
    );
    println!(
        "[DISCOVER] tokens with vol>=1.5M AND holders>=100: {}",
        pass_1_5m
    );
}
