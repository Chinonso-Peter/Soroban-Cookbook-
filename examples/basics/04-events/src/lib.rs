//! # Events Contract
//!
//! Demonstrates Soroban event emission and query-friendly topic design:
//!
//! ## Basics
//! - Event structure: topics (up to 4) + data payload
//! - Deterministic event emission for testing
//! - Multiple event types with distinct topics
//!
//! ## Query-Friendly Design Patterns
//! Off-chain indexers (e.g., Stellar Horizon, custom listeners) filter events
//! by topic position. Designing topics intentionally lets callers narrow results
//! without scanning every event.
//!
//! ### Topic Layout Convention
//! ```text
//! topic[0]  — event category / action name  (always present, used as primary filter)
//! topic[1]  — primary entity (from-address, contract-id, pool-id …)
//! topic[2]  — secondary entity (to-address, token-id …)        [optional]
//! topic[3]  — sub-type or status                               [optional]
//! data      — non-indexed payload (amounts, metadata, structs)
//! ```
//!
//! ### Best Practices
//! - Put the most-commonly filtered field in the earliest topic position.
//! - Keep topics to `Symbol` / `Address` / small integers — they must be
//!   `Val`-serialisable and live inside the 4-topic limit.
//! - Reserve the data payload for values that are *read* after filtering but
//!   not used to filter (amounts, timestamps, raw bytes).
//! - Use a consistent first-topic naming scheme across all events in a contract
//!   so indexers can discover every event type from a single contract.
//!
//! Events are published via `env.events().publish()` and can be
//! queried off-chain for indexing and monitoring.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

/// Event-emitting contract demonstrating both basic emission and
/// query-friendly topic design.
#[contract]
pub struct EventsContract;

#[contractimpl]
impl EventsContract {
    // ==================== BASIC EMISSION ====================

    /// Emits a simple event with topic ("simple",) and data value.
    ///
    /// Off-chain query: filter topic[0] == "simple"
    pub fn emit_simple(env: Env, value: u64) {
        env.events().publish((symbol_short!("simple"),), value);
    }

    /// Emits a tagged event with topics ("tagged", tag) and data value.
    ///
    /// Off-chain query: filter topic[0] == "tagged" AND topic[1] == <tag>
    pub fn emit_tagged(env: Env, tag: Symbol, value: u64) {
        env.events().publish((symbol_short!("tagged"), tag), value);
    }

    /// Emits `count` events each with topics ("multi", index) and data index.
    ///
    /// Demonstrates sequential event emission within a single invocation.
    pub fn emit_multiple(env: Env, count: u32) {
        for i in 0..count {
            env.events().publish((symbol_short!("multi"), i), i as u64);
        }
    }

    // ==================== QUERY-FRIENDLY PATTERNS ====================

    /// Emits a transfer event following the 3-topic pattern:
    ///   topic[0] = "transfer"   — filters all transfer events
    ///   topic[1] = from         — filters transfers *from* a specific address
    ///   topic[2] = to           — filters transfers *to* a specific address
    ///   data     = amount       — read after filtering; not used to filter
    ///
    /// Off-chain query examples:
    ///   • All transfers:                topic[0] == "transfer"
    ///   • All sends by Alice:           topic[0] == "transfer" AND topic[1] == Alice
    ///   • All receives by Bob:          topic[0] == "transfer" AND topic[2] == Bob
    ///   • Alice → Bob transfers only:   topic[0] == "transfer" AND topic[1] == Alice AND topic[2] == Bob
    pub fn emit_transfer(env: Env, from: Address, to: Address, amount: u64) {
        // Put action name first so every transfer is discoverable with one filter.
        // Put from/to next so indexers can build per-address history efficiently.
        env.events()
            .publish((symbol_short!("transfer"), from, to), amount);
    }

    /// Emits a namespaced event using a 3-topic hierarchy:
    ///   topic[0] = category (e.g. "defi")
    ///   topic[1] = action   (e.g. "swap")
    ///   topic[2] = pool_id  (any Symbol identifier)
    ///   data     = amount
    ///
    /// This pattern is useful when a single contract owns multiple logical
    /// sub-systems. Indexers can either:
    ///   • Catch all "defi" events   → filter topic[0] == "defi"
    ///   • Catch all swaps           → filter topic[0] == "defi" AND topic[1] == "swap"
    ///   • Catch swaps on one pool   → all three topics fixed
    ///
    /// Keep category and action as short Symbols (≤ 9 chars, symbol_short!).
    pub fn emit_namespaced(env: Env, category: Symbol, action: Symbol, pool_id: Symbol, amount: u64) {
        env.events()
            .publish((category, action, pool_id), amount);
    }

    /// Emits a status-change event with a 4-topic layout:
    ///   topic[0] = "status"
    ///   topic[1] = entity_id  (which entity changed)
    ///   topic[2] = old_status
    ///   topic[3] = new_status
    ///   data     = ledger sequence (for ordering / deduplication off-chain)
    ///
    /// Using all 4 topics lets off-chain systems query:
    ///   • Any status change for entity X
    ///   • Any transition *from* a specific state (e.g. "pending" → anything)
    ///   • Specific old → new transitions for audit trails
    pub fn emit_status_change(env: Env, entity_id: Symbol, old_status: Symbol, new_status: Symbol) {
        let ledger = env.ledger().sequence();
        env.events()
            .publish((symbol_short!("status"), entity_id, old_status, new_status), ledger);
    }
}

mod test;
