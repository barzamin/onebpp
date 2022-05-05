use tock_registers::{register_structs, register_bitfields, registers::ReadWrite};

register_structs! {
    Registers {
        (0x00 => dr: ReadWrite<u32, Data::Register>),
        (0x04 => rsrecr: ReadWrite<u32, Rsrecr::Register>),
        (0x08 => _reserved0),
        (0x18 => fr: ReadWrite<u32, Flag::Register>),
        (0x1c => _reserved1),
        (0x20 => _ilpr), // "not in use"; "disabled IrDA register".
        (0x24 => ibrd: ReadWrite<u32, IntBaudRateDivisor::Register>),
        (0x28 => fbrd: ReadWrite<u32, FracBaudRateDivisor::Register>),
        (0x2c => lcrh: ReadWrite<u32, LineControl::Register>),
        (0x30 => cr: ReadWrite<u32, Control::Register>),
        (0x34 => ifls: ReadWrite<u32, InterruptFIFOLevel::Register>),
        (0x38 => imsc: ReadWrite<u32, InterruptMask::Register>),
        (0x3c => ris: ReadWrite<u32, RawInterruptStatus::Register>),
        (0x40 => mis: ReadWrite<u32, MaskedInterruptStatus::Register>),
        (0x44 => icr: ReadWrite<u32, InterruptClear::Register>),
        (0x48 => _dmacr), // "disabled DMA control register"
        (0x80 => _itcr), // "[integration] test control register"
        (0x84 => _itip), // "integration test input reg"
        (0x88 => _itop), // "integration test output reg"
        (0x8c => _tdr), // "integration test data reg"
        (0x90 => @END),
    }
}

register_bitfields! [
    u32,

    Data [
        /// overrun error
        OE OFFSET(11) NUMBITS(1) [],
        /// break error
        BE OFFSET(10) NUMBITS(1) [],
        /// parity error
        PE OFFSET(9) NUMBITS(1) [],
        /// framing error
        FE OFFSET(8) NUMBITS(1) [],
        /// data read/written
        DATA OFFSET(7) NUMBITS(8) [],
    ],

    Rsrecr [
        /// overrun error
        OE OFFSET(3) NUMBITS(1) [],
        /// break error
        BE OFFSET(2) NUMBITS(1) [],
        /// parity error
        PE OFFSET(1) NUMBITS(1) [],
        /// framing error
        FE OFFSET(0) NUMBITS(1) [],
    ],

    Flag [
        /// transmit fifo empty
        TXFE OFFSET(7) NUMBITS(1) [],
        /// rx fifo full
        RXFF OFFSET(6) NUMBITS(1) [],
        /// tx fifo full
        TXFF OFFSET(5) NUMBITS(1) [],
        /// rx fifo empty
        RXFE OFFSET(4) NUMBITS(1) [],
        /// uart busy w/ tx
        BUSY OFFSET(3) NUMBITS(1) [],
    ],

    IntBaudRateDivisor [
        IBRD OFFSET(0) NUMBITS(16) [],
    ],

    FracBaudRateDivisor [
        FBRD OFFSET(0) NUMBITS(6) [],
    ],

    LineControl [
        /// stick parity select
        SPS OFFSET(7) NUMBITS(1) [],
        /// word length
        WLEN OFFSET(6) NUMBITS(2) [
            EightBit = 0b11,
            SevenBit = 0b10,
            SixBit   = 0b01,
            FiveBit  = 0b00,
        ],
        /// fifo enable
        FEN OFFSET(4) NUMBITS(1) [],
        /// two bits stop select
        STP2 OFFSET(3) NUMBITS(1) [],
        /// even parity select
        EPS OFFSET(2) NUMBITS(1) [],
        /// parity enable
        PEN OFFSET(1) NUMBITS(1) [],
        /// send break
        BRK OFFSET(0) NUMBITS(1) [],
    ],

    Control [
        /// enable cts
        CTSEN OFFSET(15) NUMBITS(1) [],
        /// enable rts
        RTSEN OFFSET(14) NUMBITS(1) [],
        /// rts
        RTS OFFSET(11) NUMBITS(1) [],
        /// rx enable
        RXE OFFSET(9) NUMBITS(1) [],
        /// tx enable
        TXE OFFSET(8) NUMBITS(1) [],
        /// loopback enable
        LBE OFFSET(7) NUMBITS(1) [],
        /// uart enable
        UARTEN OFFSET(0) NUMBITS(1) [],
    ],

    InterruptFIFOLevel [
        /// rx FIFO interrupt setpoint
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            ONE_EIGHTH = 0b000,
            ONE_QUARTER = 0b001,
            ONE_HALF = 0b010,
            THREE_QUARTERS = 0b011,
            SEVEN_EIGHTHS = 0b100,
        ],
        /// tx FIFO interrupt setpoint
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            ONE_EIGHTH = 0b000,
            ONE_QUARTER = 0b001,
            ONE_HALF = 0b010,
            THREE_QUARTERS = 0b011,
            SEVEN_EIGHTHS = 0b100,
        ],
    ],

    InterruptMask [
        /// overrun error interrupt mask
        OEIM OFFSET(10) NUMBITS(1) [],
        /// break error interrupt mask
        BEIM OFFSET(9) NUMBITS(1) [],
        /// parity error interrupt mask
        PEIM OFFSET(8) NUMBITS(1) [],
        /// framing error interrupt mask
        FEIM OFFSET(7) NUMBITS(1) [],
        /// rx timeout interrupt mask
        RTIM OFFSET(6) NUMBITS(1) [],
        /// tx interrupt mask
        TXIM OFFSET(5) NUMBITS(1) [],
        /// rx interrupt mask
        RXIM OFFSET(4) NUMBITS(1) [],
        /// nUARTCTS interrupt mask
        CTSMIM OFFSET(1) NUMBITS(1) [],
    ],

    RawInterruptStatus [
        /// overrun error interrupt status
        OERIS OFFSET(10) NUMBITS(1) [],
        /// break error interrupt status
        BERIS OFFSET(9) NUMBITS(1) [],
        /// parity error interrupt status
        PERIS OFFSET(8) NUMBITS(1) [],
        /// framing error interrupt status
        FERIS OFFSET(7) NUMBITS(1) [],
        /// rx timeout error interrupt status
        RTRIS OFFSET(6) NUMBITS(1) [],
        /// tx interrupt status
        TXRIS OFFSET(5) NUMBITS(1) [],
        /// rx interrupt status
        RXRIS OFFSET(4) NUMBITS(1) [],
        /// nUARTCTS interrupt status
        CTSMRIS OFFSET(1) NUMBITS(1) [],
    ],

    MaskedInterruptStatus [
        /// overrun error masked interrupt status
        OEMIS OFFSET(10) NUMBITS(1) [],
        /// break error masked interrupt status
        BEMIS OFFSET(9) NUMBITS(1) [],
        /// parity error masked interrupt status
        PEMIS OFFSET(8) NUMBITS(1) [],
        /// framing error masked interrupt status
        FEMIS OFFSET(7) NUMBITS(1) [],
        /// rx timeout error masked interrupt status
        RTMIS OFFSET(6) NUMBITS(1) [],
        /// tx masked interrupt status
        TXMIS OFFSET(5) NUMBITS(1) [],
        /// rx masked interrupt status
        RXMIS OFFSET(4) NUMBITS(1) [],
        /// nUARTCTS masked interrupt status
        CTSMMIS OFFSET(1) NUMBITS(1) [],
    ],

    InterruptClear [
        /// overrun error interrupt clear
        OEIC OFFSET(10) NUMBITS(1) [],
        /// break error interrupt clear
        BEIC OFFSET(9) NUMBITS(1) [],
        /// parity error interrupt clear
        PEIC OFFSET(8) NUMBITS(1) [],
        /// framing error interrupt clear
        FEIC OFFSET(7) NUMBITS(1) [],
        /// rx timeout error interrupt clear
        RTIC OFFSET(6) NUMBITS(1) [],
        /// tx interrupt clear
        TXIC OFFSET(5) NUMBITS(1) [],
        /// rx interrupt clear
        RXIC OFFSET(4) NUMBITS(1) [],
        /// nUARTCTS interrupt clear
        CTSMIC OFFSET(1) NUMBITS(1) [],
    ],
];

pub struct PL011 {
    regs: &'static Registers,
}

impl PL011 {
    unsafe fn from_ptr(dev_base: *mut Registers) -> Self {
        Self {
            regs: &*dev_base,
        }
    }
}

