mod objects;
mod matching_engine;
#[cfg(test)]
mod tests {

	use super::objects::Order;
 	use super::matching_engine::MatchingEngine;
 	#[test]
 	fn update_qty_inc() {
 		let mut match_eng = MatchingEngine::new();
 		// old_order: quantity = 100, price = 1000, side = sell
 		let old_order = Order::new(100, 1000, '2');
 		// another_order: quantity = 10, price = 1000, side = sell
 		let another_order = Order::new(10, 1000, '2');
 		// 1st insertion (id = 0): Insert old_order (no matching happens)
 		// old_order_traded: return order of first insertion (copy of old_order with id = 0)
 		let old_order_traded = match_eng.insert(&old_order);
 		// 2nd insertion (id = 1): Insert another_order (no matching happens)
 		match_eng.insert(&another_order);

 		// old_id = 0 (id of 1st order inserted)
 		let old_id = old_order_traded.get_id();
 		// Create new order with same order ID as old_order to update
 		// new_order: quantity = 150, price = 1000, side = sell (quantity increases from 100 to 150)
 		let mut new_order = Order::new(150, 1000, '2');
 		new_order.set_id(&old_id);
 		// Update old_order (id = 0) with new order
 		match_eng.update(&old_id, &new_order);
 		// After update, order with ID = 0 is moved to the back of the queue

 		// Check quantity of updated order
 		assert_eq!(match_eng.find_order_by_id(&old_id).get_qty(), 150);

 		// yet_another_order: quantity = 11, price = 1000, side = buy
 		let yet_another_order = Order::new(11, 1000, '1');
 		// 3rd insertion (id = 2): Insert yet_another_order (matching happens)
 		// - 1st match: with another_order (id = 1, quantity = 10) -> 10 matched, 1 remaining
 		// - 2nd match: with old_order (id = 0, quantity = 150) -> 1 matched, 0 remaining
 		// => old_order: quantity = 149
 		match_eng.insert(&yet_another_order);
 		// Check quantity of old_order (id = 0) after matching
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
 	
 	fn test_update_price() {
 		let mut match_eng = MatchingEngine::new();
 		let old_order = Order::new(100, 1000, '2');

 		let old_order_after = match_eng.insert(&old_order);


 	}
}