#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - Interrupt enables, masks and states"]
    pub interrupt_state: INTERRUPT_STATE,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Base clock divider"]
    pub base_clock: BASE_CLOCK,
    _reserved3: [u8; 0x6c],
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
#[doc = "interrupt_state (rw) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt enables, masks and states"]
pub mod interrupt_state;
#[doc = "base_clock (rw) register accessor: an alias for `Reg<BASE_CLOCK_SPEC>`"]
pub type BASE_CLOCK = crate::Reg<base_clock::BASE_CLOCK_SPEC>;
#[doc = "Base clock divider"]
pub mod base_clock;
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
