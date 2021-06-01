use std::error::Error;
use std::str::FromStr;

use tokio::net::UdpSocket;
use std::net::Ipv6Addr;
use std::net::SocketAddrV6;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("UDP sender")
        .version("0.1.0")
        .about("UDP sender")
        .arg(Arg::new("destination")
            .short('d')
            .long("destination")
            .required(true)
            .about("IPv6 destination")
            .takes_value(true))
        .arg(Arg::new("source")
            .short('s')
            .long("source")
            .about("IPv6 source address")
            .takes_value(true))
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .required(true)
            .about("Destination port")
            .takes_value(true))
        .arg(Arg::new("bind-port")
            .short('b')
            .long("bind-port")
            .about("Source port")
            .takes_value(true))
        .arg(Arg::new("payload")
            .short('l')
            .long("payload")
            .required(true)
            .about("Payload to send")
            .takes_value(true))
        .get_matches()
        ;

    let dport = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let sport = match matches.value_of("bind-port") {
        Some(p) => { p.parse::<u16>().unwrap() }
        _ => { 0 as u16 }
    };
    let dest = get_address(matches.value_of("destination").unwrap());
    let src = get_address(matches.value_of("source").unwrap());
    let payload = matches.value_of("payload").unwrap();

    let socket = if sport != 0 {
        SocketAddrV6::new(src, sport, 0, 0)
    } else {
        SocketAddrV6::new(get_address("::"), 0, 0, 0)
    };
    let socket = UdpSocket::bind(&socket).await?;
    let dest_addr = SocketAddrV6::new(dest, dport, 0, 0);
    socket.connect(dest_addr).await?;

    println!("Sending \"{}\" to [{}]:{}", payload, dest, dport);
    socket.send(payload.as_bytes()).await?;

    Ok(())
}

fn get_address(addr: &str) -> Ipv6Addr {
    match Ipv6Addr::from_str(addr) {
        Ok(a) => { a }
        Err(e) => { panic!("{}", e) }
    }
}
