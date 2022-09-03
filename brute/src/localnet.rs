use local_ip_address::local_ip;
use std::{net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr}, time::Duration};

pub struct Ipdata {
    ip_sock: SocketAddr,
    ip_string: String,
}

#[allow(unused)]
pub fn localipstem() -> Option<Vec<String>> { //return a list of online hosts.
    let mut online_hosts: Vec<String> = host_online();

    if online_hosts.len() == 0 {
        return None;
    } else {
        return Some(online_hosts);
    }
}

pub fn get_ip_stem(x: u8) -> Ipdata { //get the Socket adress stem. eg: 192.168.1.123 -> 192.168.1.{}:22
    let my_local_ip = local_ip().unwrap().to_string();

    let stem_split: Vec<&str> = my_local_ip.split(".").collect();
    let mut stem_spilt_int: Vec<u8> = Vec::new();

    for place in stem_split {
        stem_spilt_int.push(place.trim().parse().expect("Failed to parse!"));
    }



    #[allow(overflowing_literals)]
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(
        stem_spilt_int[0],
        stem_spilt_int[1],
        stem_spilt_int[2],
        x,)), 22);

    let stem = Ipdata {
        ip_sock: socket,
        ip_string: format!("{}.{}.{}.{}:22",
            stem_spilt_int[0],
            stem_spilt_int[1],
            stem_spilt_int[2],
            x,
        )
    };

    return stem;
}

pub fn host_online() -> Vec<String> { //get a list of online hosts and return it to localipstem().

    let mut online_hosts: Vec<String> = Vec::new();

    for i in 0..255 {

        let test_ip = get_ip_stem(i);

        if let Ok(_stream) = TcpStream::connect_timeout(&test_ip.ip_sock, Duration::new(0, 100000000)){
            println!("\x1b[93mConnected {}!\x1b[0m", &test_ip.ip_sock);
            online_hosts.push(test_ip.ip_string);
        } else {
            println!("Couldn't connect to {}", test_ip.ip_sock);
        }
    }

    return online_hosts;
}

/*

TODO

make tests in /brute/tests/tests.rs

*/
