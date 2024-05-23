use std::process::{Command, Stdio};
use std::env;
use std::thread;

use users::os::unix::UserExt;

mod scanning;
mod web;

fn main() {
    if users::get_current_uid() != 0 {
        println!("Please run this program with sudo privileges!");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: ./recogniseandtest <IP>");
        std::process::exit(1);
    }

    let ip = args[1].clone();
    let ipTop1000Thread = ip.clone();

    let scanTop1000Thread = thread::spawn(move || {
        scanning::scanTop1000ports(&ipTop1000Thread);
    });

    let ipScanUDP = ip.clone();
    let scanUDP = thread::spawn(move || {
        scanning::scanTopPortsUDP(&ipScanUDP);
    });

    let ipScanAllUDP = ip.clone();
    let scanAllUDP = thread::spawn(move || {
        scanning::scanAllPortsUDP(&ipScanAllUDP);
    });

    let serviceScan = scanning::scanService(&ip, &scanning::scanAllPorts(&ip));

    let webEnum = web::webEnumeration(&ip, &serviceScan); 

    scanTop1000Thread.join().unwrap();
    scanUDP.join().unwrap();
    scanAllUDP.join().unwrap();
}
