/**
    MATCHING ENGINE

    This contains all relevant properties and functions of the matching engine
*/

extern crate linked_hash_map;
extern crate serde_json;

use std::{cmp, str};
use std::net::UdpSocket;
use objects::{Order, IncrementalMessage};
use std::collections::HashMap;
use self::linked_hash_map::LinkedHashMap;
use std::sync::mpsc;

// IP address of the computer running main.rs (MUST specify the PORT)
const SERVER_ADDRESS: &str = "192.168.1.8:21003";
// IP address of the multicast group for Incremental Feed (MUST specify the PORT)
const INCREMENTAL_FEED_MULTICAST_GROUP_ADDRESS: &str = "239.194.5.3:21003";

/**
    A matching engine has the following properties:
        - sells_by_price: 
            collection of lists of orders on SELL side with different prices
        - buys_by_price:
            collection of lists of orders on BUY side with different prices
        - id_count:
            used to assign ID for new orders
        - socket:
            socket used for multicasting
        - send_channel:
            a channel to send Incremental Feed to Recovery thread in main
        - seq_number:
            sequential number of the Incremental Feed
*/
pub struct MatchingEngine {
    /** 
        Outer hash map: key = price -> value = inner hash map
        Inner hash map: key = order id -> value = order
    */
    sells_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    buys_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    id_count: i64,
    socket: UdpSocket,
    send_channel: mpsc::Sender<String>,
    seq_number: i64,
}

impl MatchingEngine {
	/**
		Constructor
		@params 
			sender: a channel to send data to the main thread (send incremental feed to recovery multicast thread in main)
		@return
			New matching engine with empty Hash Maps for sell and buy orders
	*/
    pub fn new(sender: &mpsc::Sender<String>) -> MatchingEngine { 
    	MatchingEngine {
            sells_by_price: HashMap::new(),
            buys_by_price: HashMap::new(),
            id_count: 0,
            socket: UdpSocket::bind(SERVER_ADDRESS).unwrap(),
            send_channel: sender.clone(),
            seq_number: 0,
    	}
    }

    /**
        This function does order insertion. Before inserting an order into the order book,
        this will try to match new order with existing orders. After matchings (if any),
        if the new order is NOT fully matched, then insert the new order into the order book

        @params
            order: an order object for the new order to be inserted
    */
    pub fn insert(&mut self, order: &Order) -> Order {
        let mut cur_order = order.clone();
        // The following is to differentiate between
        // - a completely new order inserted
        // - an order with ID assigned inserted (i.e, INSERT called from UPDATE, the order ID remains unchanged)
        if cur_order.get_id() == "-1".to_string() {
            // New order
            cur_order.set_id(&self.id_count.to_string());
            self.id_count += 1;
        }

        if cur_order.get_side() == '1' {
            // BUY side
            // Look at order book and match (if possible)
            while !self.sells_by_price.is_empty() && cur_order.get_qty() > 0 {
                // Find LOWEST price on SELL side
                let mut best_sell_price = i64::max_value();                
                for &avail_price in self.sells_by_price.keys() {
                    best_sell_price = cmp::min(best_sell_price, avail_price);
                }
                // Exit if there is no more SELL order 
                // or LOWEST price on SELL side is higher than price of current order
                if best_sell_price > cur_order.get_price() {
                    break;
                }
                // Get all SELL orders at LOWEST price
                {
                    while true {
                        {
                            let best_price_orders: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&best_sell_price).unwrap();
                            if best_price_orders.is_empty() || cur_order.get_qty() == 0 {
                                break;
                            }
                            let key: String;
                            let sell_order: Order;
                            // Get key and order object of the first order in Hash Map
                            {
                                let (m_key, m_sell_order) = best_price_orders.front().unwrap();
                                key = m_key.clone();
                                sell_order = m_sell_order.clone();
                            }
                            let mut min_sell = sell_order.clone();
                            // Determine quantity matched
                            // i.e, Minimum quantity of BUY and SELL order
                            let qty_trade = cmp::min(min_sell.get_qty(), cur_order.get_qty());
                            // Update the remaining quantity for SELL order
                            let min_sell_qty = min_sell.get_qty();
                            min_sell.set_qty(min_sell_qty - qty_trade);
                            // Update the remaining quantity for BUY order (new order inserted)
                            let cur_order_qty = cur_order.get_qty();
                            cur_order.set_qty(cur_order_qty - qty_trade);
                            // Delete SELL order (if fully matched)
                            if min_sell.get_qty() == 0 {
                                best_price_orders.pop_front();
                            } else {
                                best_price_orders.get_mut(&key).unwrap().set_qty(min_sell.get_qty());
                            }                            
                        }
                        // Multicast Incremental Feed after a match
                        self.incremental_feed(&best_sell_price);
                    }
                }
                // If the list of orders becomes empty, remove the list
                if self.sells_by_price.get(&best_sell_price).unwrap().is_empty() {
                    self.sells_by_price.remove(&best_sell_price);
                }
            }
            // If remaining quantity of BUY order is non-zero
            // push BUY order onto order book
            if cur_order.get_qty() > 0 {
                if !self.buys_by_price.contains_key(&cur_order.get_price()) {
                    self.buys_by_price.insert(cur_order.get_price(), LinkedHashMap::new());
                }
                {
                    let orders_list: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&cur_order.get_price()).unwrap();
                    orders_list.insert(cur_order.get_id(), cur_order.clone());
                }
                // Multicast Incremental Feed
                self.incremental_feed(&cur_order.get_price());
            }
        } else if cur_order.get_side() == '2' {
            // SELL side
            // Look at order book and match (if possible)
            while !self.buys_by_price.is_empty() && cur_order.get_qty() > 0 {
                // Find HIGHEST price on BUY side
                let mut best_buy_price = -i64::max_value();                
                for &avail_price in self.buys_by_price.keys() {
                    best_buy_price = cmp::max(best_buy_price, avail_price);
                }
                // Exit if there is no more BUY order 
                // or HIGHEST price on BUY side is lower than price of current order
                if best_buy_price < cur_order.get_price() {
                    break;
                }
                // Get all BUY orders at HIGHEST price
                {
                    while true {
                        {
                            let best_price_orders: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&best_buy_price).unwrap();
                            if best_price_orders.is_empty() || cur_order.get_qty() == 0 {
                                break;
                            }
                            let key: String;
                            let buy_order: Order;
                            // Get key and order object of the first order in Hash Map
                            {
                                let (m_key, m_buy_order) = best_price_orders.front().unwrap();
                                key = m_key.clone();
                                buy_order = m_buy_order.clone();
                            }
                            let mut max_buy = buy_order.clone();
                            // Determine quantity matched
                            // i.e, Minimum quantity of BUY and SELL order
                            let qty_trade = cmp::min(max_buy.get_qty(), cur_order.get_qty());
                            // Update the remaining quantity for BUY order
                            let max_buy_qty = max_buy.get_qty();
                            max_buy.set_qty(max_buy_qty - qty_trade);
                            // Update the remaining quantity for SELL order (new order inserted)
                            let cur_order_qty = cur_order.get_qty();
                            cur_order.set_qty(cur_order_qty - qty_trade);
                            // Delete BUY order (if fully matched)
                            if max_buy.get_qty() == 0 {
                                best_price_orders.pop_front();
                            } else {
                                best_price_orders.get_mut(&key).unwrap().set_qty(max_buy.get_qty());
                            }                            
                        }
                        // Multicast Incremental Feed after a match
                        self.incremental_feed(&best_buy_price);                        
                    }
                }
                // If the list of orders becomes empty, remove the list                
                if self.buys_by_price.get(&best_buy_price).unwrap().is_empty() {
                    self.buys_by_price.remove(&best_buy_price);
                }
            }
            // If remaining quantity of SELL order is non-zero
            // push SELL order onto order book
            if cur_order.get_qty() > 0 {
                if !self.sells_by_price.contains_key(&cur_order.get_price()) {
                    self.sells_by_price.insert(cur_order.get_price(), LinkedHashMap::new());
                }
                {
                    let orders_list: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&cur_order.get_price()).unwrap();
                    orders_list.insert(cur_order.get_id(), cur_order.clone());
                }
                // Multicast Incremental Feed
                self.incremental_feed(&cur_order.get_price());                
            }
        }

        // Return order object of current order after trading finished
        cur_order
    }  

    /**
        This function deletes an order with a specific ID from the order book

        @params
            ord_id: ID of the order to be deleted
    */
    pub fn delete(&mut self, ord_id: &String) {
        // Find existing order by order ID
        let existing_ord: Order = self.find_order_by_id(&ord_id);
        // Exit if there is no order with the specific ID
        if existing_ord.get_side() == '*' {
            ()
        } else {
            if existing_ord.get_side() == '1' {
            // Buy side
                // Remove order from the order book
                self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                // If the list of orders at that price becomes empty, then remove the list
                if self.buys_by_price.get(&existing_ord.get_price()).unwrap().is_empty() {
                    self.buys_by_price.remove(&existing_ord.get_price());
                }
            } else if existing_ord.get_side() == '2' {
            // Sell side
                // Remove order from the order book
                self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                // If the list of orders at that price becomes empty, then remove the list
                if self.sells_by_price.get(&existing_ord.get_price()).unwrap().is_empty() {
                    self.sells_by_price.remove(&existing_ord.get_price());
                }
            }
            // Multicast incremental feed after deleting
            self.incremental_feed(&existing_ord.get_price());
        }
    }    

    /**
        This function updates an order with a specific ID with a new order

        @params
            ord_id: ID of the order to be updated
            order: order object for the new order
    */
    pub fn update(&mut self, ord_id: &String, order: &Order) {
        // Find existing order by order ID
        let existing_ord: Order = self.find_order_by_id(ord_id);
        // If order not found then exit
        if existing_ord.get_side() == '*' {
            ()
        }
        let mut order_clone = order.clone();
        order_clone.set_side(existing_ord.get_side());
        order_clone.set_id(ord_id);        
        // Compare updated order object with existing order
        if order.get_price() == existing_ord.get_price() {
            // Price remains as before, only QUANTITY is updated
            if existing_ord.get_side() == '1' {
                // Buy side
                if order.get_qty() > existing_ord.get_qty() {
                    // Quantity increases, then the order moves to the rear of the queue
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().insert(ord_id.clone(), order_clone);
                } else {
                    // Quantity decreases, then the order stays in the queue and quantity is updated
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().get_mut(ord_id).unwrap().set_qty(order.get_qty());
                }
            } else if existing_ord.get_side() == '2' {
                // Sell side
                if order.get_qty() > existing_ord.get_qty() {
                    // Quantity increases, then the order moves to the rear of the queue                    
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().insert(ord_id.clone(), order_clone);
                } else {
                    // Quantity decreases, then the order stays in the queue and quantity is updated
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().get_mut(ord_id).unwrap().set_qty(order.get_qty());   
                }
            }
            // Multicast Incremental Feed after updating
            self.incremental_feed(&existing_ord.get_price());            
        } else {
            // Price changes then DELETE old order and INSERT new one
            // * NOTE: This requires 2 operations, hence there will be 2 Incremental Feeds (for DELETE and INSERT)
            self.delete(ord_id);
            self.insert(&order_clone);
        }
    }

    /**
        This function finds an order in the order book by a specific ID and returns it
        
        @params
            ord_id: order ID to find
        @return
            - Order object if FOUND
            - DUMMY order object if NOT FOUND
    */
    pub fn find_order_by_id(&self, ord_id: &String) -> Order {
        // Find order in SELL side
        for (key, inner_hashmap) in self.sells_by_price.clone() {
            // If found then return order object
            if inner_hashmap.contains_key(ord_id) {
                return inner_hashmap.get(ord_id).unwrap().clone();
            }
        }
        // Find order in BUY side
        for (key, inner_hashmap) in self.buys_by_price.clone() {
            // If found then return order object
            if inner_hashmap.contains_key(ord_id) {
                return inner_hashmap.get(ord_id).unwrap().clone();
            }
        }

        // If not found, return DUMMY order
        Order::new(-1, -1, '*')
    }

    /**
        This function displays the FULL order book, in increasing order of the prices.
        At a price, orders with HIGHER priority are CLOSER to the MIDDLE.
        (i.e, For BUY side, HIGH priority to LOW priority is from RIGHT to LEFT
            For SELL side, HIGH priority to LOW priority is from LEFT to RIGHT)

        * NOTE: Open BIG/FULL SCREEN window for better display
    */
    pub fn print_status(&self) {
        println!("{:*<1$}", "", 80);
        println!("SUMMARY");

        println!("| {0: ^40} | {1: ^10} | {2: ^40} |", 
        "buy", "PRICE", "sell");
        println!("{:-<1$}", "", 100);

        // List the BUY orders (in ascending order of price)
        let mut last_price = -i64::max_value();
        while true {
            // Initialize next price in increasing order on BUY side to be INFINITY 
            let mut cur_price = i64::max_value();
            // Find the next price on BUY side
            for price in self.buys_by_price.keys(){
                if *price > last_price {
                    cur_price = cmp::min(cur_price, *price);
                }
            }
            // If reached the maximum price on BUY side (i.e, ALL BUY orders are listed), then exit
            if cur_price == i64::max_value() {
                break;
            }
            // Update last price listed for next iteration
            last_price = cur_price;
            // Get BUY orders at current price
            let mut buy_vec = Vec::new();
            let buy_orders: & LinkedHashMap<String, Order> = self.buys_by_price.get(&cur_price).unwrap();
            for order in buy_orders.values() {
                buy_vec.push(order.clone());
            }
            // Reverse the vector to print orders with HIGHER priority CLOSER to the MIDDLE
            buy_vec.reverse();
            // Print the orders
            let mut cur_line = String::new();
            for (index, order) in buy_vec.iter().enumerate() {
                if index > 0 {
                    cur_line.push(' ');
                }
                cur_line.push_str((order.get_qty().to_string() + "(ID: " + order.get_id().as_str() + ")").as_str());
            }
            println!("| {0: >40} | {1: ^10} | {2: <40} |", 
            cur_line, cur_price, "");
        }

        println!();

        // List the SELL orders (in ascending order of price)
        let mut last_price = -i64::max_value();
        while true {
            // Initialize next price in increasing order on SELL side to be INFINITY             
            let mut cur_price = i64::max_value();
            // Find the next price on SELL side            
            for price in self.sells_by_price.keys(){
                if *price > last_price {
                    cur_price = cmp::min(cur_price, *price);
                }
            }
            // If reached the maximum price on SELL side (i.e, ALL SELL orders are listed), then exit
            if cur_price == i64::max_value() {
                break;
            }
            // Update last price listed for next iteration
            last_price = cur_price;
            // Get SELL orders at current price
            let mut sell_vec = Vec::new();
            let sell_orders: & LinkedHashMap<String, Order> = self.sells_by_price.get(&cur_price).unwrap();
            for order in sell_orders.values() {
                sell_vec.push(order.clone());
            }
            // Print the orders
            let mut cur_line = String::new();
            for (index, order) in sell_vec.iter().enumerate() {
                if index > 0 {
                    cur_line.push(' ');
                }
                cur_line.push_str((order.get_qty().to_string() + "(ID: " + order.get_id().as_str() + ")").as_str());
            }
            println!("| {0: >40} | {1: ^10} | {2: <40} |", 
            "", cur_price, cur_line);
        }
    }

    /**
        This function gets ALL the orders at a price

        @params
            price: Price at which user wants to get the orders
        @return
            - A list of orders 
            - Empty list if there is NO order at that price

        * NOTE: ALL orders at the price will be on ONE side.
        Because if they are on both side, they should already be matched.
    */
    fn get_orders_by_price(&self, price: &i64) -> Vec<Order> {
        // Get all buy orders at a specific price (A Linked Hash Map)
        let buy_orders = self.buys_by_price.get(&price);
        // Convert Linked Hash Map into a Vector
        let mut buys_vec: Vec<Order> = Vec::new();
        if buy_orders != None {
            for order in buy_orders.unwrap().values() {
                buys_vec.push(order.clone());
            }
            return buys_vec.clone();
        }
        // Get all sell orders at a specific price (A Linked Hash Map)
        let sell_orders = self.sells_by_price.get(&price);
        // Convert Linked Hash Map into a Vector
        let mut sells_vec: Vec<Order> = Vec::new();
        if sell_orders != None {
            for order in sell_orders.unwrap().values() {
                sells_vec.push(order.clone());
            }
            return sells_vec.clone();
        }        
        // Return empty list
        Vec::new()
    }

    /**
        This function multicasts Incremental Feed to the multicast group

        @params
            price_affected: The price at which orders were matched during the previous operation
    */
    fn incremental_feed(&mut self, price_affected: &i64) {
        // Update sequential number for Incremental Feed
        self.seq_number += 1;
        // Create an Incremental Message object with 
        let message = IncrementalMessage::new(*price_affected, self.seq_number, self.get_orders_by_price(&price_affected));
        // Convert incremental feed to JSON format for multicasting
        let incre_feed = serde_json::to_string(&message).unwrap();
        // Multicast latest status at the price of the new order
        self.multicast(incre_feed.clone());
        // Send update info to Recovery thread
        self.send_channel.send(incre_feed.clone());
    }

    /**
    This function multicasts an Incremental Feed to INCREMENTAL_FEED_MULTICAST_GROUP_ADDRESS

    @params
        contents: the contents to be published
    */
    fn multicast(&self, contents: String) {
        // Recovery feed must be converted to bytes for multicasting        
        let send_buffer = contents.into_bytes();
        self.socket.send_to(&send_buffer, INCREMENTAL_FEED_MULTICAST_GROUP_ADDRESS);
    }
}