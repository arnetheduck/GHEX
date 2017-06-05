extern crate linked_hash_map;

use std::cmp::min;
use std::collections::BinaryHeap;
use objects::Order;
use std::collections::HashMap;
use self::linked_hash_map::LinkedHashMap;

pub struct MatchingEngine {
    sell_orders: BinaryHeap<Order>, // All sell orders, max heap according to priority
    buy_orders: BinaryHeap<Order>, // All buy orders, max heap according to priority
    /* Outer hash map: price -> inner hash map
       Inner hash map: account id -> Order
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
            sell_orders: BinaryHeap::new(),
            buy_orders: BinaryHeap::new(),
            sells_by_price: HashMap::new(),
            buys_by_price: HashMap::new()
    	}
    }
    /*
        Get all buy orders at a given price
        @params 
            price: price being queried
        @return 
            Vector of buy orders at a given price
    */
    pub fn get_buy_orders(&self, price: &i64) -> Vec<Order> {
        // Get all buy orders at a specific price (A Linked Hash Map)
        let buy_orders = self.buys_by_price.get(price);

        // Convert Linked Hash Map into a Vector
        let mut buys_vec: Vec<Order> = Vec::new();
        if buy_orders != None {
            for order in (buy_orders.unwrap()).values() {
                let order_clone = order.clone();
                buys_vec.push(order_clone);
            }
        }

        println!("| {0: ^40} | {1: ^10} | {2: ^40} |", 
        "buy", "PRICE", "sell");
        println!("{:-<1$}", "", 100);

        // List the buy orders
        buys_vec.reverse();
        let mut cur_line = String::new();
        for (index, order) in buys_vec.iter().enumerate() {
            if cur_line.len() > 0 {
                cur_line.push(' ');
            }
            cur_line.push_str(order.get_qty().to_string().as_str());

            if index == buys_vec.len() - 1 || buys_vec[index].get_price() != buys_vec[index + 1].get_price() {
                println!("| {0: >40} | {1: ^10} | {2: <40} |", 
                cur_line, order.get_price(), "");
                cur_line = String::new();
            }
        }        
        buys_vec
    }
    /*
        Get all sell orders at a given price
        @params 
            price: price being queried
        @return 
            Vector of sell orders at a given price
    */
    pub fn get_sell_orders(&self, price: &i64) -> Vec<Order> {
        // Get all sell orders at a specific price (A Linked Hash Map)
        let sell_orders = self.sells_by_price.get(price);

        // Convert Linked Hash Map into a Vector
        let mut sells_vec: Vec<Order> = Vec::new();
        if sell_orders != None {
            for order in (sell_orders.unwrap()).values() {
                let order_clone = order.clone();
                sells_vec.push(order_clone);
            }
        }

        println!("| {0: ^40} | {1: ^10} | {2: ^40} |", 
        "buy", "PRICE", "sell");
        println!("{:-<1$}", "", 100);

        // List the buy orders
        let mut cur_line = String::new();
        for (index, order) in sells_vec.iter().enumerate() {
            if cur_line.len() > 0 {
                cur_line.push(' ');
            }
            cur_line.push_str(order.get_qty().to_string().as_str());

            if index == sells_vec.len() - 1 || sells_vec[index].get_price() != sells_vec[index + 1].get_price() {
                println!("| {0: >40} | {1: ^10} | {2: <40} |", 
                "", order.get_price(), cur_line);
                cur_line = String::new();
            }
        }  
        sells_vec
    }

    /*
    	Take an order and match it with existing orders in matching engine
    	@params
    		order: order to insert
    */
    pub fn insert(&mut self, order: &Order) {
    	let mut cur_order = order.clone();
        let price = cur_order.get_price();
    	if cur_order.get_side() == '1' { 
    		// Buy side
    		// Look at order book and match (if possible)
    		while !self.sell_orders.is_empty() {
    			// Stop matching when price of buy order is lower than all sell orders
    			// or when the current order is fully matched	
    			if self.sell_orders.peek().unwrap().get_price() > cur_order.get_price() || cur_order.get_qty() == 0 { 
    				break; 
    			}

    			// MATCHING HAPPENS
    			// Get sell order with highest priority (price and time priority)
    			let mut min_sell: Order = self.sell_orders.pop().unwrap();

    			// Determine quantity matched
    			// i.e, Minimum quantity of buy and sell order
    			let qty_trade = min(min_sell.get_qty(), cur_order.get_qty());

    			// Update the remaining quantity for sell order
    			let min_sell_qty = min_sell.get_qty();
    			min_sell.set_qty(min_sell_qty - qty_trade);
    			// Update the remaining quantity for buy order (new order inserted)
    			let cur_order_qty = cur_order.get_qty();
    			cur_order.set_qty(cur_order_qty - qty_trade);

    			// If remaining quantity of sell order is non-zero
    			// push sell order back onto order book
    			if min_sell.get_qty() > 0 {
                    (*self.sells_by_price.get_mut(&min_sell.get_price()).unwrap()).get_mut(&min_sell.get_id()).unwrap().set_qty(min_sell.get_qty());
                    self.sell_orders.push(min_sell);
                }
    			else {
                    (*self.sells_by_price.get_mut(&min_sell.get_price()).unwrap()).remove(&min_sell.get_id());
                }
    		}
    		
    		// If remaining quantity of buy order is non-zero
    		// push buy order onto order book
    		if cur_order.get_qty() > 0 {
                let order_clone = cur_order.clone();
    			self.buy_orders.push(cur_order);

                if !self.buys_by_price.contains_key(&price) {
                    self.buys_by_price.insert(price, LinkedHashMap::new());
                }
                (*self.buys_by_price.get_mut(&price).unwrap()).insert(order.get_id(), order_clone);
    		}
    	} else { 
    		// Sell side
            
    		// Look at order book and match (if possible)
    		while !self.buy_orders.is_empty() {
    			// Stop matching when price of sell order is higher than all buy orders
    			// or when the current order is matched
    			if self.buy_orders.peek().unwrap().get_price() < cur_order.get_price() || cur_order.get_qty() == 0 { 
    				break; 
    			}

    			// MATCHING HAPPENS
    			// Get buy order with highest priority (price and time priority)
    			let mut max_buy: Order = self.buy_orders.pop().unwrap();		

    			// Determine quantity matched
    			// i.e, Minimum quantity of buy and sell order    			
    			let qty_trade = min(max_buy.get_qty(), cur_order.get_qty());

    			// Update the remaining quantity for buy order    			
    			let max_buy_qty = max_buy.get_qty();
    			max_buy.set_qty(max_buy_qty - qty_trade);
    			// Update the remaining quantity for sell order (new order inserted)    			
    			let cur_order_qty = cur_order.get_qty();
    			cur_order.set_qty(cur_order_qty - qty_trade);

    			// If remaining quantity of buy order is non-zero
    			// push buy order back onto order book
    			if max_buy.get_qty() > 0 {
                    (*self.buys_by_price.get_mut(&max_buy.get_price()).unwrap()).get_mut(&max_buy.get_id()).unwrap().set_qty(max_buy.get_qty());                    
    				self.buy_orders.push(max_buy);
    			} else {
                    (*self.buys_by_price.get_mut(&max_buy.get_price()).unwrap()).remove(&max_buy.get_id());
                }
    		}

    		// If remaining quantity of sell order is non-zero
    		// push sell order onto order book    		
    		if cur_order.get_qty() > 0 {
                let order_clone = cur_order.clone();                
    			self.sell_orders.push(cur_order);

                if !self.sells_by_price.contains_key(&price) {
                    self.sells_by_price.insert(price, LinkedHashMap::new());
                }
                (*self.sells_by_price.get_mut(&price).unwrap()).insert(order.get_id(), order_clone);
    		}
    	}
    }

    /*
    	This function prints market status, listing all orders in order
    	Sell orders (if any) come first, in descending order of priority
    	Buy orders (if any) come after, in ascending order of priority
    */
    pub fn print_status(&self) {
		println!("{:*<1$}", "", 80);
    	println!("SUMMARY");

    	println!("| {0: ^40} | {1: ^10} | {2: ^40} |", 
        "buy", "PRICE", "sell");
		println!("{:-<1$}", "", 100);

        // List the buy orders
        let clone_buy_orders = self.buy_orders.clone();
        let buy_vec: Vec<Order> = clone_buy_orders.into_sorted_vec();

        let mut cur_line = String::new();
        for (index, order) in buy_vec.iter().enumerate() {
            if cur_line.len() > 0 {
                cur_line.push(' ');
            }
            cur_line.push_str(order.get_qty().to_string().as_str());

            if index == buy_vec.len() - 1 || buy_vec[index].get_price() != buy_vec[index + 1].get_price() {
                println!("| {0: >40} | {1: ^10} | {2: <40} |", 
                cur_line, order.get_price(), "");
                cur_line = String::new();
            }
        }

        println!();

        // List the sell orders
    	let clone_sell_orders = self.sell_orders.clone();
    	let sell_vec = clone_sell_orders.into_sorted_vec();
        let sell_vec: Vec<Order> = sell_vec.iter().rev().cloned().collect();

        let mut cur_line = String::new();
        for (index, order) in sell_vec.iter().enumerate() {
            if cur_line.len() > 0 {
                cur_line.push(' ');
            }
            cur_line.push_str(order.get_qty().to_string().as_str());

            if index == sell_vec.len() - 1 || sell_vec[index].get_price() != sell_vec[index + 1].get_price() {
                println!("| {0: >40} | {1: ^10} | {2: <40} |", 
                "", order.get_price(), cur_line);
                cur_line = String::new();
            }
        }
    }
}