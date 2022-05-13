#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0a00],
    #[doc = "0xa00 - ??"]
    pub touch_config: TOUCH_CONFIG,
    #[doc = "0xa04 - ??"]
    pub touch_channel: TOUCH_CHANNEL,
    #[doc = "0xa08 - ??"]
    pub touch_process: TOUCH_PROCESS,
    #[doc = "0xa0c - ??"]
    pub touch_sleep: TOUCH_SLEEP,
    #[doc = "0xa10 - ??"]
    pub touch_delay: TOUCH_DELAY,
    #[doc = "0xa14..0xa2c - ??"]
    pub touch_force: [TOUCH_FORCE; 6],
    #[doc = "0xa2c..0xa38 - ??"]
    pub touch_voltage: [TOUCH_VOLTAGE; 3],
    #[doc = "0xa38..0xa68 - ??"]
    pub touch_raw: [TOUCH_RAW; 12],
    _reserved8: [u8; 0x64],
    #[doc = "0xacc - ??"]
    pub touch_interrupt_0: TOUCH_INTERRUPT_0,
    #[doc = "0xad0 - ??"]
    pub touch_interrupt_1: TOUCH_INTERRUPT_1,
}
#[doc = "touch_config (rw) register accessor: an alias for `Reg<TOUCH_CONFIG_SPEC>`"]
pub type TOUCH_CONFIG = crate::Reg<touch_config::TOUCH_CONFIG_SPEC>;
#[doc = "??"]
pub mod touch_config;
#[doc = "touch_channel (rw) register accessor: an alias for `Reg<TOUCH_CHANNEL_SPEC>`"]
pub type TOUCH_CHANNEL = crate::Reg<touch_channel::TOUCH_CHANNEL_SPEC>;
#[doc = "??"]
pub mod touch_channel;
#[doc = "touch_process (rw) register accessor: an alias for `Reg<TOUCH_PROCESS_SPEC>`"]
pub type TOUCH_PROCESS = crate::Reg<touch_process::TOUCH_PROCESS_SPEC>;
#[doc = "??"]
pub mod touch_process;
#[doc = "touch_sleep (rw) register accessor: an alias for `Reg<TOUCH_SLEEP_SPEC>`"]
pub type TOUCH_SLEEP = crate::Reg<touch_sleep::TOUCH_SLEEP_SPEC>;
#[doc = "??"]
pub mod touch_sleep;
#[doc = "touch_delay (rw) register accessor: an alias for `Reg<TOUCH_DELAY_SPEC>`"]
pub type TOUCH_DELAY = crate::Reg<touch_delay::TOUCH_DELAY_SPEC>;
#[doc = "??"]
pub mod touch_delay;
#[doc = "touch_force (rw) register accessor: an alias for `Reg<TOUCH_FORCE_SPEC>`"]
pub type TOUCH_FORCE = crate::Reg<touch_force::TOUCH_FORCE_SPEC>;
#[doc = "??"]
pub mod touch_force;
#[doc = "touch_voltage (rw) register accessor: an alias for `Reg<TOUCH_VOLTAGE_SPEC>`"]
pub type TOUCH_VOLTAGE = crate::Reg<touch_voltage::TOUCH_VOLTAGE_SPEC>;
#[doc = "??"]
pub mod touch_voltage;
#[doc = "touch_raw (rw) register accessor: an alias for `Reg<TOUCH_RAW_SPEC>`"]
pub type TOUCH_RAW = crate::Reg<touch_raw::TOUCH_RAW_SPEC>;
#[doc = "??"]
pub mod touch_raw;
#[doc = "touch_interrupt_0 (rw) register accessor: an alias for `Reg<TOUCH_INTERRUPT_0_SPEC>`"]
pub type TOUCH_INTERRUPT_0 = crate::Reg<touch_interrupt_0::TOUCH_INTERRUPT_0_SPEC>;
#[doc = "??"]
pub mod touch_interrupt_0;
#[doc = "touch_interrupt_1 (rw) register accessor: an alias for `Reg<TOUCH_INTERRUPT_1_SPEC>`"]
pub type TOUCH_INTERRUPT_1 = crate::Reg<touch_interrupt_1::TOUCH_INTERRUPT_1_SPEC>;
#[doc = "??"]
pub mod touch_interrupt_1;
