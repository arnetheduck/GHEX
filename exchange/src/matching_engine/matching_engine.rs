extern crate linked_hash_map;

use objects::Order;
use std::collections::HashMap;
use self::linked_hash_map::LinkedHashMap;

pub struct MatchingEngine {
    /* 
        Outer hash map: key = price -> value = inner hash map
        Inner hash map: key = order id -> value = order
    */
    sells_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    buys_by_price: HashMap<i64, LinkedHashMap<String, Order>>
}

impl MatchingEngine {
	/*
		Constructor
		@params 
			None
		@return
			New matching engine with empty priority queues for sell and buy orders
	*/
    pub fn new() -> MatchingEngine { 
    	MatchingEngine {
            sells_by_price: HashMap::new(),
            buys_by_price: HashMap::new()
    	}
    }
}