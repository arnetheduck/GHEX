extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::serde::de::Deserialize;
use objects::Order;

#[derive (Serialize, Deserialize)]
pub struct RecoveryFeed {
    last_msg: i64,
    state: Vec<Vec<Order>>,
}

impl RecoveryFeed {
    pub fn new(last_msg_index: i64, cur_state: Vec<Vec<Order>>) -> RecoveryFeed { 
        // return a new incremental message
        RecoveryFeed {
        	last_msg: last_msg_index,
            state: cur_state,
        }
    }
}