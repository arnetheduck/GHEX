use std::cmp::Ordering;

extern crate time;

#[derive(PartialEq, Debug, Clone)]
pub struct Order {
    id: String,
    order_qty: i64,
    price: i64,
    side: char, // 1 = buy, 2 = sell
    transact_time: String, // UTC format: YYYYMMDD-HH:MM:SS.sss
}

impl Order {
    /*
        Constructor
        @params 
            m_qty: order quantity
            m_price: order price
            m_side: order side (1 = buy, 2 = sell)
        @return
            New order with quantity, price, side provided and transaction time is the time when order created
    */
    pub fn new(m_id: String, m_qty: i64, m_price: i64, m_side: char) -> Order { 
        // get current time in UTC format
        let mut cur_time: String = time::now_utc().strftime("%Y%m%d-%H:%M:%S.%f").unwrap().to_string();
        cur_time.truncate(21);

        // return a new order
        Order {
            id: m_id,
            order_qty: m_qty,
            price: m_price,
            side: m_side,
            transact_time: cur_time,
        }
    }

    // ID getter: return ID of order
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    // Quantity getter: return quantity of order 
    pub fn get_qty(&self) -> i64 {
        self.order_qty
    }    

    // Price getter: return price of order
    pub fn get_price(&self) -> i64 {
        self.price
    }

    // Side getter: return side of order (1 = buy, 2 = sell)
    pub fn get_side(&self) -> char {
        self.side
    }

    // Transaction time getter: return transaction time of order
    pub fn get_transact_time(&self) -> String {
        self.transact_time.clone()
    }

    /* 
        Quantity setter: set new quantity for order
        @params
            m_qty: new quantity
    */
    pub fn set_qty(&mut self, m_qty: i64) {
        self.order_qty = m_qty;
    }    

    /* 
        Price setter: set new price for order
        @params
            m_price: new price
    */
    pub fn set_price(&mut self, m_price: i64) {
        self.price = m_price;
    }

    /* 
        Side setter: set new side for order
        @params
            m_side: new side
    */
    pub fn set_side(&mut self, m_side: char) {
        self.side = m_side;
    }    

    /* 
        Transaction time setter: set new transaction time for order
        @params
            m_time: new transaction time
    */
    pub fn set_transact_time(&mut self, m_time: &String) {
        self.transact_time = m_time.clone();
    }
}

impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        /* 
            
        */
        if self.side == '2' {
            // Sell side
            // Determine priority of orders
            if other.price.eq(&self.price) {
                // Consider time priority only if the orders have same price
                // i.e, Earlier transaction time, higher priority
                other.transact_time.partial_cmp(&self.transact_time)
            } else {
                // Else consider price priority
                // i.e, Sell side: Lower price, higher priority
                other.price.partial_cmp(&self.price)
            }
        } else {
            // Buy side
            // Determine priority of orders
            if other.price.eq(&self.price) {
                // Consider time priority only if the orders have same price
                // i.e, Earlier transaction time, higher priority
                other.transact_time.partial_cmp(&self.transact_time)
            } else {            
                // Else consider price priority
                // i.e, Buy side: Higher price, higher priority
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