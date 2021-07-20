use std::env;
use std::net::{ToSocketAddrs, TcpStream, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::io::{Read, Write, Error};

fn main() -> Result<(), Error>{

    let args: Vec<String> = env::args().collect();
    let hn = args[1].clone();

    let host_tpl = (hn.clone(), 0);
    let host_ips = host_tpl.to_socket_addrs()?;

    let (ip_tx_con, ip_rx_con): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();

    let conn : JoinHandle<std::result::Result<String, Error>> = thread::spawn(move|| {
        let mut stream = ip_rx_con.recv().unwrap();
        let request = "GET / HTTP/1.1\r\nHost: ".to_owned() + &hn + &"\r\nConnection: close\r\n\r\n".to_owned();
        let _ = stream.write(request.as_bytes());
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("\nConnected to: {}", stream.peer_addr()?); 
        println!("\n{}", buf);
        Ok(buf)
    });

    let mut ip_chans =  Vec::<Sender<SocketAddr>>::new();

    for _i in 0..host_ips.len() {
        let (main_tx_ip, main_rx_ip) = mpsc::channel();
        ip_chans.push(main_tx_ip);
        let tx = ip_tx_con.clone();
        thread::spawn(move|| {
            let addr = main_rx_ip.recv().unwrap();
            let ip = addr.ip();
            if let Ok(stream) = TcpStream::connect((ip, 80)) {
                let _ = tx.send(stream);
            }
        });
    }

    for pair in host_ips.zip(ip_chans) {
        let (ip, tx) = pair;
        let _ = tx.send(ip);
    }

    conn.join().unwrap()?;

    Ok(())

}
    
