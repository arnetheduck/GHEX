use std::collections::BinaryHeap;
use objects::Order;

pub struct MatchingEngine {
    sell_orders: BinaryHeap<Order>, // min heap
    buy_orders: BinaryHeap<Order>, // max heap    
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine { 
    	MatchingEngine {
            sell_orders: BinaryHeap::new(),
            buy_orders: BinaryHeap::new(),
    	}
    }

    pub fn insert(&self, order: &Order) {
    	if order.get_side() == '1' {

    	} else {

    	}
    }
}