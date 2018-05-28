#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register"]
    pub mac_config: MAC_CONFIG,
    #[doc = "0x04 - MAC frame filter"]
    pub mac_frame_filter: MAC_FRAME_FILTER,
    #[doc = "0x08 - Hash table high register"]
    pub mac_hashtable_high: MAC_HASHTABLE_HIGH,
    #[doc = "0x0c - Hash table low register"]
    pub mac_hashtable_low: MAC_HASHTABLE_LOW,
    #[doc = "0x10 - MII address register"]
    pub mac_mii_addr: MAC_MII_ADDR,
    #[doc = "0x14 - MII data register"]
    pub mac_mii_data: MAC_MII_DATA,
    #[doc = "0x18 - Flow control register"]
    pub mac_flow_ctrl: MAC_FLOW_CTRL,
    #[doc = "0x1c - VLAN tag register"]
    pub mac_vlan_tag: MAC_VLAN_TAG,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Debug register"]
    pub mac_debug: MAC_DEBUG,
    #[doc = "0x28 - Remote wake-up frame filter"]
    pub mac_rwake_frflt: MAC_RWAKE_FRFLT,
    #[doc = "0x2c - PMT control and status"]
    pub mac_pmt_ctrl_stat: MAC_PMT_CTRL_STAT,
    _reserved1: [u8; 8usize],
    #[doc = "0x38 - Interrupt status register"]
    pub mac_intr: MAC_INTR,
    #[doc = "0x3c - Interrupt mask register"]
    pub mac_intr_mask: MAC_INTR_MASK,
    #[doc = "0x40 - MAC address 0 high register"]
    pub mac_addr0_high: MAC_ADDR0_HIGH,
    #[doc = "0x44 - MAC address 0 low register"]
    pub mac_addr0_low: MAC_ADDR0_LOW,
    _reserved2: [u8; 1720usize],
    #[doc = "0x700 - Time stamp control register"]
    pub mac_timestp_ctrl: MAC_TIMESTP_CTRL,
    #[doc = "0x704 - Sub-second increment register"]
    pub subsecond_incr: SUBSECOND_INCR,
    #[doc = "0x708 - System time seconds register"]
    pub seconds: SECONDS,
    #[doc = "0x70c - System time nanoseconds register"]
    pub nanoseconds: NANOSECONDS,
    #[doc = "0x710 - System time seconds update register"]
    pub secondsupdate: SECONDSUPDATE,
    #[doc = "0x714 - System time nanoseconds update register"]
    pub nanosecondsupdate: NANOSECONDSUPDATE,
    #[doc = "0x718 - Time stamp addend register"]
    pub addend: ADDEND,
    #[doc = "0x71c - Target time seconds register"]
    pub targetseconds: TARGETSECONDS,
    #[doc = "0x720 - Target time nanoseconds register"]
    pub targetnanoseconds: TARGETNANOSECONDS,
    #[doc = "0x724 - System time higher word seconds register"]
    pub highword: HIGHWORD,
    #[doc = "0x728 - Time stamp status register"]
    pub timestampstat: TIMESTAMPSTAT,
    _reserved3: [u8; 2260usize],
    #[doc = "0x1000 - Bus Mode Register"]
    pub dma_bus_mode: DMA_BUS_MODE,
    #[doc = "0x1004 - Transmit poll demand register"]
    pub dma_trans_poll_demand: DMA_TRANS_POLL_DEMAND,
    #[doc = "0x1008 - Receive poll demand register"]
    pub dma_rec_poll_demand: DMA_REC_POLL_DEMAND,
    #[doc = "0x100c - Receive descriptor list address register"]
    pub dma_rec_des_addr: DMA_REC_DES_ADDR,
    #[doc = "0x1010 - Transmit descriptor list address register"]
    pub dma_trans_des_addr: DMA_TRANS_DES_ADDR,
    #[doc = "0x1014 - Status register"]
    pub dma_stat: DMA_STAT,
    #[doc = "0x1018 - Operation mode register"]
    pub dma_op_mode: DMA_OP_MODE,
    #[doc = "0x101c - Interrupt enable register"]
    pub dma_int_en: DMA_INT_EN,
    #[doc = "0x1020 - Missed frame and buffer overflow register"]
    pub dma_mfrm_bufof: DMA_MFRM_BUFOF,
    #[doc = "0x1024 - Receive interrupt watchdog timer register"]
    pub dma_rec_int_wdt: DMA_REC_INT_WDT,
    _reserved4: [u8; 32usize],
    #[doc = "0x1048 - Current host transmit descriptor register"]
    pub dma_curhost_trans_des: DMA_CURHOST_TRANS_DES,
    #[doc = "0x104c - Current host receive descriptor register"]
    pub dma_curhost_rec_des: DMA_CURHOST_REC_DES,
    #[doc = "0x1050 - Current host transmit buffer address register"]
    pub dma_curhost_trans_buf: DMA_CURHOST_TRANS_BUF,
    #[doc = "0x1054 - Current host receive buffer address register"]
    pub dma_curhost_rec_buf: DMA_CURHOST_REC_BUF,
}
#[doc = "MAC configuration register"]
pub struct MAC_CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC configuration register"]
pub mod mac_config;
#[doc = "MAC frame filter"]
pub struct MAC_FRAME_FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC frame filter"]
pub mod mac_frame_filter;
#[doc = "Hash table high register"]
pub struct MAC_HASHTABLE_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash table high register"]
pub mod mac_hashtable_high;
#[doc = "Hash table low register"]
pub struct MAC_HASHTABLE_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash table low register"]
pub mod mac_hashtable_low;
#[doc = "MII address register"]
pub struct MAC_MII_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII address register"]
pub mod mac_mii_addr;
#[doc = "MII data register"]
pub struct MAC_MII_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII data register"]
pub mod mac_mii_data;
#[doc = "Flow control register"]
pub struct MAC_FLOW_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow control register"]
pub mod mac_flow_ctrl;
#[doc = "VLAN tag register"]
pub struct MAC_VLAN_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VLAN tag register"]
pub mod mac_vlan_tag;
#[doc = "Debug register"]
pub struct MAC_DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug register"]
pub mod mac_debug;
#[doc = "Remote wake-up frame filter"]
pub struct MAC_RWAKE_FRFLT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remote wake-up frame filter"]
pub mod mac_rwake_frflt;
#[doc = "PMT control and status"]
pub struct MAC_PMT_CTRL_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMT control and status"]
pub mod mac_pmt_ctrl_stat;
#[doc = "Interrupt status register"]
pub struct MAC_INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register"]
pub mod mac_intr;
#[doc = "Interrupt mask register"]
pub struct MAC_INTR_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register"]
pub mod mac_intr_mask;
#[doc = "MAC address 0 high register"]
pub struct MAC_ADDR0_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC address 0 high register"]
pub mod mac_addr0_high;
#[doc = "MAC address 0 low register"]
pub struct MAC_ADDR0_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC address 0 low register"]
pub mod mac_addr0_low;
#[doc = "Time stamp control register"]
pub struct MAC_TIMESTP_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time stamp control register"]
pub mod mac_timestp_ctrl;
#[doc = "Sub-second increment register"]
pub struct SUBSECOND_INCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sub-second increment register"]
pub mod subsecond_incr;
#[doc = "System time seconds register"]
pub struct SECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time seconds register"]
pub mod seconds;
#[doc = "System time nanoseconds register"]
pub struct NANOSECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time nanoseconds register"]
pub mod nanoseconds;
#[doc = "System time seconds update register"]
pub struct SECONDSUPDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time seconds update register"]
pub mod secondsupdate;
#[doc = "System time nanoseconds update register"]
pub struct NANOSECONDSUPDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time nanoseconds update register"]
pub mod nanosecondsupdate;
#[doc = "Time stamp addend register"]
pub struct ADDEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time stamp addend register"]
pub mod addend;
#[doc = "Target time seconds register"]
pub struct TARGETSECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target time seconds register"]
pub mod targetseconds;
#[doc = "Target time nanoseconds register"]
pub struct TARGETNANOSECONDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target time nanoseconds register"]
pub mod targetnanoseconds;
#[doc = "System time higher word seconds register"]
pub struct HIGHWORD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System time higher word seconds register"]
pub mod highword;
#[doc = "Time stamp status register"]
pub struct TIMESTAMPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time stamp status register"]
pub mod timestampstat;
#[doc = "Bus Mode Register"]
pub struct DMA_BUS_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Mode Register"]
pub mod dma_bus_mode;
#[doc = "Transmit poll demand register"]
pub struct DMA_TRANS_POLL_DEMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit poll demand register"]
pub mod dma_trans_poll_demand;
#[doc = "Receive poll demand register"]
pub struct DMA_REC_POLL_DEMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive poll demand register"]
pub mod dma_rec_poll_demand;
#[doc = "Receive descriptor list address register"]
pub struct DMA_REC_DES_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive descriptor list address register"]
pub mod dma_rec_des_addr;
#[doc = "Transmit descriptor list address register"]
pub struct DMA_TRANS_DES_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit descriptor list address register"]
pub mod dma_trans_des_addr;
#[doc = "Status register"]
pub struct DMA_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod dma_stat;
#[doc = "Operation mode register"]
pub struct DMA_OP_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operation mode register"]
pub mod dma_op_mode;
#[doc = "Interrupt enable register"]
pub struct DMA_INT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register"]
pub mod dma_int_en;
#[doc = "Missed frame and buffer overflow register"]
pub struct DMA_MFRM_BUFOF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Missed frame and buffer overflow register"]
pub mod dma_mfrm_bufof;
#[doc = "Receive interrupt watchdog timer register"]
pub struct DMA_REC_INT_WDT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive interrupt watchdog timer register"]
pub mod dma_rec_int_wdt;
#[doc = "Current host transmit descriptor register"]
pub struct DMA_CURHOST_TRANS_DES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current host transmit descriptor register"]
pub mod dma_curhost_trans_des;
#[doc = "Current host receive descriptor register"]
pub struct DMA_CURHOST_REC_DES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current host receive descriptor register"]
pub mod dma_curhost_rec_des;
#[doc = "Current host transmit buffer address register"]
pub struct DMA_CURHOST_TRANS_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current host transmit buffer address register"]
pub mod dma_curhost_trans_buf;
#[doc = "Current host receive buffer address register"]
pub struct DMA_CURHOST_REC_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current host receive buffer address register"]
pub mod dma_curhost_rec_buf;
