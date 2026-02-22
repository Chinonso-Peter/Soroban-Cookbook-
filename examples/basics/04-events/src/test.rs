//! Unit tests for Events contract — basic emission, data validation,
//! topic verification, and query-friendly pattern validation.

#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events as _},
    Address, Env, Symbol, TryFromVal,
};

#[test]
fn test_event_emission_exists() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_simple(&100);

    let events = env.events().all();
    assert!(!events.is_empty(), "At least one event must be emitted");
}

#[test]
fn test_event_count_single() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_simple(&42);

    let events = env.events().all();
    assert_eq!(events.len(), 1, "emit_simple must emit exactly one event");
}

#[test]
fn test_event_count_multiple() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_multiple(&3);

    let events = env.events().all();
    assert_eq!(
        events.len(),
        3,
        "emit_multiple(3) must emit exactly 3 events"
    );
}

#[test]
fn test_emit_multiple_topics_match_indices() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_multiple(&3);

    let events = env.events().all();
    assert_eq!(events.len(), 3);

    for i in 0..3 {
        let event = events.get(i).unwrap();
        let (_contract_id, topics, _data) = event;
        assert_eq!(topics.len(), 2, "multi event must have 2 topics");

        let name: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
        let index: u32 = u32::try_from_val(&env, &topics.get(1).unwrap()).unwrap();
        assert_eq!(name, symbol_short!("multi"));
        assert_eq!(index, i);
    }
}

#[test]
fn test_topic_structure_simple() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_simple(&99);

    let events = env.events().all();
    let event = events.get(0).unwrap();
    let (_contract_id, topics, _data) = event;
    assert_eq!(topics.len(), 1, "Simple event must have 1 topic");
    let t0: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
    assert_eq!(t0, symbol_short!("simple"));
}

#[test]
fn test_topic_structure_tagged() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let tag = symbol_short!("mytag");
    client.emit_tagged(&tag, &50);

    let events = env.events().all();
    let event = events.get(0).unwrap();
    let (_contract_id, topics, _data) = event;
    assert_eq!(topics.len(), 2, "Tagged event must have 2 topics");
    let t0: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
    let t1: Symbol = Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap();
    assert_eq!(t0, symbol_short!("tagged"));
    assert_eq!(t1, tag);
}

#[test]
fn test_payload_values() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let value = 12345u64;
    client.emit_simple(&value);

    let events = env.events().all();
    let event = events.get(0).unwrap();
    let (_contract_id, _topics, data) = event;
    let payload: u64 = u64::try_from_val(&env, &data).unwrap();
    assert_eq!(payload, value, "Event data must match emitted value");
}

#[test]
fn test_action_differentiation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_simple(&1);
    client.emit_tagged(&symbol_short!("x"), &2);

    let events = env.events().all();
    assert_eq!(events.len(), 2);

    let (_id0, topics0, _) = events.get(0).unwrap();
    let (_id1, topics1, _) = events.get(1).unwrap();

    let t0: Symbol = Symbol::try_from_val(&env, &topics0.get(0).unwrap()).unwrap();
    let t1: Symbol = Symbol::try_from_val(&env, &topics1.get(0).unwrap()).unwrap();
    assert_eq!(t0, symbol_short!("simple"));
    assert_eq!(t1, symbol_short!("tagged"));
}

#[test]
fn test_no_extra_events() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_simple(&10);
    let events = env.events().all();
    assert_eq!(events.len(), 1, "Must not emit extra events");
}

#[test]
fn test_zero_events_on_empty_emit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    client.emit_multiple(&0);
    let events = env.events().all();
    assert_eq!(events.len(), 0, "emit_multiple(0) must emit zero events");
}

// ==================== QUERY-FRIENDLY PATTERN TESTS ====================

#[test]
fn test_emit_transfer_topic_layout() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.emit_transfer(&from, &to, &500);

    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let (_id, topics, data) = events.get(0).unwrap();

    // topic[0] must always be the action name for event-type filtering
    let action: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
    assert_eq!(action, symbol_short!("transfer"));

    // topic[1] = from-address; topic[2] = to-address
    // These positions enable per-address history queries off-chain.
    let t_from: Address = Address::try_from_val(&env, &topics.get(1).unwrap()).unwrap();
    let t_to: Address = Address::try_from_val(&env, &topics.get(2).unwrap()).unwrap();
    assert_eq!(t_from, from);
    assert_eq!(t_to, to);

    // amount lives in data — readable after filtering, but not a filter key
    let amount: u64 = u64::try_from_val(&env, &data).unwrap();
    assert_eq!(amount, 500);
}

#[test]
fn test_emit_transfer_independent_senders_queryable() {
    // Verifies that multiple transfers can be distinguished by topic[1] (sender).
    // An off-chain indexer watching topic[1] == alice sees only one event.
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let carol = Address::generate(&env);

    client.emit_transfer(&alice, &carol, &100);
    client.emit_transfer(&bob, &carol, &200);

    let events = env.events().all();
    assert_eq!(events.len(), 2);

    // Both events share the same action topic, so a "get all transfers" query works.
    for i in 0..2u32 {
        let (_id, topics, _data) = events.get(i).unwrap();
        let action: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
        assert_eq!(action, symbol_short!("transfer"));
    }

    // Sender is distinguishable via topic[1].
    let (_id0, topics0, _) = events.get(0).unwrap();
    let (_id1, topics1, _) = events.get(1).unwrap();
    let sender0: Address = Address::try_from_val(&env, &topics0.get(1).unwrap()).unwrap();
    let sender1: Address = Address::try_from_val(&env, &topics1.get(1).unwrap()).unwrap();
    assert_ne!(sender0, sender1, "Senders must be distinguishable via topic[1]");
}

#[test]
fn test_emit_namespaced_three_topic_hierarchy() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let category = symbol_short!("defi");
    let action = symbol_short!("swap");
    let pool = symbol_short!("pool1");

    client.emit_namespaced(&category, &action, &pool, &1000);

    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let (_id, topics, data) = events.get(0).unwrap();
    assert_eq!(topics.len(), 3, "Namespaced event must carry 3 topics");

    let t0: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
    let t1: Symbol = Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap();
    let t2: Symbol = Symbol::try_from_val(&env, &topics.get(2).unwrap()).unwrap();
    assert_eq!(t0, category);
    assert_eq!(t1, action);
    assert_eq!(t2, pool);

    let amount: u64 = u64::try_from_val(&env, &data).unwrap();
    assert_eq!(amount, 1000);
}

#[test]
fn test_emit_status_change_four_topics() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EventsContract);
    let client = EventsContractClient::new(&env, &contract_id);

    let entity = symbol_short!("order42");
    let old_s = symbol_short!("pending");
    let new_s = symbol_short!("filled");

    client.emit_status_change(&entity, &old_s, &new_s);

    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let (_id, topics, data) = events.get(0).unwrap();
    assert_eq!(topics.len(), 4, "Status-change event must use all 4 topic slots");

    let t0: Symbol = Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap();
    let t1: Symbol = Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap();
    let t2: Symbol = Symbol::try_from_val(&env, &topics.get(2).unwrap()).unwrap();
    let t3: Symbol = Symbol::try_from_val(&env, &topics.get(3).unwrap()).unwrap();
    assert_eq!(t0, symbol_short!("status"));
    assert_eq!(t1, entity);
    assert_eq!(t2, old_s);
    assert_eq!(t3, new_s);

    // data holds the ledger sequence for off-chain ordering / deduplication
    let _ledger: u32 = u32::try_from_val(&env, &data).unwrap();
}
