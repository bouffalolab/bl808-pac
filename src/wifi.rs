#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - ??"]
    pub coex_control_0: COEX_CONTROL_0,
    #[doc = "0x404 - ??"]
    pub coex_pti: COEX_PTI,
    #[doc = "0x408 - ??"]
    pub coex_state: COEX_STATE,
    #[doc = "0x40c - ??"]
    pub coex_interrupt: COEX_INTERRUPT,
    #[doc = "0x410 - ??"]
    pub coex_control_1: COEX_CONTROL_1,
}
#[doc = "coex_control_0 (rw) register accessor: an alias for `Reg<COEX_CONTROL_0_SPEC>`"]
pub type COEX_CONTROL_0 = crate::Reg<coex_control_0::COEX_CONTROL_0_SPEC>;
#[doc = "??"]
pub mod coex_control_0;
#[doc = "coex_pti (rw) register accessor: an alias for `Reg<COEX_PTI_SPEC>`"]
pub type COEX_PTI = crate::Reg<coex_pti::COEX_PTI_SPEC>;
#[doc = "??"]
pub mod coex_pti;
#[doc = "coex_state (rw) register accessor: an alias for `Reg<COEX_STATE_SPEC>`"]
pub type COEX_STATE = crate::Reg<coex_state::COEX_STATE_SPEC>;
#[doc = "??"]
pub mod coex_state;
#[doc = "coex_interrupt (rw) register accessor: an alias for `Reg<COEX_INTERRUPT_SPEC>`"]
pub type COEX_INTERRUPT = crate::Reg<coex_interrupt::COEX_INTERRUPT_SPEC>;
#[doc = "??"]
pub mod coex_interrupt;
#[doc = "coex_control_1 (rw) register accessor: an alias for `Reg<COEX_CONTROL_1_SPEC>`"]
pub type COEX_CONTROL_1 = crate::Reg<coex_control_1::COEX_CONTROL_1_SPEC>;
#[doc = "??"]
pub mod coex_control_1;
