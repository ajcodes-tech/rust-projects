use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

struct Arguments {
    ipaddr:  IpAddr,
    threads: u16,
}

const MAX: u16 = 65535;
const ERR_MSG: &str = "Too Many args";
const HELP_MSG: &str = "Usage: -j to select how many threads you want
\r\n       -h or -help to show this help message.";


impl Arguments {
    
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("less args than expected");
        }

        if args.len() > 4 {
            return Err(ERR_MSG);
        }

        let flag = args[1].clone();

        
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Arguments { ipaddr, threads: 4});
        }

        if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
            println!("{}", HELP_MSG);
            return Err("help");
        }
        
        if flag.contains("-h") || flag.contains("--help") {
            return Err(ERR_MSG);
        }
        
        if flag.contains("-j") {
        
            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(s) => s,
                Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
            };

            let threads = match args[2].parse::<u16>() {
                Ok(s) => s,
                Err(_) => return Err("failed to parse thread number")
            };
            return Ok(Arguments { ipaddr, threads });
        }
        return Err("Invalid syntax");
    }

}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        }
        if (MAX - port) <=  num_threads { 
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let args = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0)
            }
            eprintln!("{} problem parsing args: {}", program, err);
            process::exit(0);
        }
    );

    let num_threads = args.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, args.ipaddr.clone(), num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p)
    }
    println!();
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}