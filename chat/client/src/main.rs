use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::str;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to init non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = str::from_utf8(&msg).unwrap();
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("closing connection with server");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to scoket failed");
                println!("message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        sleep();
    });
    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("bye bye!");
}
