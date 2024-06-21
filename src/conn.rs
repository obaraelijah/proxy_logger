use crate::get_formatter_by_kind;
use crate::Arguments;
use bytes::BytesMut;
use tokio::io::{self, AsyncReadExt};
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
    let (mut source_stream_read_half, mut source_stream_write_half) = io::split(
        source_stream,
        get_formatter_by_kind(arguments.formatting, arguments.separator.as_str())
    );
    let destination_stream = tokio_net::TcpStream::connect(arguments.remote_addr)
        .await
        .expect("Failed to connect to destination address");
    let (mut destination_stream_read_half, mut destination_stream_write_half) =
        io::split(destination_stream);

    let destination_stream_handle = tokio::spawn(async move {
        let mut buffer = BytesMut::with_capacity(2048);
        'destination_stream_handle: loop {
            let Ok(read_length) = destination_stream_read_half.read_buf(&mut buffer).await else {
                break 'destination_stream_handle;
            };
            if read_length == 0 {
                continue 'destination_stream_handle;
            }
            buffer.clear();
        }
    });
}
