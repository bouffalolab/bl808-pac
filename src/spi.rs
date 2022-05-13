#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - Interrupt enables, masks and states"]
    pub interrupt_state: INTERRUPT_STATE,
    #[doc = "0x08 - Bus busy state indicator"]
    pub bus_busy: BUS_BUSY,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Duration of control signals"]
    pub period_control: PERIOD_CONTROL,
    #[doc = "0x14 - Interval bitween frames"]
    pub period_interval: PERIOD_INTERVAL,
    #[doc = "0x18 - Receive ignore index configuration"]
    pub ignore_index: IGNORE_INDEX,
    #[doc = "0x1c - Slave mode transmit timeout values"]
    pub timeout: TIMEOUT,
    _reserved7: [u8; 0x60],
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
#[doc = "bus_busy (rw) register accessor: an alias for `Reg<BUS_BUSY_SPEC>`"]
pub type BUS_BUSY = crate::Reg<bus_busy::BUS_BUSY_SPEC>;
#[doc = "Bus busy state indicator"]
pub mod bus_busy;
#[doc = "period_control (rw) register accessor: an alias for `Reg<PERIOD_CONTROL_SPEC>`"]
pub type PERIOD_CONTROL = crate::Reg<period_control::PERIOD_CONTROL_SPEC>;
#[doc = "Duration of control signals"]
pub mod period_control;
#[doc = "period_interval (rw) register accessor: an alias for `Reg<PERIOD_INTERVAL_SPEC>`"]
pub type PERIOD_INTERVAL = crate::Reg<period_interval::PERIOD_INTERVAL_SPEC>;
#[doc = "Interval bitween frames"]
pub mod period_interval;
#[doc = "ignore_index (rw) register accessor: an alias for `Reg<IGNORE_INDEX_SPEC>`"]
pub type IGNORE_INDEX = crate::Reg<ignore_index::IGNORE_INDEX_SPEC>;
#[doc = "Receive ignore index configuration"]
pub mod ignore_index;
#[doc = "timeout (rw) register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Slave mode transmit timeout values"]
pub mod timeout;
#[doc = "fifo_config_0 (rw) register accessor: an alias for `Reg<FIFO_CONFIG_0_SPEC>`"]
pub type FIFO_CONFIG_0 = crate::Reg<fifo_config_0::FIFO_CONFIG_0_SPEC>;
#[doc = "FIFO configuration register 0"]
pub mod fifo_config_0;
#[doc = "fifo_config_1 (rw) register accessor: an alias for `Reg<FIFO_CONFIG_1_SPEC>`"]
pub type FIFO_CONFIG_1 = crate::Reg<fifo_config_1::FIFO_CONFIG_1_SPEC>;
#[doc = "FIFO configuration register 1"]
pub mod fifo_config_1;
#[doc = "data_write (rw) register accessor: an alias for `Reg<DATA_WRITE_SPEC>`"]
pub type DATA_WRITE = crate::Reg<data_write::DATA_WRITE_SPEC>;
#[doc = "FIFO write data register"]
pub mod data_write;
#[doc = "data_read (rw) register accessor: an alias for `Reg<DATA_READ_SPEC>`"]
pub type DATA_READ = crate::Reg<data_read::DATA_READ_SPEC>;
#[doc = "FIFO read data register"]
pub mod data_read;
