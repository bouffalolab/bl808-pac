#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register 0"]
    pub config_0: CONFIG_0,
    #[doc = "0x04 - Configuration register 1"]
    pub config_1: CONFIG_1,
    #[doc = "0x08 - Cycle duration in clock source cycles"]
    pub period: PERIOD,
    #[doc = "0x0c - ??"]
    pub dead_time: DEAD_TIME,
    #[doc = "0x10..0x20 - ??"]
    pub channel_threshold: [CHANNEL_THRESHOLD; 4],
    #[doc = "0x20 - Interrut state register"]
    pub interrupt_state: INTERRUPT_STATE,
    #[doc = "0x24 - Interrut mask register"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x28 - Clear interrupt register"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x2c - Interrupt enable register"]
    pub interrupt_enable: INTERRUPT_ENABLE,
}
#[doc = "config_0 (rw) register accessor: an alias for `Reg<CONFIG_0_SPEC>`"]
pub type CONFIG_0 = crate::Reg<config_0::CONFIG_0_SPEC>;
#[doc = "Configuration register 0"]
pub mod config_0;
#[doc = "config_1 (rw) register accessor: an alias for `Reg<CONFIG_1_SPEC>`"]
pub type CONFIG_1 = crate::Reg<config_1::CONFIG_1_SPEC>;
#[doc = "Configuration register 1"]
pub mod config_1;
#[doc = "period (rw) register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Cycle duration in clock source cycles"]
pub mod period;
#[doc = "dead_time (rw) register accessor: an alias for `Reg<DEAD_TIME_SPEC>`"]
pub type DEAD_TIME = crate::Reg<dead_time::DEAD_TIME_SPEC>;
#[doc = "??"]
pub mod dead_time;
#[doc = "channel_threshold (rw) register accessor: an alias for `Reg<CHANNEL_THRESHOLD_SPEC>`"]
pub type CHANNEL_THRESHOLD = crate::Reg<channel_threshold::CHANNEL_THRESHOLD_SPEC>;
#[doc = "??"]
pub mod channel_threshold;
#[doc = "interrupt_state (rw) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrut state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask (rw) register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrut mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear (rw) register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear interrupt register"]
pub mod interrupt_clear;
#[doc = "interrupt_enable (rw) register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
