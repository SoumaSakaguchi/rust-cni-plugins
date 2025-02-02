use std::process::Command;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn add_addr_in_jail(id: String, name: String) -> std::io::Result<()> {
    let addr = read_conf()?;

    let output = Command::new("jexec")
        .args(&[id.as_str(), "ifconfig", format!("{}a", name).as_str(), "inet", &addr, "netmask", "255.255.255.0"])
        .output()?;

    if output.status.success() {
        println!("add address successfully.");
    } else {
        eprintln!(
            "Failed to add address: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn read_conf() -> io::Result<String> {
    let file = File::open("/tmp/cni-ip.conf")?;
    let reader = BufReader::new(file);
    
    if let Some(Ok(first_line)) = reader.lines().next() {
        Ok(first_line)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "File is empty or could not be read"))
    }
}

