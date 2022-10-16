#[doc = r"Register block"]
#[repr(C)]
pub struct GROUP {
    #[doc = "0x00 - Peripheral group configuration"]
    pub config: CONFIG,
    #[doc = "0x04 - Channel configuration register"]
    pub channel: CHANNEL,
    #[doc = "0x08 - Pulse clock period register"]
    pub period: PERIOD,
    #[doc = "0x0c - Dead time for each channel"]
    pub dead_time: DEAD_TIME,
    #[doc = "0x10..0x20 - Channel internal counter threshold"]
    pub threshold: [THRESHOLD; 4],
    #[doc = "0x20 - Interrupt state register"]
    pub interrupt_state: INTERRUPT_STATE,
    #[doc = "0x24 - Interrupt mask register"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x28 - Clear interrupt register"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x2c - Interrupt enable register"]
    pub interrupt_enable: INTERRUPT_ENABLE,
}
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Peripheral group configuration"]
pub mod config;
#[doc = "channel (rw) register accessor: an alias for `Reg<CHANNEL_SPEC>`"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel configuration register"]
pub mod channel;
#[doc = "period (rw) register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Pulse clock period register"]
pub mod period;
#[doc = "dead_time (rw) register accessor: an alias for `Reg<DEAD_TIME_SPEC>`"]
pub type DEAD_TIME = crate::Reg<dead_time::DEAD_TIME_SPEC>;
#[doc = "Dead time for each channel"]
pub mod dead_time;
#[doc = "threshold (rw) register accessor: an alias for `Reg<THRESHOLD_SPEC>`"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Channel internal counter threshold"]
pub mod threshold;
#[doc = "interrupt_state (r) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask (rw) register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear (w) register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear interrupt register"]
pub mod interrupt_clear;
#[doc = "interrupt_enable (rw) register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
