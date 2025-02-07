#!/bin/sh

# OSチェック
OS_NAME=$(uname -s)
if [ "$OS_NAME" != "FreeBSD" ]; then
	echo "エラー: 本プログラムはFreeBSDでのみ実行できます。"
	exit 1
fi

# ビルドを実行
cargo b

# ビルドに成功した場合のみファイルを移動
if [ -f target/debug/rust-cni-plugin_bridge ]; then
	cp target/debug/rust-cni-plugin_bridge ./
	echo "rust-cni-plugin_bridgeを./ に移動しました。"
else
	echo "ビルドに失敗しました。"
	exit 1
fi


