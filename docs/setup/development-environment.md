# 開発環境構築ガイド

## 必要なパッケージ

OS開発に必要な以下のパッケージをインストールします：

### インストールコマンド

```bash
sudo apt-get update && sudo apt-get install -y qemu-system-x86 ovmf xorriso mtools
```

### パッケージの説明

| パッケージ | 説明 | 用途 |
|-----------|------|------|
| **qemu-system-x86** | x86アーキテクチャ用エミュレーター | 作成したOSイメージの実行・テスト |
| **ovmf** | UEFIファームウェア | UEFIブート対応のOSテスト |
| **xorriso** | ISOイメージ作成ツール | ブータブルISOイメージの作成 |
| **mtools** | FATファイルシステム操作ツール | FATファイルシステムの操作、ブータブルイメージ作成 |

## 動作確認

### QEMUの確認
```bash
qemu-system-x86_64 --version
```

### OVMFの確認
```bash
ls /usr/share/OVMF/OVMF_CODE.fd
```

## トラブルシューティング

### インストールエラー: mtoolsREQ_APP が見つかりません

コマンド内で `mtoolsREQ_APP` となっている場合、これは `mtools` と `REQ_APP` が誤って結合されています。正しいパッケージ名は `mtools` です。

**誤ったコマンド:**
```bash
sudo apt-get install -y qemu-system-x86 ovmf xorriso mtoolsREQ_APP
```

**正しいコマンド:**
```bash
sudo apt-get install -y qemu-system-x86 ovmf xorriso mtools
```

## 追加の推奨パッケージ

RustベースのOS開発には以下も推奨されます：

```bash
# Rustツールチェーン（既にインストール済みの場合はスキップ）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 追加ターゲット
rustup target add x86_64-unknown-none

# cargo-binutils（バイナリ操作用）
cargo install cargo-binutils
```

## 環境情報

- OS: Debian Trixie
- アーキテクチャ: x86_64
- 構築日: 2026/02/18