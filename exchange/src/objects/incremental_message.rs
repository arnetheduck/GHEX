extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::serde::de::Deserialize;
use objects::Order;

#[derive (Serialize, Deserialize)]
pub struct IncrementalMessage {
	price: i64,
    seq_number: i64,
    orders_vec: Vec<Order>,
}

impl IncrementalMessage {
    pub fn new(seq_number: i64, orders_vec: Vec<Order>, p_in: i64) -> IncrementalMessage { 
        // return a new incremental message
        IncrementalMessage {
        	price: p_in,
            seq_number: seq_number,
            orders_vec: orders_vec,
        }
    }  

    // Price getter: return price affected
    pub fn get_price(&self) -> i64 {
        self.price
    }

    // return sequence number
    pub fn get_num(&self) -> i64 {
    	self.seq_number
    }

    // return orders affected
    pub fn get_orders(&self) -> Vec<Order> {
    	self.orders_vec.clone()
    }
}