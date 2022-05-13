#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - Unique module identifier"]
    pub identify: [IDENTIFY; 2],
    #[doc = "0x08..0x18 - Password of debug module"]
    pub password: [PASSWORD; 4],
    #[doc = "0x18 - Module control register"]
    pub control: CONTROL,
}
#[doc = "identify (r) register accessor: an alias for `Reg<IDENTIFY_SPEC>`"]
pub type IDENTIFY = crate::Reg<identify::IDENTIFY_SPEC>;
#[doc = "Unique module identifier"]
pub mod identify;
#[doc = "password (rw) register accessor: an alias for `Reg<PASSWORD_SPEC>`"]
pub type PASSWORD = crate::Reg<password::PASSWORD_SPEC>;
#[doc = "Password of debug module"]
pub mod password;
#[doc = "control (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Module control register"]
pub mod control;
