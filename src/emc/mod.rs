#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller."]
    pub control: CONTROL,
    #[doc = "0x04 - Provides EMC status information."]
    pub status: STATUS,
    #[doc = "0x08 - Configures operation of the memory controller."]
    pub config: CONFIG,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - Controls dynamic memory operation."]
    pub dynamiccontrol: DYNAMICCONTROL,
    #[doc = "0x24 - Configures dynamic memory refresh operation."]
    pub dynamicrefresh: DYNAMICREFRESH,
    #[doc = "0x28 - Configures the dynamic memory read strategy."]
    pub dynamicreadconfig: DYNAMICREADCONFIG,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Selects the precharge command period."]
    pub dynamicrp: DYNAMICRP,
    #[doc = "0x34 - Selects the active to precharge command period."]
    pub dynamicras: DYNAMICRAS,
    #[doc = "0x38 - Selects the self-refresh exit time."]
    pub dynamicsrex: DYNAMICSREX,
    #[doc = "0x3c - Selects the last-data-out to active command time."]
    pub dynamicapr: DYNAMICAPR,
    #[doc = "0x40 - Selects the data-in to active command time."]
    pub dynamicdal: DYNAMICDAL,
    #[doc = "0x44 - Selects the write recovery time."]
    pub dynamicwr: DYNAMICWR,
    #[doc = "0x48 - Selects the active to active command period."]
    pub dynamicrc: DYNAMICRC,
    #[doc = "0x4c - Selects the auto-refresh period."]
    pub dynamicrfc: DYNAMICRFC,
    #[doc = "0x50 - Selects the exit self-refresh to active command time."]
    pub dynamicxsr: DYNAMICXSR,
    #[doc = "0x54 - Selects the active bank A to active bank B latency."]
    pub dynamicrrd: DYNAMICRRD,
    #[doc = "0x58 - Selects the load mode register to active command time."]
    pub dynamicmrd: DYNAMICMRD,
    _reserved2: [u8; 36usize],
    #[doc = "0x80 - Selects time for long static memory read and write transfers."]
    pub staticextendedwait: STATICEXTENDEDWAIT,
    _reserved3: [u8; 124usize],
    #[doc = "0x100 - Selects the configuration information for dynamic memory chip select 0."]
    pub dynamicconfig0: DYNAMICCONFIG,
    #[doc = "0x104 - Selects the RAS and CAS latencies for dynamic memory chip select 0."]
    pub dynamicrascas0: DYNAMICRASCAS,
    _reserved4: [u8; 24usize],
    #[doc = "0x120 - Selects the configuration information for dynamic memory chip select 0."]
    pub dynamicconfig1: DYNAMICCONFIG,
    #[doc = "0x124 - Selects the RAS and CAS latencies for dynamic memory chip select 0."]
    pub dynamicrascas1: DYNAMICRASCAS,
    _reserved5: [u8; 24usize],
    #[doc = "0x140 - Selects the configuration information for dynamic memory chip select 0."]
    pub dynamicconfig2: DYNAMICCONFIG,
    #[doc = "0x144 - Selects the RAS and CAS latencies for dynamic memory chip select 0."]
    pub dynamicrascas2: DYNAMICRASCAS,
    _reserved6: [u8; 24usize],
    #[doc = "0x160 - Selects the configuration information for dynamic memory chip select 0."]
    pub dynamicconfig3: DYNAMICCONFIG,
    #[doc = "0x164 - Selects the RAS and CAS latencies for dynamic memory chip select 0."]
    pub dynamicrascas3: DYNAMICRASCAS,
    _reserved7: [u8; 152usize],
    #[doc = "0x200 - Selects the memory configuration for static chip select 0."]
    pub staticconfig0: STATICCONFIG,
    #[doc = "0x204 - Selects the delay from chip select 0 to write enable."]
    pub staticwaitwen0: STATICWAITWEN,
    #[doc = "0x208 - Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
    pub staticwaitoen0: STATICWAITOEN,
    #[doc = "0x20c - Selects the delay from chip select 0 to a read access."]
    pub staticwaitrd0: STATICWAITRD,
    #[doc = "0x210 - Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
    pub staticwaitpage0: STATICWAITPAGE,
    #[doc = "0x214 - Selects the delay from chip select 0 to a write access."]
    pub staticwaitwr0: STATICWAITWR,
    #[doc = "0x218 - Selects the number of bus turnaround cycles for chip select 0."]
    pub staticwaitturn0: STATICWAITTURN,
    _reserved8: [u8; 4usize],
    #[doc = "0x220 - Selects the memory configuration for static chip select 0."]
    pub staticconfig1: STATICCONFIG,
    #[doc = "0x224 - Selects the delay from chip select 0 to write enable."]
    pub staticwaitwen1: STATICWAITWEN,
    #[doc = "0x228 - Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
    pub staticwaitoen1: STATICWAITOEN,
    #[doc = "0x22c - Selects the delay from chip select 0 to a read access."]
    pub staticwaitrd1: STATICWAITRD,
    #[doc = "0x230 - Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
    pub staticwaitpage1: STATICWAITPAGE,
    #[doc = "0x234 - Selects the delay from chip select 0 to a write access."]
    pub staticwaitwr1: STATICWAITWR,
    #[doc = "0x238 - Selects the number of bus turnaround cycles for chip select 0."]
    pub staticwaitturn1: STATICWAITTURN,
    _reserved9: [u8; 4usize],
    #[doc = "0x240 - Selects the memory configuration for static chip select 0."]
    pub staticconfig2: STATICCONFIG,
    #[doc = "0x244 - Selects the delay from chip select 0 to write enable."]
    pub staticwaitwen2: STATICWAITWEN,
    #[doc = "0x248 - Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
    pub staticwaitoen2: STATICWAITOEN,
    #[doc = "0x24c - Selects the delay from chip select 0 to a read access."]
    pub staticwaitrd2: STATICWAITRD,
    #[doc = "0x250 - Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
    pub staticwaitpage2: STATICWAITPAGE,
    #[doc = "0x254 - Selects the delay from chip select 0 to a write access."]
    pub staticwaitwr2: STATICWAITWR,
    #[doc = "0x258 - Selects the number of bus turnaround cycles for chip select 0."]
    pub staticwaitturn2: STATICWAITTURN,
    _reserved10: [u8; 4usize],
    #[doc = "0x260 - Selects the memory configuration for static chip select 0."]
    pub staticconfig3: STATICCONFIG,
    #[doc = "0x264 - Selects the delay from chip select 0 to write enable."]
    pub staticwaitwen3: STATICWAITWEN,
    #[doc = "0x268 - Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
    pub staticwaitoen3: STATICWAITOEN,
    #[doc = "0x26c - Selects the delay from chip select 0 to a read access."]
    pub staticwaitrd3: STATICWAITRD,
    #[doc = "0x270 - Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
    pub staticwaitpage3: STATICWAITPAGE,
    #[doc = "0x274 - Selects the delay from chip select 0 to a write access."]
    pub staticwaitwr3: STATICWAITWR,
    #[doc = "0x278 - Selects the number of bus turnaround cycles for chip select 0."]
    pub staticwaitturn3: STATICWAITTURN,
}
#[doc = "Controls operation of the memory controller."]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls operation of the memory controller."]
pub mod control;
#[doc = "Provides EMC status information."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Provides EMC status information."]
pub mod status;
#[doc = "Configures operation of the memory controller."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures operation of the memory controller."]
pub mod config;
#[doc = "Controls dynamic memory operation."]
pub struct DYNAMICCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls dynamic memory operation."]
pub mod dynamiccontrol;
#[doc = "Configures dynamic memory refresh operation."]
pub struct DYNAMICREFRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures dynamic memory refresh operation."]
pub mod dynamicrefresh;
#[doc = "Configures the dynamic memory read strategy."]
pub struct DYNAMICREADCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures the dynamic memory read strategy."]
pub mod dynamicreadconfig;
#[doc = "Selects the precharge command period."]
pub struct DYNAMICRP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the precharge command period."]
pub mod dynamicrp;
#[doc = "Selects the active to precharge command period."]
pub struct DYNAMICRAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the active to precharge command period."]
pub mod dynamicras;
#[doc = "Selects the self-refresh exit time."]
pub struct DYNAMICSREX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the self-refresh exit time."]
pub mod dynamicsrex;
#[doc = "Selects the last-data-out to active command time."]
pub struct DYNAMICAPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the last-data-out to active command time."]
pub mod dynamicapr;
#[doc = "Selects the data-in to active command time."]
pub struct DYNAMICDAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the data-in to active command time."]
pub mod dynamicdal;
#[doc = "Selects the write recovery time."]
pub struct DYNAMICWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the write recovery time."]
pub mod dynamicwr;
#[doc = "Selects the active to active command period."]
pub struct DYNAMICRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the active to active command period."]
pub mod dynamicrc;
#[doc = "Selects the auto-refresh period."]
pub struct DYNAMICRFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the auto-refresh period."]
pub mod dynamicrfc;
#[doc = "Selects the exit self-refresh to active command time."]
pub struct DYNAMICXSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the exit self-refresh to active command time."]
pub mod dynamicxsr;
#[doc = "Selects the active bank A to active bank B latency."]
pub struct DYNAMICRRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the active bank A to active bank B latency."]
pub mod dynamicrrd;
#[doc = "Selects the load mode register to active command time."]
pub struct DYNAMICMRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the load mode register to active command time."]
pub mod dynamicmrd;
#[doc = "Selects time for long static memory read and write transfers."]
pub struct STATICEXTENDEDWAIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects time for long static memory read and write transfers."]
pub mod staticextendedwait;
#[doc = "Selects the configuration information for dynamic memory chip select 0."]
pub struct DYNAMICCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the configuration information for dynamic memory chip select 0."]
pub mod dynamicconfig;
#[doc = "Selects the RAS and CAS latencies for dynamic memory chip select 0."]
pub struct DYNAMICRASCAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the RAS and CAS latencies for dynamic memory chip select 0."]
pub mod dynamicrascas;
#[doc = "Selects the memory configuration for static chip select 0."]
pub struct STATICCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the memory configuration for static chip select 0."]
pub mod staticconfig;
#[doc = "Selects the delay from chip select 0 to write enable."]
pub struct STATICWAITWEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the delay from chip select 0 to write enable."]
pub mod staticwaitwen;
#[doc = "Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
pub struct STATICWAITOEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the delay from chip select 0 or address change, whichever is later, to output enable."]
pub mod staticwaitoen;
#[doc = "Selects the delay from chip select 0 to a read access."]
pub struct STATICWAITRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the delay from chip select 0 to a read access."]
pub mod staticwaitrd;
#[doc = "Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
pub struct STATICWAITPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the delay for asynchronous page mode sequential accesses for chip select 0."]
pub mod staticwaitpage;
#[doc = "Selects the delay from chip select 0 to a write access."]
pub struct STATICWAITWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the delay from chip select 0 to a write access."]
pub mod staticwaitwr;
#[doc = "Selects the number of bus turnaround cycles for chip select 0."]
pub struct STATICWAITTURN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the number of bus turnaround cycles for chip select 0."]
pub mod staticwaitturn;
