use std::io;
use std::thread;
use std::sync::mpsc::channel;
use std::net::UdpSocket;

mod objects;
mod matching_engine;
const MULTICAST_GROUP_ADDRESS: &str = "239.255.255.255:21003";

fn insert_new_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to input new order
	println!("Insert new order: ");

	println!("Side: (1 = buy, 2 = sell)");
	let mut m_side = String::new();
	io::stdin().read_line(&mut m_side);
	let m_side: char = m_side.chars().nth(0).unwrap();
	// println!("{}", m_side);

	println!("Price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();
	// println!("{}", m_price);

	println!("Quantity: ");
	let mut m_qty = String::new();
	io::stdin().read_line(&mut m_qty);
	let m_qty = m_qty.trim().parse::<i64>().unwrap();
	// println!("{}", m_qty);		

	match_eng.insert(&objects::Order::new(m_qty, m_price, m_side));
}

fn delete_existing_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to input order order_id
	println!("Enter order ID:");
	let mut m_id = String::new();
	io::stdin().read_line(&mut m_id);
	let m_id = m_id.trim().to_string();

	match_eng.delete(&m_id);
}

fn update_existing_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to input new order
	println!("Enter order ID:");
	let mut m_id = String::new();
	io::stdin().read_line(&mut m_id);
	let m_id = m_id.trim().to_string();

	println!("Price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();
	// println!("{}", m_price);	

	println!("Quantity: ");
	let mut m_qty = String::new();
	io::stdin().read_line(&mut m_qty);
	let m_qty = m_qty.trim().parse::<i64>().unwrap();
	// println!("{}", m_qty);		

	let mut new_order = objects::Order::new(m_qty, m_price, '*');
	match_eng.update(&m_id, &new_order);
}
fn publish_snaphot(state: String, socket: &UdpSocket) {
	socket.send_to(&state.into_bytes(), MULTICAST_GROUP_ADDRESS);
}

fn main() {
	// create channel for ME thread to communicate with recovery thread
	let (tx, rx) = channel();

	let mut match_eng = matching_engine::MatchingEngine::new(&tx);
	
	// run MDS recovery thread
	// how often do we need to send snapshots?
	// do clients subscribe to different multicast group for recovery vs increment?
	let snapshot_thread = thread::spawn(move || {
		let sock = UdpSocket::bind("0.0.0.0:21003").unwrap();
		loop {
			let msg = rx.recv();
			match msg {
				Ok(v) => publish_snaphot(v.clone(), &sock),
				Err(r) => continue,
			}		
		}

	});

	loop {
		println!("{:*<1$}", "", 80);
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

		println!("OPTIONS:");
		println!("1. Insert new order");
		println!("2. Delete existing order");
		println!("3. Update existing order");
		println!("Enter 1 option (1 or 2 or 3)");

		let mut option_cmd = String::new();
		io::stdin().read_line(&mut option_cmd);

		match &option_cmd.trim() as &str {
			"1" => insert_new_order(&mut match_eng),
			"2" => delete_existing_order(&mut match_eng),
			"3" => update_existing_order(&mut match_eng),
			_	=> {
				println!("Invalid option!");
				continue;
			}
		}

		match_eng.print_status();
	}
}