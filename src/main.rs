use std::env;

mod bridge;
mod iface;

fn main() {
    let (command, id, ns_id, if_name) = get_env();

    if id != ns_id {
        eprintln!("CNI_CONTAINERIDとCNI_NETNSは同じ値が入る");
    }

    match command.as_str() {
        "ADD" => add(ns_id, if_name),
        "DEL" => del(ns_id, if_name),
        _     => eprintln!("未実装"),
    }

}

fn get_env() -> (String, String, String, String) {
    let command = match env::var("CNI_COMMAND") {
        Ok(cmd) if cmd == "ADD" || cmd == "DEL" || cmd == "CHECK" || cmd == "VERSION" => {
            println!("CNI_COMMAND: {}", cmd);
            cmd
        }
        Ok(cmd) => {
            eprintln!("ADD, DEL, CHECK, VERSIONのいずれかを設定してください: {}", cmd);
            cmd
        }
        Err(e) => {
            eprintln!("CNI_COMMANDを設定してください: {}", e);
            String::new()
        }
    };

    let id = match env::var("CNI_CONTAINERID") {
        Ok(id) => {
            println!("CNI_CONTAINERID: {}", id);
            id
        }
        Err(e) => {
            eprintln!("CNI_CONTAINERIDを設定してください: {}", e);
            String::new()
        }
    };

    let ns_id = match env::var("CNI_NETNS") {
        Ok(ns) => {
            println!("CNI_NETNS: {}", ns);
            ns
        }
        Err(e) => {
            eprintln!("CNI_NETNSを設定してください: {}", e);
            String::new()
        }
    };

    let if_name = match env::var("CNI_IFNAME") {
        Ok(ifn) => {
            println!("CNI_IFNAME: {}", ifn);
            ifn
        }
        Err(e) => {
            eprintln!("CNI_IFNAMEを設定してください: {}", e);
            String::new()
        }
    };

    (command, id, ns_id, if_name)
}

fn add(id: String,if_name: String) {
    if let Err(e) = bridge::create_bridge() {
        eprintln!("ブリッジの作成に失敗: {}", e);
    }

    if let Err(e) = iface::create_iface(if_name.clone()) {
        eprintln!("インタフェースの作成に失敗: {}", e);
    }

    if let Err(e) = iface::add_iface_in_jail(id, if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }

    if let Err(e) = bridge::add_iface_in_bridge(if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }
}

fn del(id: String, if_name: String) {
    if let Err(e) = bridge::del_iface_in_bridge(if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }

    if let Err(e) = iface::del_iface_in_jail(id, if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }

    if let Err(e) = iface::delete_iface(if_name.clone()) {
        eprintln!("インタフェースの削除に失敗: {}", e);
    }

    if let Err(e) = bridge::delete_bridge() {
        eprintln!("ブリッジの削除に失敗: {}", e);
    }
}


