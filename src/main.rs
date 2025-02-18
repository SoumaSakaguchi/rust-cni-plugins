use std::env;
use serde::{Serialize, Deserialize};
use serde_json;

mod bridge;
mod iface;
mod addr;

fn main() {
    /* 環境変数の取得 */
    let (command, id, ns_id, if_name) = get_env();

    if id != ns_id {
        eprintln!("CNI_CONTAINERIDとCNI_NETNSは同じ値が入る");
    }

    /* ADD, DEL, VERSIONの実行 */
    match command.as_str() {
        "ADD"     => add(ns_id, if_name),
        "DEL"     => del(ns_id, if_name),
        "VERSION" => println!("\"cniVersion\":\"0.0.0\""),
        _         => eprintln!("未実装"),
    }

}

/* 環境変数の取得 */
fn get_env() -> (String, String, String, String) {
    /* CNI_COMMAND: 実行するコマンド */
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

    /* CNI_CONTAINERD: 設定するコンテナ（CNI_NETNSと同じものを指定） */
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

    /* CNI_NETNS: 設定するnetns（CNI_CONTAINERIDと同じものを指定） */
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

    /* CNI_IFNAME: 任意のインターフェース名 */
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

/* ADDの実行 */
fn add(id: String,if_name: String) {
    if let Err(e) = bridge::create_bridge() {
        eprintln!("ブリッジの作成に失敗: {}", e);
    }

    if let Err(e) = iface::create_iface(if_name.clone()) {
        eprintln!("インタフェースの作成に失敗: {}", e);
    }

    if let Err(e) = iface::add_iface_in_jail(id.clone(), if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }

    if let Err(e) = bridge::add_iface_in_bridge(if_name.clone()) {
        eprintln!("インタフェースの接続に失敗: {}", e);
    }

    if let Err(e) = addr::add_addr_in_jail(id.clone(), if_name.clone()) {
        eprintln!("アドレス割り当てに失敗: {}", e);
    }

    let json_output = print_result(id.clone(), if_name.clone());
    println!("{}", json_output);
}

/* DELの実行 */
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

/* 結果出力用の構造体 */
#[derive(Serialize, Deserialize)]
struct Ip4 {
    ns_name: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    cniVersion: String,
    bridge: String,
    ip4: Ip4,
}

/* 結果出力 */
fn print_result(id: String, if_name: String) -> String {
    let config = Config {
        cniVersion: "0.0.0".to_string(),
        bridge: "cni0".to_string(),
        ip4: Ip4 {
            ns_name: id,
            name: if_name,
        },
    };

    match serde_json::to_string(&config) {
        Ok(json_str) => json_str,
        Err(e) => format!("Error generating JSON: {}", e),
    }
}
