# カスタムターゲットファイル（x86_64-learning-os.json）

## このファイルは何？

Rustのコンパイルターゲットを定義する設定ファイルです。

## なぜ必要なのか？

通常、Rustは `x86_64-unknown-linux-gnu` のような標準ターゲットでコンパイルします。
これは「Linux上で動く64bitプログラム」を作る設定です。

しかし、OS自体を開発する場合：
- OSがない環境（ベアメタル）で動くコードが必要
- 標準ライブラリ（libcなど）が使えない
- メモリ配置を自分で制御する必要がある

そのため、独自のターゲット設定を定義します。

## 各設定の意味

| 設定 | 値 | 説明 |
|------|-----|------|
| `llvm-target` | `x86_64-unknown-none` | LLVMのコード生成ターゲット。`none`はOSなしを意味する |
| `arch` | `x86_64` | CPUアーキテクチャ |
| `target-endian` | `little` | バイトオーダー（Intel/AMDはリトルエンディアン） |
| `target-pointer-width` | `64` | ポインタのサイズ（64bit） |
| `os` | `none` | ターゲットOS（なし） |
| `linker-flavor` | `ld.lld` | リンカの種類（LLVMのlldを使用） |
| `linker` | `rust-lld` | Rustに同梱されているlldリンカを使用 |
| `panic-strategy` | `abort` | パニック時は即座に停止（スタックアンワインド不可） |
| `disable-redzone` | `true` | レッドゾーン無効化（割り込み対応のため必須） |
| `features` | `-mmx,-sse,+soft-float` | 浮動小数点をソフトウェアエミュレーション |

## 重要な設定の詳細

### disable-redzone: true

**レッドゾーンとは？**
x86_64のSystem V ABIでは、スタックポインタの下256バイトを「レッドゾーン」として予約している。
関数はこの領域を一時的なデータ保存に使える。

**なぜ無効化する？**
OSカーネルでは割り込みがいつ発生するかわからない。
割り込みハンドラがレッドゾーンを上書きしてしまう可能性があるため、無効化が必要。

### features: "-mmx,-sse,+soft-float"

**なぜSSEを無効化？**
SSEレジスタ（xmm0-xmm15）は浮動小数点演算に使われる。
カーネル開発では：
1. コンテキストスイッチ時に全レジスタを保存/復元する必要がある
2. SSEレジスタを保存するとオーバーヘッドが増える
3. カーネルでは浮動小数点演算をあまり使わない

そのため、SSEを無効化して浮動小数点演算をソフトウェアエミュレーションにする。

## 標準ターゲットとの違い

`rustup target add x86_64-unknown-none` で追加できる標準ターゲットと同じ設定だが、
カスタムターゲットファイルを使うメリット：
1. プロジェクト固有の設定を追加できる
2. 設定内容を明示的に確認できる
3. 将来的に特殊なハードウェア対応などが可能

## 参考資料

- [Rust Embedded Book - Custom Targets](https://docs.rust-embedded.org/embedonomicon/custom-target.html)
- [OSDev Wiki - x86_64](https://wiki.osdev.org/x86-64)