use std::process::Command;

pub fn create_bridge() -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&["bridge", "create", "name", "cni0"])
        .output()?;

    if output.status.success() {
        println!("Bridge name set to cni0 successfully.");
    } else {
        eprintln!(
            "Failed to create bridge: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn add_iface_in_bridge(name: String) -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&["cni0", "addm", format!("{}b", name).as_str()])
        .output()?;

    if output.status.success() {
        println!("set interface to cni0 successfully.");
    } else {
        eprintln!(
            "Failed to set interface: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn delete_bridge() -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&["cni0", "destroy"])
        .output()?;

    if output.status.success() {
        println!("Delete Bridge successfully.");
    } else {
        eprintln!(
            "Failed to delete bridge: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn del_iface_in_bridge(name: String) -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&["cni0", "deletem", format!("{}b", name).as_str()])
        .output()?;

    if output.status.success() {
        println!("del interface to cni0 successfully.");
    } else {
        eprintln!(
            "Failed to del interface: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

