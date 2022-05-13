#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip information register"]
    pub chip_inform: CHIP_INFORM,
    _reserved1: [u8; 0x4c],
    #[doc = "0x50 - Core configuration register 16"]
    pub core_config_16: CORE_CONFIG_16,
    #[doc = "0x54 - Core configuration register 17"]
    pub core_config_17: CORE_CONFIG_17,
    #[doc = "0x58 - Core configuration register 18"]
    pub core_config_18: CORE_CONFIG_18,
    #[doc = "0x5c - Core configuration register 19"]
    pub core_config_19: CORE_CONFIG_19,
    #[doc = "0x60 - Core configuration register 20"]
    pub core_config_20: CORE_CONFIG_20,
    #[doc = "0x64 - Core configuration register 21"]
    pub core_config_21: CORE_CONFIG_21,
    #[doc = "0x68 - Core configuration register 22"]
    pub core_config_22: CORE_CONFIG_22,
    #[doc = "0x6c - Core configuration register 23"]
    pub core_config_23: CORE_CONFIG_23,
    #[doc = "0x70 - Core configuration register 24"]
    pub core_config_24: CORE_CONFIG_24,
    #[doc = "0x74 - Core configuration register 25"]
    pub core_config_25: CORE_CONFIG_25,
    _reserved11: [u8; 0x18],
    #[doc = "0x90 - System configuration register 0"]
    pub sys_config_0: SYS_CONFIG_0,
    #[doc = "0x94 - System configuration register 1"]
    pub sys_config_1: SYS_CONFIG_1,
    _reserved13: [u8; 0x08],
    #[doc = "0xa0 - Bus configuration register 0"]
    pub bus_config_0: BUS_CONFIG_0,
    _reserved14: [u8; 0x3c],
    #[doc = "0xe0 - Electromagnetic interference configuration"]
    pub emi_config: EMI_CONFIG,
    _reserved15: [u8; 0x0c],
    #[doc = "0xf0 - Real-time clock configuration"]
    pub rtc_config: RTC_CONFIG,
    _reserved16: [u8; 0x1c],
    #[doc = "0x110 - Analog-to-digital convert configuration"]
    pub adc_config: ADC_CONFIG,
    _reserved17: [u8; 0x0c],
    #[doc = "0x120 - Digital-to-analog convert configuration 0"]
    pub dac_config_0: DAC_CONFIG_0,
    #[doc = "0x124 - Digital-to-analog convert configuration 1"]
    pub dac_config_1: DAC_CONFIG_1,
    #[doc = "0x128 - Digital-to-analog convert configuration 2"]
    pub dac_config_2: DAC_CONFIG_2,
    #[doc = "0x12c - Digital-to-analog convert configuration 3"]
    pub dac_config_3: DAC_CONFIG_3,
    #[doc = "0x130 - Direct memory access configuration 0"]
    pub dma_config_0: DMA_CONFIG_0,
    #[doc = "0x134 - Direct memory access configuration 1"]
    pub dma_config_1: DMA_CONFIG_1,
    #[doc = "0x138 - Direct memory access configuration 2"]
    pub dma_config_2: DMA_CONFIG_2,
    _reserved24: [u8; 0x04],
    #[doc = "0x140 - Infrared configuration register 0"]
    pub ir_config_0: IR_CONFIG_0,
    #[doc = "0x144 - Infrared configuration register 1"]
    pub ir_config_1: IR_CONFIG_1,
    _reserved26: [u8; 0x08],
    #[doc = "0x150 - Universal Asynchronous Receiver/Transmitter configuration"]
    pub uart_config: UART_CONFIG,
    #[doc = "0x154 - Universal Asynchronous Receiver/Transmitter signal configuration 0"]
    pub uart_signal_0: UART_SIGNAL_0,
    #[doc = "0x158 - Universal Asynchronous Receiver/Transmitter signal configuration 1"]
    pub uart_signal_1: UART_SIGNAL_1,
    _reserved29: [u8; 0x14],
    #[doc = "0x170 - Serial flash configuration"]
    pub flash_config: FLASH_CONFIG,
    _reserved30: [u8; 0x0c],
    #[doc = "0x180 - Inter-Integrated Circuit bus configuration"]
    pub i2c_config: I2C_CONFIG,
    _reserved31: [u8; 0x0c],
    #[doc = "0x190 - Inter-IC Sound configuration"]
    pub i2s_config: I2S_CONFIG,
    _reserved32: [u8; 0x1c],
    #[doc = "0x1b0 - Serial Peripheral Interface configuration"]
    pub spi_config: SPI_CONFIG,
    _reserved33: [u8; 0x0c],
    #[doc = "0x1c0 - Quadrature decoder configuration"]
    pub quad_config: QUAD_CONFIG,
    _reserved34: [u8; 0x8c],
    #[doc = "0x250 - Digital clock configuration 0"]
    pub digit_clock_0: DIGIT_CLOCK_0,
    #[doc = "0x254 - Digital clock configuration 1"]
    pub digit_clock_1: DIGIT_CLOCK_1,
    #[doc = "0x258 - Digital clock configuration 2"]
    pub digit_clock_2: DIGIT_CLOCK_2,
    _reserved37: [u8; 0x04],
    #[doc = "0x260 - Radio configuration register"]
    pub radio_config: RADIO_CONFIG,
    _reserved38: [u8; 0x7c],
    #[doc = "0x2e0 - Debug configuration register 0"]
    pub debug_config_0: DEBUG_CONFIG_0,
    #[doc = "0x2e4 - Debug configuration register 1"]
    pub debug_config_1: DEBUG_CONFIG_1,
    #[doc = "0x2e8 - Debug configuration register 2"]
    pub debug_config_2: DEBUG_CONFIG_2,
    #[doc = "0x2ec - Debug configuration register 3"]
    pub debug_config_3: DEBUG_CONFIG_3,
    #[doc = "0x2f0 - Debug configuration register 4"]
    pub debug_config_4: DEBUG_CONFIG_4,
    _reserved43: [u8; 0x0c],
    #[doc = "0x300 - Memory Built-in Self Test mode"]
    pub test_mode: TEST_MODE,
    _reserved44: [u8; 0x04],
    #[doc = "0x308 - Memory Built-in Self Test done state"]
    pub test_done: TEST_DONE,
    _reserved45: [u8; 0x04],
    #[doc = "0x310 - Memory Built-in Self Test fail state"]
    pub test_fail: TEST_FAIL,
    _reserved46: [u8; 0x2c],
    #[doc = "0x340 - Audio configuration register 0"]
    pub audio_config_0: AUDIO_CONFIG_0,
    #[doc = "0x344 - Audio configuration register 1"]
    pub audio_config_1: AUDIO_CONFIG_1,
    _reserved48: [u8; 0x48],
    #[doc = "0x390 - Ethernet Media Access Control register"]
    pub emac_config: EMAC_CONFIG,
    _reserved49: [u8; 0x8c],
    #[doc = "0x420 - ??"]
    pub cam_config: CAM_CONFIG,
    _reserved50: [u8; 0x04a0],
    #[doc = "0x8c4..0x97c - Generic Purpose Input/Output config"]
    pub gpio_config: [GPIO_CONFIG; 46],
    _reserved51: [u8; 0x0148],
    #[doc = "0xac4..0xacc - Read value from Generic Purpose Input/Output pins"]
    pub gpio_input: [GPIO_INPUT; 2],
    _reserved52: [u8; 0x18],
    #[doc = "0xae4..0xaec - Write value to Generic Purpose Input/Output pins"]
    pub gpio_output: [GPIO_OUTPUT; 2],
    #[doc = "0xaec..0xaf4 - Set pin output value to high"]
    pub gpio_set: [GPIO_SET; 2],
    #[doc = "0xaf4..0xafc - Set pin output value to low"]
    pub gpio_clear: [GPIO_CLEAR; 2],
}
#[doc = "chip_inform (rw) register accessor: an alias for `Reg<CHIP_INFORM_SPEC>`"]
pub type CHIP_INFORM = crate::Reg<chip_inform::CHIP_INFORM_SPEC>;
#[doc = "Chip information register"]
pub mod chip_inform;
#[doc = "core_config_16 (rw) register accessor: an alias for `Reg<CORE_CONFIG_16_SPEC>`"]
pub type CORE_CONFIG_16 = crate::Reg<core_config_16::CORE_CONFIG_16_SPEC>;
#[doc = "Core configuration register 16"]
pub mod core_config_16;
#[doc = "core_config_17 (rw) register accessor: an alias for `Reg<CORE_CONFIG_17_SPEC>`"]
pub type CORE_CONFIG_17 = crate::Reg<core_config_17::CORE_CONFIG_17_SPEC>;
#[doc = "Core configuration register 17"]
pub mod core_config_17;
#[doc = "core_config_18 (rw) register accessor: an alias for `Reg<CORE_CONFIG_18_SPEC>`"]
pub type CORE_CONFIG_18 = crate::Reg<core_config_18::CORE_CONFIG_18_SPEC>;
#[doc = "Core configuration register 18"]
pub mod core_config_18;
#[doc = "core_config_19 (rw) register accessor: an alias for `Reg<CORE_CONFIG_19_SPEC>`"]
pub type CORE_CONFIG_19 = crate::Reg<core_config_19::CORE_CONFIG_19_SPEC>;
#[doc = "Core configuration register 19"]
pub mod core_config_19;
#[doc = "core_config_20 (rw) register accessor: an alias for `Reg<CORE_CONFIG_20_SPEC>`"]
pub type CORE_CONFIG_20 = crate::Reg<core_config_20::CORE_CONFIG_20_SPEC>;
#[doc = "Core configuration register 20"]
pub mod core_config_20;
#[doc = "core_config_21 (rw) register accessor: an alias for `Reg<CORE_CONFIG_21_SPEC>`"]
pub type CORE_CONFIG_21 = crate::Reg<core_config_21::CORE_CONFIG_21_SPEC>;
#[doc = "Core configuration register 21"]
pub mod core_config_21;
#[doc = "core_config_22 (rw) register accessor: an alias for `Reg<CORE_CONFIG_22_SPEC>`"]
pub type CORE_CONFIG_22 = crate::Reg<core_config_22::CORE_CONFIG_22_SPEC>;
#[doc = "Core configuration register 22"]
pub mod core_config_22;
#[doc = "core_config_23 (rw) register accessor: an alias for `Reg<CORE_CONFIG_23_SPEC>`"]
pub type CORE_CONFIG_23 = crate::Reg<core_config_23::CORE_CONFIG_23_SPEC>;
#[doc = "Core configuration register 23"]
pub mod core_config_23;
#[doc = "core_config_24 (rw) register accessor: an alias for `Reg<CORE_CONFIG_24_SPEC>`"]
pub type CORE_CONFIG_24 = crate::Reg<core_config_24::CORE_CONFIG_24_SPEC>;
#[doc = "Core configuration register 24"]
pub mod core_config_24;
#[doc = "core_config_25 (rw) register accessor: an alias for `Reg<CORE_CONFIG_25_SPEC>`"]
pub type CORE_CONFIG_25 = crate::Reg<core_config_25::CORE_CONFIG_25_SPEC>;
#[doc = "Core configuration register 25"]
pub mod core_config_25;
#[doc = "sys_config_0 (rw) register accessor: an alias for `Reg<SYS_CONFIG_0_SPEC>`"]
pub type SYS_CONFIG_0 = crate::Reg<sys_config_0::SYS_CONFIG_0_SPEC>;
#[doc = "System configuration register 0"]
pub mod sys_config_0;
#[doc = "sys_config_1 (rw) register accessor: an alias for `Reg<SYS_CONFIG_1_SPEC>`"]
pub type SYS_CONFIG_1 = crate::Reg<sys_config_1::SYS_CONFIG_1_SPEC>;
#[doc = "System configuration register 1"]
pub mod sys_config_1;
#[doc = "bus_config_0 (rw) register accessor: an alias for `Reg<BUS_CONFIG_0_SPEC>`"]
pub type BUS_CONFIG_0 = crate::Reg<bus_config_0::BUS_CONFIG_0_SPEC>;
#[doc = "Bus configuration register 0"]
pub mod bus_config_0;
#[doc = "emi_config (rw) register accessor: an alias for `Reg<EMI_CONFIG_SPEC>`"]
pub type EMI_CONFIG = crate::Reg<emi_config::EMI_CONFIG_SPEC>;
#[doc = "Electromagnetic interference configuration"]
pub mod emi_config;
#[doc = "rtc_config (rw) register accessor: an alias for `Reg<RTC_CONFIG_SPEC>`"]
pub type RTC_CONFIG = crate::Reg<rtc_config::RTC_CONFIG_SPEC>;
#[doc = "Real-time clock configuration"]
pub mod rtc_config;
#[doc = "adc_config (rw) register accessor: an alias for `Reg<ADC_CONFIG_SPEC>`"]
pub type ADC_CONFIG = crate::Reg<adc_config::ADC_CONFIG_SPEC>;
#[doc = "Analog-to-digital convert configuration"]
pub mod adc_config;
#[doc = "dac_config_0 (rw) register accessor: an alias for `Reg<DAC_CONFIG_0_SPEC>`"]
pub type DAC_CONFIG_0 = crate::Reg<dac_config_0::DAC_CONFIG_0_SPEC>;
#[doc = "Digital-to-analog convert configuration 0"]
pub mod dac_config_0;
#[doc = "dac_config_1 (rw) register accessor: an alias for `Reg<DAC_CONFIG_1_SPEC>`"]
pub type DAC_CONFIG_1 = crate::Reg<dac_config_1::DAC_CONFIG_1_SPEC>;
#[doc = "Digital-to-analog convert configuration 1"]
pub mod dac_config_1;
#[doc = "dac_config_2 (rw) register accessor: an alias for `Reg<DAC_CONFIG_2_SPEC>`"]
pub type DAC_CONFIG_2 = crate::Reg<dac_config_2::DAC_CONFIG_2_SPEC>;
#[doc = "Digital-to-analog convert configuration 2"]
pub mod dac_config_2;
#[doc = "dac_config_3 (rw) register accessor: an alias for `Reg<DAC_CONFIG_3_SPEC>`"]
pub type DAC_CONFIG_3 = crate::Reg<dac_config_3::DAC_CONFIG_3_SPEC>;
#[doc = "Digital-to-analog convert configuration 3"]
pub mod dac_config_3;
#[doc = "dma_config_0 (rw) register accessor: an alias for `Reg<DMA_CONFIG_0_SPEC>`"]
pub type DMA_CONFIG_0 = crate::Reg<dma_config_0::DMA_CONFIG_0_SPEC>;
#[doc = "Direct memory access configuration 0"]
pub mod dma_config_0;
#[doc = "dma_config_1 (rw) register accessor: an alias for `Reg<DMA_CONFIG_1_SPEC>`"]
pub type DMA_CONFIG_1 = crate::Reg<dma_config_1::DMA_CONFIG_1_SPEC>;
#[doc = "Direct memory access configuration 1"]
pub mod dma_config_1;
#[doc = "dma_config_2 (rw) register accessor: an alias for `Reg<DMA_CONFIG_2_SPEC>`"]
pub type DMA_CONFIG_2 = crate::Reg<dma_config_2::DMA_CONFIG_2_SPEC>;
#[doc = "Direct memory access configuration 2"]
pub mod dma_config_2;
#[doc = "ir_config_0 (rw) register accessor: an alias for `Reg<IR_CONFIG_0_SPEC>`"]
pub type IR_CONFIG_0 = crate::Reg<ir_config_0::IR_CONFIG_0_SPEC>;
#[doc = "Infrared configuration register 0"]
pub mod ir_config_0;
#[doc = "ir_config_1 (rw) register accessor: an alias for `Reg<IR_CONFIG_1_SPEC>`"]
pub type IR_CONFIG_1 = crate::Reg<ir_config_1::IR_CONFIG_1_SPEC>;
#[doc = "Infrared configuration register 1"]
pub mod ir_config_1;
#[doc = "uart_config (rw) register accessor: an alias for `Reg<UART_CONFIG_SPEC>`"]
pub type UART_CONFIG = crate::Reg<uart_config::UART_CONFIG_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter configuration"]
pub mod uart_config;
#[doc = "uart_signal_0 (rw) register accessor: an alias for `Reg<UART_SIGNAL_0_SPEC>`"]
pub type UART_SIGNAL_0 = crate::Reg<uart_signal_0::UART_SIGNAL_0_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 0"]
pub mod uart_signal_0;
#[doc = "uart_signal_1 (rw) register accessor: an alias for `Reg<UART_SIGNAL_1_SPEC>`"]
pub type UART_SIGNAL_1 = crate::Reg<uart_signal_1::UART_SIGNAL_1_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 1"]
pub mod uart_signal_1;
#[doc = "flash_config (rw) register accessor: an alias for `Reg<FLASH_CONFIG_SPEC>`"]
pub type FLASH_CONFIG = crate::Reg<flash_config::FLASH_CONFIG_SPEC>;
#[doc = "Serial flash configuration"]
pub mod flash_config;
#[doc = "i2c_config (rw) register accessor: an alias for `Reg<I2C_CONFIG_SPEC>`"]
pub type I2C_CONFIG = crate::Reg<i2c_config::I2C_CONFIG_SPEC>;
#[doc = "Inter-Integrated Circuit bus configuration"]
pub mod i2c_config;
#[doc = "i2s_config (rw) register accessor: an alias for `Reg<I2S_CONFIG_SPEC>`"]
pub type I2S_CONFIG = crate::Reg<i2s_config::I2S_CONFIG_SPEC>;
#[doc = "Inter-IC Sound configuration"]
pub mod i2s_config;
#[doc = "spi_config (rw) register accessor: an alias for `Reg<SPI_CONFIG_SPEC>`"]
pub type SPI_CONFIG = crate::Reg<spi_config::SPI_CONFIG_SPEC>;
#[doc = "Serial Peripheral Interface configuration"]
pub mod spi_config;
#[doc = "quad_config (rw) register accessor: an alias for `Reg<QUAD_CONFIG_SPEC>`"]
pub type QUAD_CONFIG = crate::Reg<quad_config::QUAD_CONFIG_SPEC>;
#[doc = "Quadrature decoder configuration"]
pub mod quad_config;
#[doc = "digit_clock_0 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_0_SPEC>`"]
pub type DIGIT_CLOCK_0 = crate::Reg<digit_clock_0::DIGIT_CLOCK_0_SPEC>;
#[doc = "Digital clock configuration 0"]
pub mod digit_clock_0;
#[doc = "digit_clock_1 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_1_SPEC>`"]
pub type DIGIT_CLOCK_1 = crate::Reg<digit_clock_1::DIGIT_CLOCK_1_SPEC>;
#[doc = "Digital clock configuration 1"]
pub mod digit_clock_1;
#[doc = "digit_clock_2 (rw) register accessor: an alias for `Reg<DIGIT_CLOCK_2_SPEC>`"]
pub type DIGIT_CLOCK_2 = crate::Reg<digit_clock_2::DIGIT_CLOCK_2_SPEC>;
#[doc = "Digital clock configuration 2"]
pub mod digit_clock_2;
#[doc = "radio_config (rw) register accessor: an alias for `Reg<RADIO_CONFIG_SPEC>`"]
pub type RADIO_CONFIG = crate::Reg<radio_config::RADIO_CONFIG_SPEC>;
#[doc = "Radio configuration register"]
pub mod radio_config;
#[doc = "debug_config_0 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_0_SPEC>`"]
pub type DEBUG_CONFIG_0 = crate::Reg<debug_config_0::DEBUG_CONFIG_0_SPEC>;
#[doc = "Debug configuration register 0"]
pub mod debug_config_0;
#[doc = "debug_config_1 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_1_SPEC>`"]
pub type DEBUG_CONFIG_1 = crate::Reg<debug_config_1::DEBUG_CONFIG_1_SPEC>;
#[doc = "Debug configuration register 1"]
pub mod debug_config_1;
#[doc = "debug_config_2 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_2_SPEC>`"]
pub type DEBUG_CONFIG_2 = crate::Reg<debug_config_2::DEBUG_CONFIG_2_SPEC>;
#[doc = "Debug configuration register 2"]
pub mod debug_config_2;
#[doc = "debug_config_3 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_3_SPEC>`"]
pub type DEBUG_CONFIG_3 = crate::Reg<debug_config_3::DEBUG_CONFIG_3_SPEC>;
#[doc = "Debug configuration register 3"]
pub mod debug_config_3;
#[doc = "debug_config_4 (rw) register accessor: an alias for `Reg<DEBUG_CONFIG_4_SPEC>`"]
pub type DEBUG_CONFIG_4 = crate::Reg<debug_config_4::DEBUG_CONFIG_4_SPEC>;
#[doc = "Debug configuration register 4"]
pub mod debug_config_4;
#[doc = "test_mode (rw) register accessor: an alias for `Reg<TEST_MODE_SPEC>`"]
pub type TEST_MODE = crate::Reg<test_mode::TEST_MODE_SPEC>;
#[doc = "Memory Built-in Self Test mode"]
pub mod test_mode;
#[doc = "test_done (rw) register accessor: an alias for `Reg<TEST_DONE_SPEC>`"]
pub type TEST_DONE = crate::Reg<test_done::TEST_DONE_SPEC>;
#[doc = "Memory Built-in Self Test done state"]
pub mod test_done;
#[doc = "test_fail (rw) register accessor: an alias for `Reg<TEST_FAIL_SPEC>`"]
pub type TEST_FAIL = crate::Reg<test_fail::TEST_FAIL_SPEC>;
#[doc = "Memory Built-in Self Test fail state"]
pub mod test_fail;
#[doc = "audio_config_0 (rw) register accessor: an alias for `Reg<AUDIO_CONFIG_0_SPEC>`"]
pub type AUDIO_CONFIG_0 = crate::Reg<audio_config_0::AUDIO_CONFIG_0_SPEC>;
#[doc = "Audio configuration register 0"]
pub mod audio_config_0;
#[doc = "audio_config_1 (rw) register accessor: an alias for `Reg<AUDIO_CONFIG_1_SPEC>`"]
pub type AUDIO_CONFIG_1 = crate::Reg<audio_config_1::AUDIO_CONFIG_1_SPEC>;
#[doc = "Audio configuration register 1"]
pub mod audio_config_1;
#[doc = "emac_config (rw) register accessor: an alias for `Reg<EMAC_CONFIG_SPEC>`"]
pub type EMAC_CONFIG = crate::Reg<emac_config::EMAC_CONFIG_SPEC>;
#[doc = "Ethernet Media Access Control register"]
pub mod emac_config;
#[doc = "cam_config (rw) register accessor: an alias for `Reg<CAM_CONFIG_SPEC>`"]
pub type CAM_CONFIG = crate::Reg<cam_config::CAM_CONFIG_SPEC>;
#[doc = "??"]
pub mod cam_config;
#[doc = "gpio_config (rw) register accessor: an alias for `Reg<GPIO_CONFIG_SPEC>`"]
pub type GPIO_CONFIG = crate::Reg<gpio_config::GPIO_CONFIG_SPEC>;
#[doc = "Generic Purpose Input/Output config"]
pub mod gpio_config;
#[doc = "gpio_input (rw) register accessor: an alias for `Reg<GPIO_INPUT_SPEC>`"]
pub type GPIO_INPUT = crate::Reg<gpio_input::GPIO_INPUT_SPEC>;
#[doc = "Read value from Generic Purpose Input/Output pins"]
pub mod gpio_input;
#[doc = "gpio_output (rw) register accessor: an alias for `Reg<GPIO_OUTPUT_SPEC>`"]
pub type GPIO_OUTPUT = crate::Reg<gpio_output::GPIO_OUTPUT_SPEC>;
#[doc = "Write value to Generic Purpose Input/Output pins"]
pub mod gpio_output;
#[doc = "gpio_set (rw) register accessor: an alias for `Reg<GPIO_SET_SPEC>`"]
pub type GPIO_SET = crate::Reg<gpio_set::GPIO_SET_SPEC>;
#[doc = "Set pin output value to high"]
pub mod gpio_set;
#[doc = "gpio_clear (rw) register accessor: an alias for `Reg<GPIO_CLEAR_SPEC>`"]
pub type GPIO_CLEAR = crate::Reg<gpio_clear::GPIO_CLEAR_SPEC>;
#[doc = "Set pin output value to low"]
pub mod gpio_clear;
