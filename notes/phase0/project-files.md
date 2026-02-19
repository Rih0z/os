# プロジェクトファイル構成と説明

## Cargo.toml とは？

Rustのパッケージマネージャー「Cargo」が使用する設定ファイルです。
npmの`package.json`やPythonの`setup.py`に相当します。

### 主な役割

1. **パッケージ情報の定義** - 名前、バージョン、エディション
2. **依存関係の管理** - 外部ライブラリ（クレート）の指定
3. **ビルド設定** - コンパイルオプション、ターゲット指定
4. **ワークスペース管理** - 複数パッケージの統合

---

## 作成したファイル一覧

### 1. `src/Cargo.toml` - ワークスペースルート

```toml
[workspace]
members = [
    "kernel",
]
resolver = "2"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

| 設定 | 説明 |
|------|------|
| `[workspace]` | 複数パッケージを管理するワークスペースの定義 |
| `members` | ワークスペースに含まれるパッケージのリスト |
| `resolver = "2"` | 新しい依存関係解決アルゴリズムを使用 |
| `[profile.dev]` | 開発ビルド時の設定 |
| `[profile.release]` | リリースビルド時の設定 |
| `panic = "abort"` | パニック時に即座に停止（アンワインドなし） |

---

### 2. `src/kernel/Cargo.toml` - カーネルパッケージ

```toml
[package]
name = "kernel"
version = "0.1.0"
edition = "2021"

[dependencies]
```

| 設定 | 説明 |
|------|------|
| `name` | パッケージ名 |
| `version` | バージョン番号 |
| `edition` | Rustエディション（2021は最新） |
| `[dependencies]` | 外部クレートの依存関係（現在はなし） |

---

### 3. `src/kernel/src/main.rs` - カーネルのエントリポイント

```rust
#![no_std]   // 標準ライブラリを使用しない
#![no_main]  // main関数を使用しない

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGAバッファに文字を表示
    let vga_buffer = 0xb8000 as *mut u8;
    // ... 画面出力処理 ...
    loop {}
}
```

| 要素 | 説明 |
|------|------|
| `#![no_std]` | 標準ライブラリをリンクしない（ベアメタル必須） |
| `#![no_main]` | Rustのmain関数を使わない（ブートローダーが呼び出す） |
| `#[panic_handler]` | パニック時のハンドラ関数 |
| `#[no_mangle]` | 関数名をマングリングしない（リンカが探せるように） |
| `extern "C"` | C言語の呼び出し規約を使用 |
| `_start` | エントリポイントの名前（リンカが探す） |
| `-> !` | 戻り値なし（無限ループする関数） |

---

### 4. `src/kernel/x86_64-learning-os.json` - カスタムターゲット

```json
{
    "llvm-target": "x86_64-unknown-none",
    "arch": "x86_64",
    "os": "none",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float"
}
```

詳細は `notes/phase0/custom-target.md` を参照。

---

### 5. `src/Cargo.lock` - 依存関係のロックファイル

Cargoが自動生成するファイル。依存クレートの正確なバージョンを記録。
現在は依存関係がないため、最小限の内容。

---

### 6. `notes/phase0/custom-target.md` - 学習ノート

カスタムターゲットファイルの詳細説明。

---

### 7. `notes/phase0/stack-unwinding.md` - 学習ノート

スタックアンワインドと`panic = "abort"`の説明。

---

## ディレクトリ構造

```
os/
├── src/
│   ├── Cargo.toml          # ワークスペース設定
│   ├── Cargo.lock          # 依存関係ロック
│   └── kernel/
│       ├── Cargo.toml      # カーネルパッケージ設定
│       ├── src/
│       │   └── main.rs     # カーネル本体
│       └── x86_64-learning-os.json  # カスタムターゲット
└── notes/
    └── phase0/
        ├── custom-target.md
        └── stack-unwinding.md
```

---

## ビルドコマンド

```bash
cd src/kernel
cargo build --target x86_64-unknown-none
```

`--target x86_64-unknown-none` でベアメタル用にコンパイル。