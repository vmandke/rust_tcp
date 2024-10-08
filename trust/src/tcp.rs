use std::{io, vec};

pub enum Connection {
    Closed = 0,
    Listen = 1,
}

impl Default for Connection {
    fn default() -> Self {
        Connection::Listen
    }
}

// Transmission Control Block
struct TCB {
    
}

impl Connection {
    pub fn on_packet(
        &mut self, 
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice, 
        tcph: etherparse::TcpHeaderSlice, 
        data: &[u8]
    ) -> io::Result<usize>{
        let mut buf  = vec![0u8; 1504];
        
        match *self {
            Connection::Closed => {
                return Ok(0);
            },
            Connection::Listen => {
                if !tcph.syn() {
                    return Ok(0);
                }
                // send syn,ack to establis a connection
                let mut syn_ack = etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 0);
                syn_ack.ack = true;
                syn_ack.syn = true;
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len_u16(), 
                    4, 
                    etherparse::IpNumber::TCP, 
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ], 
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ], 
                );
                let written = {
                    let mut unwritten = &mut buf[..];
                    let _ = ip.unwrap().write(&mut unwritten);
                    let _ = syn_ack.write(&mut unwritten);
                    unwritten.len()
                };
                return nic.send(&buf[..written]);
            }
        }
    }
}