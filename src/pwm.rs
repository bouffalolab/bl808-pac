#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt state and clear register"]
    pub interrupt_config: INTERRUPT_CONFIG,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - Pulse-Width Modulation channel group"]
    pub group: crate::ArrayProxy<GROUP, 2, 0x40>,
}
#[doc = "interrupt_config (rw) register accessor: an alias for `Reg<INTERRUPT_CONFIG_SPEC>`"]
pub type INTERRUPT_CONFIG = crate::Reg<interrupt_config::INTERRUPT_CONFIG_SPEC>;
#[doc = "Interrupt state and clear register"]
pub mod interrupt_config;
#[doc = "Pulse-Width Modulation channel group"]
pub use self::group::GROUP;
#[doc = r"Cluster"]
#[doc = "Pulse-Width Modulation channel group"]
pub mod group;
