use std::result;

use tokio::net::TcpStream;
use hyper_util::rt::TokioIo;

type Result<T> = result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn do_request<U>(request: hyper::Request<U>) -> Result<()>
where U:
    hyper::body::Body + Send,
{
    let addr = request.uri().authority().unwrap().as_str();
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let mut res = sender.send_request(request).await?;

    println!("Response status: {}", res.status());

    Ok(())
}
