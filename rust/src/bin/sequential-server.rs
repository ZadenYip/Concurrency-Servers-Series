use std::{io::{Read, Write}, net::TcpListener};

use utils;

enum ProcessingState {
    WaitForMsg,
    InMsg
}

fn main() {
    let args: Vec<String> = std::env::args().collect();    
    let mut port = 9090;

    if args.len() >= 2 {
        port = args[1].parse().unwrap();
    }
    
    println!("Serving on port {}", port);
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    
    loop {
        let (connection, addr) = listener.accept().expect("ERROR on accept");
        utils::report_peer_connected(addr);
        serve_connection(connection);
        println!("peer done");
    }

}

fn serve_connection(mut stream: std::net::TcpStream) -> () {
    // 客户端在尝试连接并发送数据时，即使服务器尚未通过 accept() 接受连接，也会成功。
    // 因此，为了更好地模拟在服务一个客户端时阻塞其他客户端的情况，
    // 服务器会发送一个 "ack"（确认信号），客户端在继续之前会期望收到这个信号。
    // 以上是原代码注释翻译，考核的对应 * 处理见 python 文件。

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

