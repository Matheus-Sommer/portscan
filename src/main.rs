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

    }
}
fn scan_ports(addr: &string, list: Vec<u16>){

}