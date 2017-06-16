mod objects;
mod matching_engine;
#[cfg(test)]
mod tests {

	use super::objects::Order;
 	use super::matching_engine::MatchingEngine;
 	#[test]
 	fn update_qty_inc() {
 		let mut match_eng = MatchingEngine::new();
 		let old_order = Order::new(100, 1000, '2');
 		let another_order = Order::new(10, 1000, '2');
 		let old_order_traded = match_eng.insert(&old_order);
 		match_eng.insert(&another_order);

 		let old_id = old_order_traded.get_id();
 		let mut new_order = Order::new(150, 1000, '2');
 		new_order.set_id(&old_id);
 		
 		match_eng.update(&old_id, &new_order);
 		assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 150);

 		let yet_another_order = Order::new(11, 1000, '1');
 		match_eng.insert(&yet_another_order);
 		assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 149);
 	}
 	#[test]
 	fn update_qty_dec() {
 		let mut match_eng = MatchingEngine::new();
 		let old_order = Order::new(100, 1000, '2');
 		let another_order = Order::new(10, 1000, '2');
 		let old_order_after = match_eng.insert(&old_order);
 		match_eng.insert(&another_order);

 		let old_id = old_order_after.get_id();
 		let mut new_order = Order::new(95, 1000, '2');
 		new_order.set_id(&old_id);
 		
 		match_eng.update(&old_id, &new_order);
 		assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 95);

 		let yet_another_order = Order::new(11, 1000, '1');
 		match_eng.insert(&yet_another_order);
 		assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 84);
 	}
 	
 	// fn test_update_price() {
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let old_order = Order::new(100, 1000, '2');

 	// 	let old_order_after = match_eng.insert(&old_order);


 	// }
 	// // fn test_update_quantity_position() {

 	// }
 	// fn test_1() {
 	// 	let mut match_eng = MatchingEngine::new();
 	// 	let mut order_0 = Order::new(2, 850, '1');
 	// 	let mut order_1 = Order::new(50, 900, '1');
 	// 	let mut order_2 = Order::new(5, 900, '1');

 	// 	let order_0_clone = order_0.clone(); 
 	// 	let order_1_clone = order_1.clone();
 	// 	let order_2_clone = order_2.clone();
 	// 	order_0.set_transact_time(&(order_0_clone.get_transact_time() + "1".to_string().as_str()));
 	// 	order_1.set_transact_time(&(order_1_clone.get_transact_time() + "2".to_string().as_str()));
 	// 	order_2.set_transact_time(&(order_2_clone.get_transact_time() + "3".to_string().as_str()));
 	// 	match_eng.insert(&order_0);
 	// 	match_eng.insert(&order_1);
 	// 	match_eng.insert(&order_2);
 	// 	let mut expected_1 = Vec::new();
 	// 	expected_1.push(&order_1);
 	// 	expected_1.push(&order_2);
 	// 	assert_eq!(match_eng.get_buy_orders(&900i64), expected_1);
 	// 	match_eng.print_status();

 	// 	let order_3 = &Order::new(3, 850, '2');
 		
 	// 	match_eng.insert(&order_3);
 	// 	match_eng.print_status();
 	// 	let mut order_1_after = order_1.clone();
 	// 	order_1_after.set_qty(47);
 	// 	let mut expected_2 = Vec::new();
 	// 	expected_2.push(&order_1_after);
 	// 	expected_2.push(&order_2);
 	// 	assert_eq!(match_eng.get_buy_orders(&900i64), expected_2);


 	// 	let order_4 = &Order::new(55, 800, '2');
 	// 	match_eng.insert(order_4);
 	// 	let mut order_4_after = order_4.clone();
 	// 	order_4_after.set_qty(1);
 	// 	let mut expected_3 = Vec::new();
 	// 	expected_3.push(&order_4_after);
 		
 	// 	assert_eq!(match_eng.get_sell_orders(&800i64), expected_3);

 	// }
//     #[test]
//   	fn test_insert_basic() {
//     	let mut match_eng = MatchingEngine::new();
//     	// original orders
//     	let sell_order_1 = &Order::new(100, 1000, '2');
//     	let sell_order_2 = &Order::new(50, 1500, '2');
// 		let buy_order_1 = &Order::new(200, 2000, '1');

//     	match_eng.insert(sell_order_1);
//     	match_eng.insert(sell_order_2);
//     	let result = match_eng.insert(buy_order_1); // store resulting vector of traded order pairs 

//     	let mut expected: Vec<(Order, Order)> = Vec::new();
// 		let mut buy_1_traded = Order::new(200, 2000, '1');
//     	buy_1_traded.set_transact_time(&buy_order_1.get_transact_time()); // need same transact time
//     	let mut sell_1_traded = Order::new(100, 1000, '2');
//     	sell_1_traded.set_transact_time(&sell_order_1.get_transact_time());
// 		expected.push((buy_1_traded, sell_1_traded)); // first expected pair of orders traded

//     	let mut buy_1_traded = Order::new(100, 2000, '1');
//     	buy_1_traded.set_transact_time(&buy_order_1.get_transact_time()); 
//     	let mut sell_2_traded = Order::new(50, 1500, '2');
//     	sell_2_traded.set_transact_time(&sell_order_2.get_transact_time());
// 		expected.push((buy_1_traded, sell_2_traded)); // second expected pair of orders traded 
//     	/*
//     		Buy      		Sell
//     		2000 - 200 <--> 1000 - 100
//     		2000 - 100 <--> 1500 - 50
// 		*/
//     	assert_eq!(result, expected);
//     }
}