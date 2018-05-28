#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read (DLAB = 0)."]
    pub rbr: RBR,
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1)."]
    pub dlm: DLM,
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    pub iir: IIR,
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Scratch Pad Register. Eight-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    #[doc = "0x24 - IrDA control register (USART3 only)"]
    pub icr: ICR,
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    #[doc = "0x2c - Oversampling Register. Controls the degree of oversampling during each bit time."]
    pub osr: OSR,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - Half-duplex enable Register"]
    pub hden: HDEN,
    _reserved3: [u8; 4usize],
    #[doc = "0x48 - Smart card interface control register"]
    pub scictrl: SCICTRL,
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
    #[doc = "0x58 - Synchronous mode control register."]
    pub syncctrl: SYNCCTRL,
    #[doc = "0x5c - Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
    pub ter: TER,
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read (DLAB = 0)."]
pub struct RBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read (DLAB = 0)."]
pub mod rbr;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here (DLAB = 0)."]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register. The next character to be transmitted is written here (DLAB = 0)."]
pub mod thr;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1)."]
pub struct DLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1)."]
pub mod dll;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1)."]
pub struct DLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1)."]
pub mod dlm;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts (DLAB = 0)."]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts (DLAB = 0)."]
pub mod ier;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub struct IIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub mod fcr;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub struct LCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub mod scr;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "IrDA control register (USART3 only)"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IrDA control register (USART3 only)"]
pub mod icr;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub struct OSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "Half-duplex enable Register"]
pub struct HDEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Half-duplex enable Register"]
pub mod hden;
#[doc = "Smart card interface control register"]
pub struct SCICTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Smart card interface control register"]
pub mod scictrl;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub struct RS485CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub struct RS485ADRMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS-485/EIA-485 direction control delay."]
pub struct RS485DLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "Synchronous mode control register."]
pub struct SYNCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub struct TER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub mod ter;
