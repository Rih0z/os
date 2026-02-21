//! アーキテクチャ依存モジュール
//! 
//! このモジュールは異なるCPUアーキテクチャ（x86_64, aarch64, riscv64など）
//! に対応するための共通インターフェースを定義します。
//!
//! # 設計思想
//! 
//! - Linux方式: アーキテクチャごとにディレクトリを分ける
//! - Rustのtrait: 共通インターフェースを定義
//! - ゼロコスト抽象化: 静的ディスパッチで実行時オーバーヘッドなし
//!
//! # ディレクトリ構造
//! ```text
//! arch/
//! ├── mod.rs           // このファイル（共通トレイト定義）
//! ├── x86_64/          // Intel/AMD 64bit
//! │   ├── mod.rs
//! │   └── context.rs
//! ├── aarch64/         // ARM 64bit（Android対応）
//! └── riscv64/         // RISC-V 64bit（将来）
//! ```

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

// 将来的に他のアーキテクチャを追加
// #[cfg(target_arch = "aarch64")]
// mod aarch64;
// 
// #[cfg(target_arch = "aarch64")]
// pub use aarch64::*;

/// アーキテクチャ共通のコンテキストインターフェース
/// 
/// コンテキストは、プロセス切り替え時に保存・復元される
/// CPUの状態（レジスタなど）を表します。
pub trait Context: Default + Clone {
    /// 新しいコンテキストを作成
    /// 
    /// # 引数
    /// - `entry_point`: プログラムの開始アドレス
    /// - `stack_pointer`: スタックの先頭アドレス
    fn new(entry_point: u64, stack_pointer: u64) -> Self;
    
    /// 命令ポインタ（プログラムカウンタ）を設定
    fn set_instruction_pointer(&mut self, addr: u64);
    
    /// スタックポインタを設定
    fn set_stack_pointer(&mut self, addr: u64);
    
    /// 命令ポインタを取得
    fn instruction_pointer(&self) -> u64;
    
    /// スタックポインタを取得
    fn stack_pointer(&self) -> u64;
}

/// アーキテクチャ共通の割り込みコントローラインターフェース
pub trait InterruptController {
    /// 割り込みを有効化
    fn enable(&mut self, irq: u32);
    
    /// 割り込みを無効化
    fn disable(&mut self, irq: u32);
    
    /// 割り込みが発生したか確認
    fn is_pending(&self, irq: u32) -> bool;
    
    /// 割り込みをクリア
    fn clear(&mut self, irq: u32);
}

/// アーキテクチャ共通のCPU操作インターフェース
pub trait CpuOps {
    /// 割り込みを有効化
    /// 
    /// # Safety
    /// 割り込み状態を変更するため、安全でない操作
    unsafe fn enable_interrupts();
    
    /// 割り込みを無効化
    /// 
    /// # Safety
    /// 割り込み状態を変更するため、安全でない操作
    unsafe fn disable_interrupts();
    
    /// 現在のCPU IDを取得（マルチコア対応）
    fn cpu_id() -> u32;
}