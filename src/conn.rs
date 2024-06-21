use crate::Arguments;
use tokio::net as tokio_net;

pub async fn initialize_tcp_listener(arguments: Arguments) {
    let listener = tokio_net::TcpListener::bind(arguments.bind_listener_addr)
        .await
        .expect("Failed to bind tcp listener");

    log::info!("Listener binded, waiting for incoming connections...");

    loop {
        let cloned_args = arguments.clone();
        let accept = listener.accept().await;
        match accept {
            Ok((stream, addr)) => {
                log::info!("Incoming connection from {addr}");
                tokio::spawn(incoming_connection_handle(cloned_args, stream));
            }
            Err(e) => {
                log::error!("Failed to accept incoming connection due to {e}");
            }
        }
    }
}

async fn incoming_connection_handle(arguments: Arguments, source_stream: tokio_net::TcpStream) {
    let destination_stream = tokio_net::TcpStream::connect(arguments.remote_addr)
        .await
        .expect("Failed to connect to destination address");
}
