use std::io::{ Read, Write, };
use std::env;
use std::net::{ ToSocketAddrs, TcpStream };
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    for hn in args[1..].iter() {

        let host = (hn.clone(), 0);
        let address_iter = host.to_socket_addrs()?;

        for socket in address_iter {        

            let ip = socket.ip();

            let ver = if ip.is_ipv4() { "IPv4" } else { "IPv6" };

            println!("{} {} {}", hn, ver, ip);
            if let Ok(mut stream) = TcpStream::connect((ip, 80)) {
                println!("Success! Connected to {}", ip); 
                let request = "GET / HTTP/1.1\r\nHost: ".to_owned() + hn + &"\r\nConnection: close\r\n\r\n".to_owned();
                println!("{}", request);
                stream.write(request.as_bytes())?;

                let mut buf = String::new();                
                let _response = stream.read_to_string(&mut buf)?;

                println!("{}", buf);
                
                break;

            } else {
                println!("Could not connect...");
            }

        }
    }
    Ok(())
}
