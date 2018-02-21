# GHEX: Grasshopper Exchange Simulator


**Description:**

An exchange simulator that will be used for testing new features/algorithms, by allowing for the manipulation of specific conditions and set up of different scenarios under which to perform trades.

Current iteration (version 1): Users send requests to matching engine via a basic command-line interface, which allows a user to insert, update, or delete an order. Users specify buy/sell side, price, and quantity, and the matching engine processes the request, performs trades, and updates the orderbook. Basic MDS delivery is also implemented. Users can subscribe to the incremental feed and the recovery (snapshot) feed to receive real-time updates as events occur in the matching engine.


**Setup and Running:**

cd into the exchange/ directory and run the matching engine using *cargo run*

Some setup is required if a user/client wishes to subscribe to the MDS feed(s):

One machine:

An example client program is located at src/example_client.rs. Open 2 terminals, start up the client first, then run the matching engine. Be sure the SERVER_ADDRESS the matching engine is bound to is 'Wireless LAN adapter Wi-Fi' found under ipconfig. Examples are provided below.

Two machines:

Currently, an ethernet cord is needed for communication between 2 machines. After connecting the machines, start up the client on one machine, then run the matching engine on the other. 

If the client isn't receiving data, double check the following:

- Type *netsh interface ip show joins* to double check that the multicast address to which the client is listening is listed under the Ethernet interface. If it is listed elsewhere, then those interfaces have higher priority than Ethernet, so be sure to disable those before running the client.

- Check that the SERVER_ADDRESS the matching engine is bound to is the 'Ethernet adapter Ethernet' address found under ipconfig.

- In example_client.rs, check that the port bound to by the client matches the port of the feed address


**Examples:**

- Inserting an order:

![Alt text](/images/insert.png?raw=true)

Buy orders are listed in the format '[qty](ID: #)' on the left of the price column, sell orders are listed on the right

![Alt text](/images/insert_inc.PNG?raw=true)

If a client is subscribed to the incremental feed, it should receive an update once the order gets inserted (currently, the feed just broadcasts affected orders in the orderbook, which in this case would be the order just inserted).

![Alt text](/images/insert_snap.PNG?raw=true)

If a client is subscribed to the snapshot feed, the client should receive updates at an interval specified by RECOVERY_PERIOD. When the market is empty, the feed should broadcast empty state. Once an order gets inserted, the state will get updated, and the next snapshot sent should contain the new order.


- Order priority

![Alt text](/images/priority.PNG?raw=true)

Orders at the same price are listed in order based on timestamp: from right to left for buys, left to right for sells (i.e. orders closer to the center column have higher priority)


- Matching

![Alt text](/images/matching.PNG?raw=true)

When an inserted order triggers a match, it will be reflected in the output, as can be seen above. The higher priority buy order was matched first, then the remaining quantity of 1 was matched with quantity 1 from the remaining buy order.

**Testing:**

The test module, located in src/lib.rs contains case/unit tests. 

Be sure to run using the following command: 

*cargo test -- --test-threads=1*

**License:**

The code is licensed under the Apache License 2.0. Contributions are welcome.
Copyright (c) 2018 Grasshopper Asia Pte Ltd, unless otherwise noted.

