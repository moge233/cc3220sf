#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_PROPERTIES_REGISTER"]
    pub i2c_properties_register: I2C_PROPERTIES_REGISTER,
    #[doc = "0x04 - SPI_PROPERTIES_REGISTER"]
    pub spi_properties_register: SPI_PROPERTIES_REGISTER,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - APPS_SH_RESOURCE_INTERRUPT_ENABLE"]
    pub apps_sh_resource_interrupt_enable: APPS_SH_RESOURCE_INTERRUPT_ENABLE,
    #[doc = "0x10 - APPS_SH_RESOURCE_INTERRUPT_STATUS"]
    pub apps_sh_resource_interrupt_status: APPS_SH_RESOURCE_INTERRUPT_STATUS,
    #[doc = "0x14 - NWP_SH_RESOURCE_INTERRUPT_ENABLE"]
    pub nwp_sh_resource_interrupt_enable: NWP_SH_RESOURCE_INTERRUPT_ENABLE,
    #[doc = "0x18 - NWP_SH_RESOURCE_INTERRUPT_STATUS"]
    pub nwp_sh_resource_interrupt_status: NWP_SH_RESOURCE_INTERRUPT_STATUS,
    #[doc = "0x1c - FLASH_CTRL_REG"]
    pub flash_ctrl_reg: FLASH_CTRL_REG,
    _reserved7: [u8; 4usize],
    #[doc = "0x24 - BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m0_segment_access_config: BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x28 - BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m1_segment_access_config: BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x2c - BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m2_segment_access_config: BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x30 - BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m3_segment_access_config: BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x34 - BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m4_segment_access_config: BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x38 - BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG"]
    pub bus_matrix_m5_segment_access_config: BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG,
    #[doc = "0x3c - GPIO_PROPERTIES_REGISTER"]
    pub gpio_properties_register: GPIO_PROPERTIES_REGISTER,
    #[doc = "0x40 - APPS_NW_SEMAPHORE1"]
    pub apps_nw_semaphore1: APPS_NW_SEMAPHORE1,
    #[doc = "0x44 - APPS_NW_SEMAPHORE2"]
    pub apps_nw_semaphore2: APPS_NW_SEMAPHORE2,
    #[doc = "0x48 - APPS_NW_SEMAPHORE3"]
    pub apps_nw_semaphore3: APPS_NW_SEMAPHORE3,
    #[doc = "0x4c - APPS_NW_SEMAPHORE4"]
    pub apps_nw_semaphore4: APPS_NW_SEMAPHORE4,
    #[doc = "0x50 - APPS_NW_SEMAPHORE5"]
    pub apps_nw_semaphore5: APPS_NW_SEMAPHORE5,
    #[doc = "0x54 - APPS_NW_SEMAPHORE6"]
    pub apps_nw_semaphore6: APPS_NW_SEMAPHORE6,
    #[doc = "0x58 - APPS_NW_SEMAPHORE7"]
    pub apps_nw_semaphore7: APPS_NW_SEMAPHORE7,
    #[doc = "0x5c - APPS_NW_SEMAPHORE8"]
    pub apps_nw_semaphore8: APPS_NW_SEMAPHORE8,
    #[doc = "0x60 - APPS_NW_SEMAPHORE9"]
    pub apps_nw_semaphore9: APPS_NW_SEMAPHORE9,
    #[doc = "0x64 - APPS_NW_SEMAPHORE10"]
    pub apps_nw_semaphore10: APPS_NW_SEMAPHORE10,
    #[doc = "0x68 - APPS_NW_SEMAPHORE11"]
    pub apps_nw_semaphore11: APPS_NW_SEMAPHORE11,
    #[doc = "0x6c - APPS_NW_SEMAPHORE12"]
    pub apps_nw_semaphore12: APPS_NW_SEMAPHORE12,
    #[doc = "0x70 - APPS_SEMAPPHORE_PEND"]
    pub apps_semapphore_pend: APPS_SEMAPPHORE_PEND,
    #[doc = "0x74 - NW_SEMAPPHORE_PEND"]
    pub nw_semapphore_pend: NW_SEMAPPHORE_PEND,
    #[doc = "0x78 - SEMAPHORE_STATUS"]
    pub semaphore_status: SEMAPHORE_STATUS,
    #[doc = "0x7c - IDMEM_TIM_UPDATE"]
    pub idmem_tim_update: IDMEM_TIM_UPDATE,
    #[doc = "0x80 - FPGA_ROM_WR_EN"]
    pub fpga_rom_wr_en: FPGA_ROM_WR_EN,
    #[doc = "0x84 - NW_INT_MASK"]
    pub nw_int_mask: NW_INT_MASK,
    #[doc = "0x88 - NW_INT_MASK_SET"]
    pub nw_int_mask_set: NW_INT_MASK_SET,
    #[doc = "0x8c - NW_INT_MASK_CLR"]
    pub nw_int_mask_clr: NW_INT_MASK_CLR,
    #[doc = "0x90 - NW_INT_STS_CLR"]
    pub nw_int_sts_clr: NW_INT_STS_CLR,
    #[doc = "0x94 - NW_INT_ACK"]
    pub nw_int_ack: NW_INT_ACK,
    #[doc = "0x98 - NW_INT_TRIG"]
    pub nw_int_trig: NW_INT_TRIG,
    #[doc = "0x9c - NW_INT_STS_MASKED"]
    pub nw_int_sts_masked: NW_INT_STS_MASKED,
    #[doc = "0xa0 - NW_INT_STS_RAW"]
    pub nw_int_sts_raw: NW_INT_STS_RAW,
    #[doc = "0xa4 - APPS_INT_MASK"]
    pub apps_int_mask: APPS_INT_MASK,
    #[doc = "0xa8 - APPS_INT_MASK_SET"]
    pub apps_int_mask_set: APPS_INT_MASK_SET,
    #[doc = "0xac - APPS_INT_MASK_CLR"]
    pub apps_int_mask_clr: APPS_INT_MASK_CLR,
    #[doc = "0xb0 - APPS_INT_STS_CLR"]
    pub apps_int_sts_clr: APPS_INT_STS_CLR,
    #[doc = "0xb4 - APPS_INT_ACK"]
    pub apps_int_ack: APPS_INT_ACK,
    #[doc = "0xb8 - APPS_INT_TRIG"]
    pub apps_int_trig: APPS_INT_TRIG,
    #[doc = "0xbc - APPS_INT_STS_MASKED"]
    pub apps_int_sts_masked: APPS_INT_STS_MASKED,
    #[doc = "0xc0 - APPS_INT_STS_RAW"]
    pub apps_int_sts_raw: APPS_INT_STS_RAW,
    #[doc = "0xc4 - IDMEM_TIM_UPDATED"]
    pub idmem_tim_updated: IDMEM_TIM_UPDATED,
    #[doc = "0xc8 - APPS_GPIO_TRIG_EN"]
    pub apps_gpio_trig_en: APPS_GPIO_TRIG_EN,
    #[doc = "0xcc - EMU_DEBUG_REG"]
    pub emu_debug_reg: EMU_DEBUG_REG,
    #[doc = "0xd0 - SEMAPHORE_STATUS2"]
    pub semaphore_status2: SEMAPHORE_STATUS2,
    #[doc = "0xd4 - SEMAPHORE_PREV_OWNER1"]
    pub semaphore_prev_owner1: SEMAPHORE_PREV_OWNER1,
    #[doc = "0xd8 - SEMAPHORE_PREV_OWNER2"]
    pub semaphore_prev_owner2: SEMAPHORE_PREV_OWNER2,
}
#[doc = "I2C_PROPERTIES_REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_properties_register](i2c_properties_register) module"]
pub type I2C_PROPERTIES_REGISTER = crate::Reg<u32, _I2C_PROPERTIES_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PROPERTIES_REGISTER;
#[doc = "`read()` method returns [i2c_properties_register::R](i2c_properties_register::R) reader structure"]
impl crate::Readable for I2C_PROPERTIES_REGISTER {}
#[doc = "`write(|w| ..)` method takes [i2c_properties_register::W](i2c_properties_register::W) writer structure"]
impl crate::Writable for I2C_PROPERTIES_REGISTER {}
#[doc = "I2C_PROPERTIES_REGISTER"]
pub mod i2c_properties_register;
#[doc = "SPI_PROPERTIES_REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_properties_register](spi_properties_register) module"]
pub type SPI_PROPERTIES_REGISTER = crate::Reg<u32, _SPI_PROPERTIES_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_PROPERTIES_REGISTER;
#[doc = "`read()` method returns [spi_properties_register::R](spi_properties_register::R) reader structure"]
impl crate::Readable for SPI_PROPERTIES_REGISTER {}
#[doc = "`write(|w| ..)` method takes [spi_properties_register::W](spi_properties_register::W) writer structure"]
impl crate::Writable for SPI_PROPERTIES_REGISTER {}
#[doc = "SPI_PROPERTIES_REGISTER"]
pub mod spi_properties_register;
#[doc = "APPS_SH_RESOURCE_INTERRUPT_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_sh_resource_interrupt_enable](apps_sh_resource_interrupt_enable) module"]
pub type APPS_SH_RESOURCE_INTERRUPT_ENABLE = crate::Reg<u32, _APPS_SH_RESOURCE_INTERRUPT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SH_RESOURCE_INTERRUPT_ENABLE;
#[doc = "`read()` method returns [apps_sh_resource_interrupt_enable::R](apps_sh_resource_interrupt_enable::R) reader structure"]
impl crate::Readable for APPS_SH_RESOURCE_INTERRUPT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [apps_sh_resource_interrupt_enable::W](apps_sh_resource_interrupt_enable::W) writer structure"]
impl crate::Writable for APPS_SH_RESOURCE_INTERRUPT_ENABLE {}
#[doc = "APPS_SH_RESOURCE_INTERRUPT_ENABLE"]
pub mod apps_sh_resource_interrupt_enable;
#[doc = "APPS_SH_RESOURCE_INTERRUPT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_sh_resource_interrupt_status](apps_sh_resource_interrupt_status) module"]
pub type APPS_SH_RESOURCE_INTERRUPT_STATUS = crate::Reg<u32, _APPS_SH_RESOURCE_INTERRUPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SH_RESOURCE_INTERRUPT_STATUS;
#[doc = "`read()` method returns [apps_sh_resource_interrupt_status::R](apps_sh_resource_interrupt_status::R) reader structure"]
impl crate::Readable for APPS_SH_RESOURCE_INTERRUPT_STATUS {}
#[doc = "`write(|w| ..)` method takes [apps_sh_resource_interrupt_status::W](apps_sh_resource_interrupt_status::W) writer structure"]
impl crate::Writable for APPS_SH_RESOURCE_INTERRUPT_STATUS {}
#[doc = "APPS_SH_RESOURCE_INTERRUPT_STATUS"]
pub mod apps_sh_resource_interrupt_status;
#[doc = "NWP_SH_RESOURCE_INTERRUPT_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_sh_resource_interrupt_enable](nwp_sh_resource_interrupt_enable) module"]
pub type NWP_SH_RESOURCE_INTERRUPT_ENABLE = crate::Reg<u32, _NWP_SH_RESOURCE_INTERRUPT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SH_RESOURCE_INTERRUPT_ENABLE;
#[doc = "`read()` method returns [nwp_sh_resource_interrupt_enable::R](nwp_sh_resource_interrupt_enable::R) reader structure"]
impl crate::Readable for NWP_SH_RESOURCE_INTERRUPT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [nwp_sh_resource_interrupt_enable::W](nwp_sh_resource_interrupt_enable::W) writer structure"]
impl crate::Writable for NWP_SH_RESOURCE_INTERRUPT_ENABLE {}
#[doc = "NWP_SH_RESOURCE_INTERRUPT_ENABLE"]
pub mod nwp_sh_resource_interrupt_enable;
#[doc = "NWP_SH_RESOURCE_INTERRUPT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_sh_resource_interrupt_status](nwp_sh_resource_interrupt_status) module"]
pub type NWP_SH_RESOURCE_INTERRUPT_STATUS = crate::Reg<u32, _NWP_SH_RESOURCE_INTERRUPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SH_RESOURCE_INTERRUPT_STATUS;
#[doc = "`read()` method returns [nwp_sh_resource_interrupt_status::R](nwp_sh_resource_interrupt_status::R) reader structure"]
impl crate::Readable for NWP_SH_RESOURCE_INTERRUPT_STATUS {}
#[doc = "`write(|w| ..)` method takes [nwp_sh_resource_interrupt_status::W](nwp_sh_resource_interrupt_status::W) writer structure"]
impl crate::Writable for NWP_SH_RESOURCE_INTERRUPT_STATUS {}
#[doc = "NWP_SH_RESOURCE_INTERRUPT_STATUS"]
pub mod nwp_sh_resource_interrupt_status;
#[doc = "FLASH_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctrl_reg](flash_ctrl_reg) module"]
pub type FLASH_CTRL_REG = crate::Reg<u32, _FLASH_CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CTRL_REG;
#[doc = "`read()` method returns [flash_ctrl_reg::R](flash_ctrl_reg::R) reader structure"]
impl crate::Readable for FLASH_CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [flash_ctrl_reg::W](flash_ctrl_reg::W) writer structure"]
impl crate::Writable for FLASH_CTRL_REG {}
#[doc = "FLASH_CTRL_REG"]
pub mod flash_ctrl_reg;
#[doc = "BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m0_segment_access_config](bus_matrix_m0_segment_access_config) module"]
pub type BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m0_segment_access_config::R](bus_matrix_m0_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m0_segment_access_config::W](bus_matrix_m0_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M0_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m0_segment_access_config;
#[doc = "BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m1_segment_access_config](bus_matrix_m1_segment_access_config) module"]
pub type BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m1_segment_access_config::R](bus_matrix_m1_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m1_segment_access_config::W](bus_matrix_m1_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M1_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m1_segment_access_config;
#[doc = "BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m2_segment_access_config](bus_matrix_m2_segment_access_config) module"]
pub type BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m2_segment_access_config::R](bus_matrix_m2_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m2_segment_access_config::W](bus_matrix_m2_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M2_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m2_segment_access_config;
#[doc = "BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m3_segment_access_config](bus_matrix_m3_segment_access_config) module"]
pub type BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m3_segment_access_config::R](bus_matrix_m3_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m3_segment_access_config::W](bus_matrix_m3_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m3_segment_access_config;
#[doc = "BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m4_segment_access_config](bus_matrix_m4_segment_access_config) module"]
pub type BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m4_segment_access_config::R](bus_matrix_m4_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m4_segment_access_config::W](bus_matrix_m4_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M4_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m4_segment_access_config;
#[doc = "BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_matrix_m5_segment_access_config](bus_matrix_m5_segment_access_config) module"]
pub type BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG =
    crate::Reg<u32, _BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG;
#[doc = "`read()` method returns [bus_matrix_m5_segment_access_config::R](bus_matrix_m5_segment_access_config::R) reader structure"]
impl crate::Readable for BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [bus_matrix_m5_segment_access_config::W](bus_matrix_m5_segment_access_config::W) writer structure"]
impl crate::Writable for BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG {}
#[doc = "BUS_MATRIX_M5_SEGMENT_ACCESS_CONFIG"]
pub mod bus_matrix_m5_segment_access_config;
#[doc = "GPIO_PROPERTIES_REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_properties_register](gpio_properties_register) module"]
pub type GPIO_PROPERTIES_REGISTER = crate::Reg<u32, _GPIO_PROPERTIES_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PROPERTIES_REGISTER;
#[doc = "`read()` method returns [gpio_properties_register::R](gpio_properties_register::R) reader structure"]
impl crate::Readable for GPIO_PROPERTIES_REGISTER {}
#[doc = "`write(|w| ..)` method takes [gpio_properties_register::W](gpio_properties_register::W) writer structure"]
impl crate::Writable for GPIO_PROPERTIES_REGISTER {}
#[doc = "GPIO_PROPERTIES_REGISTER"]
pub mod gpio_properties_register;
#[doc = "APPS_NW_SEMAPHORE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore1](apps_nw_semaphore1) module"]
pub type APPS_NW_SEMAPHORE1 = crate::Reg<u32, _APPS_NW_SEMAPHORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE1;
#[doc = "`read()` method returns [apps_nw_semaphore1::R](apps_nw_semaphore1::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE1 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore1::W](apps_nw_semaphore1::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE1 {}
#[doc = "APPS_NW_SEMAPHORE1"]
pub mod apps_nw_semaphore1;
#[doc = "APPS_NW_SEMAPHORE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore2](apps_nw_semaphore2) module"]
pub type APPS_NW_SEMAPHORE2 = crate::Reg<u32, _APPS_NW_SEMAPHORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE2;
#[doc = "`read()` method returns [apps_nw_semaphore2::R](apps_nw_semaphore2::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE2 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore2::W](apps_nw_semaphore2::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE2 {}
#[doc = "APPS_NW_SEMAPHORE2"]
pub mod apps_nw_semaphore2;
#[doc = "APPS_NW_SEMAPHORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore3](apps_nw_semaphore3) module"]
pub type APPS_NW_SEMAPHORE3 = crate::Reg<u32, _APPS_NW_SEMAPHORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE3;
#[doc = "`read()` method returns [apps_nw_semaphore3::R](apps_nw_semaphore3::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE3 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore3::W](apps_nw_semaphore3::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE3 {}
#[doc = "APPS_NW_SEMAPHORE3"]
pub mod apps_nw_semaphore3;
#[doc = "APPS_NW_SEMAPHORE4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore4](apps_nw_semaphore4) module"]
pub type APPS_NW_SEMAPHORE4 = crate::Reg<u32, _APPS_NW_SEMAPHORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE4;
#[doc = "`read()` method returns [apps_nw_semaphore4::R](apps_nw_semaphore4::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE4 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore4::W](apps_nw_semaphore4::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE4 {}
#[doc = "APPS_NW_SEMAPHORE4"]
pub mod apps_nw_semaphore4;
#[doc = "APPS_NW_SEMAPHORE5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore5](apps_nw_semaphore5) module"]
pub type APPS_NW_SEMAPHORE5 = crate::Reg<u32, _APPS_NW_SEMAPHORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE5;
#[doc = "`read()` method returns [apps_nw_semaphore5::R](apps_nw_semaphore5::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE5 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore5::W](apps_nw_semaphore5::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE5 {}
#[doc = "APPS_NW_SEMAPHORE5"]
pub mod apps_nw_semaphore5;
#[doc = "APPS_NW_SEMAPHORE6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore6](apps_nw_semaphore6) module"]
pub type APPS_NW_SEMAPHORE6 = crate::Reg<u32, _APPS_NW_SEMAPHORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE6;
#[doc = "`read()` method returns [apps_nw_semaphore6::R](apps_nw_semaphore6::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE6 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore6::W](apps_nw_semaphore6::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE6 {}
#[doc = "APPS_NW_SEMAPHORE6"]
pub mod apps_nw_semaphore6;
#[doc = "APPS_NW_SEMAPHORE7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore7](apps_nw_semaphore7) module"]
pub type APPS_NW_SEMAPHORE7 = crate::Reg<u32, _APPS_NW_SEMAPHORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE7;
#[doc = "`read()` method returns [apps_nw_semaphore7::R](apps_nw_semaphore7::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE7 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore7::W](apps_nw_semaphore7::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE7 {}
#[doc = "APPS_NW_SEMAPHORE7"]
pub mod apps_nw_semaphore7;
#[doc = "APPS_NW_SEMAPHORE8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore8](apps_nw_semaphore8) module"]
pub type APPS_NW_SEMAPHORE8 = crate::Reg<u32, _APPS_NW_SEMAPHORE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE8;
#[doc = "`read()` method returns [apps_nw_semaphore8::R](apps_nw_semaphore8::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE8 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore8::W](apps_nw_semaphore8::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE8 {}
#[doc = "APPS_NW_SEMAPHORE8"]
pub mod apps_nw_semaphore8;
#[doc = "APPS_NW_SEMAPHORE9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore9](apps_nw_semaphore9) module"]
pub type APPS_NW_SEMAPHORE9 = crate::Reg<u32, _APPS_NW_SEMAPHORE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE9;
#[doc = "`read()` method returns [apps_nw_semaphore9::R](apps_nw_semaphore9::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE9 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore9::W](apps_nw_semaphore9::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE9 {}
#[doc = "APPS_NW_SEMAPHORE9"]
pub mod apps_nw_semaphore9;
#[doc = "APPS_NW_SEMAPHORE10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore10](apps_nw_semaphore10) module"]
pub type APPS_NW_SEMAPHORE10 = crate::Reg<u32, _APPS_NW_SEMAPHORE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE10;
#[doc = "`read()` method returns [apps_nw_semaphore10::R](apps_nw_semaphore10::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE10 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore10::W](apps_nw_semaphore10::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE10 {}
#[doc = "APPS_NW_SEMAPHORE10"]
pub mod apps_nw_semaphore10;
#[doc = "APPS_NW_SEMAPHORE11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore11](apps_nw_semaphore11) module"]
pub type APPS_NW_SEMAPHORE11 = crate::Reg<u32, _APPS_NW_SEMAPHORE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE11;
#[doc = "`read()` method returns [apps_nw_semaphore11::R](apps_nw_semaphore11::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE11 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore11::W](apps_nw_semaphore11::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE11 {}
#[doc = "APPS_NW_SEMAPHORE11"]
pub mod apps_nw_semaphore11;
#[doc = "APPS_NW_SEMAPHORE12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_nw_semaphore12](apps_nw_semaphore12) module"]
pub type APPS_NW_SEMAPHORE12 = crate::Reg<u32, _APPS_NW_SEMAPHORE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_NW_SEMAPHORE12;
#[doc = "`read()` method returns [apps_nw_semaphore12::R](apps_nw_semaphore12::R) reader structure"]
impl crate::Readable for APPS_NW_SEMAPHORE12 {}
#[doc = "`write(|w| ..)` method takes [apps_nw_semaphore12::W](apps_nw_semaphore12::W) writer structure"]
impl crate::Writable for APPS_NW_SEMAPHORE12 {}
#[doc = "APPS_NW_SEMAPHORE12"]
pub mod apps_nw_semaphore12;
#[doc = "APPS_SEMAPPHORE_PEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_semapphore_pend](apps_semapphore_pend) module"]
pub type APPS_SEMAPPHORE_PEND = crate::Reg<u32, _APPS_SEMAPPHORE_PEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SEMAPPHORE_PEND;
#[doc = "`read()` method returns [apps_semapphore_pend::R](apps_semapphore_pend::R) reader structure"]
impl crate::Readable for APPS_SEMAPPHORE_PEND {}
#[doc = "`write(|w| ..)` method takes [apps_semapphore_pend::W](apps_semapphore_pend::W) writer structure"]
impl crate::Writable for APPS_SEMAPPHORE_PEND {}
#[doc = "APPS_SEMAPPHORE_PEND"]
pub mod apps_semapphore_pend;
#[doc = "NW_SEMAPPHORE_PEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_semapphore_pend](nw_semapphore_pend) module"]
pub type NW_SEMAPPHORE_PEND = crate::Reg<u32, _NW_SEMAPPHORE_PEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_SEMAPPHORE_PEND;
#[doc = "`read()` method returns [nw_semapphore_pend::R](nw_semapphore_pend::R) reader structure"]
impl crate::Readable for NW_SEMAPPHORE_PEND {}
#[doc = "`write(|w| ..)` method takes [nw_semapphore_pend::W](nw_semapphore_pend::W) writer structure"]
impl crate::Writable for NW_SEMAPPHORE_PEND {}
#[doc = "NW_SEMAPPHORE_PEND"]
pub mod nw_semapphore_pend;
#[doc = "SEMAPHORE_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore_status](semaphore_status) module"]
pub type SEMAPHORE_STATUS = crate::Reg<u32, _SEMAPHORE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE_STATUS;
#[doc = "`read()` method returns [semaphore_status::R](semaphore_status::R) reader structure"]
impl crate::Readable for SEMAPHORE_STATUS {}
#[doc = "`write(|w| ..)` method takes [semaphore_status::W](semaphore_status::W) writer structure"]
impl crate::Writable for SEMAPHORE_STATUS {}
#[doc = "SEMAPHORE_STATUS"]
pub mod semaphore_status;
#[doc = "IDMEM_TIM_UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmem_tim_update](idmem_tim_update) module"]
pub type IDMEM_TIM_UPDATE = crate::Reg<u32, _IDMEM_TIM_UPDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMEM_TIM_UPDATE;
#[doc = "`read()` method returns [idmem_tim_update::R](idmem_tim_update::R) reader structure"]
impl crate::Readable for IDMEM_TIM_UPDATE {}
#[doc = "`write(|w| ..)` method takes [idmem_tim_update::W](idmem_tim_update::W) writer structure"]
impl crate::Writable for IDMEM_TIM_UPDATE {}
#[doc = "IDMEM_TIM_UPDATE"]
pub mod idmem_tim_update;
#[doc = "FPGA_ROM_WR_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpga_rom_wr_en](fpga_rom_wr_en) module"]
pub type FPGA_ROM_WR_EN = crate::Reg<u32, _FPGA_ROM_WR_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPGA_ROM_WR_EN;
#[doc = "`read()` method returns [fpga_rom_wr_en::R](fpga_rom_wr_en::R) reader structure"]
impl crate::Readable for FPGA_ROM_WR_EN {}
#[doc = "`write(|w| ..)` method takes [fpga_rom_wr_en::W](fpga_rom_wr_en::W) writer structure"]
impl crate::Writable for FPGA_ROM_WR_EN {}
#[doc = "FPGA_ROM_WR_EN"]
pub mod fpga_rom_wr_en;
#[doc = "NW_INT_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_mask](nw_int_mask) module"]
pub type NW_INT_MASK = crate::Reg<u32, _NW_INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_MASK;
#[doc = "`read()` method returns [nw_int_mask::R](nw_int_mask::R) reader structure"]
impl crate::Readable for NW_INT_MASK {}
#[doc = "`write(|w| ..)` method takes [nw_int_mask::W](nw_int_mask::W) writer structure"]
impl crate::Writable for NW_INT_MASK {}
#[doc = "NW_INT_MASK"]
pub mod nw_int_mask;
#[doc = "NW_INT_MASK_SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_mask_set](nw_int_mask_set) module"]
pub type NW_INT_MASK_SET = crate::Reg<u32, _NW_INT_MASK_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_MASK_SET;
#[doc = "`read()` method returns [nw_int_mask_set::R](nw_int_mask_set::R) reader structure"]
impl crate::Readable for NW_INT_MASK_SET {}
#[doc = "`write(|w| ..)` method takes [nw_int_mask_set::W](nw_int_mask_set::W) writer structure"]
impl crate::Writable for NW_INT_MASK_SET {}
#[doc = "NW_INT_MASK_SET"]
pub mod nw_int_mask_set;
#[doc = "NW_INT_MASK_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_mask_clr](nw_int_mask_clr) module"]
pub type NW_INT_MASK_CLR = crate::Reg<u32, _NW_INT_MASK_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_MASK_CLR;
#[doc = "`read()` method returns [nw_int_mask_clr::R](nw_int_mask_clr::R) reader structure"]
impl crate::Readable for NW_INT_MASK_CLR {}
#[doc = "`write(|w| ..)` method takes [nw_int_mask_clr::W](nw_int_mask_clr::W) writer structure"]
impl crate::Writable for NW_INT_MASK_CLR {}
#[doc = "NW_INT_MASK_CLR"]
pub mod nw_int_mask_clr;
#[doc = "NW_INT_STS_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_sts_clr](nw_int_sts_clr) module"]
pub type NW_INT_STS_CLR = crate::Reg<u32, _NW_INT_STS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_STS_CLR;
#[doc = "`read()` method returns [nw_int_sts_clr::R](nw_int_sts_clr::R) reader structure"]
impl crate::Readable for NW_INT_STS_CLR {}
#[doc = "`write(|w| ..)` method takes [nw_int_sts_clr::W](nw_int_sts_clr::W) writer structure"]
impl crate::Writable for NW_INT_STS_CLR {}
#[doc = "NW_INT_STS_CLR"]
pub mod nw_int_sts_clr;
#[doc = "NW_INT_ACK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_ack](nw_int_ack) module"]
pub type NW_INT_ACK = crate::Reg<u32, _NW_INT_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_ACK;
#[doc = "`read()` method returns [nw_int_ack::R](nw_int_ack::R) reader structure"]
impl crate::Readable for NW_INT_ACK {}
#[doc = "`write(|w| ..)` method takes [nw_int_ack::W](nw_int_ack::W) writer structure"]
impl crate::Writable for NW_INT_ACK {}
#[doc = "NW_INT_ACK"]
pub mod nw_int_ack;
#[doc = "NW_INT_TRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_trig](nw_int_trig) module"]
pub type NW_INT_TRIG = crate::Reg<u32, _NW_INT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_TRIG;
#[doc = "`read()` method returns [nw_int_trig::R](nw_int_trig::R) reader structure"]
impl crate::Readable for NW_INT_TRIG {}
#[doc = "`write(|w| ..)` method takes [nw_int_trig::W](nw_int_trig::W) writer structure"]
impl crate::Writable for NW_INT_TRIG {}
#[doc = "NW_INT_TRIG"]
pub mod nw_int_trig;
#[doc = "NW_INT_STS_MASKED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_sts_masked](nw_int_sts_masked) module"]
pub type NW_INT_STS_MASKED = crate::Reg<u32, _NW_INT_STS_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_STS_MASKED;
#[doc = "`read()` method returns [nw_int_sts_masked::R](nw_int_sts_masked::R) reader structure"]
impl crate::Readable for NW_INT_STS_MASKED {}
#[doc = "`write(|w| ..)` method takes [nw_int_sts_masked::W](nw_int_sts_masked::W) writer structure"]
impl crate::Writable for NW_INT_STS_MASKED {}
#[doc = "NW_INT_STS_MASKED"]
pub mod nw_int_sts_masked;
#[doc = "NW_INT_STS_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nw_int_sts_raw](nw_int_sts_raw) module"]
pub type NW_INT_STS_RAW = crate::Reg<u32, _NW_INT_STS_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NW_INT_STS_RAW;
#[doc = "`read()` method returns [nw_int_sts_raw::R](nw_int_sts_raw::R) reader structure"]
impl crate::Readable for NW_INT_STS_RAW {}
#[doc = "`write(|w| ..)` method takes [nw_int_sts_raw::W](nw_int_sts_raw::W) writer structure"]
impl crate::Writable for NW_INT_STS_RAW {}
#[doc = "NW_INT_STS_RAW"]
pub mod nw_int_sts_raw;
#[doc = "APPS_INT_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_mask](apps_int_mask) module"]
pub type APPS_INT_MASK = crate::Reg<u32, _APPS_INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_MASK;
#[doc = "`read()` method returns [apps_int_mask::R](apps_int_mask::R) reader structure"]
impl crate::Readable for APPS_INT_MASK {}
#[doc = "`write(|w| ..)` method takes [apps_int_mask::W](apps_int_mask::W) writer structure"]
impl crate::Writable for APPS_INT_MASK {}
#[doc = "APPS_INT_MASK"]
pub mod apps_int_mask;
#[doc = "APPS_INT_MASK_SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_mask_set](apps_int_mask_set) module"]
pub type APPS_INT_MASK_SET = crate::Reg<u32, _APPS_INT_MASK_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_MASK_SET;
#[doc = "`read()` method returns [apps_int_mask_set::R](apps_int_mask_set::R) reader structure"]
impl crate::Readable for APPS_INT_MASK_SET {}
#[doc = "`write(|w| ..)` method takes [apps_int_mask_set::W](apps_int_mask_set::W) writer structure"]
impl crate::Writable for APPS_INT_MASK_SET {}
#[doc = "APPS_INT_MASK_SET"]
pub mod apps_int_mask_set;
#[doc = "APPS_INT_MASK_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_mask_clr](apps_int_mask_clr) module"]
pub type APPS_INT_MASK_CLR = crate::Reg<u32, _APPS_INT_MASK_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_MASK_CLR;
#[doc = "`read()` method returns [apps_int_mask_clr::R](apps_int_mask_clr::R) reader structure"]
impl crate::Readable for APPS_INT_MASK_CLR {}
#[doc = "`write(|w| ..)` method takes [apps_int_mask_clr::W](apps_int_mask_clr::W) writer structure"]
impl crate::Writable for APPS_INT_MASK_CLR {}
#[doc = "APPS_INT_MASK_CLR"]
pub mod apps_int_mask_clr;
#[doc = "APPS_INT_STS_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_sts_clr](apps_int_sts_clr) module"]
pub type APPS_INT_STS_CLR = crate::Reg<u32, _APPS_INT_STS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_STS_CLR;
#[doc = "`read()` method returns [apps_int_sts_clr::R](apps_int_sts_clr::R) reader structure"]
impl crate::Readable for APPS_INT_STS_CLR {}
#[doc = "`write(|w| ..)` method takes [apps_int_sts_clr::W](apps_int_sts_clr::W) writer structure"]
impl crate::Writable for APPS_INT_STS_CLR {}
#[doc = "APPS_INT_STS_CLR"]
pub mod apps_int_sts_clr;
#[doc = "APPS_INT_ACK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_ack](apps_int_ack) module"]
pub type APPS_INT_ACK = crate::Reg<u32, _APPS_INT_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_ACK;
#[doc = "`read()` method returns [apps_int_ack::R](apps_int_ack::R) reader structure"]
impl crate::Readable for APPS_INT_ACK {}
#[doc = "`write(|w| ..)` method takes [apps_int_ack::W](apps_int_ack::W) writer structure"]
impl crate::Writable for APPS_INT_ACK {}
#[doc = "APPS_INT_ACK"]
pub mod apps_int_ack;
#[doc = "APPS_INT_TRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_trig](apps_int_trig) module"]
pub type APPS_INT_TRIG = crate::Reg<u32, _APPS_INT_TRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_TRIG;
#[doc = "`read()` method returns [apps_int_trig::R](apps_int_trig::R) reader structure"]
impl crate::Readable for APPS_INT_TRIG {}
#[doc = "`write(|w| ..)` method takes [apps_int_trig::W](apps_int_trig::W) writer structure"]
impl crate::Writable for APPS_INT_TRIG {}
#[doc = "APPS_INT_TRIG"]
pub mod apps_int_trig;
#[doc = "APPS_INT_STS_MASKED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_sts_masked](apps_int_sts_masked) module"]
pub type APPS_INT_STS_MASKED = crate::Reg<u32, _APPS_INT_STS_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_STS_MASKED;
#[doc = "`read()` method returns [apps_int_sts_masked::R](apps_int_sts_masked::R) reader structure"]
impl crate::Readable for APPS_INT_STS_MASKED {}
#[doc = "`write(|w| ..)` method takes [apps_int_sts_masked::W](apps_int_sts_masked::W) writer structure"]
impl crate::Writable for APPS_INT_STS_MASKED {}
#[doc = "APPS_INT_STS_MASKED"]
pub mod apps_int_sts_masked;
#[doc = "APPS_INT_STS_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_int_sts_raw](apps_int_sts_raw) module"]
pub type APPS_INT_STS_RAW = crate::Reg<u32, _APPS_INT_STS_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_INT_STS_RAW;
#[doc = "`read()` method returns [apps_int_sts_raw::R](apps_int_sts_raw::R) reader structure"]
impl crate::Readable for APPS_INT_STS_RAW {}
#[doc = "`write(|w| ..)` method takes [apps_int_sts_raw::W](apps_int_sts_raw::W) writer structure"]
impl crate::Writable for APPS_INT_STS_RAW {}
#[doc = "APPS_INT_STS_RAW"]
pub mod apps_int_sts_raw;
#[doc = "IDMEM_TIM_UPDATED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmem_tim_updated](idmem_tim_updated) module"]
pub type IDMEM_TIM_UPDATED = crate::Reg<u32, _IDMEM_TIM_UPDATED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMEM_TIM_UPDATED;
#[doc = "`read()` method returns [idmem_tim_updated::R](idmem_tim_updated::R) reader structure"]
impl crate::Readable for IDMEM_TIM_UPDATED {}
#[doc = "`write(|w| ..)` method takes [idmem_tim_updated::W](idmem_tim_updated::W) writer structure"]
impl crate::Writable for IDMEM_TIM_UPDATED {}
#[doc = "IDMEM_TIM_UPDATED"]
pub mod idmem_tim_updated;
#[doc = "APPS_GPIO_TRIG_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_gpio_trig_en](apps_gpio_trig_en) module"]
pub type APPS_GPIO_TRIG_EN = crate::Reg<u32, _APPS_GPIO_TRIG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_GPIO_TRIG_EN;
#[doc = "`read()` method returns [apps_gpio_trig_en::R](apps_gpio_trig_en::R) reader structure"]
impl crate::Readable for APPS_GPIO_TRIG_EN {}
#[doc = "`write(|w| ..)` method takes [apps_gpio_trig_en::W](apps_gpio_trig_en::W) writer structure"]
impl crate::Writable for APPS_GPIO_TRIG_EN {}
#[doc = "APPS_GPIO_TRIG_EN"]
pub mod apps_gpio_trig_en;
#[doc = "EMU_DEBUG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emu_debug_reg](emu_debug_reg) module"]
pub type EMU_DEBUG_REG = crate::Reg<u32, _EMU_DEBUG_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMU_DEBUG_REG;
#[doc = "`read()` method returns [emu_debug_reg::R](emu_debug_reg::R) reader structure"]
impl crate::Readable for EMU_DEBUG_REG {}
#[doc = "`write(|w| ..)` method takes [emu_debug_reg::W](emu_debug_reg::W) writer structure"]
impl crate::Writable for EMU_DEBUG_REG {}
#[doc = "EMU_DEBUG_REG"]
pub mod emu_debug_reg;
#[doc = "SEMAPHORE_STATUS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore_status2](semaphore_status2) module"]
pub type SEMAPHORE_STATUS2 = crate::Reg<u32, _SEMAPHORE_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE_STATUS2;
#[doc = "`read()` method returns [semaphore_status2::R](semaphore_status2::R) reader structure"]
impl crate::Readable for SEMAPHORE_STATUS2 {}
#[doc = "`write(|w| ..)` method takes [semaphore_status2::W](semaphore_status2::W) writer structure"]
impl crate::Writable for SEMAPHORE_STATUS2 {}
#[doc = "SEMAPHORE_STATUS2"]
pub mod semaphore_status2;
#[doc = "SEMAPHORE_PREV_OWNER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore_prev_owner1](semaphore_prev_owner1) module"]
pub type SEMAPHORE_PREV_OWNER1 = crate::Reg<u32, _SEMAPHORE_PREV_OWNER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE_PREV_OWNER1;
#[doc = "`read()` method returns [semaphore_prev_owner1::R](semaphore_prev_owner1::R) reader structure"]
impl crate::Readable for SEMAPHORE_PREV_OWNER1 {}
#[doc = "`write(|w| ..)` method takes [semaphore_prev_owner1::W](semaphore_prev_owner1::W) writer structure"]
impl crate::Writable for SEMAPHORE_PREV_OWNER1 {}
#[doc = "SEMAPHORE_PREV_OWNER1"]
pub mod semaphore_prev_owner1;
#[doc = "SEMAPHORE_PREV_OWNER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore_prev_owner2](semaphore_prev_owner2) module"]
pub type SEMAPHORE_PREV_OWNER2 = crate::Reg<u32, _SEMAPHORE_PREV_OWNER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE_PREV_OWNER2;
#[doc = "`read()` method returns [semaphore_prev_owner2::R](semaphore_prev_owner2::R) reader structure"]
impl crate::Readable for SEMAPHORE_PREV_OWNER2 {}
#[doc = "`write(|w| ..)` method takes [semaphore_prev_owner2::W](semaphore_prev_owner2::W) writer structure"]
impl crate::Writable for SEMAPHORE_PREV_OWNER2 {}
#[doc = "SEMAPHORE_PREV_OWNER2"]
pub mod semaphore_prev_owner2;
