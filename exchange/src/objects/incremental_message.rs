/** 
    INCREMENTAL MESSAGE

    This contains all relevant properties and functions of an incremental message
*/

extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::serde::de::Deserialize;
use objects::Order;

/**
    An incremental message has the following properties:
        - price:               
            Price affected after an operation
            (i.e, List of orders at this price is modified (inserted new order, deleted an order, updated an order))
        - seq_number:
            Sequential number of the incremental message. Assigned by the exchange
        - orders_vec:
            A vector containing all the orders at the affected price 
            (After an operation, ONLY list orders at this price is modified.)
            At a time, at a specific price during TRADING period, orders at that price can be on ONE side only.
            Because if there are orders on both side, they will be matched immediately until one side is completely matched.
*/
#[derive (Serialize, Deserialize)]
pub struct IncrementalMessage {
	price: i64,
    seq_number: i64,
    orders_vec: Vec<Order>,
}

impl IncrementalMessage {
    /*
        Constructor
        @params 
            p_in: price affected
            seq_number: sequential number of incremental message
            orders_vec: list of orders affected
        @return
            New incremental message with:
            - Price affected
            - Sequential number assigned by the exchange
            - List of orders affected (a vector)
    */
    pub fn new(p_in: i64, seq_number: i64, orders_vec: Vec<Order>) -> IncrementalMessage { 
        // Return a new incremental message
        IncrementalMessage {
        	price: p_in,
            seq_number: seq_number,
            orders_vec: orders_vec,
        }
    }  

    // Return price affected
    pub fn get_price(&self) -> i64 {
        self.price
    }

    // Return sequential number
    pub fn get_num(&self) -> i64 {
    	self.seq_number
    }

    // Return list of orders affected
    pub fn get_orders(&self) -> Vec<Order> {
    	self.orders_vec.clone()
    }
}