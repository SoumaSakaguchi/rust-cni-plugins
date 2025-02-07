use std::process::Command;
use std::io;

/* インタフェースの作成 */
pub fn create_iface(name: String) -> io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&["epair", "create"])
        .output()?;
    
    if !output.status.success() {
        eprintln!(
            "Failed to create epair: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(());
    }
    
    let iface = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !iface.starts_with("epair") || !iface.ends_with("a") {
        eprintln!("Unexpected interface name: {}", iface);
        return Ok(());
    }
    
    let peer_iface = format!("{}b", &iface[..iface.len() - 1]);
    
    let iface_a = format!("{}a", name);
    let iface_b = format!("{}b", name);
    
    let rename_a = Command::new("ifconfig")
        .args(&[&iface, "name", &iface_a])
        .output()?;
    
    if !rename_a.status.success() {
        eprintln!(
            "Failed to rename {} to {}: {}",
            iface, iface_a, String::from_utf8_lossy(&rename_a.stderr)
        );
        return Ok(());
    }
    
    let rename_b = Command::new("ifconfig")
        .args(&[&peer_iface, "name", &iface_b])
        .output()?;
    
    if !rename_b.status.success() {
        eprintln!(
            "Failed to rename {} to {}: {}",
            peer_iface, iface_b, String::from_utf8_lossy(&rename_b.stderr)
        );
        return Ok(());
    }
    
    println!("Successfully created and renamed epair: {} and {}", iface_a, iface_b);
    Ok(())
}

/* jailへのインタフェースの接続 */
pub fn add_iface_in_jail(id: String ,name: String) -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&[format!("{}a", name).as_str(), "vnet", id.as_str()])
        .output()?;

    if output.status.success() {
        println!("Add interface in jail successfully.");
    } else {
        eprintln!(
            "Failed to add interface: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/* インタフェースの削除 */
pub fn delete_iface(name: String) -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&[format!("{}b", name).as_str(), "destroy"])
        .output()?;

    if output.status.success() {
        println!("delete interface successfully.");
    } else {
        eprintln!(
            "Failed to delete interface: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/* jailからのインタフェースの削除 */
pub fn del_iface_in_jail(id: String ,name: String) -> std::io::Result<()> {
    let output = Command::new("ifconfig")
        .args(&[format!("{}a", name).as_str(), "-vnet", id.as_str()])
        .output()?;

    if output.status.success() {
        println!("del interface in jail successfully.");
    } else {
        eprintln!(
            "Failed to del interface: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
