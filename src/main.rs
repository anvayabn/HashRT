use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::ops::BitAnd;


/* HashMap
   <Network_address, Route>
   the matching Ip is combination of
   Bitwise AND of Ip and Subnet mask
*/
struct HashRoutingTable {
    hrt: HashMap<Ipv4Addr, String>,
}

impl HashRoutingTable {

    //Get new hash route table instance
    pub fn new() -> HashRoutingTable {

        HashRoutingTable{
            hrt: HashMap::new(),
        }

    }

    fn network_address(ipaddr_oct: &[u8;4], mask_oct: &[u8;4])  -> Ipv4Addr {
        let matching_ip: [u8; 4] = [
            ipaddr_oct[0] & mask_oct[0],
            ipaddr_oct[1] & mask_oct[1],
            ipaddr_oct[2] & mask_oct[2],
            ipaddr_oct[3] & mask_oct[3],
        ];

        let matching_ip_addr = Ipv4Addr::new(matching_ip[0], matching_ip[1], matching_ip[2], matching_ip[3]);

        return matching_ip_addr;

    }


    //insert to hash table
    pub fn insert(&mut self, ip:Ipv4Addr, mask: Ipv4Addr, route: String ) -> Result<String, &'static str>{

        //convert ip to octets
        let ipaddr_oct =  ip.octets();
        let mask_oct = mask.octets();

        let net_addr = Self::network_address(&ipaddr_oct, &mask_oct);

        //insert to hash map if doesn't already exists
        if self.hrt.contains_key(&net_addr) {
            return Err("Already exists");
        }else {
            self.hrt.insert(net_addr, route);
        }

        Ok(String::from("Route has been added to the hashtable"))
    }

    //search the hash table and return route
    pub fn search(&self, ip:Ipv4Addr, mask: Ipv4Addr) -> Result<String, &'static str> {

        //convert ip to octets
        let ipaddr_oct =  ip.octets();
        let mask_oct = mask.octets();

        let net_addr = Self::network_address(&ipaddr_oct, &mask_oct);

        if let Some(route) = self.hrt.get(&net_addr) {
            Ok(route.clone())
        }else {
            Err("No matching route")
        }


    }
}

fn main() {

    let mut routing_table = HashRoutingTable::new();

    let result = routing_table.insert(Ipv4Addr::new(192,168,1,0),
                                      Ipv4Addr::new(255,255,255,0),
                                      String::from("eth0"));
    match result {
        Ok(route) => println!("{}", route),
        Err(e) => println!("Error: {}", e),

    }

    let result = routing_table.insert(Ipv4Addr::new(10,10,0,0),
                                      Ipv4Addr::new(255,255,0,0),
                                      String::from("eth1"));
    match result {
        Ok(route) => println!("{}", route),
        Err(e) => println!("Error: {}", e),

    }

    let result = routing_table.insert(Ipv4Addr::new(192,168,0,0),
                                      Ipv4Addr::new(255,255,0,0),
                                      String::from("eth2"));
    match result {
        Ok(route) => println!("{}", route),
        Err(e) => println!("Error: {}", e),

    }

    //get route for destination ip
    let dst_ip = Ipv4Addr::new(192,168,1,16);
    let dst_mask =  Ipv4Addr::new(255,255,255,0);

    let result_route = routing_table.search(dst_ip, dst_mask);
    match result_route {
        Ok(route) => println!("DestinationIP :{} should be routed to {}", dst_ip, route),
        Err(e) => println!("Error: {}", e),
    }

    /*
    Output:
    DestinationIP :192.168.1.16 should be routed to eth0

     */

}
