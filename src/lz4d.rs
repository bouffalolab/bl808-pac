#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Decompressor peripheral configuration"]
    pub config: CONFIG,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Writes source address before decompression"]
    pub source_start: SOURCE_START,
    #[doc = "0x14 - Reads the end address of source after decompression"]
    pub source_end: SOURCE_END,
    #[doc = "0x18 - Writes destination address before decompression"]
    pub destination_start: DESTINATION_START,
    #[doc = "0x1c - Reads the end address of destination after decompression"]
    pub destination_end: DESTINATION_END,
    #[doc = "0x20 - Interrupt enable register"]
    pub interrupt_enable: INTERRUPT_ENABLE,
    #[doc = "0x24 - Interrupt state register"]
    pub interrupt_state: INTERRUPT_STATE,
}
#[doc = "config (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Decompressor peripheral configuration"]
pub mod config;
#[doc = "source_start (rw) register accessor: an alias for `Reg<SOURCE_START_SPEC>`"]
pub type SOURCE_START = crate::Reg<source_start::SOURCE_START_SPEC>;
#[doc = "Writes source address before decompression"]
pub mod source_start;
#[doc = "source_end (r) register accessor: an alias for `Reg<SOURCE_END_SPEC>`"]
pub type SOURCE_END = crate::Reg<source_end::SOURCE_END_SPEC>;
#[doc = "Reads the end address of source after decompression"]
pub mod source_end;
#[doc = "destination_start (rw) register accessor: an alias for `Reg<DESTINATION_START_SPEC>`"]
pub type DESTINATION_START = crate::Reg<destination_start::DESTINATION_START_SPEC>;
#[doc = "Writes destination address before decompression"]
pub mod destination_start;
#[doc = "destination_end (r) register accessor: an alias for `Reg<DESTINATION_END_SPEC>`"]
pub type DESTINATION_END = crate::Reg<destination_end::DESTINATION_END_SPEC>;
#[doc = "Reads the end address of destination after decompression"]
pub mod destination_end;
#[doc = "interrupt_enable (rw) register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
#[doc = "interrupt_state (r) register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt state register"]
pub mod interrupt_state;
