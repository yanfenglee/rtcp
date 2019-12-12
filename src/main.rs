use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::spawn;
use futures::future::*;
use std::env;
mod util;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: rtcp ip1:1234 ip2:5678");
    }

    let fromaddr = args[1].parse::<String>().unwrap();
    let toaddr = args[2].parse::<String>().unwrap();

    let mut listener = TcpListener::bind(fromaddr).await?;

    loop {
        let (mut inbound, _) = listener.accept().await?;

        let toaddr = toaddr.clone();

        spawn(async move {
            let mut outbound = TcpStream::connect(toaddr).await.unwrap();

            let (mut ri, mut wi) = inbound.split();
            let (mut ro, mut wo) = outbound.split();
            
            let client_to_server = util::copy(&mut ri, &mut wo);
            let server_to_client = util::copy(&mut ro, &mut wi);

            match join(client_to_server,server_to_client).await {
                _ => ()
            }

        });
        
    }
}