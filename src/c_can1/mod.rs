#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN control"]
    pub cntl: CNTL,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Error counter"]
    pub ec: EC,
    #[doc = "0x0c - Bit timing register"]
    pub bt: BT,
    #[doc = "0x10 - Interrupt register"]
    pub int: INT,
    #[doc = "0x14 - Test register"]
    pub test: TEST,
    #[doc = "0x18 - Baud rate prescaler extension register"]
    pub brpe: BRPE,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Message interface command request"]
    pub if1_cmdreq: IF_CMDREQ,
    #[doc = "0x24 - Message interface command mask (write direction)"]
    pub if1_cmdmsk_w: IF_CMDMSK_W,
    #[doc = "0x28 - Message interface mask 1"]
    pub if1_msk1: IF_MSK1,
    #[doc = "0x2c - Message interface 1 mask 2"]
    pub if1_msk2: IF_MSK2,
    #[doc = "0x30 - Message interface 1 arbitration 1"]
    pub if1_arb1: IF_ARB1,
    #[doc = "0x34 - Message interface 1 arbitration 2"]
    pub if1_arb2: IF_ARB2,
    #[doc = "0x38 - Message interface 1 message control"]
    pub if1_mctrl: IF_MCTRL,
    #[doc = "0x3c - Message interface data A1"]
    pub if1_da1: IF_DA1,
    #[doc = "0x40 - Message interface 1 data A2"]
    pub if1_da2: IF_DA2,
    #[doc = "0x44 - Message interface 1 data B1"]
    pub if1_db1: IF_DB1,
    #[doc = "0x48 - Message interface 1 data B2"]
    pub if1_db2: IF_DB2,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - Message interface command request"]
    pub if2_cmdreq: IF_CMDREQ,
    #[doc = "0x84 - Message interface command mask (write direction)"]
    pub if2_cmdmsk_w: IF_CMDMSK_W,
    #[doc = "0x88 - Message interface mask 1"]
    pub if2_msk1: IF_MSK1,
    #[doc = "0x8c - Message interface 1 mask 2"]
    pub if2_msk2: IF_MSK2,
    #[doc = "0x90 - Message interface 1 arbitration 1"]
    pub if2_arb1: IF_ARB1,
    #[doc = "0x94 - Message interface 1 arbitration 2"]
    pub if2_arb2: IF_ARB2,
    #[doc = "0x98 - Message interface 1 message control"]
    pub if2_mctrl: IF_MCTRL,
    #[doc = "0x9c - Message interface data A1"]
    pub if2_da1: IF_DA1,
    #[doc = "0xa0 - Message interface 1 data A2"]
    pub if2_da2: IF_DA2,
    #[doc = "0xa4 - Message interface 1 data B1"]
    pub if2_db1: IF_DB1,
    #[doc = "0xa8 - Message interface 1 data B2"]
    pub if2_db2: IF_DB2,
    _reserved2: [u8; 84usize],
    #[doc = "0x100 - Transmission request 1"]
    pub txreq1: TXREQ1,
    #[doc = "0x104 - Transmission request 2"]
    pub txreq2: TXREQ2,
    _reserved3: [u8; 24usize],
    #[doc = "0x120 - New data 1"]
    pub nd1: ND1,
    #[doc = "0x124 - New data 2"]
    pub nd2: ND2,
    _reserved4: [u8; 24usize],
    #[doc = "0x140 - Interrupt pending 1"]
    pub ir1: IR1,
    #[doc = "0x144 - Interrupt pending 2"]
    pub ir2: IR2,
    _reserved5: [u8; 24usize],
    #[doc = "0x160 - Message valid 1"]
    pub msgv1: MSGV1,
    #[doc = "0x164 - Message valid 2"]
    pub msgv2: MSGV2,
    _reserved6: [u8; 24usize],
    #[doc = "0x180 - CAN clock divider register"]
    pub clkdiv: CLKDIV,
}
#[doc = "CAN control"]
pub struct CNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN control"]
pub mod cntl;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
#[doc = "Error counter"]
pub struct EC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error counter"]
pub mod ec;
#[doc = "Bit timing register"]
pub struct BT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "Interrupt register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt register"]
pub mod int;
#[doc = "Test register"]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test register"]
pub mod test;
#[doc = "Baud rate prescaler extension register"]
pub struct BRPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate prescaler extension register"]
pub mod brpe;
#[doc = "Message interface command request"]
pub struct IF_CMDREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command request"]
pub mod if_cmdreq;
#[doc = "Message interface command mask (write direction)"]
pub struct IF_CMDMSK_W {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command mask (write direction)"]
pub mod if_cmdmsk_w;
#[doc = "Message interface command mask (read direction)"]
pub struct IF_CMDMSK_R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface command mask (read direction)"]
pub mod if_cmdmsk_r;
#[doc = "Message interface mask 1"]
pub struct IF_MSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface mask 1"]
pub mod if_msk1;
#[doc = "Message interface 1 mask 2"]
pub struct IF_MSK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 mask 2"]
pub mod if_msk2;
#[doc = "Message interface 1 arbitration 1"]
pub struct IF_ARB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 arbitration 1"]
pub mod if_arb1;
#[doc = "Message interface 1 arbitration 2"]
pub struct IF_ARB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 arbitration 2"]
pub mod if_arb2;
#[doc = "Message interface 1 message control"]
pub struct IF_MCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 message control"]
pub mod if_mctrl;
#[doc = "Message interface data A1"]
pub struct IF_DA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface data A1"]
pub mod if_da1;
#[doc = "Message interface 1 data A2"]
pub struct IF_DA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data A2"]
pub mod if_da2;
#[doc = "Message interface 1 data B1"]
pub struct IF_DB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data B1"]
pub mod if_db1;
#[doc = "Message interface 1 data B2"]
pub struct IF_DB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message interface 1 data B2"]
pub mod if_db2;
#[doc = "Transmission request 1"]
pub struct TXREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 1"]
pub mod txreq1;
#[doc = "Transmission request 2"]
pub struct TXREQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 2"]
pub mod txreq2;
#[doc = "New data 1"]
pub struct ND1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 1"]
pub mod nd1;
#[doc = "New data 2"]
pub struct ND2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 2"]
pub mod nd2;
#[doc = "Interrupt pending 1"]
pub struct IR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt pending 1"]
pub mod ir1;
#[doc = "Interrupt pending 2"]
pub struct IR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt pending 2"]
pub mod ir2;
#[doc = "Message valid 1"]
pub struct MSGV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message valid 1"]
pub mod msgv1;
#[doc = "Message valid 2"]
pub struct MSGV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message valid 2"]
pub mod msgv2;
#[doc = "CAN clock divider register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN clock divider register"]
pub mod clkdiv;
