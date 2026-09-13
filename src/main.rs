use std::net::TcpStream;
#[derive(Parser)]
struct Cli{
    #[arg(short, long )]
    address: String,
    
    #[arg(short, long )]
    single: Option<usi>,
    
    #[arg(short, long )]
    range: Option<String>,

    #[arg(short, long )]
    list: Option<String>,
    
}
fn main(){
    let args = Cli::parse();

    if let Some(single) = args.single{
        scan_ports(&args.address, vec![single]);
    }
    if let Some(range) = args.range{
        let mut split = range.split('-');
        let start: u16 = match split.next(){
            Some(s) => s.parse().unwrap(),
            None => panic!("Valor do range não é válido"),
        };
        let mut list = vec![];
        (start..=end).into_iter().for_each(| x | list.push(x));
        scan_ports(addr, list);
    }
}
fn scan_ports(addr: &String, list: Vec<u16>){

}