use clap::Parser;
use std::net::SocketAddr;
use std::str::FromStr;
use std::{net::TcpStream, time::Duration};
#[derive(Parser)]
struct Cli{
    #[arg(short, long )]
    addr: String,

    #[arg(short, long )]
    single: Option<u16>,
    
    #[arg(short, long )]
    range: Option<String>,

    #[arg(short, long )]
    list: Option<String>,
}
fn main(){
    let args = Cli::parse();

    if let Some(single) = args.single{
        scan_ports(&args.addr, vec![single]);
    }
    if let Some(range) = args.range{
        let mut split = range.split('-');
        let start: u16 = match split.next(){
            Some(s) => s.parse().unwrap(),
            None => panic!("Valor do range não é válido"),
        };
        let mut list = vec![];
        let end: u16 = match split.next(){
            Some(s) => s.parse().unwrap(),
            None => panic!("Valor do range não é válido"),
        };
        (start..=end).into_iter().for_each(| x | list.push(x));
        scan_ports(&args.addr, list);
    }
    if let Some(ports) = args.list{
        let mut list = vec![];
        let mut split = ports.split(",");
        loop{
            match split.next() {
                Some(v) =>  {
                    list.push(v.parse().unwrap());
                }
                None => {break;}
            }
        }
        scan_ports(&args.addr, list);
    }
}
fn scan_ports(addr: &String, list: Vec<u16>){
    let mut handles = vec![];
    for port in list{
        let host = format!("{}:{}", addr, port );
        let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();
        let handle = std::thread::spawn(move ||{
            match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
                Ok(_) =>println!("A porta {} Esta aberta", port),
                Err(_) =>println!("A porta {} Esta fechada", port)
            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}