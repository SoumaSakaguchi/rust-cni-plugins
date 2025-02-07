# rust-cni-plugins
FreeBSD上で動作するRust製のcni-bridgeプラグインです。<br>
- ADD: jailのネットワーク設定
- DEL: jailのネットワーク削除
- VERSION: バージョン情報の出力

## ビルド・実行方法
```
./build.sh
CNI_COMMAND=VERSION ./rust-cni-plugin_bridge
```

## ADD実行
```
CNI_COMMAND=ADD CNI_CONTAINERID=<jid> CNI_NETNS=<jid> CNI_IFNAME=<Interface_Name> sudo -E ./rust-cni-plugin_bridge 
```
- インタフェースの作成
- ブリッジの作成
- コンテナ・ブリッジへの接続
- IPアドレスの割当
	- 好きなIPアドレスを/tmp/cni-ip.confに位置行書き込んでください

<b>/tmp/cni-ip.conf</b>
```
10.0.0.1
```

## DEL実行
```
CNI_COMMAND=DEL CNI_CONTAINERID=<jid> CNI_NETNS=<jid> CNI_IFNAME=<Interface_Name> sudo -E ./rust-cni-plugin_bridge 
```
