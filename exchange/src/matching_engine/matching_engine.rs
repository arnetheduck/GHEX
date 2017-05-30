use std::cmp::min;
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

    pub fn insert(&mut self, order: &Order) {
    	let mut cur_order = *order;

    	if cur_order.get_side() == '1' { 
    		// Buy side
    		// Look at order book and match (if possible)

    		while !self.sell_orders.is_empty() {
    			// stop matching when price of buy order is slower than all sell orders
    			// or when the current order is fully matched	
    			if self.sell_orders.peek().unwrap().get_price() > cur_order.get_price() || cur_order.get_qty() == 0 { 
    				break; 
    			}

    			// match happens
    			let mut min_sell: Order = self.sell_orders.pop().unwrap();
    			let qty_trade = min(min_sell.get_qty(), cur_order.get_qty());
    			
    			let min_sell_qty = min_sell.get_qty();
    			min_sell.set_qty(min_sell_qty - qty_trade);
    			let cur_order_qty = cur_order.get_qty();
    			cur_order.set_qty(cur_order_qty - qty_trade);

    			if min_sell.get_qty() > 0 {
    				self.sell_orders.push(min_sell);
    			}
    		}
    		
    		if cur_order.get_qty() > 0 {
    			self.buy_orders.push(cur_order);
    		}
    	} else { 
    		// Sell side
    		// Look at order book and match (if possible)

    		while !self.buy_orders.is_empty() {
    			// stop matching when price of sell order is higher than all buy orders
    			// or when the current order is matched
    			if self.buy_orders.peek().unwrap().get_price() < cur_order.get_price() || cur_order.get_qty() == 0 { 
    				break; 
    			}

    			// match happens
    			let mut max_buy: Order = self.buy_orders.pop().unwrap();
    			let qty_trade = min(max_buy.get_qty(), cur_order.get_qty());
    			
    			let max_buy_qty = max_buy.get_qty();
    			max_buy.set_qty(max_buy_qty - qty_trade);
    			let cur_order_qty = cur_order.get_qty();
    			cur_order.set_qty(cur_order_qty - qty_trade);

    			if max_buy.get_qty() > 0 {
    				self.buy_orders.push(max_buy);
    			}
    		}
    		
    		if cur_order.get_qty() > 0 {
    			self.sell_orders.push(cur_order);
    		}
    	}
    }

    pub fn print_status(&self) {
    	println!("---------------------------------------------------");
    	println!("SUMMARY");
    	println!();

    	println!("{0: <10} | {1: <10} | {2: <10}", 
        "buy", "PRICE", "sell");
        println!("---------------------------------");


    	let clone_sell_orders = self.sell_orders.clone();
    	let sell_vec = clone_sell_orders.into_sorted_vec();
    	for order in &sell_vec {
    		println!("{0: <10} | {1: <10} | {2: <10}", 
        	0, order.get_price(), order.get_qty());
    	}

    	println!();

    	let clone_buy_orders = self.buy_orders.clone();
    	let mut buy_vec: Vec<Order> = clone_buy_orders.into_sorted_vec();
    	let buy_vec: Vec<Order> = buy_vec.iter().rev().cloned().collect();
    	for order in &buy_vec {
    		println!("{0: <10} | {1: <10} | {2: <10}", 
        	order.get_qty(), order.get_price(), 0);
    	}

    	println!("---------------------------------------------------");
    }
}