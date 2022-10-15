#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Peripheral silicon revision"]
    pub revision: REVISION,
    #[doc = "0x404 - Arbitration configuration register 0"]
    pub config_0: CONFIG_0,
    #[doc = "0x408 - ??"]
    pub bluetooth_transmit: BLUETOOTH_TRANSMIT,
    _reserved3: [u8; 0x04],
    #[doc = "0x410 - ??"]
    pub bluetooth_receive: BLUETOOTH_RECEIVE,
    _reserved4: [u8; 0x04],
    #[doc = "0x418 - ??"]
    pub wifi_transmit: WIFI_TRANSMIT,
    _reserved5: [u8; 0x04],
    #[doc = "0x420 - ??"]
    pub wifi_receive: WIFI_RECEIVE,
    _reserved6: [u8; 0x04],
    #[doc = "0x428 - ??"]
    pub config_1: CONFIG_1,
}
#[doc = "revision (rw) register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Peripheral silicon revision"]
pub mod revision;
#[doc = "config_0 (rw) register accessor: an alias for `Reg<CONFIG_0_SPEC>`"]
pub type CONFIG_0 = crate::Reg<config_0::CONFIG_0_SPEC>;
#[doc = "Arbitration configuration register 0"]
pub mod config_0;
#[doc = "bluetooth_transmit (rw) register accessor: an alias for `Reg<BLUETOOTH_TRANSMIT_SPEC>`"]
pub type BLUETOOTH_TRANSMIT = crate::Reg<bluetooth_transmit::BLUETOOTH_TRANSMIT_SPEC>;
#[doc = "??"]
pub mod bluetooth_transmit;
#[doc = "bluetooth_receive (rw) register accessor: an alias for `Reg<BLUETOOTH_RECEIVE_SPEC>`"]
pub type BLUETOOTH_RECEIVE = crate::Reg<bluetooth_receive::BLUETOOTH_RECEIVE_SPEC>;
#[doc = "??"]
pub mod bluetooth_receive;
#[doc = "wifi_transmit (rw) register accessor: an alias for `Reg<WIFI_TRANSMIT_SPEC>`"]
pub type WIFI_TRANSMIT = crate::Reg<wifi_transmit::WIFI_TRANSMIT_SPEC>;
#[doc = "??"]
pub mod wifi_transmit;
#[doc = "wifi_receive (rw) register accessor: an alias for `Reg<WIFI_RECEIVE_SPEC>`"]
pub type WIFI_RECEIVE = crate::Reg<wifi_receive::WIFI_RECEIVE_SPEC>;
#[doc = "??"]
pub mod wifi_receive;
#[doc = "config_1 (rw) register accessor: an alias for `Reg<CONFIG_1_SPEC>`"]
pub type CONFIG_1 = crate::Reg<config_1::CONFIG_1_SPEC>;
#[doc = "??"]
pub mod config_1;
