# Source Code

自作OSのソースコード。

## ディレクトリ構成

```
src/
├── kernel/     # カーネル実装
├── servers/    # ユーザー空間サーバー（将来）
└── drivers/    # デバイスドライバ（将来）
```

## ビルド

```bash
# カーネルビルド
cd kernel
cargo build --target x86_64-unknown-none
```

## 実行

```bash
# QEMUで実行（予定）
cargo run
```

## 詳細

プロジェクト全体の説明は [../README.md](../README.md) を参照。