# 学習アプローチ

## 比較学習法

MikanOSで実装した内容をベースに、MINIX 3が同じ問題をどう解決しているかを比較する。

### 例

- 「MikanOSではタスク切り替えをこう実装したけど、MINIX 3ではどうやってる？」
- 「MikanOSのメモリ管理と、MINIX 3のメモリ管理サーバーの違いは？」

## 学習者のプロフィール

- MikanOS（ゼロからのOS自作入門）を完成させた経験あり
- MINIX 3.1.0のソースコードが `.oss/minix-3.1.0/` にある
- Linuxカーネルのソースコードが `.oss/linux/` にある
- 参考書籍: `.books/Operating_Systems_Design_Implementation_3.pdf`

## 重要なソースコードの場所

### MINIX 3
- カーネル: `.oss/minix-3.1.0/kernel/`
- プロセス管理: `.oss/minix-3.1.0/kernel/proc.c`
- メモリ管理サーバー: `.oss/minix-3.1.0/servers/pm/`
- ファイルシステムサーバー: `.oss/minix-3.1.0/servers/fs/`
- デバイスドライバ: `.oss/minix-3.1.0/drivers/`

### MikanOS（参考）
- カーネル: `.oss/mikanos/kernel/`

### Linux（発展）
- カーネル: `.oss/linux/kernel/`

## 学習記録の管理

学習者が学んだことを `notes/` フォルダに記録するよう促す。

- `notes/phase1/` - ブートプロセス
- `notes/phase2/` - プロセス管理
- `notes/phase3/` - メモリ管理
- `notes/phase4/` - IPC
- `notes/phase5/` - ファイルシステム
- `notes/phase6/` - デバイスドライバ