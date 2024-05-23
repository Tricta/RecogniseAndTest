use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn dirSearch(protocol: &str, ip: &str, port: &str){
    let dir = Command::new("ffuf")
        .args([
            "-w",
            "/usr/share/seclists/Discovery/Web-Content/directory-list-lowercase-2.3-medium.txt:FUZZ",
            "-u",
            &format!("{}://{}.{}/FUZZ", protocol, ip, port),
        ])
        .stdout(Stdio::piped())
        .spawn();

    match dir {
        Ok(mut child) => {
            println!("Child process spawned");

            if let Some(stdout) = child.stdout.take() {
                println!("Captured stdout");

                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("{}", line);
                    } else {
                        eprintln!("Error reading stdout line");
                    }
                }
            } else {
                eprintln!("Failed to capture standard output");
            }

            match child.wait() {
                Ok(status) => println!("Child process exited with: {}", status),
                Err(e) => eprintln!("Error waiting for child process: {}", e),
            }
        }
        Err(e) => eprintln!("Error spawning dir process: {}", e),
    }
}

pub fn webEnumeration(ip: &str, scan: &str){
    let ports: Vec<&str> = scan
        .lines()
        .filter(|line| line.contains("http") || line.contains("https"))
        .filter_map(|line| {
            let index = line.find("/tcp")?;
            Some(line[..index].trim())
        })
        .collect();

    if !ports.is_empty() {
        for port in ports {
            let protocol = if port.contains("https") { "https" } else { "http" };
            let url = format!("{}://{}:{}", protocol, ip, port);
            println!("----------------------Web Scan-------------------------");
            println!("{}", url);
            dirSearch(protocol, ip, port);
        }
    }
}

