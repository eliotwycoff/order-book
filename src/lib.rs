use std::collections::{BTreeMap, VecDeque};

pub struct Order {
    id: u64,
    amount: u64,
}

pub struct Orders {
    pub total: u64,
    pub queue: VecDeque<Order>,
}

pub struct OrderBook {
    best_ask: u64,
    best_bid: u64,
    asks: BTreeMap<u64, Orders>,
    bids: BTreeMap<u64, Orders>,
}

impl OrderBook {
    pub fn insert(&mut self, order: impl Into<Order>) {}
}
