#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interface enables and configurations"]
    pub mode: MODE,
    #[doc = "0x04 - Interrupt source register"]
    pub interrupt_source: INTERRUPT_SOURCE,
    #[doc = "0x08 - Interrupt mask register"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x0c - Back-to-back inter-packet gap register"]
    pub backed_gap: BACKED_GAP,
    #[doc = "0x10 - Non back-to-back inter-packet gap register 1"]
    pub non_backed_gap_1: NON_BACKED_GAP_1,
    #[doc = "0x14 - Non back-to-back inter-packet gap register 2"]
    pub non_backed_gap_2: NON_BACKED_GAP_2,
    #[doc = "0x18 - Minimum and maximum ethernet frame length"]
    pub frame_length: FRAME_LENGTH,
    #[doc = "0x1c - Collision time window and maximum retries"]
    pub collision: COLLISION,
    #[doc = "0x20 - Transmit buffer descriptor"]
    pub transmit_buffer: TRANSMIT_BUFFER,
    #[doc = "0x24 - Control frame function register"]
    pub flow_control: FLOW_CONTROL,
    #[doc = "0x28 - MII clock divider and premable enable"]
    pub mii_mode: MII_MODE,
    #[doc = "0x2c - MII control data, read and scan state"]
    pub mii_command: MII_COMMAND,
    #[doc = "0x30 - Physical layer bus address"]
    pub mii_address: MII_ADDRESS,
    #[doc = "0x34 - Write data to MII physcial layer"]
    pub control_write: CONTROL_WRITE,
    #[doc = "0x38 - Read data from MII physcial layer"]
    pub control_read: CONTROL_READ,
    #[doc = "0x3c - MII bus and link layer state"]
    pub mii_state: MII_STATE,
    #[doc = "0x40..0x48 - Media Access Control address"]
    pub mac_address: [MAC_ADDRESS; 2],
    #[doc = "0x48..0x50 - Hash register"]
    pub hash: [HASH; 2],
    #[doc = "0x50 - Transmit control register"]
    pub transmit_control: TRANSMIT_CONTROL,
}
#[doc = "mode (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Interface enables and configurations"]
pub mod mode;
#[doc = "interrupt_source (rw) register accessor: an alias for `Reg<INTERRUPT_SOURCE_SPEC>`"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "Interrupt source register"]
pub mod interrupt_source;
#[doc = "interrupt_mask (rw) register accessor: an alias for `Reg<INTERRUPT_MASK_SPEC>`"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod interrupt_mask;
#[doc = "backed_gap (rw) register accessor: an alias for `Reg<BACKED_GAP_SPEC>`"]
pub type BACKED_GAP = crate::Reg<backed_gap::BACKED_GAP_SPEC>;
#[doc = "Back-to-back inter-packet gap register"]
pub mod backed_gap;
#[doc = "non_backed_gap_1 (rw) register accessor: an alias for `Reg<NON_BACKED_GAP_1_SPEC>`"]
pub type NON_BACKED_GAP_1 = crate::Reg<non_backed_gap_1::NON_BACKED_GAP_1_SPEC>;
#[doc = "Non back-to-back inter-packet gap register 1"]
pub mod non_backed_gap_1;
#[doc = "non_backed_gap_2 (rw) register accessor: an alias for `Reg<NON_BACKED_GAP_2_SPEC>`"]
pub type NON_BACKED_GAP_2 = crate::Reg<non_backed_gap_2::NON_BACKED_GAP_2_SPEC>;
#[doc = "Non back-to-back inter-packet gap register 2"]
pub mod non_backed_gap_2;
#[doc = "frame_length (rw) register accessor: an alias for `Reg<FRAME_LENGTH_SPEC>`"]
pub type FRAME_LENGTH = crate::Reg<frame_length::FRAME_LENGTH_SPEC>;
#[doc = "Minimum and maximum ethernet frame length"]
pub mod frame_length;
#[doc = "collision (rw) register accessor: an alias for `Reg<COLLISION_SPEC>`"]
pub type COLLISION = crate::Reg<collision::COLLISION_SPEC>;
#[doc = "Collision time window and maximum retries"]
pub mod collision;
#[doc = "transmit_buffer (rw) register accessor: an alias for `Reg<TRANSMIT_BUFFER_SPEC>`"]
pub type TRANSMIT_BUFFER = crate::Reg<transmit_buffer::TRANSMIT_BUFFER_SPEC>;
#[doc = "Transmit buffer descriptor"]
pub mod transmit_buffer;
#[doc = "flow_control (rw) register accessor: an alias for `Reg<FLOW_CONTROL_SPEC>`"]
pub type FLOW_CONTROL = crate::Reg<flow_control::FLOW_CONTROL_SPEC>;
#[doc = "Control frame function register"]
pub mod flow_control;
#[doc = "mii_mode (rw) register accessor: an alias for `Reg<MII_MODE_SPEC>`"]
pub type MII_MODE = crate::Reg<mii_mode::MII_MODE_SPEC>;
#[doc = "MII clock divider and premable enable"]
pub mod mii_mode;
#[doc = "mii_command (rw) register accessor: an alias for `Reg<MII_COMMAND_SPEC>`"]
pub type MII_COMMAND = crate::Reg<mii_command::MII_COMMAND_SPEC>;
#[doc = "MII control data, read and scan state"]
pub mod mii_command;
#[doc = "mii_address (rw) register accessor: an alias for `Reg<MII_ADDRESS_SPEC>`"]
pub type MII_ADDRESS = crate::Reg<mii_address::MII_ADDRESS_SPEC>;
#[doc = "Physical layer bus address"]
pub mod mii_address;
#[doc = "control_write (rw) register accessor: an alias for `Reg<CONTROL_WRITE_SPEC>`"]
pub type CONTROL_WRITE = crate::Reg<control_write::CONTROL_WRITE_SPEC>;
#[doc = "Write data to MII physcial layer"]
pub mod control_write;
#[doc = "control_read (rw) register accessor: an alias for `Reg<CONTROL_READ_SPEC>`"]
pub type CONTROL_READ = crate::Reg<control_read::CONTROL_READ_SPEC>;
#[doc = "Read data from MII physcial layer"]
pub mod control_read;
#[doc = "mii_state (rw) register accessor: an alias for `Reg<MII_STATE_SPEC>`"]
pub type MII_STATE = crate::Reg<mii_state::MII_STATE_SPEC>;
#[doc = "MII bus and link layer state"]
pub mod mii_state;
#[doc = "mac_address (rw) register accessor: an alias for `Reg<MAC_ADDRESS_SPEC>`"]
pub type MAC_ADDRESS = crate::Reg<mac_address::MAC_ADDRESS_SPEC>;
#[doc = "Media Access Control address"]
pub mod mac_address;
#[doc = "hash (rw) register accessor: an alias for `Reg<HASH_SPEC>`"]
pub type HASH = crate::Reg<hash::HASH_SPEC>;
#[doc = "Hash register"]
pub mod hash;
#[doc = "transmit_control (rw) register accessor: an alias for `Reg<TRANSMIT_CONTROL_SPEC>`"]
pub type TRANSMIT_CONTROL = crate::Reg<transmit_control::TRANSMIT_CONTROL_SPEC>;
#[doc = "Transmit control register"]
pub mod transmit_control;
