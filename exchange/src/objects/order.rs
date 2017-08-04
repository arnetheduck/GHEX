/** 
    ORDER OBJECT

    This contains all relevant properties and functions of an order object
*/
extern crate time;
extern crate serde;

use std::cmp::Ordering;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};

/**
    An order has the following properties:
        - id:               
            ID of an order. Assigned by the exchange
            (e.g, ID is used when users send an UPDATE or DELETE request)
        - order_qty:
            Quantity of an order. Assigned by users when entering the order
        - price:
            Price of an order. Assigned by users when entering the order
        - side:             
            Side of an order. Assigned by users when entering the order 
            ('1' for BUY, '2' for SELL)
        - transact_time:
            Time stamp of an order. Assigned by the exchange at the time the order was processed
            (UTC format: YYYYMMDD-HH:MM:SS.sss)
*/
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Order {
    id: String,
    order_qty: i64,
    price: i64,
    side: char, // '1' = BUY, '2' = SELL
    transact_time: String, // UTC format: YYYYMMDD-HH:MM:SS.sss
}

impl Order {
    /**
        Constructor
        @params 
            m_qty: order's quantity
            m_price: order's price
            m_side: order's side ('1' = BUY, '2' = SELL)
        @return
            New order with:
            - DUMMY ID (-1). Wait to be assigned by the exchange
            - Quantity assigned by user
            - Price assigned by user
            - Side assigned by user
            - Transaction time is the time when order created, assigned automatically when a new order created
    */
    pub fn new(m_qty: i64, m_price: i64, m_side: char) -> Order { 
        // Get current time in UTC format
        let mut cur_time: String = time::now_utc().strftime("%Y%m%d-%H:%M:%S.%f").unwrap().to_string();
        // Remove unnecessary characters to ensure correct UTC format
        cur_time.truncate(21);

        // Return a new order object
        Order {
        	id: "-1".to_string(),
            order_qty: m_qty,
            price: m_price,
            side: m_side,
            transact_time: cur_time,
        }
    }

    // Return ID of order
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    
    // Return quantity of order 
    pub fn get_qty(&self) -> i64 {
        self.order_qty
    }    

    // Return price of order
    pub fn get_price(&self) -> i64 {
        self.price
    }

    // Return side of order ('1'= BUY, '2' = SELL)
    pub fn get_side(&self) -> char {
        self.side
    }

    // Return transaction time of order
    pub fn get_transact_time(&self) -> String {
        self.transact_time.clone()
    }

    /**
        Set new ID for order (used by the exchange)
        @params
            m_id: new ID
    */
	pub fn set_id(&mut self, m_id: &String) {
        self.id = m_id.clone();
    }    

    /**
        Set new quantity for order
        @params
            m_qty: new quantity
    */
    pub fn set_qty(&mut self, m_qty: i64) {
        self.order_qty = m_qty;
    }    

    /**
        Set new price for order
        @params
            m_price: new price
    */
    pub fn set_price(&mut self, m_price: i64) {
        self.price = m_price;
    }

    /**
        Set new side for order
        @params
            m_side: new side
    */
    pub fn set_side(&mut self, m_side: char) {
        self.side = m_side;
    }    

    /**
        Set new transaction time for order
        @params
            m_time: new transaction time
    */
    pub fn set_transact_time(&mut self, m_time: &String) {
        self.transact_time = m_time.clone();
    }
}

impl Eq for Order {}

/**
    This function defines ordering of the orders regarding price/time priority.
    Priority of an order is assessed following the rules:
        1. Orders with different prices are ranked by their prices
            + SELL side: LOWER price, HIGHER priority
            + BUY side: HIGHER price, HIGHER priority
        2. Else, orders with same price are ranked by their transaction time
            + EARLIER order has HIGHER priority
*/
impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.side == '2' {
            // Sell side
            // Determine priority of orders
            if other.price.eq(&self.price) {
                // Consider time priority only if the orders have same price
                // i.e, EARLIER transaction time, HIGHER priority
                other.transact_time.partial_cmp(&self.transact_time)
            } else {
                // Else consider price priority
                // i.e, SELL side: LOWER price, HIGHER priority
                other.price.partial_cmp(&self.price)
            }
        } else {
            // Buy side
            // Determine priority of orders
            if other.price.eq(&self.price) {
                // Consider time priority only if the orders have same price
                // i.e, EARLIER transaction time, HIGHER priority
                other.transact_time.partial_cmp(&self.transact_time)
            } else {            
                // Else consider price priority
                // i.e, BUY side: HIGER price, HIGHER priority
                self.price.partial_cmp(&other.price)
            }
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}