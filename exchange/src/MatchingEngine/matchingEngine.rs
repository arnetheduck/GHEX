use std::collections::BinaryHeap;
use std::cmp::Ordering;

mod Objects;

pub struct MatchingEngine {
    id: u32,
    sellOrders: BinaryHeap::<Order>, // min heap
    // buyOrders: BinaryHeap::new(); // max heap
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine { 
    	MatchingEngine {
            id: 10 
    	}
    }
}