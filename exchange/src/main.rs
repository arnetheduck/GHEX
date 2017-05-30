use std::io;

mod objects;
mod matching_engine;

fn main() {
	let mut match_eng = matching_engine::MatchingEngine::new();
	// Create new order: quantity, price, side (1 = buy, 2 = sell)

	// for i in 1..5 {
	// 	println!("NEW ORDER");
	// 	println!();

	// 	let mut curOrder = objects::Order::new(i, 1000 * i, '2');
	// 	match_eng.insert(&curOrder);
	// 	println!("{:?}", curOrder);
	// 	match_eng.print_status();
	// }

	// for i in (1..5).rev() {
	// 	println!("NEW ORDER");
	// 	println!();

	// 	let mut curOrder = objects::Order::new(i + 5, 1000 * i, '1');
	// 	match_eng.insert(&curOrder);
	// 	println!("{:?}", curOrder);
	// 	match_eng.print_status();
	// }
	
	loop {
		println!("**********************************");
		println!("Continue? (y/n) ");
		let mut continue_cmd = String::new();
		io::stdin().read_line(&mut continue_cmd);
		
		match &continue_cmd.trim() as &str {
			"y" => { },
			"n" => break,
			_ 	=> {
				println!("Enter y or n!");
				continue;
			}
		}

		// Ask user to input new order
		println!("Insert new order: ");
		
		println!("Quantity: ");
		let mut m_qty = String::new();
		io::stdin().read_line(&mut m_qty);
		let m_qty = m_qty.trim().parse::<u64>().unwrap();
		// println!("{}", m_qty);

		println!("Price: ");
		let mut m_price = String::new();
		io::stdin().read_line(&mut m_price);
		let m_price = m_price.trim().parse::<u64>().unwrap();
		// println!("{}", m_price);		

		println!("Side: (1 = buy, 2 = sell)");
		let mut m_side = String::new();
		io::stdin().read_line(&mut m_side);
		let m_side: char = m_side.chars().nth(0).unwrap();
		// println!("{}", m_side);

		match_eng.insert(&objects::Order::new(m_qty, m_price, m_side));
		match_eng.print_status();
	}
}