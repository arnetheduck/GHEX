#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


mod objects;
mod matching_engine;
// NOTE: Run tests using 'cargo test -- --test-threads=1'
#[cfg(test)]
mod tests {
	/**
		* blackbox test
		* snapshot/incremental updates should always be checked after each order operation
		* check reverse side
		* notation 1x2: 1 (price) x 2 (quantity)

		scenarios:
		1. buy 1x1, sell 2x1 -> no match
		2. buy 1x1, sell 1x1 -> match
		3. buy 1x1, buy 2x1, sell 3x1 -> no match, sell 2x1 -> match buy 2
		4. buy 1x1, buy 2x1, sell 1x1 -> match buy 2
		5. buy 1x1, buy 1x1, buy 3x1, buy 3x1, sell 3x2 -> match buy 3s
		6. buy 1x1, buy 1x1, buy 3x1, buy 3x1, sell 1x4 -> match all buys
		7. buy 1x1, del buy 1x1 -> market empty
		8. buy 1x2, sell 1x1 -> match buy 1, del buy 1 -> market empty

	**/
	use super::objects::Order;
	use super::objects::IncrementalMessage;
 	use super::matching_engine::MatchingEngine;
 	use std::sync::mpsc::channel;
 	use std::{io, thread, time};
 	// case tests
 	#[test]
 	//#[ignore]
 	fn test_no_match() {
 		// buy 1 at 1, sell 1 at 2 --> no match
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
		
 		// create listener thread to check increment/snapshot
 		let thread_2 = thread::spawn(move || {
 			let mut id_1 = "".to_string();
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// if seq num of incremental msg is 1, check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						// check order inserted correctly
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 2);
	 						// check previous order not affected (no match, see below) 
	 						tx_2.send(id_1.clone());
	 					}
	 					else {
	 						// error: only 2 increment msgs should be broadcast in this scenario
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut sell_order = Order::new(1, 2, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&sell_order);
 		
		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// no match: check that previously inserted order was not affected
				assert_eq!(match_eng.find_order_by_id(&v).get_qty(), 1);
				
			}
			Err(r) => {println!("error");},
		}
		
	}

 	#[test]
 	//#[ignore]
 	fn test_match() {
 		//buy 1 at 1, sell 1 at 1 --> match
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
 		let mut id_1 = "".to_string();
 		let thread_2 = thread::spawn(move || {
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						// check order inserted was matched (see below)
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(id_1.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut sell_order = Order::new(1, 1, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&sell_order);
 	
 		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// match: check first order no longer exists
				assert_eq!(match_eng.find_order_by_id(&v).get_side(), '*');
				
			}
			Err(r) => {println!("error");},
		}
 	}
 	#[test]
 	//#[ignore]
 	fn test_no_match_match() {
 		// buy 1 at 1, buy 1 at 2, sell 1 at 3 --> no match
 		// sell 1 at 2 --> match buy at 2
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
		
 		let thread_2 = thread::spawn(move || {
 			let mut id_1 = "".to_string(); 	
 			let mut id_2 = "".to_string();
 			let mut ids: Vec<String> = Vec::new();		
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// if seq num of incremental msg is 1, check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						ids.push(id_1.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						id_2 = affected_ords[0].get_id();
	 						ids.push(id_2.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 2);
	 						
	 					}
	 					else if val.get_num() == 3 {
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);

	 						tx_2.send(ids.clone());
	 					}
	 					else if val.get_num() == 4 {
	 						// check only buy order at 2 was matched (see below)
	 						assert_eq!(affected_ords.len(), 0);
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut buy_order_2 = Order::new(1, 2, '1');
 		let mut sell_order = Order::new(1, 3, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&buy_order_2);
 		match_eng.insert(&sell_order);
 		
		let id = rx_2.recv();
		let mut id_1 = "".to_string();
		let mut id_2 = "".to_string();
		match id {
			Ok(v) => {
				id_1 = v[0].clone();
				id_2 = v[1].clone();
				assert_eq!(match_eng.find_order_by_id(&v[0]).get_qty(), 1);
				assert_eq!(match_eng.find_order_by_id(&v[1]).get_qty(), 1);
			}
			Err(r) => {println!("error");},
		}

		let mut sell_order_2 = Order::new(1, 2, '2');
		match_eng.insert(&sell_order_2);
		// check buy order at 1 not affected
		assert_eq!(match_eng.find_order_by_id(&id_1).get_qty(), 1);
		// check match occured (assert buy at 2 no longer exists)
		assert_eq!(match_eng.find_order_by_id(&id_2).get_side(), '*');

 	}

 	#[test]
 	//#[ignore]
 	fn test_match_2() {
 		// buy 1 at 1, buy 1 at 2, sell 1 at 1 --> match buy at 2
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
		
 		let thread_2 = thread::spawn(move || {
 			let mut id_1 = "".to_string(); 	
 			let mut id_2 = "".to_string();
 			let mut ids: Vec<String> = Vec::new();		
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// if seq num of incremental msg is 1, check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						ids.push(id_1.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						id_2 = affected_ords[0].get_id();
	 						ids.push(id_2.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 2);
	 						
	 					}
	 					else if val.get_num() == 3 {
	 						// check order inserted was matched correctly (see below)
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(ids.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut buy_order_2 = Order::new(1, 2, '1');
 		let mut sell_order = Order::new(1, 1, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&buy_order_2);
 		match_eng.insert(&sell_order);
 		
		let id = rx_2.recv();
		match id {
			Ok(v) => {
				assert_eq!(match_eng.find_order_by_id(&v[0]).get_qty(), 1);
				assert_eq!(match_eng.find_order_by_id(&v[1]).get_side(), '*');
			}
			Err(r) => {println!("error");},
		}
 	}

 	#[test]
 	//#[ignore]
 	fn test_match_3() {
 		// buy 1 at 1, buy 1 at 1, buy 1 at 3, buy 1 at 3, sell 2 at 3 --> match 2 buys at 3
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
		
 		let thread_2 = thread::spawn(move || {
 			let mut id_1 = "".to_string(); 	
 			let mut id_2 = "".to_string();
 			let mut id_3 = "".to_string();
 			let mut id_4 = "".to_string();
 			let mut ids: Vec<String> = Vec::new();		
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// if seq num of incremental msg is 1, check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						ids.push(id_1.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						id_2 = affected_ords[0].get_id();
	 						ids.push(id_2.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 						
	 					}
	 					else if val.get_num() == 3 {
	 						id_3 = affected_ords[0].get_id();
	 						ids.push(id_3.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);
	 					}
	 					else if val.get_num() == 4 {
	 						id_4 = affected_ords[0].get_id();
	 						ids.push(id_4.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);
	 					}
	 					else if val.get_num() == 5 {
	 						// 1 more buy at 3 should be remaining to be matched
	 						assert_eq!(affected_ords.len(), 1);
	 					}
	 					else if val.get_num() == 6 {
	 						// check order inserted was matched correctly (see below)
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(ids.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut buy_order_2 = Order::new(1, 1, '1');
 		let mut buy_order_3 = Order::new(1, 3, '1');
 		let mut buy_order_4 = Order::new(1, 3, '1');
 		let mut sell_order = Order::new(2, 3, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&buy_order_2);
 		match_eng.insert(&buy_order_3);
 		match_eng.insert(&buy_order_4);
 		match_eng.insert(&sell_order);
 		
		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// check buys at 1 not affected
				assert_eq!(match_eng.find_order_by_id(&v[0]).get_qty(), 1);
				assert_eq!(match_eng.find_order_by_id(&v[1]).get_qty(), 1);
				// check buys at 3 got matched (no longer exist)
				assert_eq!(match_eng.find_order_by_id(&v[2]).get_side(), '*');
				assert_eq!(match_eng.find_order_by_id(&v[3]).get_side(), '*');
			}
			Err(r) => {println!("error");},
		}
 	}

 	#[test]
 	//#[ignore]
 	fn test_match_4() {
 		// buy 1 at 1, buy 1 at 1, buy 1 at 3, buy 1 at 3, sell 4 at 1 --> match all
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
		
 		let thread_2 = thread::spawn(move || {
 			let mut id_1 = "".to_string(); 	
 			let mut id_2 = "".to_string();
 			let mut id_3 = "".to_string();
 			let mut id_4 = "".to_string();
 			let mut ids: Vec<String> = Vec::new();		
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					// if seq num of incremental msg is 1, check buy order was inserted correctly
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						ids.push(id_1.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						id_2 = affected_ords[0].get_id();
	 						ids.push(id_2.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 						
	 					}
	 					else if val.get_num() == 3 {
	 						id_3 = affected_ords[0].get_id();
	 						ids.push(id_3.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);
	 					}
	 					else if val.get_num() == 4 {
	 						id_4 = affected_ords[0].get_id();
	 						ids.push(id_4.clone());
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);
	 					}
	 					else if val.get_num() == 5 {
	 						// 1 more buy at 3 to be matched
	 						assert_eq!(affected_ords.len(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 3);
	 					}
	 					else if val.get_num() == 6 {
	 						// all buys at 3 matched
	 						assert_eq!(affected_ords.len(), 0);
	 					}
	 					else if val.get_num() == 7 {
	 						// 1 more buy at 1 to be matched
	 						assert_eq!(affected_ords.len(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 8 {
	 						// match completed
	 						// check order inserted was matched correctly (see below)
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(ids.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let mut buy_order_2 = Order::new(1, 1, '1');
 		let mut buy_order_3 = Order::new(1, 3, '1');
 		let mut buy_order_4 = Order::new(1, 3, '1');
 		let mut sell_order = Order::new(4, 1, '2');
 		match_eng.insert(&buy_order);
 		match_eng.insert(&buy_order_2);
 		match_eng.insert(&buy_order_3);
 		match_eng.insert(&buy_order_4);
 		match_eng.insert(&sell_order);
 		
		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// check all buys got matched (no longer exist)
				assert_eq!(match_eng.find_order_by_id(&v[0]).get_side(), '*');
				assert_eq!(match_eng.find_order_by_id(&v[1]).get_side(), '*');
				assert_eq!(match_eng.find_order_by_id(&v[2]).get_side(), '*');
				assert_eq!(match_eng.find_order_by_id(&v[3]).get_side(), '*');
			}
			Err(r) => {println!("error");},
		}
 	}

 	#[test]
 	//#[ignore]
 	fn test_empty() {
 		// buy 1 at 1, delete buy 1 at 1 --> market empty
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
 		let mut id_1 = "".to_string();
 		let thread_2 = thread::spawn(move || {
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(id_1.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(1, 1, '1');
 		let id = match_eng.insert(&buy_order).get_id();
 		match_eng.delete(&id);
 	
 		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// check order was deleted
				assert_eq!(match_eng.find_order_by_id(&v).get_side(), '*');
				
			}
			Err(r) => {println!("error");},
		}

 		
 	}
 	#[test]
 	fn test_match_empty() {
 		// buy 2 at 1, sell 1 at 1 --> match buy at 1
 		// delete buy at 1 --> market empty
 		// buy 1 at 1, delete buy 1 at 1 --> market empty
 		let (tx, rx) = channel();
 		let (tx_2, rx_2) = channel();
 		let mut match_eng = MatchingEngine::new(&tx);
 		let mut id_1 = "".to_string();
 		let thread_2 = thread::spawn(move || {
 			loop {
	 			let data = rx.try_recv();
	 			match data {
	 				Ok(v) => {
	 					let val: IncrementalMessage = ::serde_json::from_str(v.as_str()).unwrap();
	 					let affected_ords = val.get_orders();
	 					if val.get_num() == 1 {
	 						id_1 = affected_ords[0].get_id();
	 						assert_eq!(affected_ords[0].get_qty(), 2);
	 						assert_eq!(affected_ords[0].get_price(), 1);
	 					}
	 					else if val.get_num() == 2 {
	 						// check that remaining quantity of buy order is 1 after match
	 						assert_eq!(affected_ords[0].get_qty(), 1);
	 					}
	 					else if val.get_num() == 3 {
	 						assert_eq!(affected_ords.len(), 0);
	 						tx_2.send(id_1.clone());
	 					}
	 					else {
	 						panic!("Error: incorrect number of increments received");
	 					}

	 				}
	 				Err(r) => {},
	 			}
	 		}
 		});

 		let mut buy_order = Order::new(2, 1, '1');
 		let mut sell_order = Order::new(1, 1, '2');
 		let id = match_eng.insert(&buy_order).get_id();
 		match_eng.insert(&sell_order);
 		match_eng.delete(&id);
 	
 		let id = rx_2.recv();
		match id {
			Ok(v) => {
				// check buy order was deleted
				assert_eq!(match_eng.find_order_by_id(&v).get_side(), '*');
				
			}
			Err(r) => {println!("error");},
		}
 	}
 	// unit tests
 	//#[test]
 	// fn test_find_order_by_id() {
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let mut first_order = Order::new(100, 100, '1');
 	// 	// first_order: ID = 0
 	// 	match_eng.insert(&first_order);
 	// 	first_order.set_id(&'0'.to_string());
 	// 	assert_eq!(match_eng.find_order_by_id(&first_order.get_id()), first_order);

 	// 	let mut second_order = Order::new(50, 10, '1');
 	// 	// second order: ID = 1
 	// 	match_eng.insert(&second_order);
 	// 	second_order.set_id(&'1'.to_string());
 	// 	assert_eq!(match_eng.find_order_by_id(&second_order.get_id()), second_order);

 	// 	let mut third_order = Order::new(125, 1, '2');
 	// 	// third order: ID = 2
 	// 	// Matching happens hear
 	// 	match_eng.insert(&third_order);

 	// 	third_order.set_id(&'2'.to_string());
 	// 	assert_eq!(match_eng.find_order_by_id(&third_order.get_id()), Order::new(-1, -1, '*'));
 	// 	assert_eq!(match_eng.find_order_by_id(&'0'.to_string()), Order::new(-1, -1, '*'));

 	// 	second_order.set_qty(25);
 	// 	assert_eq!(match_eng.find_order_by_id(&'1'.to_string()), second_order);
 	// }

 	// #[test]
 	// fn update_qty_inc() {
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	// old_order: quantity = 100, price = 1000, side = sell
 	// 	let old_order = Order::new(100, 1000, '2');
 	// 	// another_order: quantity = 10, price = 1000, side = sell
 	// 	let another_order = Order::new(10, 1000, '2');
 	// 	// 1st insertion (id = 0): Insert old_order (no matching happens)
 	// 	// old_order_traded: return order of first insertion (copy of old_order with id = 0)
 	// 	let old_order_traded = match_eng.insert(&old_order);
 	// 	// 2nd insertion (id = 1): Insert another_order (no matching happens)
 	// 	match_eng.insert(&another_order);

 	// 	// old_id = 0 (id of 1st order inserted)
 	// 	let old_id = old_order_traded.get_id();
 	// 	// Create new order with same order ID as old_order to update
 	// 	// new_order: quantity = 150, price = 1000, side = sell (quantity increases from 100 to 150)
 	// 	let mut new_order = Order::new(150, 1000, '2');
 	// 	new_order.set_id(&old_id);
 	// 	// Update old_order (id = 0) with new order
 	// 	match_eng.update(&old_id, &new_order);
 	// 	// After update, order with ID = 0 is moved to the back of the queue

 	// 	// Check quantity of updated order
 	// 	assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 150);

 	// 	// yet_another_order: quantity = 11, price = 1000, side = buy
 	// 	let yet_another_order = Order::new(11, 1000, '1');
 	// 	// 3rd insertion (id = 2): Insert yet_another_order (matching happens)
 	// 	// - 1st match: with another_order (id = 1, quantity = 10) -> 10 matched, 1 remaining
 	// 	// - 2nd match: with old_order (id = 0, quantity = 150) -> 1 matched, 0 remaining
 	// 	// => old_order: quantity = 149
 	// 	match_eng.insert(&yet_another_order);
 	// 	// Check quantity of old_order (id = 0) after matching
 	// 	assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 149);
 	// }
 	
 	// #[test]
 	// fn update_qty_dec() {
 	// 	// insert orders
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let old_order = Order::new(100, 1000, '2');
 	// 	let another_order = Order::new(10, 1000, '2');
 	// 	let old_order_after = match_eng.insert(&old_order);
 	// 	match_eng.insert(&another_order);

 	// 	// update old_order with new quantity 95
 	// 	let old_id = old_order_after.get_id();
 	// 	let mut new_order = Order::new(95, 1000, '2');
 	// 	new_order.set_id(&old_id);
 	// 	match_eng.update(&old_id, &new_order);

 	// 	// check that order quantity was updated correctly
 	// 	assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 95);

 	// 	// insert buy order to trigger trade
 	// 	let yet_another_order = Order::new(11, 1000, '1');
 	// 	match_eng.insert(&yet_another_order);

 	// 	// check that order position is correct after update
 	// 	// since qty decreased, old_order keeps its place in queue
 	// 	// so quantity traded should be 95 - 11 = 84
 	// 	assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 84);
 	// }
 	
 	// #[test]
 	// fn test_update_price() {
 	// 	// insert order
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let old_order = Order::new(100, 1000, '2');
 	// 	let old_order_after = match_eng.insert(&old_order);

 	// 	// update price of inserted order
 	// 	let old_id = old_order_after.get_id();
 	// 	let mut new_order = Order::new(100, 1200, '2');
 	// 	match_eng.update(&old_id, &new_order);

 	// 	// check that the order price was updated correctly
 	// 	assert_eq!(match_eng.find_order_by_id(&old_id).get_price(), 1200);
 	// }

 	// #[test]
 	// fn test_delete() {
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let order = Order::new(100, 1000, '1');
 	// 	let order_id = match_eng.insert(&order).get_id();
 	// 	match_eng.delete(&order_id);

 	// 	// check that order was deleted
 	// 	// if order not found, find_order_by_id() returns dummy order with qty = -1
 	// 	assert_eq!(match_eng.find_order_by_id(&order_id).get_qty(), -1);
 	// }
 	
}