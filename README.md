# Learning OS

MikanOSとMINIX 3をベースに、Rustで学習用OSを自作しながらオペレーティングシステムの概念を学ぶプロジェクト。

## 目標

MINIX 3のマイクロカーネル設計を学び、Rustで自作OSを実装する。

### 学習目標
- OSの基本概念（プロセス、メモリ管理、ファイルシステム、I/O）を深く理解する
- モノリシックカーネルとマイクロカーネルの設計思想の違いを理解する
- Rustでのベアメタルプログラミングを習得する

### 実装目標
- QEMUで動作する最小限のOS
- 最終的にUSBブート可能なOSイメージを作成

## 学習アプローチ

### 比較学習
1. **MikanOS**（知っている）→ モノリシックカーネルの実装例
2. **MINIX 3**（学ぶ）→ マイクロカーネルの設計と実装
3. **自作OS**（実装）→ 学んだことをRustで実装

### アーキテクチャ移行戦略
モノリシックカーネルから開始し、段階的にマイクロカーネルへ移行：

1. **Phase 0-2**: モノリシックカーネル（基本機能実装）
2. **Phase 3**: カーネル内機能をユーザー空間サーバーとして分離開始
3. **Phase 4-5**: マイクロカーネル設計へ移行

## 開発環境

### 必要なツール

| ツール | 用途 |
|--------|------|
| Rust | OS開発言語 |
| QEMU | 仮想マシン（開発・テスト用） |
| cargo | ビルドシステム |

### Rust環境構築

```bash
# Rustインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# ソースを読むためのコンポーネント
rustup component add rust-src
```

### ビルドと実行

```bash
# カーネルビルド
cd src/kernel
cargo build --target x86_64-unknown-none

# QEMUで実行（予定）
cargo run
```

## プロジェクト構成

```
os/
├── src/                    # 自作OS（Rust）
│   ├── kernel/            # カーネル
│   ├── servers/           # ユーザー空間サーバー（将来）
│   └── drivers/           # デバイスドライバ（将来）
├── notes/                  # 学習ノート
│   ├── phase0/            # Introduction
│   ├── phase1/            # Processes
│   ├── phase2/            # Input/Output
│   ├── phase3/            # Memory Management
│   ├── phase4/            # File Systems
│   └── phase5/            # Linux比較
├── docs/
│   └── plan/
│       └── learning-plan.md  # 学習計画
├── .oss/                   # OSソースコード（gitignore）
│   ├── mikanos/           # MikanOS
│   ├── minix-3.1.0/       # MINIX 3
│   └── linux/             # Linux（発展学習）
└── .books/                 # 書籍（gitignore）
    └── Operating_Systems_Design_Implementation_3.pdf
```

## 学習リソース

### 書籍
- **Operating Systems Design and Implementation, Third Edition** - Andrew S. Tanenbaum, Albert S. Woodhull
  - MINIX 3の教科書

### ソースコード
- **MikanOS** - モノリシックカーネル（自作経験あり）
- **MINIX 3.1.0** - マイクロカーネル（メイン学習対象）
- **Linux** - モノリシックカーネル（発展学習）

## 学習フェーズ

| Phase | トピック | 書籍の章 | 自作OSの実装 |
|-------|----------|----------|--------------|
| 0 | Introduction | Chapter 1 | 開発環境構築、ブートローダー |
| 1 | Processes | Chapter 2 | プロセス管理、スケジューラ、IPC |
| 2 | Input/Output | Chapter 3 | デバイスドライバ |
| 3 | Memory Management | Chapter 4 | メモリ管理、ページング |
| 4 | File Systems | Chapter 5 | ファイルシステム |
| 5 | 発展学習 | - | Linux比較、機能拡張 |

## 参考リンク

- [MINIX 3](https://www.minix3.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [MikanOS](https://github.com/uchan-nos/mikanos)

## ライセンス

MIT License