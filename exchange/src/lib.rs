mod objects;
mod matching_engine;

// #[cfg(test)]
// mod tests {
// 	use super::objects::Order;
// 	use super::matching_engine::MatchingEngine;
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
// }