#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host Controller Capability Registers"]
    pub capability: CAPABILITY,
    _reserved1: [u8; 0x10],
    #[doc = "0x10 - Host Controller Operational Registers"]
    pub operation: OPERATION,
}
#[doc = "Host Controller Capability Registers"]
pub use self::capability::CAPABILITY;
#[doc = r"Cluster"]
#[doc = "Host Controller Capability Registers"]
pub mod capability;
#[doc = "Host Controller Operational Registers"]
pub use self::operation::OPERATION;
#[doc = r"Cluster"]
#[doc = "Host Controller Operational Registers"]
pub mod operation;
