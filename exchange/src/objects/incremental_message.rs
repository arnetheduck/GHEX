extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::serde::de::Deserialize;
use objects::Order;

#[derive (Serialize, Deserialize)]
pub struct IncrementalMessage {
    seq_number: i64,
    orders_vec: Vec<Order>,
}

impl IncrementalMessage {
    pub fn new(seq_number: i64, orders_vec: Vec<Order>) -> IncrementalMessage { 
        // return a new incremental message
        IncrementalMessage {
            seq_number: seq_number,
            orders_vec: orders_vec,
        }
    }
}