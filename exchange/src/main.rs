/**
	MAIN

	This is a main program that runs the exchange. This program contains 2 threads:
		- 1 thread to multicast recovery feed
		- 1 (main) thread to allow users to perform operations on the exchange (INSERT, DELETE, UPDATE)
*/

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::{io, thread, time};
use std::sync::mpsc::channel;
use std::net::UdpSocket;
use std::collections::HashMap;
use objects::Order;
use std::time::{Duration, SystemTime};
mod objects;
mod matching_engine;

// IP address of the computer running main.rs (MUST specify the PORT)
const SERVER_ADDRESS: &str ="0.0.0.0:21001";
// IP address of the multicast group for Recovery Feed (MUST specify the PORT)
const RECOVERY_MULTICAST_GROUP_ADDRESS: &str = "239.255.255.255:21003";
// Standard period for a Recovery Feed to be sent (e.g, after (at least) 5 seconds since the last Recovery Feed, a new feed will be sent)
const RECOVERY_PERIOD: u64 = 5;

/**
	This function asks users to enter neccessary information in an insertion request:
		- Side of the order ('1' = BUY, '2' = SELL)
		- Price of the order 
		- Quantity of the order

	* NOTE: User inputs are assumed to be in correct format/types.
*/
fn insert_new_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to enter new order
	println!("Insert new order: ");
	// Process user input for side of the order
	println!("Side: (1 = buy, 2 = sell)");
	let mut m_side = String::new();
	io::stdin().read_line(&mut m_side);
	let m_side: char = m_side.chars().nth(0).unwrap();
	// Process user input for price of the order
	println!("Price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();
	// Process user input for quantity of the order
	println!("Quantity: ");
	let mut m_qty = String::new();
	io::stdin().read_line(&mut m_qty);
	let m_qty = m_qty.trim().parse::<i64>().unwrap();
	// Call INSERT function in Matching Engine 
	match_eng.insert(&objects::Order::new(m_qty, m_price, m_side));
}

/**
	This function asks users to enter neccessary information in a deletion request:
		- ID of the order to be deleted

	* NOTE: User inputs are assumed to be in correct format/types.
*/
fn delete_existing_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Ask user to input order ID
	println!("Enter order ID:");
	// Process user input for ID of the order
	let mut m_id = String::new();
	io::stdin().read_line(&mut m_id);
	let m_id = m_id.trim().to_string();
	// Call DELETE function in Matching Engine
	match_eng.delete(&m_id);
}

/**
	This function asks users to enter neccessary information in an update request:
		- ID of the order to be updated
		- New price for the order
		- New quantity for the order

	* NOTE: User inputs are assumed to be in correct format/types.
*/
fn update_existing_order(match_eng: &mut matching_engine::MatchingEngine) {
	// Process user input for ID of the order
	println!("Enter order ID:");
	let mut m_id = String::new();
	io::stdin().read_line(&mut m_id);
	let m_id = m_id.trim().to_string();
	// Process user input for price of the order
	println!("Price: ");
	let mut m_price = String::new();
	io::stdin().read_line(&mut m_price);
	let m_price = m_price.trim().parse::<i64>().unwrap();
	// Process user input for quantity of the order
	println!("Quantity: ");
	let mut m_qty = String::new();
	io::stdin().read_line(&mut m_qty);
	let m_qty = m_qty.trim().parse::<i64>().unwrap();
	// Create a new order with quantity and price entered by user, 
	// "DUMMY" value for side (which will be determined later by Matching Engine using order ID)
	let mut new_order = objects::Order::new(m_qty, m_price, '*');
	// Call UPDATE function in Matching Engine
	match_eng.update(&m_id, &new_order);
}

/**
	This function multicasts a recovery feed to RECOVERY_MULTICAST_GROUP_ADDRESS

	@params
		state: the state to be published
		socket: the socket for multicasting
*/
fn publish_snaphot(state: String, socket: &UdpSocket) {
	// Recovery feed must be converted to bytes for multicasting
	socket.send_to(&state.into_bytes(), RECOVERY_MULTICAST_GROUP_ADDRESS);
}

fn main() {
	// Create channel for Matching Engine to communicate with Recovery Thread
	// to receive Incremental Feed from Matching Engine to build up the latest state of Market Data
	let (tx, rx) = channel();

	// Start the Matching Engine
	let mut match_eng = matching_engine::MatchingEngine::new(&tx);
	
	/**
		Create and run a Recovery Thread
		This thread receives Incremental Feed from Matching Engine to build up the latest state
		of the Market Data and multicasts Recovery Feed (in a constant period of time)
	*/
	let snapshot_thread = thread::spawn(move || {
		// Create UDP socket for mulcasting
		let sock = UdpSocket::bind(SERVER_ADDRESS).unwrap();
		// Create hashmaps (for SELL and BUY) to maintain current state of the Market Data
		// (key = price, value = list of all orders at a specific price)
		let mut sells_by_price: HashMap<i64, Vec<Order>> = HashMap::new();
    	let mut buys_by_price: HashMap<i64, Vec<Order>> = HashMap::new();
		// Combine ALL orders in one state vector to represent current Market Data by price
		// (State vector sorted by increasing price)
		let mut cur_state: Vec<Vec<Order>> = Vec::new();
		// last_msg_index: Index of the last Incremental Feed received from Matching Engine (to be sent in Recovery Feed) 
		let mut last_msg_index: i64 = 0;
		// timer: Keep track of the time passed since the last Recovery Feed was multicasted
		// (to ensure that after a constant period of time, a new Recovery Feed will be multicasted)
		let mut timer = SystemTime::now();

		/**
			Receive Incremental Feed from Matching Engine, build up the latest state and multicast
		*/
		loop {
			/**
				MULTICAST (RECOVERY FEED)
			*/
			// Calculate the time from the last Recovery Feed till current time
			let cur_time = SystemTime::now();
			if cur_time.duration_since(timer).unwrap() >= Duration::new(RECOVERY_PERIOD, 0) {
				// If the period of time is more than the Standard RECOVERY_PERIOD set, create a new Recovery Feed
				let rec_feed = objects::RecoveryFeed::new(last_msg_index, cur_state.clone());
				// Multicast the Recovery Feed (latest state of Market Data)
				publish_snaphot(serde_json::to_string(&rec_feed).unwrap(), &sock);
				// Reset the timer (mark current time as when the last Recovery Feed was sent)
				timer = SystemTime::now();
			}

			/**
				RECEIVE INCREMENTAL FEED
				Wait to receive Incremental Feed from Matching Engine
			*/
			let msg = rx.try_recv();
			match msg {
				Ok(v) => {
					/**
						BUILD UP THE STATE
						When receving Incremental Feed, build up the latest state
					*/
					// Convert JSON object received from Matching Engine to Incremental Message object
					let val: objects::IncrementalMessage = serde_json::from_str(v.as_str()).unwrap();
					
					// Update state of Market Data
					// * NOTE: After an operation, only a list of orders at ONE price is modified.
					
					// After the operation, list of orders at the price affected is non-empty
					if (val.get_orders().len() > 0) {
						// Determine list of orders from which side (BUY or SELL) was modified
						let side = val.get_orders()[0].get_side();
						// Overwrite list of orders at that price with updated list of orders received from Incremental Feed
						if side == '1' {
							// BUY side
							buys_by_price.insert(val.get_price(), val.get_orders());
						} else if side == '2' {
							// SELL side
							sells_by_price.insert(val.get_price(), val.get_orders());
						}
					} else {
						// After the operation, list of orders at the price affected is empty
						// Then remove list of orders at that price from BUY and SELL sides (if any)
						buys_by_price.remove(&val.get_price());
						sells_by_price.remove(&val.get_price());
					}
					// Update index of the last Incremental Feed received from Matching Engine
					last_msg_index = val.get_num();

					// Build up the state of ALL orders with the prices increasing
					let mut state: Vec<Vec<Order>> = Vec::new();
					let mut state_sells: Vec<Vec<Order>> = Vec::new();
					for vector in sells_by_price.values() {
						state_sells.push(vector.clone());
					}
					state_sells.sort();
					state_sells.reverse();
					for vector in buys_by_price.values() {
						state.push(vector.clone());
					}
					state.sort();
					state.append(&mut state_sells);
					// Update latest state of Market Data with the state built
					cur_state = state;
				}
				Err(r) => {},
			}
		}
	});

	/** 
		User interface for users to send request (INSERT, DELETE, UPDATE)
		and enter relevant information needed for the request

		* NOTE: Open BIG/FULL SCREEN window for better display
	*/
	loop {
		println!("{:*<1$}", "", 80);
		// Ask user when to stop
		println!("Continue? (y/n) ");
		let mut continue_cmd = String::new();
		io::stdin().read_line(&mut continue_cmd);
		// The program will stop when user enters "n"
		match &continue_cmd.trim() as &str {
			"y" => { },
			"n" => break,
			_ 	=> {
				println!("Enter y or n!");
				continue;
			}
		}
		// Display options available
		println!("OPTIONS:");
		println!("1. Insert new order");
		println!("2. Delete existing order");
		println!("3. Update existing order");
		println!("Enter 1 option (1 or 2 or 3)");
		// Receive option entered by user
		let mut option_cmd = String::new();
		io::stdin().read_line(&mut option_cmd);
		// Call appropriate function with the option entered 
		match &option_cmd.trim() as &str {
			"1" => insert_new_order(&mut match_eng),
			"2" => delete_existing_order(&mut match_eng),
			"3" => update_existing_order(&mut match_eng),
			_	=> {
				println!("Invalid option!");
				continue;
			}
		}
		// Print out market status after every operation (for DEBUGGING)
		match_eng.print_status();
	}
}