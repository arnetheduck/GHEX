pub struct MatchingEngine {
    account: String,
    ciord_id: String,
    order_qty: u64,
    order_type: char,
    price: u64,
    side: char,
    symbol: String
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine { 
    	MatchingEngine {
            account: "String".to_string(),
            ciord_id: "String".to_string(),
            order_qty: 0,
            order_type: '0',
            price: 0,
            side: '0',
            symbol: "String".to_string()
    	}
    }
}