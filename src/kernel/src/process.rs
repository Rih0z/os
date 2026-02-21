//! プロセス管理モジュール
//! MINIX 3の proc.h から学んだ構造をRustで実装

use core::cell::UnsafeCell;

/// プロセス番号の型
/// MINIX 3では負の値はカーネルタスク、0以上はユーザープロセス
pub type ProcessId = i32;

/// プロセスの最大数
pub const MAX_PROCESSES: usize = 16;

/// プロセスの実行状態フラグ
/// MINIX 3の p_rts_flags に相当
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessFlags(u8);

impl ProcessFlags {
    // ===== Phase 1: プロセス管理で使用 =====
    /// ビット0: スロットは空き
    pub const SLOT_FREE: u8 = 0x01;
    /// ビット2: 送信待ち
    pub const SENDING: u8 = 0x04;
    /// ビット3: 受信待ち
    pub const RECEIVING: u8 = 0x08;
    
    // ===== Phase 3: メモリ管理で使用 =====
    // /// ビット1: メモリマップ未設定（fork直後の子プロセス）
    // pub const NO_MAP: u8 = 0x02;
    
    // ===== Phase 4: ファイルシステムで使用 =====
    // /// ビット4: シグナル受信
    // pub const SIGNALED: u8 = 0x10;
    // /// ビット5: シグナル処理中
    // pub const SIG_PENDING: u8 = 0x20;
    
    // ===== 発展: デバッグ・権限管理で使用 =====
    // /// ビット6: デバッグ用停止
    // pub const P_STOP: u8 = 0x40;
    // /// ビット7: 権限なし
    // pub const NO_PRIV: u8 = 0x80;
    
    /// 新しいフラグを作成
    pub const fn new() -> Self {
        Self(0)
    }
    
    /// フラグを設定
    pub fn set(&mut self, flag: u8) {
        self.0 |= flag;
    }
    
    /// フラグをクリア
    pub fn clear(&mut self, flag: u8) {
        self.0 &= !flag;
    }
    
    /// フラグが設定されているか確認
    pub fn is_set(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }
    
    /// 実行可能かどうか（フラグが0なら実行可能）
    pub fn is_runnable(&self) -> bool {
        self.0 == 0
    }
}

/// スケジューリング優先度
/// MINIX 3では 0 が最高優先度、数値が大きいほど低優先度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(u8);

impl Priority {
    /// カーネルタスク用（最高優先度）
    pub const TASK_Q: u8 = 0;
    /// デフォルトユーザー優先度
    pub const USER_Q: u8 = 7;
    /// アイドルプロセス（最低優先度）
    pub const IDLE_Q: u8 = 15;
    
    /// 新しい優先度を作成
    pub const fn new(priority: u8) -> Self {
        Self(priority)
    }
    
    /// 値を取得
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// 時間量子（タイマーティック単位）
#[derive(Debug, Clone, Copy)]
pub struct Quantum(u8);

impl Quantum {
    /// デフォルトの時間量子
    pub const DEFAULT: u8 = 8;
    
    pub const fn new(ticks: u8) -> Self {
        Self(ticks)
    }
    
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// コンテキストスイッチ用のレジスタ保存領域
/// x86_64の呼び出し規約に従って保存するレジスタ
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StackFrame {
    /// 汎用レジスタ
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    /// 割り込み発生時のRFLAGS
    pub rflags: u64,
    /// 割り込み発生時のRIP（リターンアドレス）
    pub rip: u64,
    /// 割り込み発生時のRSP
    pub rsp: u64,
}

impl StackFrame {
    /// ゼロ初期化されたStackFrameを作成（const fn対応）
    pub const fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rflags: 0, rip: 0, rsp: 0,
        }
    }
}

impl Default for StackFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// プロセス構造体
/// MINIX 3の struct proc に相当
#[derive(Debug)]
pub struct Process {
    /// 保存されたレジスタ（コンテキスト）
    pub registers: StackFrame,
    
    /// プロセス番号
    pub pid: ProcessId,
    
    /// 実行状態フラグ
    pub flags: ProcessFlags,
    
    /// 現在の優先度
    pub priority: Priority,
    
    /// 最大優先度（優先度継承用）
    pub max_priority: Priority,
    
    /// 残り時間量子
    pub ticks_left: u8,
    
    /// 時間量子サイズ
    pub quantum_size: u8,
    
    /// プロセス名
    pub name: [u8; 16],
}

impl Process {
    /// 新しいプロセスを作成
    pub const fn new(pid: ProcessId) -> Self {
        Self {
            registers: StackFrame::new(),
            pid,
            flags: ProcessFlags::new(),
            priority: Priority::new(Priority::USER_Q),
            max_priority: Priority::new(Priority::USER_Q),
            ticks_left: Quantum::DEFAULT,
            quantum_size: Quantum::DEFAULT,
            name: [0; 16],
        }
    }
    
    /// 実行可能かどうか
    pub fn is_runnable(&self) -> bool {
        self.flags.is_runnable()
    }
    
    /// 名前を設定
    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(self.name.len() - 1);
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name[len] = 0; // null終端
    }
    
    /// 名前を取得
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("")
    }
}

/// プロセステーブル
/// MINIX 3の proc[] 配列に相当
pub struct ProcessTable {
    processes: [UnsafeCell<Process>; MAX_PROCESSES],
}

impl ProcessTable {
    /// 新しいプロセステーブルを作成
    pub const fn new() -> Self {
        // 配列の初期化（const fn で配列を初期化するためのパターン）
        const EMPTY_PROCESS: UnsafeCell<Process> = UnsafeCell::new(Process::new(0));
        Self {
            processes: [EMPTY_PROCESS; MAX_PROCESSES],
        }
    }
    
    /// プロセスを取得（可変参照）
    pub fn get_mut(&self, index: usize) -> Option<&mut Process> {
        if index < MAX_PROCESSES {
            // フラグを確認してスロットが使用中かチェック
            unsafe { Some(&mut *self.processes[index].get()) }
        } else {
            None
        }
    }
    
    /// 空きスロットを探す
    pub fn find_free_slot(&self) -> Option<usize> {
        for (i, cell) in self.processes.iter().enumerate() {
            unsafe {
                let proc = &*cell.get();
                if proc.flags.is_set(ProcessFlags::SLOT_FREE) || proc.pid == 0 && i > 0 {
                    return Some(i);
                }
            }
        }
        None
    }
}

// 安全性: ProcessTableはシングルスレッド環境でのみ使用される
unsafe impl Sync for ProcessTable {}

// グローバルプロセステーブル
#[no_mangle]
pub static PROCESS_TABLE: ProcessTable = ProcessTable::new();