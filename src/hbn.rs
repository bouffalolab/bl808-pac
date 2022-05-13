#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Miscellaneous control register"]
    pub control: CONTROL,
    #[doc = "0x04 - Low bits of hibernate time"]
    pub time_lo: TIME_LO,
    #[doc = "0x08 - High bits of hibernate time"]
    pub time_hi: TIME_HI,
    #[doc = "0x0c - Low bits of Real-Time Clock time"]
    pub rtc_time_lo: RTC_TIME_LO,
    #[doc = "0x10 - High bits of Real-Time Clock time"]
    pub rtc_time_hi: RTC_TIME_HI,
    #[doc = "0x14 - Hibernate interrupt contol"]
    pub interrupt_mode: INTERRUPT_MODE,
    #[doc = "0x18 - Hibernate interrupt state"]
    pub interrupt_state: INTERRUPT_STATE,
    #[doc = "0x1c - Clear hibernate interrupt"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - Global hibernate configuration"]
    pub global: GLOBAL,
    #[doc = "0x34 - Static Random-Access Memory hibernate control"]
    pub sram: SRAM,
    _reserved10: [u8; 0x01c8],
    #[doc = "0x200 - 32-kHz internal RC oscillator control"]
    pub rc32k: RC32K,
    #[doc = "0x204 - External crystal oscillator control"]
    pub xtal32k: XTAL32K,
    #[doc = "0x208 - Real-Time Clock control and reset register 0"]
    pub rtc_control_0: RTC_CONTROL_0,
    #[doc = "0x20c - Real-Time Clock control and reset register 1"]
    pub rtc_control_1: RTC_CONTROL_1,
}
#[doc = "control (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Miscellaneous control register"]
pub mod control;
#[doc = "time_lo (rw) register accessor: an alias for `Reg<TIME_LO_SPEC>`"]
pub type TIME_LO = crate::Reg<time_lo::TIME_LO_SPEC>;
#[doc = "Low bits of hibernate time"]
pub mod time_lo;
#[doc = "time_hi (rw) register accessor: an alias for `Reg<TIME_HI_SPEC>`"]
pub type TIME_HI = crate::Reg<time_hi::TIME_HI_SPEC>;
#[doc = "High bits of hibernate time"]
pub mod time_hi;
#[doc = "rtc_time_lo (rw) register accessor: an alias for `Reg<RTC_TIME_LO_SPEC>`"]
pub type RTC_TIME_LO = crate::Reg<rtc_time_lo::RTC_TIME_LO_SPEC>;
#[doc = "Low bits of Real-Time Clock time"]
pub mod rtc_time_lo;
#[doc = "rtc_time_hi (rw) register accessor: an alias for `Reg<RTC_TIME_HI_SPEC>`"]
pub type RTC_TIME_HI = crate::Reg<rtc_time_hi::RTC_TIME_HI_SPEC>;
#[doc = "High bits of Real-Time Clock time"]
pub mod rtc_time_hi;
#[doc = "interrupt_mode (rw) register accessor: an alias for `Reg<INTERRUPT_MODE_SPEC>`"]
pub type INTERRUPT_MODE = crate::Reg<interrupt_mode::INTERRUPT_MODE_SPEC>;
#[doc = "Hibernate interrupt contol"]
pub mod interrupt_mode;
#[doc = "interrupt_state (rw) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Hibernate interrupt state"]
pub mod interrupt_state;
#[doc = "interrupt_clear (rw) register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear hibernate interrupt"]
pub mod interrupt_clear;
#[doc = "global (rw) register accessor: an alias for `Reg<GLOBAL_SPEC>`"]
pub type GLOBAL = crate::Reg<global::GLOBAL_SPEC>;
#[doc = "Global hibernate configuration"]
pub mod global;
#[doc = "sram (rw) register accessor: an alias for `Reg<SRAM_SPEC>`"]
pub type SRAM = crate::Reg<sram::SRAM_SPEC>;
#[doc = "Static Random-Access Memory hibernate control"]
pub mod sram;
#[doc = "rc32k (rw) register accessor: an alias for `Reg<RC32K_SPEC>`"]
pub type RC32K = crate::Reg<rc32k::RC32K_SPEC>;
#[doc = "32-kHz internal RC oscillator control"]
pub mod rc32k;
#[doc = "xtal32k (rw) register accessor: an alias for `Reg<XTAL32K_SPEC>`"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "External crystal oscillator control"]
pub mod xtal32k;
#[doc = "rtc_control_0 (rw) register accessor: an alias for `Reg<RTC_CONTROL_0_SPEC>`"]
pub type RTC_CONTROL_0 = crate::Reg<rtc_control_0::RTC_CONTROL_0_SPEC>;
#[doc = "Real-Time Clock control and reset register 0"]
pub mod rtc_control_0;
#[doc = "rtc_control_1 (rw) register accessor: an alias for `Reg<RTC_CONTROL_1_SPEC>`"]
pub type RTC_CONTROL_1 = crate::Reg<rtc_control_1::RTC_CONTROL_1_SPEC>;
#[doc = "Real-Time Clock control and reset register 1"]
pub mod rtc_control_1;
