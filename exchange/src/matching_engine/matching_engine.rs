extern crate linked_hash_map;

use std::cmp;
use objects::Order;
use std::collections::HashMap;
use self::linked_hash_map::LinkedHashMap;

pub struct MatchingEngine {
    /* 
        Outer hash map: key = price -> value = inner hash map
        Inner hash map: key = order id -> value = order
    */
    sells_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    buys_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    id_count: i64
}

impl MatchingEngine {
	/*
		Constructor
		@params 
			None
		@return
			New matching engine with empty Hash Maps for sell and buy orders
	*/
    pub fn new() -> MatchingEngine { 
    	MatchingEngine {
            sells_by_price: HashMap::new(),
            buys_by_price: HashMap::new(),
            id_count: 0
    	}
    }

    pub fn insert(&mut self, order: &Order) {
        let mut cur_order = order.clone();
        if cur_order.get_side() == '1' {
            // Buy side
            // Look at order book and match (if possible)
            while !self.sells_by_price.is_empty() && cur_order.get_qty() > 0 {
                // Find lowest price on sell side
                let mut best_sell_price = i64::max_value();                
                for &avail_price in self.sells_by_price.keys() {
                    best_sell_price = cmp::min(best_sell_price, avail_price);
                }
                // Exit if there is no more sell order 
                // or lowest price on sell side is higher than price of current order
                if best_sell_price > cur_order.get_price() {
                    break;
                }
                // Get all sell orders at lowest price
                let best_price_orders: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&best_sell_price).unwrap();
                while !best_price_orders.is_empty() && cur_order.get_qty() > 0 {
                    let mut key: String;
                    let mut sell_order: Order;
                    // Get key and order object of the first order in Hash Map
                    {
                        let (m_key, m_sell_order) = best_price_orders.front().unwrap();
                        key = m_key.clone();
                        sell_order = m_sell_order.clone();
                    }
                    let mut min_sell = sell_order.clone();
                    // Determine quantity matched
                    // i.e, Minimum quantity of buy and sell order
                    let qty_trade = cmp::min(min_sell.get_qty(), cur_order.get_qty());
                    // Update the remaining quantity for sell order
                    let min_sell_qty = min_sell.get_qty();
                    min_sell.set_qty(min_sell_qty - qty_trade);
                    // Update the remaining quantity for buy order (new order inserted)
                    let cur_order_qty = cur_order.get_qty();
                    cur_order.set_qty(cur_order_qty - qty_trade);
                    // Delete sell order (if fully matched)
                    if min_sell.get_qty() == 0 {
                        best_price_orders.pop_front();
                    }
                }
            }
            // If remaining quantity of buy order is non-zero
            // push buy order onto order book
            if cur_order.get_qty() > 0 {
                let orders_list: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&cur_order.get_price()).unwrap();
                cur_order.set_id(&self.id_count.to_string());
                self.id_count += 1;
                orders_list.insert(cur_order.get_id(), cur_order);
            }
        } else {
            // Sell side
            // Look at order book and match (if possible)
            while !self.buys_by_price.is_empty() && cur_order.get_qty() > 0 {
                // Find highest price on buy side
                let mut best_buy_price = -i64::max_value();                
                for &avail_price in self.buys_by_price.keys() {
                    best_buy_price = cmp::max(best_buy_price, avail_price);
                }
                // Exit if there is no more buy order 
                // or highest price on buy side is lower than price of current order
                if best_buy_price < cur_order.get_price() {
                    break;
                }
                // Get all buy orders at highest price
                let best_price_orders: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&best_buy_price).unwrap();
                while !best_price_orders.is_empty() && cur_order.get_qty() > 0 {
                    let mut key: String;
                    let mut buy_order: Order;
                    // Get key and order object of the first order in Hash Map
                    {
                        let (m_key, m_buy_order) = best_price_orders.front().unwrap();
                        key = m_key.clone();
                        buy_order = m_buy_order.clone();
                    }
                    let mut max_buy = buy_order.clone();
                    // Determine quantity matched
                    // i.e, Minimum quantity of buy and sell order
                    let qty_trade = cmp::min(max_buy.get_qty(), cur_order.get_qty());
                    // Update the remaining quantity for buy order
                    let max_buy_qty = max_buy.get_qty();
                    max_buy.set_qty(max_buy_qty - qty_trade);
                    // Update the remaining quantity for sell order (new order inserted)
                    let cur_order_qty = cur_order.get_qty();
                    cur_order.set_qty(cur_order_qty - qty_trade);
                    // Delete buy order (if fully matched)
                    if max_buy.get_qty() == 0 {
                        best_price_orders.pop_front();
                    }
                }
            }
            // If remaining quantity of sell order is non-zero
            // push sell order onto order book
            if cur_order.get_qty() > 0 {
                let orders_list: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&cur_order.get_price()).unwrap();
                cur_order.set_id(&self.id_count.to_string());
                self.id_count += 1;
                orders_list.insert(cur_order.get_id(), cur_order);
            }
        }
    }  
}