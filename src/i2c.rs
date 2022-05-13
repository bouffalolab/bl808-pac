#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - Interrupt enables, states and masks"]
    pub interrupt: INTERRUPT,
    #[doc = "0x08 - Register address of slave device"]
    pub sub_address: SUB_ADDRESS,
    #[doc = "0x0c - Bus busy state indicator"]
    pub bus_busy: BUS_BUSY,
    #[doc = "0x10 - Duration of start phase"]
    pub period_start: PERIOD_START,
    #[doc = "0x14 - Duration of stop phase"]
    pub period_stop: PERIOD_STOP,
    #[doc = "0x18 - Duration of data phase"]
    pub period_data: PERIOD_DATA,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - FIFO configuration register 0"]
    pub fifo_config_0: FIFO_CONFIG_0,
    #[doc = "0x84 - FIFO configuration register 1"]
    pub fifo_config_1: FIFO_CONFIG_1,
    #[doc = "0x88 - FIFO write data register"]
    pub data_write: DATA_WRITE,
    #[doc = "0x8c - FIFO read data register"]
    pub data_read: DATA_READ,
}
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Function configuration register"]
pub mod config;
#[doc = "interrupt (rw) register accessor: an alias for `Reg<INTERRUPT_SPEC>`"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = "Interrupt enables, states and masks"]
pub mod interrupt;
#[doc = "sub_address (rw) register accessor: an alias for `Reg<SUB_ADDRESS_SPEC>`"]
pub type SUB_ADDRESS = crate::Reg<sub_address::SUB_ADDRESS_SPEC>;
#[doc = "Register address of slave device"]
pub mod sub_address;
#[doc = "bus_busy (rw) register accessor: an alias for `Reg<BUS_BUSY_SPEC>`"]
pub type BUS_BUSY = crate::Reg<bus_busy::BUS_BUSY_SPEC>;
#[doc = "Bus busy state indicator"]
pub mod bus_busy;
#[doc = "period_start (rw) register accessor: an alias for `Reg<PERIOD_START_SPEC>`"]
pub type PERIOD_START = crate::Reg<period_start::PERIOD_START_SPEC>;
#[doc = "Duration of start phase"]
pub mod period_start;
#[doc = "period_stop (rw) register accessor: an alias for `Reg<PERIOD_STOP_SPEC>`"]
pub type PERIOD_STOP = crate::Reg<period_stop::PERIOD_STOP_SPEC>;
#[doc = "Duration of stop phase"]
pub mod period_stop;
#[doc = "period_data (rw) register accessor: an alias for `Reg<PERIOD_DATA_SPEC>`"]
pub type PERIOD_DATA = crate::Reg<period_data::PERIOD_DATA_SPEC>;
#[doc = "Duration of data phase"]
pub mod period_data;
#[doc = "fifo_config_0 (rw) register accessor: an alias for `Reg<FIFO_CONFIG_0_SPEC>`"]
pub type FIFO_CONFIG_0 = crate::Reg<fifo_config_0::FIFO_CONFIG_0_SPEC>;
#[doc = "FIFO configuration register 0"]
pub mod fifo_config_0;
#[doc = "fifo_config_1 (rw) register accessor: an alias for `Reg<FIFO_CONFIG_1_SPEC>`"]
pub type FIFO_CONFIG_1 = crate::Reg<fifo_config_1::FIFO_CONFIG_1_SPEC>;
#[doc = "FIFO configuration register 1"]
pub mod fifo_config_1;
#[doc = "data_write (w) register accessor: an alias for `Reg<DATA_WRITE_SPEC>`"]
pub type DATA_WRITE = crate::Reg<data_write::DATA_WRITE_SPEC>;
#[doc = "FIFO write data register"]
pub mod data_write;
#[doc = "data_read (r) register accessor: an alias for `Reg<DATA_READ_SPEC>`"]
pub type DATA_READ = crate::Reg<data_read::DATA_READ_SPEC>;
#[doc = "FIFO read data register"]
pub mod data_read;
