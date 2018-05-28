#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Power Enable Register"]
    pub pwren: PWREN,
    #[doc = "0x08 - Clock Divider Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x0c - SD Clock Source Register"]
    pub clksrc: CLKSRC,
    #[doc = "0x10 - Clock Enable Register"]
    pub clkena: CLKENA,
    #[doc = "0x14 - Time-out Register"]
    pub tmout: TMOUT,
    #[doc = "0x18 - Card Type Register"]
    pub ctype: CTYPE,
    #[doc = "0x1c - Block Size Register"]
    pub blksiz: BLKSIZ,
    #[doc = "0x20 - Byte Count Register"]
    pub bytcnt: BYTCNT,
    #[doc = "0x24 - Interrupt Mask Register"]
    pub intmask: INTMASK,
    #[doc = "0x28 - Command Argument Register"]
    pub cmdarg: CMDARG,
    #[doc = "0x2c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x30 - Response Register 0"]
    pub resp0: RESP0,
    #[doc = "0x34 - Response Register 1"]
    pub resp1: RESP1,
    #[doc = "0x38 - Response Register 2"]
    pub resp2: RESP2,
    #[doc = "0x3c - Response Register 3"]
    pub resp3: RESP3,
    #[doc = "0x40 - Masked Interrupt Status Register"]
    pub mintsts: MINTSTS,
    #[doc = "0x44 - Raw Interrupt Status Register"]
    pub rintsts: RINTSTS,
    #[doc = "0x48 - Status Register"]
    pub status: STATUS,
    #[doc = "0x4c - FIFO Threshold Watermark Register"]
    pub fifoth: FIFOTH,
    #[doc = "0x50 - Card Detect Register"]
    pub cdetect: CDETECT,
    #[doc = "0x54 - Write Protect Register"]
    pub wrtprt: WRTPRT,
    _reserved0: [u8; 4usize],
    #[doc = "0x5c - Transferred CIU Card Byte Count Register"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count Register"]
    pub tbbcnt: TBBCNT,
    #[doc = "0x64 - Debounce Count Register"]
    pub debnce: DEBNCE,
    _reserved1: [u8; 16usize],
    #[doc = "0x78 - Hardware Reset"]
    pub rst_n: RST_N,
    _reserved2: [u8; 4usize],
    #[doc = "0x80 - Bus Mode Register"]
    pub bmod: BMOD,
    #[doc = "0x84 - Poll Demand Register"]
    pub pldmnd: PLDMND,
    #[doc = "0x88 - Descriptor List Base Address Register"]
    pub dbaddr: DBADDR,
    #[doc = "0x8c - Internal DMAC Status Register"]
    pub idsts: IDSTS,
    #[doc = "0x90 - Internal DMAC Interrupt Enable Register"]
    pub idinten: IDINTEN,
    #[doc = "0x94 - Current Host Descriptor Address Register"]
    pub dscaddr: DSCADDR,
    #[doc = "0x98 - Current Buffer Descriptor Address Register"]
    pub bufaddr: BUFADDR,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Power Enable Register"]
pub struct PWREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Enable Register"]
pub mod pwren;
#[doc = "Clock Divider Register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Divider Register"]
pub mod clkdiv;
#[doc = "SD Clock Source Register"]
pub struct CLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD Clock Source Register"]
pub mod clksrc;
#[doc = "Clock Enable Register"]
pub struct CLKENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Enable Register"]
pub mod clkena;
#[doc = "Time-out Register"]
pub struct TMOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-out Register"]
pub mod tmout;
#[doc = "Card Type Register"]
pub struct CTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Card Type Register"]
pub mod ctype;
#[doc = "Block Size Register"]
pub struct BLKSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Size Register"]
pub mod blksiz;
#[doc = "Byte Count Register"]
pub struct BYTCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Byte Count Register"]
pub mod bytcnt;
#[doc = "Interrupt Mask Register"]
pub struct INTMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod intmask;
#[doc = "Command Argument Register"]
pub struct CMDARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Argument Register"]
pub mod cmdarg;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Response Register 0"]
pub struct RESP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register 0"]
pub mod resp0;
#[doc = "Response Register 1"]
pub struct RESP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register 1"]
pub mod resp1;
#[doc = "Response Register 2"]
pub struct RESP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register 2"]
pub mod resp2;
#[doc = "Response Register 3"]
pub struct RESP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register 3"]
pub mod resp3;
#[doc = "Masked Interrupt Status Register"]
pub struct MINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status Register"]
pub mod mintsts;
#[doc = "Raw Interrupt Status Register"]
pub struct RINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod rintsts;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "FIFO Threshold Watermark Register"]
pub struct FIFOTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Threshold Watermark Register"]
pub mod fifoth;
#[doc = "Card Detect Register"]
pub struct CDETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Card Detect Register"]
pub mod cdetect;
#[doc = "Write Protect Register"]
pub struct WRTPRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Register"]
pub mod wrtprt;
#[doc = "Transferred CIU Card Byte Count Register"]
pub struct TCBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transferred CIU Card Byte Count Register"]
pub mod tcbcnt;
#[doc = "Transferred Host to BIU-FIFO Byte Count Register"]
pub struct TBBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transferred Host to BIU-FIFO Byte Count Register"]
pub mod tbbcnt;
#[doc = "Debounce Count Register"]
pub struct DEBNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debounce Count Register"]
pub mod debnce;
#[doc = "Hardware Reset"]
pub struct RST_N {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "Bus Mode Register"]
pub struct BMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Mode Register"]
pub mod bmod;
#[doc = "Poll Demand Register"]
pub struct PLDMND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Poll Demand Register"]
pub mod pldmnd;
#[doc = "Descriptor List Base Address Register"]
pub struct DBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor List Base Address Register"]
pub mod dbaddr;
#[doc = "Internal DMAC Status Register"]
pub struct IDSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal DMAC Status Register"]
pub mod idsts;
#[doc = "Internal DMAC Interrupt Enable Register"]
pub struct IDINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal DMAC Interrupt Enable Register"]
pub mod idinten;
#[doc = "Current Host Descriptor Address Register"]
pub struct DSCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Descriptor Address Register"]
pub mod dscaddr;
#[doc = "Current Buffer Descriptor Address Register"]
pub struct BUFADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Buffer Descriptor Address Register"]
pub mod bufaddr;
