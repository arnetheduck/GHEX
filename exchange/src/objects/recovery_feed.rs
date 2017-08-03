/** 
    RECOVERY FEED

    This contains all relevant properties and functions of a recovery feed
*/
extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::serde::de::Deserialize;
use objects::Order;

/**
    A recovery feed has the following properties:
        - last_msg:               
            Index of the last sequential message received BEFORE the recovery feed being built and sent
        - state:
            A collection of lists of orders at ALL prices in the order book
*/
#[derive (Serialize, Deserialize)]
pub struct RecoveryFeed {
    last_msg: i64,
    state: Vec<Vec<Order>>,
}

impl RecoveryFeed {
    /*
        Constructor
        @params 
            last_msg_index: Index of the last sequential meesage, assigned by the exchange
            cur_state: Collection of the lists of orders at ALL prices, describing current state of the order book
        @return
            New recovery feed with:
            - Index of the last sequential message, assigned by the exchange
            - Current state of the order book (Collection of the lists of all orders)
    */
    pub fn new(last_msg_index: i64, cur_state: Vec<Vec<Order>>) -> RecoveryFeed { 
        // Return a new recovery feed
        RecoveryFeed {
        	last_msg: last_msg_index,
            state: cur_state,
        }
    }
}