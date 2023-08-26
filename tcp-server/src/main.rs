use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use clap::Parser;
use std::error::Error;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(about)]
struct Args {
    #[arg(short, long, default_value="0.0.0.0:3000")]
    addr: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let addr = args.addr.to_string();
    
    let listener = TcpListener::bind(addr.parse::<SocketAddr>()?).await?;
    println!("Listening on: {}", args.addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                
                println!("Recebeu a mensagem: {:?}", &buf[0..n]);
                println!("Equivale a: {}", String::from_utf8((&buf[0..n]).to_vec()).unwrap());
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
