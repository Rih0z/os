//! Learning OS Kernel
//! Phase 0: 最小限のカーネル - QEMUでブートして画面に文字を表示する
//! Phase 1: プロセス管理 - MINIX 3から学んだ構造を実装

#![no_std]  // 標準ライブラリを使用しない
#![no_main] // main関数を使用しない

mod process;

use core::panic::PanicInfo;
use process::{Process, ProcessFlags, ProcessTable, Priority, MAX_PROCESSES};

/// パニックハンドラ
/// パニックが発生した際に呼ばれる（現時点では無限ループ）
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// カーネルのエントリポイント
/// ブートローダーがこの関数を呼び出す
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGAテキストバッファのアドレス（0xB8000）
    // 80x25のテキストモード画面
    let vga_buffer = 0xb8000 as *mut u8;

    // 画面をクリア（スペースで埋める）
    for i in 0..80 * 25 {
        unsafe {
            // 文字
            *vga_buffer.offset(i * 2) = b' ';
            // 属性（ライトグレー on ブラック）
            *vga_buffer.offset(i * 2 + 1) = 0x0f;
        }
    }

    // "Hello, Learning OS!" を表示
    let message = b"Hello, Learning OS!";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            // 文字
            *vga_buffer.offset(i as isize * 2) = byte;
            // 属性（ライトグレー on ブラック）
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    // 無限ループ（OSは終了しない）
    loop {}
}