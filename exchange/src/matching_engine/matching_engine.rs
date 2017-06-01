use std::cmp::min;
use std::collections::BinaryHeap;
use objects::Order;

pub struct MatchingEngine {
    sell_orders: BinaryHeap<Order>, // All sell orders, min heap
    buy_orders: BinaryHeap<Order>, // All buy orders, max heap    
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
    	}
    }

    /*
    	Take an order and match it with existing orders in matching engine
    	@params
    		order: order to insert
    */
    pub fn insert(&mut self, order: &Order) -> Vec<(Order, Order)> {
    	let mut matched_orders: Vec<(Order, Order)> = Vec::new();

    	let mut cur_order = order.clone();

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

    			let mut m_sell_order = min_sell.clone();
    			let mut m_buy_order = cur_order.clone();
    			matched_orders.push((m_buy_order, m_sell_order));

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
    				self.sell_orders.push(min_sell);
    			}
    		}
    		
    		// If remaining quantity of buy order is non-zero
    		// push buy order onto order book
    		if cur_order.get_qty() > 0 {
    			self.buy_orders.push(cur_order);
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

    			let mut m_sell_order = cur_order.clone();
    			let mut m_buy_order = max_buy.clone();
    			matched_orders.push((m_buy_order, m_sell_order));    			

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
    				self.buy_orders.push(max_buy);
    			}
    		}

    		// If remaining quantity of sell order is non-zero
    		// push sell order onto order book    		
    		if cur_order.get_qty() > 0 {
    			self.sell_orders.push(cur_order);
    		}
    	}

    	matched_orders
    }

    /*
    	This function prints market status, listing all orders in order
    	Sell orders (if any) come first, in descending order of priority
    	Buy orders (if any) come after, in ascending order of priority
    */
    pub fn print_status(&self) {
		println!("{:*<1$}", "", 80);
    	println!("SUMMARY");

    	println!("{0: ^25}|{1: ^10}|{2: ^10}|{3: ^10}|", 
        "TransactTime", "buy", "PRICE", "sell");
		println!("{:-<1$}", "", 59);

        // List the sell orders
    	let clone_sell_orders = self.sell_orders.clone();
    	let sell_vec = clone_sell_orders.into_sorted_vec();
    	for order in &sell_vec {
    		println!("{0: ^25}|{1: ^10}|{2: ^10}|{3: ^10}|", 
        	order.get_transact_time(), " ", order.get_price(), order.get_qty());
    	}

    	println!();

    	// List the buy orders
    	let clone_buy_orders = self.buy_orders.clone();
    	let buy_vec: Vec<Order> = clone_buy_orders.into_sorted_vec();
    	let buy_vec: Vec<Order> = buy_vec.iter().rev().cloned().collect();
    	for order in &buy_vec {
    		println!("{0: ^25}|{1: ^10}|{2: ^10}|{3: ^10}|", 
        	order.get_transact_time(), order.get_qty(), order.get_price(), " ");
    	}
    }
}