mod objects;
mod matching_engine;

fn main() {
	let match_eng = matching_engine::MatchingEngine::new();
	// Create new order: quantity, price, side (1 = buy, 2 = sell)
	let mut curOrder = objects::Order::new(10, 1000, '2');
	match_eng.insert(&curOrder);
	println!("{:?}", curOrder);
}