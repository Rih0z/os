# OS学習計画

## 目標
MINIX 3をベースに学んだことを、実際にRustで自作OS（`src/`）として実装しながら理解を深める

## 学習スタイル
- **AIが先生役** - `CLAUDE.md` の指示に従ってAIが学習をサポート
- **実装しながら学ぶ** - MINIX 3の概念を理解したら、Rustで自作OSに実装
- **比較学習** - MikanOS（知っている）→ MINIX 3（学ぶ）→ 自作OS（実装）の順
- **ソースコードを読む** - MINIX 3の実際のコードを読んで理解を深める
- **ノートを書く** - 学んだことを `notes/` フォルダに記録

## 自作OSの仕様
- **言語**: Rust（必須）
- **実行環境**: QEMU（開発時）→ USBブート（最終目標）
- **ベース**: モノリシックカーネルから開始 → マイクロカーネルへ移行
- **ソースコード**: `src/`

### アーキテクチャ移行戦略
モノリシックからマイクロカーネルへの移行は**簡単ではない**ため、段階的に進める：

1. **Phase 0-2**: モノリシックカーネル（基本機能実装）
2. **Phase 3**: カーネル内機能をユーザー空間サーバーとして分離開始
3. **Phase 4-5**: マイクロカーネル設計へ移行

**理由**: 最初からマイクロカーネル設計すると、IPC・メモリ保護・コンテキストスイッチなど全てを同時に実装する必要があり、学習負荷が高すぎる。

### USBブート対応
- UEFIブートローダーを実装する
- 起動可能なUSBイメージを作成する機能を用意する

## 学習リソース

### ソースコード（.oss/ ※gitignore済み）
| OS | 場所 | 種類 |
|---|---|---|
| MikanOS | `.oss/mikanos/` | モノリシックカーネル（自作経験あり） |
| MINIX 3.1.0 | `.oss/minix-3.1.0/` | マイクロカーネル（メイン学習対象） |
| Linux | `.oss/linux/` | モノリシックカーネル（発展学習） |

### 書籍（.books/ ※gitignore済み）
- `Operating_Systems_Design_Implementation_3.pdf` - MINIX 3の教科書

---

## Phase 0: Introduction & Setup

**目標**: OSの基本概念を理解し、開発環境を構築する

### 書籍を読む
- [ ] Chapter 1 Introduction
  - [ ] 1.1 What Is an Operating System?
  - [ ] 1.2 History of Operating Systems
  - [ ] 1.3 Operating System Concepts
  - [ ] 1.4 System Calls
  - [ ] 1.5 Operating System Structure
  - [ ] 1.6 Outline of the Rest of This Book
  - [ ] 1.7 Summary

### MINIX 3を学ぶ
- [ ] MINIX 3の全体アーキテクチャを理解する
- [ ] カーネル、プロセスマネージャ、ファイルシステムの関係を理解する

### 自作OSの実装
- [ ] Rustでベアメタル開発環境を構築する
- [ ] QEMUでブートできる最小限のカーネルを作る
- [ ] `src/` の初期構造を決める

### 比較・考察
- [ ] MikanOS（モノリシック）vs MINIX 3（マイクロカーネル）の設計思想の違いをノートに書く
- [ ] 自作OSのアーキテクチャを決定する

**ノート**: `notes/phase0/introduction.md`

---

## Phase 1: Processes

**目標**: プロセスの作成・切り替え・スケジューリング・IPCを理解し、実装する

### 書籍を読む
- [ ] Chapter 2 Processes
  - [ ] 2.1 Introduction to Processes
  - [ ] 2.2 Interprocess Communication
  - [ ] 2.3 Classical IPC Problems
  - [ ] 2.4 Scheduling
  - [ ] 2.5 Overview of Processes in MINIX 3
  - [ ] 2.6 Implementation of Processes in MINIX 3
  - [ ] 2.7 The System Task in MINIX 3
  - [ ] 2.8 The Clock Task in MINIX 3
  - [ ] 2.9 Summary

### 事前知識の確認（MikanOS）
- [ ] MikanOSのタスク構造体（`kernel/task.hpp`）を振り返る
- [ ] コンテキストスイッチの実装を確認する
- [ ] MikanOSのメッセージパッシング実装を振り返る

### MINIX 3を学ぶ
- [ ] `.oss/minix-3.1.0/kernel/proc.h` - プロセステーブルの構造を読む
- [ ] `.oss/minix-3.1.0/kernel/proc.c` - プロセス管理の実装を読む
- [ ] `.oss/minix-3.1.0/kernel/ipc.c` - IPCの実装を読む
- [ ] スケジューラの優先度キューを理解する
- [ ] `send()` / `receive()` / `sendrec()` の仕組みを理解する

### 自作OSの実装
- [ ] プロセス構造体をRustで定義する
- [ ] コンテキストスイッチを実装する
- [ ] 簡単なスケジューラを実装する
- [ ] IPC（メッセージパッシング）を実装する

### 比較・考察
- [ ] MikanOSのタスク vs MINIX 3のプロセスの違いをノートに書く
- [ ] 自作OSのプロセスモデルを文書化する

**ノート**: `notes/phase1/processes.md`

---

## Phase 2: Input/Output

**目標**: I/Oの仕組み、デッドロック、デバイスドライバを理解し、実装する

### 書籍を読む
- [ ] Chapter 3 Input/Output
  - [ ] 3.1 Principles of I/O Hardware
  - [ ] 3.2 Principles of I/O Software
  - [ ] 3.3 Deadlocks
  - [ ] 3.4 Overview of I/O in MINIX 3
  - [ ] 3.5 Block Devices in MINIX 3
  - [ ] 3.6 RAM Disks
  - [ ] 3.7 Disks
  - [ ] 3.8 Terminals
  - [ ] 3.9 Summary

### 事前知識の確認（MikanOS）
- [ ] MikanOSのUSBドライバ実装を振り返る
- [ ] カーネル内ドライバの問題点を考える

### MINIX 3を学ぶ
- [ ] `.oss/minix-3.1.0/drivers/` - ドライバ一覧を確認する
- [ ] `drivers/tty/` - TTYドライバを読む
- [ ] ドライバがクラッシュしても再起動できる仕組みを理解する
- [ ] デッドロック検出・回避の仕組みを理解する

### 自作OSの実装
- [ ] シリアル出力ドライバを実装する
- [ ] キーボードドライバを実装する
- [ ] ユーザー空間ドライバの仕組みを検討する

### 比較・考察
- [ ] カーネル内ドライバ（MikanOS）vs ユーザー空間ドライバ（MINIX 3）
- [ ] 自作OSのドライバモデルを決定する

**ノート**: `notes/phase2/input-output.md`

---

## Phase 3: Memory Management

**目標**: 物理・仮想メモリの管理方法を理解し、実装する

### 書籍を読む
- [ ] Chapter 4 Memory Management
  - [ ] 4.1 Basic Memory Management
  - [ ] 4.2 Swapping
  - [ ] 4.3 Virtual Memory
  - [ ] 4.4 Page Replacement Algorithms
  - [ ] 4.5 Design Issues for Paging Systems
  - [ ] 4.6 Segmentation
  - [ ] 4.7 Overview of the MINIX 3 Process Manager
  - [ ] 4.8 Implementation of the MINIX 3 Process Manager
  - [ ] 4.9 Summary

### 事前知識の確認（MikanOS）
- [ ] MikanOSのページング実装を振り返る
- [ ] 物理メモリマップの管理方法を確認する

### MINIX 3を学ぶ
- [ ] `.oss/minix-3.1.0/servers/pm/` - メモリ管理サーバーを読む
- [ ] `alloc.c` - メモリ割り当てアルゴリズムを理解する
- [ ] なぜメモリ管理がユーザー空間サーバーなのかを理解する

### 自作OSの実装
- [ ] 物理メモリ管理を実装する
- [ ] 仮想メモリ（ページング）を実装する
- [ ] ヒープアロケータを実装する

### 比較・考察
- [ ] カーネル内メモリ管理（MikanOS）vs サーバーベース（MINIX 3）の違い
- [ ] 自作OSのメモリ管理戦略を文書化する

**ノート**: `notes/phase3/memory-management.md`

---

## Phase 4: File Systems

**目標**: ファイルシステムの設計と実装を理解し、実装する

### 書籍を読む
- [ ] Chapter 5 File Systems
  - [ ] 5.1 Files
  - [ ] 5.2 Directories
  - [ ] 5.3 File System Implementation
  - [ ] 5.4 Security
  - [ ] 5.5 Protection Mechanisms
  - [ ] 5.6 Overview of the MINIX 3 File System
  - [ ] 5.7 Implementation of the MINIX 3 File System
  - [ ] 5.8 Summary

### 事前知識の確認（MikanOS）
- [ ] MikanOSのFATファイルシステム実装を振り返る

### MINIX 3を学ぶ
- [ ] `.oss/minix-3.1.0/servers/fs/` - ファイルシステムサーバーを読む
- [ ] iノードの構造を理解する
- [ ] VFS（仮想ファイルシステム）の概念を学ぶ

### 自作OSの実装
- [ ] 簡単なファイルシステムを実装する
- [ ] 仮想ファイルシステム（VFS）層を実装する

### 比較・考察
- [ ] FAT（MikanOS）vs MINIXファイルシステムの違い
- [ ] 自作OSのファイルシステム設計を文書化する

**ノート**: `notes/phase4/filesystem.md`

---

## Phase 5: 発展学習 - Linuxカーネル

**目標**: 実用的なOSでの実装を学び、理解を深める

- [ ] 各フェーズのトピックについてLinuxの実装を調査する
- [ ] MINIX 3との設計思想の違いを比較する
- [ ] Linuxがモノリシックでありながら高い信頼性を持つ理由を考察する
- [ ] 自作OSに取り入れるべき機能を検討する

**ノート**: `notes/phase5/linux-comparison.md`

---

## 学習の進め方

### 1セッションの流れ
1. このファイルで今日のタスクを確認
2. 書籍の該当セクションを読む
3. AIに「MINIX 3の〇〇を教えて」と聞いて理解を深める
4. AIと一緒にMINIX 3のソースコードを読む
5. 理解したことをRustで `src/` に実装する
6. QEMUで動作確認する
7. 理解したことをnotesに書く
8. チェックボックスを更新する

### AIへの話しかけ方の例
- 「Phase 0を始めたい。MINIX 3の全体像を教えて」
- 「MINIX 3の `proc.c` を読んでいるけど、この関数が何をしているか説明して」
- 「MINIX 3のIPCを理解したので、Rustで実装したい。手伝って」
- 「自作OSのコンテキストスイッチが動かない。デバッグして」

---

## 書籍の章構成との対応

| Phase | 書籍の章 | トピック | 自作OSの実装 |
|---|---|---|---|
| Phase 0 | Chapter 1 | Introduction | 開発環境構築、ブートローダー |
| Phase 1 | Chapter 2 | Processes | プロセス管理、スケジューラ、IPC |
| Phase 2 | Chapter 3 | Input/Output | デバイスドライバ |
| Phase 3 | Chapter 4 | Memory Management | メモリ管理、ページング |
| Phase 4 | Chapter 5 | File Systems | ファイルシステム |
| Phase 5 | - | 発展学習 | Linux比較、機能拡張 |

---

## プロジェクト構成

```
os/
├── src/                    # 自作OS（Rust）
│   ├── kernel/            # カーネル
│   ├── servers/           # ユーザー空間サーバー
│   └── drivers/           # デバイスドライバ
├── notes/                  # 学習ノート
│   ├── phase0/
│   ├── phase1/
│   ├── phase2/
│   ├── phase3/
│   ├── phase4/
│   └── phase5/
├── docs/
│   └── plan/
│       └── learning-plan.md  # このファイル
├── .oss/                   # OSソースコード（gitignore）
│   ├── mikanos/
│   ├── minix-3.1.0/
│   └── linux/
└── .books/                 # 書籍（gitignore）
    └── Operating_Systems_Design_Implementation_3.pdf
```

---

作成日: 2026-02-17
最終更新: 2026-02-17