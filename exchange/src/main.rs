mod objects;
mod matching_engine;

fn main() {
	let mut match_eng = matching_engine::MatchingEngine::new();
	// Create new order: quantity, price, side (1 = buy, 2 = sell)
	for i in 1..3 {
		let mut curOrder = objects::Order::new(i, 1000 * i, '2');
		match_eng.insert(&curOrder);
		println!("{:?}", curOrder);
	}

	match_eng.print_status();
}