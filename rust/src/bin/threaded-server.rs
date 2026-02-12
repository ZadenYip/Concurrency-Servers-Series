use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, os::windows::io::AsRawSocket, thread};

use utils;

enum ProcessingState {
    WaitForMsg,
    InMsg
}



fn main() {
    let args: Vec<String> = std::env::args().collect();    
    let mut port = 9090;

    if args.len() >= 2 {
        port = args[1].parse::<i32>().unwrap();
    }
    println!("Serving on port {}", port);
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    
    loop {
        let (connection, addr) = listener.accept().expect("ERROR on accept");
        utils::report_peer_connected(addr);

        thread::spawn(||{
            server_thread(connection);
        });        
    }
}

fn server_thread(stream: TcpStream) {
    let thread_id = thread::current().id();

    println!("Thrread {:?} created to handle connection with socket {}", thread_id, stream.as_raw_socket());
    serve_connection(stream);
    println!("Thread {:?} done", thread_id);
}

fn serve_connection(mut stream: std::net::TcpStream) -> () {
    if stream.write(b"*").expect("send") < 1 {
        panic!("send");
    }

    let mut state = ProcessingState::WaitForMsg;

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let len = stream.read(&mut buf).expect("recv");
        if len == 0 {
            break;
        }

        for i in 0..len {
            let c = buf[i];
            match state {
                ProcessingState::WaitForMsg => {
                    if c == b'^' {
                        state = ProcessingState::InMsg;
                    }
                },
                ProcessingState::InMsg => {
                    if c == b'$' {
                        state = ProcessingState::WaitForMsg;
                    } else {
                        buf[i] += 1;
                        if stream.write(&buf[i..i+1]).expect("send error") < 1 {
                            return;
                        }
                    }
                }
            }
        }
    }

}