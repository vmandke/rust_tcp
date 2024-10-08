use std::net::Ipv4Addr;
use std::{collections::HashMap, fmt::Error};
extern crate tun_tap;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    scr: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}


fn main() -> Result<(), Error> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
    println!("Will wait for something");
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("Failed to create tun");
    let mut buf  = vec![0; 1504];
    
    loop {
        let nbytes = nic.recv(&mut buf).unwrap();
        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_frame_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_frame_proto != 0x0800 {
            // Non ipv4 protocol
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                // println!("read {} bytes, flags :: {:x} proto :: {:x}  data :: {:?}", nbytes - 4, flags, proto, p);
                let src: std::net::Ipv4Addr = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                let proto = ip_header.protocol();
                let ip_header_sz = ip_header.slice().len();

                if proto != etherparse::IpNumber::TCP {
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_header_sz..nbytes]) {
                    Ok(tcp_header)   => {
                        let datai = 4 + ip_header_sz + tcp_header.slice().len();
                        connections.entry(Quad{
                            scr: (src, tcp_header.source_port()),
                            dst: (dst, tcp_header.destination_port())
                        }).or_default().on_packet(&mut nic, ip_header, tcp_header, &buf[datai..nbytes]).unwrap();                        
                    },
                    Err(e) => {
                        println!("Error in IPV4 header {:?}", e);
                    }
                }

            },
            Err(e) => {
                println!("Error in IPV4 header {:?}", e);
            }
        }
        
    }
}
