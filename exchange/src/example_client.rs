use std::net::UdpSocket;
use std::net::Ipv6Addr;
use std::str;


fn main() {
    /** NOTE: port number must match
        RECOVERY_MULTICAST_GROUP_ADDRESS/INCREMENTAL_FEED_MULTICAST_GROUP_ADDRESS port number
    **/
    let socket = UdpSocket::bind("0.0.0.0:21003").unwrap();

    // specify multicast address for incremental feed or recovery feed
    socket.join_multicast_v4(&"239.255.255.255".parse().unwrap(), &"0.0.0.0".parse().unwrap());


    while(true) {
        let mut recv_buffer = vec![0u8; 1000];
        let (size, addr) = socket.recv_from(&mut recv_buffer).unwrap();
	    let content = str::from_utf8(&recv_buffer[..size]).unwrap().trim();
        println!("From {:?}: {:?}", addr, content);
    }
}
