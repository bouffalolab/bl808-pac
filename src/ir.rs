#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ??"]
    pub transmit_config: TRANSMIT_CONFIG,
    #[doc = "0x04 - ??"]
    pub transmit_interrupt: TRANSMIT_INTERRUPT,
    #[doc = "0x08..0x10 - ??"]
    pub transmit_data: [TRANSMIT_DATA; 2],
    #[doc = "0x10 - ??"]
    pub transmit_width: TRANSMIT_WIDTH,
    _reserved4: [u8; 0x6c],
    #[doc = "0x80 - ??"]
    pub receive_config: RECEIVE_CONFIG,
    #[doc = "0x84 - ??"]
    pub receive_interrupt: RECEIVE_INTERRUPT,
    #[doc = "0x88 - ??"]
    pub receive_width: RECEIVE_WIDTH,
    _reserved7: [u8; 0x04],
    #[doc = "0x90 - ??"]
    pub receive_bit_count: RECEIVE_BIT_COUNT,
    #[doc = "0x94..0x9c - ??"]
    pub receive_data: [RECEIVE_DATA; 2],
}
#[doc = "transmit_config (rw) register accessor: an alias for `Reg<TRANSMIT_CONFIG_SPEC>`"]
pub type TRANSMIT_CONFIG = crate::Reg<transmit_config::TRANSMIT_CONFIG_SPEC>;
#[doc = "??"]
pub mod transmit_config;
#[doc = "transmit_interrupt (rw) register accessor: an alias for `Reg<TRANSMIT_INTERRUPT_SPEC>`"]
pub type TRANSMIT_INTERRUPT = crate::Reg<transmit_interrupt::TRANSMIT_INTERRUPT_SPEC>;
#[doc = "??"]
pub mod transmit_interrupt;
#[doc = "transmit_data (rw) register accessor: an alias for `Reg<TRANSMIT_DATA_SPEC>`"]
pub type TRANSMIT_DATA = crate::Reg<transmit_data::TRANSMIT_DATA_SPEC>;
#[doc = "??"]
pub mod transmit_data;
#[doc = "transmit_width (rw) register accessor: an alias for `Reg<TRANSMIT_WIDTH_SPEC>`"]
pub type TRANSMIT_WIDTH = crate::Reg<transmit_width::TRANSMIT_WIDTH_SPEC>;
#[doc = "??"]
pub mod transmit_width;
#[doc = "receive_config (rw) register accessor: an alias for `Reg<RECEIVE_CONFIG_SPEC>`"]
pub type RECEIVE_CONFIG = crate::Reg<receive_config::RECEIVE_CONFIG_SPEC>;
#[doc = "??"]
pub mod receive_config;
#[doc = "receive_interrupt (rw) register accessor: an alias for `Reg<RECEIVE_INTERRUPT_SPEC>`"]
pub type RECEIVE_INTERRUPT = crate::Reg<receive_interrupt::RECEIVE_INTERRUPT_SPEC>;
#[doc = "??"]
pub mod receive_interrupt;
#[doc = "receive_width (rw) register accessor: an alias for `Reg<RECEIVE_WIDTH_SPEC>`"]
pub type RECEIVE_WIDTH = crate::Reg<receive_width::RECEIVE_WIDTH_SPEC>;
#[doc = "??"]
pub mod receive_width;
#[doc = "receive_bit_count (rw) register accessor: an alias for `Reg<RECEIVE_BIT_COUNT_SPEC>`"]
pub type RECEIVE_BIT_COUNT = crate::Reg<receive_bit_count::RECEIVE_BIT_COUNT_SPEC>;
#[doc = "??"]
pub mod receive_bit_count;
#[doc = "receive_data (rw) register accessor: an alias for `Reg<RECEIVE_DATA_SPEC>`"]
pub type RECEIVE_DATA = crate::Reg<receive_data::RECEIVE_DATA_SPEC>;
#[doc = "??"]
pub mod receive_data;
