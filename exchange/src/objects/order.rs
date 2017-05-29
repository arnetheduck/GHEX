use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct Order {
    account: String,
    ciord_id: String,
    order_qty: u64,
    order_type: char,
    price: u64,
    side: char, // 1: buy, 2: sell
    symbol: String,
}

impl Order {
    pub fn new(m_qty: u64, m_price: u64, m_side: char) -> Order { 
        Order {
            account: "String".to_string(),
            ciord_id: "String".to_string(),
            order_qty: m_qty,
            order_type: '0',
            price: 0,
            side: '0',
            symbol: "String".to_string()
        }
    }

    pub fn get_account(&self) -> &String {
        &self.account
    }    

    pub fn get_id(&self) -> &String {
        &self.ciord_id
    }

    pub fn get_qty(&self) -> u64 {
        &self.order_qty
    }    

    pub fn get_type(&self) -> char {
        &self.order_type
    }

    pub fn get_price(&self) -> u64 {
        &self.price
    }

    pub fn get_side(&self) -> char {
        self.side
    }

    pub fn get_symbol(&self) -> &String {
        &self.ciord_id
    }

    pub fn set_account(&mut self, m_acc: &String) {
        self.account = m_acc.to_string();
    }

    pub fn set_account(&mut self, m_acc: &String) {
        self.account = m_acc.to_string();
    }

    pub fn set_account(&mut self, m_acc: &String) {
        self.account = m_acc.to_string();
    }    
}

impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.side == '2' {
            other.price.partial_cmp(&self.price)
        } else {
            self.price.partial_cmp(&other.price)
        }
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
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
