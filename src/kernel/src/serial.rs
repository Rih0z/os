//! シリアルポート（UART）ドライバ
//! MINIX 3の drivers/rs232/ に相当するシリアル通信機能
//!
//! x86のUART 16550Aチップを制御して、シリアルポート経由で文字を送受信する。
//! QEMUでは、ここに書いた文字がターミナルに表示される。

/// COMポートのベースアドレス
/// x86では COM1=0x3F8, COM2=0x2F8, COM3=0x3E8, COM4=0x2E8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ComPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
}

/// UARTレジスタのオフセット
/// MINIX 3の rs232.c でも同様のレジスタ定義がある
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum UartRegister {
    /// データレジスタ（読み書き）/ ボーレート下位（DLAB=1時）
    Data = 0,
    /// 割り込み許可レジスタ / ボーレート上位（DLAB=1時）
    InterruptEnable = 1,
    /// FIFO制御レジスタ（書き込み）/ 割り込みIDレジスタ（読み込み）
    FifoControl = 2,
    /// ライン制御レジスタ（データフォーマット設定）
    LineControl = 3,
    /// モデム制御レジスタ
    ModemControl = 4,
    /// ライン状態レジスタ（送受信状態の確認）
    LineStatus = 5,
}

/// ライン状態レジスタ（LSR）のビットフラグ
/// UART 16550Aの仕様書に基づく
pub struct LineStatus;

impl LineStatus {
    /// データ受信可能（ビット0）
    pub const DATA_READY: u8 = 0x01;
    /// 送信バッファ空き（ビット5）- 次の文字を送れる
    pub const TRANSMIT_EMPTY: u8 = 0x20;
}

/// I/Oポート操作のトレイト
/// テスト時はモック、実機ではx86のin/out命令を使う
pub trait PortIO {
    /// ポートから1バイト読み込み
    fn read(&self, port: u16) -> u8;
    /// ポートに1バイト書き込み
    fn write(&self, port: u16, value: u8);
}

/// 参照に対してもPortIOを実装
/// これにより、&MockPortIO のように共有参照でも使える
impl<T: PortIO> PortIO for &T {
    fn read(&self, port: u16) -> u8 {
        (*self).read(port)
    }
    fn write(&self, port: u16, value: u8) {
        (*self).write(port, value)
    }
}

/// シリアルポートドライバ
/// MINIX 3の rs232構造体に相当
pub struct SerialPort<P: PortIO> {
    /// COMポートのベースアドレス
    base: u16,
    /// I/Oポート操作
    port_io: P,
}

impl<P: PortIO> SerialPort<P> {
    /// 新しいシリアルポートドライバを作成
    pub fn new(com: ComPort, port_io: P) -> Self {
        Self {
            base: com as u16,
            port_io,
        }
    }

    /// UARTを初期化する
    /// ボーレート設定、データフォーマット設定、FIFO有効化を行う
    pub fn init(&self) {
        // 割り込みを無効化
        self.write_register(UartRegister::InterruptEnable, 0x00);

        // DLAB（Divisor Latch Access Bit）を有効化してボーレート設定
        self.write_register(UartRegister::LineControl, 0x80);

        // ボーレート 115200 を設定（divisor = 1）
        // 下位バイト
        self.write_register(UartRegister::Data, 0x01);
        // 上位バイト
        self.write_register(UartRegister::InterruptEnable, 0x00);

        // データフォーマット: 8ビット、パリティなし、ストップビット1（8N1）
        // DLAB=0に戻す
        self.write_register(UartRegister::LineControl, 0x03);

        // FIFOを有効化、14バイト閾値
        self.write_register(UartRegister::FifoControl, 0xC7);

        // モデム制御: DTR + RTS + OUT2（割り込み用）
        self.write_register(UartRegister::ModemControl, 0x0B);
    }

    /// 送信バッファが空くまで待って1バイト送信
    pub fn send_byte(&self, byte: u8) {
        self.wait_transmit_empty();
        self.write_register(UartRegister::Data, byte);
    }

    /// 文字列を送信
    pub fn send_str(&self, s: &str) {
        for byte in s.bytes() {
            self.send_byte(byte);
        }
    }

    /// データが受信可能か確認
    pub fn has_data(&self) -> bool {
        self.read_register(UartRegister::LineStatus) & LineStatus::DATA_READY != 0
    }

    /// 1バイト受信（データがない場合はNone）
    pub fn receive_byte(&self) -> Option<u8> {
        if self.has_data() {
            Some(self.read_register(UartRegister::Data))
        } else {
            None
        }
    }

    /// 送信バッファが空くまで待つ
    fn wait_transmit_empty(&self) {
        while self.read_register(UartRegister::LineStatus) & LineStatus::TRANSMIT_EMPTY == 0 {
            // ビジーウェイト（ポーリング）
            // 将来は割り込み駆動に改善可能
        }
    }

    /// レジスタに書き込み
    fn write_register(&self, reg: UartRegister, value: u8) {
        self.port_io.write(self.base + reg as u16, value);
    }

    /// レジスタから読み込み
    fn read_register(&self, reg: UartRegister) -> u8 {
        self.port_io.read(self.base + reg as u16)
    }
}

// =============================================================================
// テスト
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// テスト用のモックI/Oポート
    /// 実際のハードウェアの代わりに、メモリ上でI/O操作をシミュレート
    struct MockPortIO {
        /// ポートアドレス → 値 のマッピング
        ports: RefCell<std::collections::HashMap<u16, u8>>,
        /// 書き込み履歴（ポートアドレス, 値）
        write_log: RefCell<Vec<(u16, u8)>>,
    }

    impl MockPortIO {
        fn new() -> Self {
            Self {
                ports: RefCell::new(std::collections::HashMap::new()),
                write_log: RefCell::new(Vec::new()),
            }
        }

        /// 特定のポートに初期値を設定
        fn set_port(&self, port: u16, value: u8) {
            self.ports.borrow_mut().insert(port, value);
        }

        /// 書き込み履歴を取得
        fn get_write_log(&self) -> Vec<(u16, u8)> {
            self.write_log.borrow().clone()
        }
    }

    impl PortIO for MockPortIO {
        fn read(&self, port: u16) -> u8 {
            *self.ports.borrow().get(&port).unwrap_or(&0)
        }

        fn write(&self, port: u16, value: u8) {
            self.ports.borrow_mut().insert(port, value);
            self.write_log.borrow_mut().push((port, value));
        }
    }

    // ===== ComPort のテスト =====

    #[test]
    fn test_com_port_addresses() {
        assert_eq!(ComPort::COM1 as u16, 0x3F8);
        assert_eq!(ComPort::COM2 as u16, 0x2F8);
    }

    // ===== LineStatus のテスト =====

    #[test]
    fn test_line_status_flags() {
        assert_eq!(LineStatus::DATA_READY, 0x01);
        assert_eq!(LineStatus::TRANSMIT_EMPTY, 0x20);
    }

    // ===== SerialPort の初期化テスト =====

    #[test]
    fn test_serial_port_init() {
        let mock = MockPortIO::new();
        let serial = SerialPort::new(ComPort::COM1, &mock);
        serial.init();

        let log = mock.get_write_log();

        // 初期化シーケンスを確認
        // 1. 割り込み無効化
        assert_eq!(log[0], (0x3F8 + 1, 0x00), "割り込み無効化");
        // 2. DLAB有効化
        assert_eq!(log[1], (0x3F8 + 3, 0x80), "DLAB有効化");
        // 3. ボーレート下位
        assert_eq!(log[2], (0x3F8 + 0, 0x01), "ボーレート下位");
        // 4. ボーレート上位
        assert_eq!(log[3], (0x3F8 + 1, 0x00), "ボーレート上位");
        // 5. 8N1設定 + DLAB無効化
        assert_eq!(log[4], (0x3F8 + 3, 0x03), "8N1設定");
        // 6. FIFO有効化
        assert_eq!(log[5], (0x3F8 + 2, 0xC7), "FIFO有効化");
        // 7. モデム制御
        assert_eq!(log[6], (0x3F8 + 4, 0x0B), "モデム制御");
    }

    // ===== 送信テスト =====

    #[test]
    fn test_send_byte() {
        let mock = MockPortIO::new();
        // 送信バッファは空き状態に設定
        mock.set_port(0x3F8 + 5, LineStatus::TRANSMIT_EMPTY);

        let serial = SerialPort::new(ComPort::COM1, &mock);
        serial.send_byte(b'A');

        let log = mock.get_write_log();
        // データレジスタに 'A' が書かれたか確認
        assert_eq!(log.last(), Some(&(0x3F8, b'A')));
    }

    #[test]
    fn test_send_str() {
        let mock = MockPortIO::new();
        mock.set_port(0x3F8 + 5, LineStatus::TRANSMIT_EMPTY);

        let serial = SerialPort::new(ComPort::COM1, &mock);
        serial.send_str("Hi");

        let log = mock.get_write_log();
        // 'H' と 'i' がデータレジスタに書かれたか確認
        let data_writes: Vec<u8> = log
            .iter()
            .filter(|(port, _)| *port == 0x3F8)
            .map(|(_, val)| *val)
            .collect();
        assert_eq!(data_writes, vec![b'H', b'i']);
    }

    // ===== 受信テスト =====

    #[test]
    fn test_has_data_when_ready() {
        let mock = MockPortIO::new();
        mock.set_port(0x3F8 + 5, LineStatus::DATA_READY);

        let serial = SerialPort::new(ComPort::COM1, &mock);
        assert!(serial.has_data());
    }

    #[test]
    fn test_has_data_when_not_ready() {
        let mock = MockPortIO::new();
        mock.set_port(0x3F8 + 5, 0x00);

        let serial = SerialPort::new(ComPort::COM1, &mock);
        assert!(!serial.has_data());
    }

    #[test]
    fn test_receive_byte_when_data_available() {
        let mock = MockPortIO::new();
        mock.set_port(0x3F8 + 5, LineStatus::DATA_READY);
        mock.set_port(0x3F8, b'Z');

        let serial = SerialPort::new(ComPort::COM1, &mock);
        assert_eq!(serial.receive_byte(), Some(b'Z'));
    }

    #[test]
    fn test_receive_byte_when_no_data() {
        let mock = MockPortIO::new();
        mock.set_port(0x3F8 + 5, 0x00);

        let serial = SerialPort::new(ComPort::COM1, &mock);
        assert_eq!(serial.receive_byte(), None);
    }

    // ===== COM2テスト =====

    #[test]
    fn test_com2_uses_correct_base_address() {
        let mock = MockPortIO::new();
        mock.set_port(0x2F8 + 5, LineStatus::TRANSMIT_EMPTY);

        let serial = SerialPort::new(ComPort::COM2, &mock);
        serial.send_byte(b'X');

        let log = mock.get_write_log();
        assert_eq!(log.last(), Some(&(0x2F8, b'X')));
    }
}
