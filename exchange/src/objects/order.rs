use std::cmp::Ordering;

extern crate time;

#[derive(PartialEq, Debug, Clone)]
pub struct Order {
    order_qty: u64,
    price: u64,
    side: char, // 1: buy, 2: sell
    transact_time: String,
}

impl Order {
    pub fn new(m_qty: u64, m_price: u64, m_side: char) -> Order { 
        let mut cur_time: String = time::now_utc().strftime("%Y%m%d-%H:%M:%S.%f").unwrap().to_string();
        cur_time.truncate(21);

        Order {
            order_qty: m_qty,
            price: m_price,
            side: m_side,
            transact_time: cur_time,
        }
    }

    pub fn get_qty(&self) -> u64 {
        self.order_qty
    }    

    pub fn get_price(&self) -> u64 {
        self.price
    }

    pub fn get_side(&self) -> char {
        self.side
    }

    pub fn get_transact_time(&self) -> String {
        self.transact_time.clone()
    }

    pub fn set_qty(&mut self, m_qty: u64) {
        self.order_qty = m_qty;
    }    

    pub fn set_price(&mut self, m_price: u64) {
        self.price = m_price;
    }

    pub fn set_side(&mut self, m_side: char) {
        self.side = m_side;
    }    

    pub fn set_transact_time(&mut self, m_time: &String) {
        self.transact_time = m_time.clone();
    }
}

impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        /* 
            Sell side
            Reverse default ordering to obtain min heap
        */
        if self.side == '2' {
            if other.price.eq(&self.price) {
                other.transact_time.partial_cmp(&self.transact_time)
            } else {
                other.price.partial_cmp(&self.price)
            }
        } else {
            if other.price.eq(&self.price) {
                other.transact_time.partial_cmp(&self.transact_time)
            } else {            
                self.price.partial_cmp(&other.price)
            }
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        /* 
            Sell side
            Reverse default ordering to obtain min heap
        */        
        if self.side == '2' {
            match ord {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => ord,
            }
        } else {
            match ord {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => ord,
            }       
        }
    }
}