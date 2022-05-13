#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ??"]
    pub todo: crate::Reg<todo::TODO_SPEC>,
}
#[doc = "todo register accessor: an alias for `Reg<TODO_SPEC>`"]
pub type TODO = crate::Reg<todo::TODO_SPEC>;
#[doc = "??"]
pub mod todo;
