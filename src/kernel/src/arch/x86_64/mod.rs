//! x86_64アーキテクチャサポート
//! 
//! Intel/AMD 64bitプロセッサ用の実装

mod context;

pub use context::Context;
pub use context::StackFrame;

use crate::arch::CpuOps;

/// x86_64 CPU操作
pub struct X86_64;

impl CpuOps for X86_64 {
    /// 割り込みを有効化（sti命令）
    /// 
    /// # Safety
    /// 割り込み状態を変更する
    #[inline(always)]
    unsafe fn enable_interrupts() {
        core::arch::asm!("sti");
    }
    
    /// 割り込みを無効化（cli命令）
    /// 
    /// # Safety
    /// 割り込み状態を変更する
    #[inline(always)]
    unsafe fn disable_interrupts() {
        core::arch::asm!("cli");
    }
    
    /// 現在のCPU IDを取得
    /// 
    /// x86_64では、ローカルAPIC IDから取得
    fn cpu_id() -> u32 {
        // TODO: ローカルAPICからCPU IDを取得
        0
    }
}