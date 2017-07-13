extern crate linked_hash_map;
extern crate serde_json;

use std::cmp;
use std::net::UdpSocket;
use std::str;
use objects::Order;
use std::collections::HashMap;
use self::linked_hash_map::LinkedHashMap;
use std::sync::mpsc;


const SERVER_ADDRESS: &str = "192.168.1.7:21003";
const MULTICAST_GROUP_ADDRESS: &str = "239.194.5.3:21003";

pub struct MatchingEngine {
    /* 
        Outer hash map: key = price -> value = inner hash map
        Inner hash map: key = order id -> value = order
    */
    sells_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    buys_by_price: HashMap<i64, LinkedHashMap<String, Order>>,
    id_count: i64,
    socket: UdpSocket,
    send_channel: mpsc::Sender<String>
}

impl MatchingEngine {
	/*
		Constructor
		@params 
			None
		@return
			New matching engine with empty Hash Maps for sell and buy orders
	*/
    pub fn new(sender: &mpsc::Sender<String>) -> MatchingEngine { 
    	MatchingEngine {
            sells_by_price: HashMap::new(),
            buys_by_price: HashMap::new(),
            id_count: 0,
            socket: UdpSocket::bind(SERVER_ADDRESS).unwrap(),
            send_channel: sender.clone()
    	}


    }

    pub fn insert(&mut self, order: &Order) -> Order {
        let mut cur_order = order.clone();
        if cur_order.get_id() == "-1".to_string() {
            // New order
            cur_order.set_id(&self.id_count.to_string());
            self.id_count += 1;
        }

        // Multicast new order inserted
        self.multicast(serde_json::to_string(&order).unwrap());
        // send update info to recovery thread
        self.send_channel.send("1".to_string());

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
                {
                    let best_price_orders: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&best_sell_price).unwrap();
                    while !best_price_orders.is_empty() && cur_order.get_qty() > 0 {
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
                        } else {
                            best_price_orders.get_mut(&key).unwrap().set_qty(min_sell.get_qty());
                        }
                    }
                }
                if self.sells_by_price.get(&best_sell_price).unwrap().is_empty() {
                    self.sells_by_price.remove(&best_sell_price);
                }
            }
            // If remaining quantity of buy order is non-zero
            // push buy order onto order book
            if cur_order.get_qty() > 0 {
                if !self.buys_by_price.contains_key(&cur_order.get_price()) {
                    self.buys_by_price.insert(cur_order.get_price(), LinkedHashMap::new());
                }
                let orders_list: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&cur_order.get_price()).unwrap();
                orders_list.insert(cur_order.get_id(), cur_order.clone());
            }
        } else if cur_order.get_side() == '2' {
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
                {
                    let best_price_orders: &mut LinkedHashMap<String, Order> = self.buys_by_price.get_mut(&best_buy_price).unwrap();
                    while !best_price_orders.is_empty() && cur_order.get_qty() > 0 {
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
                        } else {
                            best_price_orders.get_mut(&key).unwrap().set_qty(max_buy.get_qty());
                        }
                    }
                }
                if self.buys_by_price.get(&best_buy_price).unwrap().is_empty() {
                    self.buys_by_price.remove(&best_buy_price);
                }
            }
            // If remaining quantity of sell order is non-zero
            // push sell order onto order book
            if cur_order.get_qty() > 0 {
                if !self.sells_by_price.contains_key(&cur_order.get_price()) {
                    self.sells_by_price.insert(cur_order.get_price(), LinkedHashMap::new());
                }
                let orders_list: &mut LinkedHashMap<String, Order> = self.sells_by_price.get_mut(&cur_order.get_price()).unwrap();
                orders_list.insert(cur_order.get_id(), cur_order.clone());
            }
        }

        // return order object of current order after trading finished
        cur_order
    }  

    pub fn delete(&mut self, ord_id: &String) {
        let existing_ord: Order = self.find_order_by_id(&ord_id);

        if existing_ord.get_side() == '*' {
            ()
        } else if existing_ord.get_side() == '1' {
            // Buy side
            self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
            if self.buys_by_price.get(&existing_ord.get_price()).unwrap().is_empty() {
                self.buys_by_price.remove(&existing_ord.get_price());
            }
        } else if existing_ord.get_side() == '2' {
            // Sell side
            self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
            if self.sells_by_price.get(&existing_ord.get_price()).unwrap().is_empty() {
                self.sells_by_price.remove(&existing_ord.get_price());
            }
        }
    }    

    pub fn update(&mut self, ord_id: &String, order: &Order) {
        // Find existing order by order ID
        let existing_ord: Order = self.find_order_by_id(ord_id);
        // Order not found, exit
        if existing_ord.get_side() == '*' {
            ()
        }
        let mut order_clone = order.clone();
        order_clone.set_side(existing_ord.get_side());
        order_clone.set_id(ord_id);        
        // Compare updated order object with existing order
        if order.get_price() == existing_ord.get_price() {
            if existing_ord.get_side() == '1' {
                // Buy side
                if order.get_qty() > existing_ord.get_qty() {
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().insert(ord_id.clone(), order_clone);
                } else {
                    self.buys_by_price.get_mut(&existing_ord.get_price()).unwrap().get_mut(ord_id).unwrap().set_qty(order.get_qty());
                }
            } else if existing_ord.get_side() == '2' {
                // Sell side
                if order.get_qty() > existing_ord.get_qty() {
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().remove(ord_id);
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().insert(ord_id.clone(), order_clone);
                } else {
                    self.sells_by_price.get_mut(&existing_ord.get_price()).unwrap().get_mut(ord_id).unwrap().set_qty(order.get_qty());   
                }
            }
        } else {
            self.delete(ord_id);
            self.insert(&order_clone);
        }
    }

    pub fn find_order_by_id(&self, ord_id: &String) -> Order {
        for (key, inner_hashmap) in self.sells_by_price.clone() {
            if inner_hashmap.contains_key(ord_id) {
                return inner_hashmap.get(ord_id).unwrap().clone();
            }
        }
        for (key, inner_hashmap) in self.buys_by_price.clone() {
            if inner_hashmap.contains_key(ord_id) {
                return inner_hashmap.get(ord_id).unwrap().clone();
            }
        }

        // If not found, return DUMMY order
        Order::new(-1, -1, '*')
    }

    pub fn print_status(&self) {
        println!("{:*<1$}", "", 80);
        println!("SUMMARY");

        println!("| {0: ^40} | {1: ^10} | {2: ^40} |", 
        "buy", "PRICE", "sell");
        println!("{:-<1$}", "", 100);

        // List the buy orders (in ascending order of price)
        let mut last_price = -i64::max_value();
        while true {
            let mut cur_price = i64::max_value();
            for price in self.buys_by_price.keys(){
                if *price > last_price {
                    cur_price = cmp::min(cur_price, *price);
                }
            }

            if cur_price == i64::max_value() {
                break;
            }

            last_price = cur_price;

            let mut buy_vec = Vec::new();
            let buy_orders: & LinkedHashMap<String, Order> = self.buys_by_price.get(&cur_price).unwrap();
            for order in buy_orders.values() {
                buy_vec.push(order.clone());
            }
            buy_vec.reverse();

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

        // List the sell orders (in ascending order of price)
        let mut last_price = -i64::max_value();
        while true {
            let mut cur_price = i64::max_value();
            for price in self.sells_by_price.keys(){
                if *price > last_price {
                    cur_price = cmp::min(cur_price, *price);
                }
            }

            if cur_price == i64::max_value() {
                break;
            }

            last_price = cur_price;

            let mut sell_vec = Vec::new();
            let sell_orders: & LinkedHashMap<String, Order> = self.sells_by_price.get(&cur_price).unwrap();
            for order in sell_orders.values() {
                sell_vec.push(order.clone());
            }

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

    fn multicast(&self, contents: String) {
        let send_buffer = contents.into_bytes();
        self.socket.send_to(&send_buffer, MULTICAST_GROUP_ADDRESS);
    }
}