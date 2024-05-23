use std::process::Command;

pub fn scanTop1000ports(ip: &str){
    let nmap = Command::new("nmap").args(["-Pn", "-sCV", "-T4", &ip]).output().expect("[ERROR] scanTop100Ports failed!");
    println!("----------------------Scan Top 1000 Ports-------------------------");
    println!("{}", String::from_utf8_lossy(&nmap.stdout));
}

pub fn scanAllPorts(ip: &str) -> String{
    let nmap = Command::new("nmap").args(["-Pn", "-p-", "-T4", &ip]).output().expect("[ERROR] scanAllPorts failed!");
    let mut nmapScan: Vec<String> = String::from_utf8_lossy(&nmap.stdout).lines().filter(|line| line.contains("/tcp")).map(|line| line.to_string()).collect();

    println!("----------------------Scan All Ports-------------------------");
    for line in nmapScan{
        println!("{}", line);
    }

    nmapScan = String::from_utf8_lossy(&nmap.stdout).lines().filter_map(|line| {
        let index = line.find("/tcp")?;
        Some(line[..index].trim().to_string())
    }).collect();

    return nmapScan.join(",");
}

pub fn scanTopPortsUDP(ip: &str){
    let nmap = Command::new("nmap").args(["-Pn", "-sU", "--top-ports", "100", "-T4", &ip]).output().expect("[ERROR] scanUDP failed!");
    println!("----------------------Scan UDP-------------------------");
    println!("{}", String::from_utf8_lossy(&nmap.stdout));
}

pub fn scanAllPortsUDP(ip: &str){
    let nmap = Command::new("nmap").args(["-Pn", "-sU", "-p-", "-T4", &ip]).output().expect("[ERROR] scanAllUDPPorts failed!");
    println!("----------------------Scan All UDP Ports-------------------------");
    println!("{}", String::from_utf8_lossy(&nmap.stdout));
}

pub fn scanService(ip: &str, ports: &str) -> String{
    let serviceScan = Command::new("nmap").args(["-Pn", "-sC", "-sV", "-p", ports, "-T4", ip]).output().expect("[ERROR] scanService Failed");

    println!("----------------------Service Scan-------------------------");
    println!("{}",String::from_utf8_lossy(&serviceScan.stdout));

    return String::from_utf8_lossy(&serviceScan.stdout).to_string();
}
