use std::{
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn init_tcp_sink(
    app_name: &str,
    remote_addr: impl ToSocketAddrs + Send + 'static,
) -> ProbiusFlusher {
    let (buffer_sender, mut buffer_receiver) = bab::buffer_queue();

    let headroom = 2; // headroom for u16 length prefix
    let buffer_size = 8192 - headroom;
    let buffer_pool = bab::HeapBufferPool::new(buffer_size, 16, 64);
    crate::init(headroom, buffer_pool.clone());

    // Build handshake payload
    let handshake = probius_mproto::SinkHandshakeGen {
        app_name,
        session_id_hi: fastrand::u64(..),
        session_id_lo: fastrand::u64(..),
    };
    let handshake_len = mproto::encoded_len(&handshake);
    let mut handshake_buf = vec![0u8; 2 + handshake_len];
    handshake_buf[..2].copy_from_slice(&(handshake_len as u16).to_le_bytes());
    mproto::encode_value(handshake, &mut handshake_buf[2..]);

    std::thread::spawn(move || {
        loop {
            println!("Retrying probius tcp");
            let Ok(mut stream) = TcpStream::connect(&remote_addr) else {
                std::thread::sleep(Duration::from_secs(1));
                continue;
            };
            println!("Reconnected probius tcp");

            // Perform handshake
            if let Err(_) = stream.write_all(&handshake_buf[..]) {
                todo!();
            }

            buffer_receiver = pollster::block_on(async move {
                loop {
                    for buffer in buffer_receiver.recv().await {
                        let buffer_len = bab::WriterFlushSender::get_complete_buffer_len(buffer) as usize;
                        let payload_len = buffer_len - 2;
                        let payload = unsafe { buffer.slice_mut(0..buffer_len) };
                        payload[..2].copy_from_slice(&(payload_len as u16).to_le_bytes());

                        if let Err(_e) = stream.write_all(payload) {
                            println!("Probius connection broke");
                            // TODO this will lose data - the current buffer and the remaining
                            // buffers in the buffer_receiver.recv() iterator.
                            unsafe { buffer.release(); }
                            return buffer_receiver;
                        }

                        unsafe { buffer.release(); }
                    }

                    std::thread::sleep(Duration::from_millis(5));
                }
            });

            std::thread::sleep(Duration::from_secs(1));
        }
    });

    ProbiusFlusher { buffer_sender }
}

#[derive(Clone)]
pub struct ProbiusFlusher {
    buffer_sender: bab::BufferQueueSender,
}

impl ProbiusFlusher {
    pub fn flush(&self) {
        for buffer in crate::flush() {
            self.buffer_sender.push(buffer);
        }
        self.buffer_sender.flush();
    }
}

