//! Learning OS Kernel
//! Phase 0: 最小限のカーネル - QEMUでブートして画面に文字を表示する
//! Phase 1: プロセス管理 - MINIX 3から学んだ構造を実装
//! Phase 2: I/O - シリアルポート（UART）ドライバを実装

// テスト時は標準ライブラリを使用
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod process;
mod serial;

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use serial::{ComPort, PortIO, SerialPort};

/// 実機用のI/Oポート操作
/// x86の in/out 命令でハードウェアレジスタを読み書きする
///
/// MINIX 3では kernel/i8259.c などで同様のポート操作を行っている
#[cfg(not(test))]
pub struct X86PortIO;

#[cfg(not(test))]
impl PortIO for X86PortIO {
    fn read(&self, port: u16) -> u8 {
        let value: u8;
        unsafe {
            core::arch::asm!(
                "in al, dx",
                out("al") value,
                in("dx") port,
                options(nomem, nostack, preserves_flags)
            );
        }
        value
    }

    fn write(&self, port: u16, value: u8) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("al") value,
                in("dx") port,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

/// パニックハンドラ
/// パニックが発生した際に呼ばれる
/// シリアルポートにエラー情報を出力してからハルト
/// テスト時は標準ライブラリのpanicハンドラを使用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // パニック時もシリアルに出力を試みる
    let serial = SerialPort::new(ComPort::COM1, X86PortIO);
    serial.send_str("KERNEL PANIC!\r\n");
    loop {}
}

/// カーネルのエントリポイント
/// ブートローダーがこの関数を呼び出す
/// テスト時は除外
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // ===== Phase 2: シリアルポートを初期化 =====
    let serial = SerialPort::new(ComPort::COM1, X86PortIO);
    serial.init();
    serial.send_str("Learning OS booting...\r\n");

    // ===== Phase 0: VGA画面に文字を表示 =====
    let vga_buffer = 0xb8000 as *mut u8;

    // 画面をクリア（スペースで埋める）
    for i in 0..80 * 25 {
        unsafe {
            *vga_buffer.offset(i * 2) = b' ';
            *vga_buffer.offset(i * 2 + 1) = 0x0f;
        }
    }

    // "Hello, Learning OS!" を表示
    let message = b"Hello, Learning OS!";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    serial.send_str("VGA initialized. System ready.\r\n");

    // 無限ループ（OSは終了しない）
    loop {}
}
