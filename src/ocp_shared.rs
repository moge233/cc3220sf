#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SEMAPHORE1"]
    pub semaphore1: SEMAPHORE1,
    #[doc = "0x04 - SEMAPHORE2"]
    pub semaphore2: SEMAPHORE2,
    #[doc = "0x08 - SEMAPHORE3"]
    pub semaphore3: SEMAPHORE3,
    #[doc = "0x0c - SEMAPHORE4"]
    pub semaphore4: SEMAPHORE4,
    #[doc = "0x10 - SEMAPHORE5"]
    pub semaphore5: SEMAPHORE5,
    #[doc = "0x14 - SEMAPHORE6"]
    pub semaphore6: SEMAPHORE6,
    #[doc = "0x18 - SEMAPHORE7"]
    pub semaphore7: SEMAPHORE7,
    #[doc = "0x1c - SEMAPHORE8"]
    pub semaphore8: SEMAPHORE8,
    #[doc = "0x20 - SEMAPHORE9"]
    pub semaphore9: SEMAPHORE9,
    #[doc = "0x24 - SEMAPHORE10"]
    pub semaphore10: SEMAPHORE10,
    #[doc = "0x28 - SEMAPHORE11"]
    pub semaphore11: SEMAPHORE11,
    #[doc = "0x2c - SEMAPHORE12"]
    pub semaphore12: SEMAPHORE12,
    #[doc = "0x30 - IC_LOCKER_ID"]
    pub ic_locker_id: IC_LOCKER_ID,
    #[doc = "0x34 - MCU_SEMAPHORE_PEND"]
    pub mcu_semaphore_pend: MCU_SEMAPHORE_PEND,
    #[doc = "0x38 - WL_SEMAPHORE_PEND"]
    pub wl_semaphore_pend: WL_SEMAPHORE_PEND,
    #[doc = "0x3c - PLATFORM_DETECTION_RD_ONLY"]
    pub platform_detection_rd_only: PLATFORM_DETECTION_RD_ONLY,
    #[doc = "0x40 - SEMAPHORES_STATUS_RD_ONLY"]
    pub semaphores_status_rd_only: SEMAPHORES_STATUS_RD_ONLY,
    #[doc = "0x44 - CC3XX_CONFIG_CTRL"]
    pub cc3xx_config_ctrl: CC3XX_CONFIG_CTRL,
    #[doc = "0x48 - CC3XX_SHARED_MEM_SEL_LSB"]
    pub cc3xx_shared_mem_sel_lsb: CC3XX_SHARED_MEM_SEL_LSB,
    #[doc = "0x4c - CC3XX_SHARED_MEM_SEL_MSB"]
    pub cc3xx_shared_mem_sel_msb: CC3XX_SHARED_MEM_SEL_MSB,
    #[doc = "0x50 - WLAN_ELP_WAKE_EN"]
    pub wlan_elp_wake_en: WLAN_ELP_WAKE_EN,
    #[doc = "0x54 - DEVINIT_ROM_START_ADDR"]
    pub devinit_rom_start_addr: DEVINIT_ROM_START_ADDR,
    #[doc = "0x58 - DEVINIT_ROM_END_ADDR"]
    pub devinit_rom_end_addr: DEVINIT_ROM_END_ADDR,
    #[doc = "0x5c - SSBD_SEED"]
    pub ssbd_seed: SSBD_SEED,
    #[doc = "0x60 - SSBD_CHK"]
    pub ssbd_chk: SSBD_CHK,
    #[doc = "0x64 - SSBD_POLY_SEL"]
    pub ssbd_poly_sel: SSBD_POLY_SEL,
    #[doc = "0x68 - SPARE_REG_0"]
    pub spare_reg_0: SPARE_REG_0,
    #[doc = "0x6c - SPARE_REG_1"]
    pub spare_reg_1: SPARE_REG_1,
    #[doc = "0x70 - SPARE_REG_2"]
    pub spare_reg_2: SPARE_REG_2,
    #[doc = "0x74 - SPARE_REG_3"]
    pub spare_reg_3: SPARE_REG_3,
    _reserved30: [u8; 40usize],
    #[doc = "0xa0 - GPIO_PAD_CONFIG_0"]
    pub gpio_pad_config_0: GPIO_PAD_CONFIG_0,
    #[doc = "0xa4 - GPIO_PAD_CONFIG_1"]
    pub gpio_pad_config_1: GPIO_PAD_CONFIG_1,
    #[doc = "0xa8 - GPIO_PAD_CONFIG_2"]
    pub gpio_pad_config_2: GPIO_PAD_CONFIG_2,
    #[doc = "0xac - GPIO_PAD_CONFIG_3"]
    pub gpio_pad_config_3: GPIO_PAD_CONFIG_3,
    #[doc = "0xb0 - GPIO_PAD_CONFIG_4"]
    pub gpio_pad_config_4: GPIO_PAD_CONFIG_4,
    #[doc = "0xb4 - GPIO_PAD_CONFIG_5"]
    pub gpio_pad_config_5: GPIO_PAD_CONFIG_5,
    #[doc = "0xb8 - GPIO_PAD_CONFIG_6"]
    pub gpio_pad_config_6: GPIO_PAD_CONFIG_6,
    #[doc = "0xbc - GPIO_PAD_CONFIG_7"]
    pub gpio_pad_config_7: GPIO_PAD_CONFIG_7,
    #[doc = "0xc0 - GPIO_PAD_CONFIG_8"]
    pub gpio_pad_config_8: GPIO_PAD_CONFIG_8,
    #[doc = "0xc4 - GPIO_PAD_CONFIG_9"]
    pub gpio_pad_config_9: GPIO_PAD_CONFIG_9,
    #[doc = "0xc8 - GPIO_PAD_CONFIG_10"]
    pub gpio_pad_config_10: GPIO_PAD_CONFIG_10,
    #[doc = "0xcc - GPIO_PAD_CONFIG_11"]
    pub gpio_pad_config_11: GPIO_PAD_CONFIG_11,
    #[doc = "0xd0 - GPIO_PAD_CONFIG_12"]
    pub gpio_pad_config_12: GPIO_PAD_CONFIG_12,
    #[doc = "0xd4 - GPIO_PAD_CONFIG_13"]
    pub gpio_pad_config_13: GPIO_PAD_CONFIG_13,
    #[doc = "0xd8 - GPIO_PAD_CONFIG_14"]
    pub gpio_pad_config_14: GPIO_PAD_CONFIG_14,
    #[doc = "0xdc - GPIO_PAD_CONFIG_15"]
    pub gpio_pad_config_15: GPIO_PAD_CONFIG_15,
    #[doc = "0xe0 - GPIO_PAD_CONFIG_16"]
    pub gpio_pad_config_16: GPIO_PAD_CONFIG_16,
    #[doc = "0xe4 - GPIO_PAD_CONFIG_17"]
    pub gpio_pad_config_17: GPIO_PAD_CONFIG_17,
    #[doc = "0xe8 - GPIO_PAD_CONFIG_18"]
    pub gpio_pad_config_18: GPIO_PAD_CONFIG_18,
    #[doc = "0xec - GPIO_PAD_CONFIG_19"]
    pub gpio_pad_config_19: GPIO_PAD_CONFIG_19,
    #[doc = "0xf0 - GPIO_PAD_CONFIG_20"]
    pub gpio_pad_config_20: GPIO_PAD_CONFIG_20,
    #[doc = "0xf4 - GPIO_PAD_CONFIG_21"]
    pub gpio_pad_config_21: GPIO_PAD_CONFIG_21,
    #[doc = "0xf8 - GPIO_PAD_CONFIG_22"]
    pub gpio_pad_config_22: GPIO_PAD_CONFIG_22,
    #[doc = "0xfc - GPIO_PAD_CONFIG_23"]
    pub gpio_pad_config_23: GPIO_PAD_CONFIG_23,
    #[doc = "0x100 - GPIO_PAD_CONFIG_24"]
    pub gpio_pad_config_24: GPIO_PAD_CONFIG_24,
    #[doc = "0x104 - GPIO_PAD_CONFIG_25"]
    pub gpio_pad_config_25: GPIO_PAD_CONFIG_25,
    #[doc = "0x108 - GPIO_PAD_CONFIG_26"]
    pub gpio_pad_config_26: GPIO_PAD_CONFIG_26,
    #[doc = "0x10c - GPIO_PAD_CONFIG_27"]
    pub gpio_pad_config_27: GPIO_PAD_CONFIG_27,
    #[doc = "0x110 - GPIO_PAD_CONFIG_28"]
    pub gpio_pad_config_28: GPIO_PAD_CONFIG_28,
    #[doc = "0x114 - GPIO_PAD_CONFIG_29"]
    pub gpio_pad_config_29: GPIO_PAD_CONFIG_29,
    #[doc = "0x118 - GPIO_PAD_CONFIG_30"]
    pub gpio_pad_config_30: GPIO_PAD_CONFIG_30,
    #[doc = "0x11c - GPIO_PAD_CONFIG_31"]
    pub gpio_pad_config_31: GPIO_PAD_CONFIG_31,
    #[doc = "0x120 - GPIO_PAD_CONFIG_32"]
    pub gpio_pad_config_32: GPIO_PAD_CONFIG_32,
    #[doc = "0x124 - GPIO_PAD_CONFIG_33"]
    pub gpio_pad_config_33: GPIO_PAD_CONFIG_33,
    #[doc = "0x128 - GPIO_PAD_CONFIG_34"]
    pub gpio_pad_config_34: GPIO_PAD_CONFIG_34,
    #[doc = "0x12c - GPIO_PAD_CONFIG_35"]
    pub gpio_pad_config_35: GPIO_PAD_CONFIG_35,
    #[doc = "0x130 - GPIO_PAD_CONFIG_36"]
    pub gpio_pad_config_36: GPIO_PAD_CONFIG_36,
    #[doc = "0x134 - GPIO_PAD_CONFIG_37"]
    pub gpio_pad_config_37: GPIO_PAD_CONFIG_37,
    #[doc = "0x138 - GPIO_PAD_CONFIG_38"]
    pub gpio_pad_config_38: GPIO_PAD_CONFIG_38,
    #[doc = "0x13c - GPIO_PAD_CONFIG_39"]
    pub gpio_pad_config_39: GPIO_PAD_CONFIG_39,
    #[doc = "0x140 - GPIO_PAD_CONFIG_40"]
    pub gpio_pad_config_40: GPIO_PAD_CONFIG_40,
    #[doc = "0x144 - This register provide control to GPIO_CC3XXV1 IO PAD. Common control signals to all bottom Die IO's are controlled via this."]
    pub gpio_pad_cmn_config: GPIO_PAD_CMN_CONFIG,
    #[doc = "0x148 - D2D_DEV_PAD_CMN_CONFIG"]
    pub d2d_dev_pad_cmn_config: D2D_DEV_PAD_CMN_CONFIG,
    #[doc = "0x14c - D2D_TOSTACK_PAD_CONF"]
    pub d2d_tostack_pad_conf: D2D_TOSTACK_PAD_CONF,
    #[doc = "0x150 - D2D_MISC_PAD_CONF"]
    pub d2d_misc_pad_conf: D2D_MISC_PAD_CONF,
    #[doc = "0x154 - SOP_CONF_OVERRIDE"]
    pub sop_conf_override: SOP_CONF_OVERRIDE,
    #[doc = "0x158 - CC3XX_DEBUGSS_STATUS"]
    pub cc3xx_debugss_status: CC3XX_DEBUGSS_STATUS,
    #[doc = "0x15c - CC3XX_DEBUGMUX_SEL"]
    pub cc3xx_debugmux_sel: CC3XX_DEBUGMUX_SEL,
    #[doc = "0x160 - ALT_PC_VAL_NW"]
    pub alt_pc_val_nw: ALT_PC_VAL_NW,
    #[doc = "0x164 - ALT_PC_VAL_APPS"]
    pub alt_pc_val_apps: ALT_PC_VAL_APPS,
    #[doc = "0x168 - SPARE_REG_4"]
    pub spare_reg_4: SPARE_REG_4,
    #[doc = "0x16c - SPARE_REG_5"]
    pub spare_reg_5: SPARE_REG_5,
    #[doc = "0x170 - SH_SPI_CS_MASK"]
    pub sh_spi_cs_mask: SH_SPI_CS_MASK,
    #[doc = "0x174 - CC3XX_DEVICE_TYPE"]
    pub cc3xx_device_type: CC3XX_DEVICE_TYPE,
    #[doc = "0x178 - MEM_TOPMUXCTRL_IFORCE"]
    pub mem_topmuxctrl_iforce: MEM_TOPMUXCTRL_IFORCE,
    #[doc = "0x17c - CC3XX_DEV_PACKAGE_DETECT"]
    pub cc3xx_dev_package_detect: CC3XX_DEV_PACKAGE_DETECT,
    #[doc = "0x180 - AUTONMS_SPICLK_SEL"]
    pub autonms_spiclk_sel: AUTONMS_SPICLK_SEL,
    #[doc = "0x184 - CC3XX_DEV_PADCONF"]
    pub cc3xx_dev_padconf: CC3XX_DEV_PADCONF,
    #[doc = "0x188 - SPARE_REG_8"]
    pub spare_reg_8: SPARE_REG_8,
    #[doc = "0x18c - SPARE_REG_6"]
    pub spare_reg_6: SPARE_REG_6,
    #[doc = "0x190 - SPARE_REG_7"]
    pub spare_reg_7: SPARE_REG_7,
    #[doc = "0x194 - APPS_WLAN_ORBIT"]
    pub apps_wlan_orbit: APPS_WLAN_ORBIT,
    #[doc = "0x198 - APPS_WLAN_SCRATCH_PAD"]
    pub apps_wlan_scratch_pad: APPS_WLAN_SCRATCH_PAD,
}
#[doc = "SEMAPHORE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore1](semaphore1) module"]
pub type SEMAPHORE1 = crate::Reg<u32, _SEMAPHORE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE1;
#[doc = "`read()` method returns [semaphore1::R](semaphore1::R) reader structure"]
impl crate::Readable for SEMAPHORE1 {}
#[doc = "`write(|w| ..)` method takes [semaphore1::W](semaphore1::W) writer structure"]
impl crate::Writable for SEMAPHORE1 {}
#[doc = "SEMAPHORE1"]
pub mod semaphore1;
#[doc = "SEMAPHORE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore2](semaphore2) module"]
pub type SEMAPHORE2 = crate::Reg<u32, _SEMAPHORE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE2;
#[doc = "`read()` method returns [semaphore2::R](semaphore2::R) reader structure"]
impl crate::Readable for SEMAPHORE2 {}
#[doc = "`write(|w| ..)` method takes [semaphore2::W](semaphore2::W) writer structure"]
impl crate::Writable for SEMAPHORE2 {}
#[doc = "SEMAPHORE2"]
pub mod semaphore2;
#[doc = "SEMAPHORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore3](semaphore3) module"]
pub type SEMAPHORE3 = crate::Reg<u32, _SEMAPHORE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE3;
#[doc = "`read()` method returns [semaphore3::R](semaphore3::R) reader structure"]
impl crate::Readable for SEMAPHORE3 {}
#[doc = "`write(|w| ..)` method takes [semaphore3::W](semaphore3::W) writer structure"]
impl crate::Writable for SEMAPHORE3 {}
#[doc = "SEMAPHORE3"]
pub mod semaphore3;
#[doc = "SEMAPHORE4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore4](semaphore4) module"]
pub type SEMAPHORE4 = crate::Reg<u32, _SEMAPHORE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE4;
#[doc = "`read()` method returns [semaphore4::R](semaphore4::R) reader structure"]
impl crate::Readable for SEMAPHORE4 {}
#[doc = "`write(|w| ..)` method takes [semaphore4::W](semaphore4::W) writer structure"]
impl crate::Writable for SEMAPHORE4 {}
#[doc = "SEMAPHORE4"]
pub mod semaphore4;
#[doc = "SEMAPHORE5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore5](semaphore5) module"]
pub type SEMAPHORE5 = crate::Reg<u32, _SEMAPHORE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE5;
#[doc = "`read()` method returns [semaphore5::R](semaphore5::R) reader structure"]
impl crate::Readable for SEMAPHORE5 {}
#[doc = "`write(|w| ..)` method takes [semaphore5::W](semaphore5::W) writer structure"]
impl crate::Writable for SEMAPHORE5 {}
#[doc = "SEMAPHORE5"]
pub mod semaphore5;
#[doc = "SEMAPHORE6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore6](semaphore6) module"]
pub type SEMAPHORE6 = crate::Reg<u32, _SEMAPHORE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE6;
#[doc = "`read()` method returns [semaphore6::R](semaphore6::R) reader structure"]
impl crate::Readable for SEMAPHORE6 {}
#[doc = "`write(|w| ..)` method takes [semaphore6::W](semaphore6::W) writer structure"]
impl crate::Writable for SEMAPHORE6 {}
#[doc = "SEMAPHORE6"]
pub mod semaphore6;
#[doc = "SEMAPHORE7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore7](semaphore7) module"]
pub type SEMAPHORE7 = crate::Reg<u32, _SEMAPHORE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE7;
#[doc = "`read()` method returns [semaphore7::R](semaphore7::R) reader structure"]
impl crate::Readable for SEMAPHORE7 {}
#[doc = "`write(|w| ..)` method takes [semaphore7::W](semaphore7::W) writer structure"]
impl crate::Writable for SEMAPHORE7 {}
#[doc = "SEMAPHORE7"]
pub mod semaphore7;
#[doc = "SEMAPHORE8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore8](semaphore8) module"]
pub type SEMAPHORE8 = crate::Reg<u32, _SEMAPHORE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE8;
#[doc = "`read()` method returns [semaphore8::R](semaphore8::R) reader structure"]
impl crate::Readable for SEMAPHORE8 {}
#[doc = "`write(|w| ..)` method takes [semaphore8::W](semaphore8::W) writer structure"]
impl crate::Writable for SEMAPHORE8 {}
#[doc = "SEMAPHORE8"]
pub mod semaphore8;
#[doc = "SEMAPHORE9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore9](semaphore9) module"]
pub type SEMAPHORE9 = crate::Reg<u32, _SEMAPHORE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE9;
#[doc = "`read()` method returns [semaphore9::R](semaphore9::R) reader structure"]
impl crate::Readable for SEMAPHORE9 {}
#[doc = "`write(|w| ..)` method takes [semaphore9::W](semaphore9::W) writer structure"]
impl crate::Writable for SEMAPHORE9 {}
#[doc = "SEMAPHORE9"]
pub mod semaphore9;
#[doc = "SEMAPHORE10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore10](semaphore10) module"]
pub type SEMAPHORE10 = crate::Reg<u32, _SEMAPHORE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE10;
#[doc = "`read()` method returns [semaphore10::R](semaphore10::R) reader structure"]
impl crate::Readable for SEMAPHORE10 {}
#[doc = "`write(|w| ..)` method takes [semaphore10::W](semaphore10::W) writer structure"]
impl crate::Writable for SEMAPHORE10 {}
#[doc = "SEMAPHORE10"]
pub mod semaphore10;
#[doc = "SEMAPHORE11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore11](semaphore11) module"]
pub type SEMAPHORE11 = crate::Reg<u32, _SEMAPHORE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE11;
#[doc = "`read()` method returns [semaphore11::R](semaphore11::R) reader structure"]
impl crate::Readable for SEMAPHORE11 {}
#[doc = "`write(|w| ..)` method takes [semaphore11::W](semaphore11::W) writer structure"]
impl crate::Writable for SEMAPHORE11 {}
#[doc = "SEMAPHORE11"]
pub mod semaphore11;
#[doc = "SEMAPHORE12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphore12](semaphore12) module"]
pub type SEMAPHORE12 = crate::Reg<u32, _SEMAPHORE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORE12;
#[doc = "`read()` method returns [semaphore12::R](semaphore12::R) reader structure"]
impl crate::Readable for SEMAPHORE12 {}
#[doc = "`write(|w| ..)` method takes [semaphore12::W](semaphore12::W) writer structure"]
impl crate::Writable for SEMAPHORE12 {}
#[doc = "SEMAPHORE12"]
pub mod semaphore12;
#[doc = "IC_LOCKER_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_locker_id](ic_locker_id) module"]
pub type IC_LOCKER_ID = crate::Reg<u32, _IC_LOCKER_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC_LOCKER_ID;
#[doc = "`read()` method returns [ic_locker_id::R](ic_locker_id::R) reader structure"]
impl crate::Readable for IC_LOCKER_ID {}
#[doc = "`write(|w| ..)` method takes [ic_locker_id::W](ic_locker_id::W) writer structure"]
impl crate::Writable for IC_LOCKER_ID {}
#[doc = "IC_LOCKER_ID"]
pub mod ic_locker_id;
#[doc = "MCU_SEMAPHORE_PEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_semaphore_pend](mcu_semaphore_pend) module"]
pub type MCU_SEMAPHORE_PEND = crate::Reg<u32, _MCU_SEMAPHORE_PEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_SEMAPHORE_PEND;
#[doc = "`read()` method returns [mcu_semaphore_pend::R](mcu_semaphore_pend::R) reader structure"]
impl crate::Readable for MCU_SEMAPHORE_PEND {}
#[doc = "`write(|w| ..)` method takes [mcu_semaphore_pend::W](mcu_semaphore_pend::W) writer structure"]
impl crate::Writable for MCU_SEMAPHORE_PEND {}
#[doc = "MCU_SEMAPHORE_PEND"]
pub mod mcu_semaphore_pend;
#[doc = "WL_SEMAPHORE_PEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_semaphore_pend](wl_semaphore_pend) module"]
pub type WL_SEMAPHORE_PEND = crate::Reg<u32, _WL_SEMAPHORE_PEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_SEMAPHORE_PEND;
#[doc = "`read()` method returns [wl_semaphore_pend::R](wl_semaphore_pend::R) reader structure"]
impl crate::Readable for WL_SEMAPHORE_PEND {}
#[doc = "`write(|w| ..)` method takes [wl_semaphore_pend::W](wl_semaphore_pend::W) writer structure"]
impl crate::Writable for WL_SEMAPHORE_PEND {}
#[doc = "WL_SEMAPHORE_PEND"]
pub mod wl_semaphore_pend;
#[doc = "PLATFORM_DETECTION_RD_ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [platform_detection_rd_only](platform_detection_rd_only) module"]
pub type PLATFORM_DETECTION_RD_ONLY = crate::Reg<u32, _PLATFORM_DETECTION_RD_ONLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLATFORM_DETECTION_RD_ONLY;
#[doc = "`read()` method returns [platform_detection_rd_only::R](platform_detection_rd_only::R) reader structure"]
impl crate::Readable for PLATFORM_DETECTION_RD_ONLY {}
#[doc = "`write(|w| ..)` method takes [platform_detection_rd_only::W](platform_detection_rd_only::W) writer structure"]
impl crate::Writable for PLATFORM_DETECTION_RD_ONLY {}
#[doc = "PLATFORM_DETECTION_RD_ONLY"]
pub mod platform_detection_rd_only;
#[doc = "SEMAPHORES_STATUS_RD_ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphores_status_rd_only](semaphores_status_rd_only) module"]
pub type SEMAPHORES_STATUS_RD_ONLY = crate::Reg<u32, _SEMAPHORES_STATUS_RD_ONLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORES_STATUS_RD_ONLY;
#[doc = "`read()` method returns [semaphores_status_rd_only::R](semaphores_status_rd_only::R) reader structure"]
impl crate::Readable for SEMAPHORES_STATUS_RD_ONLY {}
#[doc = "`write(|w| ..)` method takes [semaphores_status_rd_only::W](semaphores_status_rd_only::W) writer structure"]
impl crate::Writable for SEMAPHORES_STATUS_RD_ONLY {}
#[doc = "SEMAPHORES_STATUS_RD_ONLY"]
pub mod semaphores_status_rd_only;
#[doc = "CC3XX_CONFIG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_config_ctrl](cc3xx_config_ctrl) module"]
pub type CC3XX_CONFIG_CTRL = crate::Reg<u32, _CC3XX_CONFIG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_CONFIG_CTRL;
#[doc = "`read()` method returns [cc3xx_config_ctrl::R](cc3xx_config_ctrl::R) reader structure"]
impl crate::Readable for CC3XX_CONFIG_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc3xx_config_ctrl::W](cc3xx_config_ctrl::W) writer structure"]
impl crate::Writable for CC3XX_CONFIG_CTRL {}
#[doc = "CC3XX_CONFIG_CTRL"]
pub mod cc3xx_config_ctrl;
#[doc = "CC3XX_SHARED_MEM_SEL_LSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_shared_mem_sel_lsb](cc3xx_shared_mem_sel_lsb) module"]
pub type CC3XX_SHARED_MEM_SEL_LSB = crate::Reg<u32, _CC3XX_SHARED_MEM_SEL_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_SHARED_MEM_SEL_LSB;
#[doc = "`read()` method returns [cc3xx_shared_mem_sel_lsb::R](cc3xx_shared_mem_sel_lsb::R) reader structure"]
impl crate::Readable for CC3XX_SHARED_MEM_SEL_LSB {}
#[doc = "`write(|w| ..)` method takes [cc3xx_shared_mem_sel_lsb::W](cc3xx_shared_mem_sel_lsb::W) writer structure"]
impl crate::Writable for CC3XX_SHARED_MEM_SEL_LSB {}
#[doc = "CC3XX_SHARED_MEM_SEL_LSB"]
pub mod cc3xx_shared_mem_sel_lsb;
#[doc = "CC3XX_SHARED_MEM_SEL_MSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_shared_mem_sel_msb](cc3xx_shared_mem_sel_msb) module"]
pub type CC3XX_SHARED_MEM_SEL_MSB = crate::Reg<u32, _CC3XX_SHARED_MEM_SEL_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_SHARED_MEM_SEL_MSB;
#[doc = "`read()` method returns [cc3xx_shared_mem_sel_msb::R](cc3xx_shared_mem_sel_msb::R) reader structure"]
impl crate::Readable for CC3XX_SHARED_MEM_SEL_MSB {}
#[doc = "`write(|w| ..)` method takes [cc3xx_shared_mem_sel_msb::W](cc3xx_shared_mem_sel_msb::W) writer structure"]
impl crate::Writable for CC3XX_SHARED_MEM_SEL_MSB {}
#[doc = "CC3XX_SHARED_MEM_SEL_MSB"]
pub mod cc3xx_shared_mem_sel_msb;
#[doc = "WLAN_ELP_WAKE_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlan_elp_wake_en](wlan_elp_wake_en) module"]
pub type WLAN_ELP_WAKE_EN = crate::Reg<u32, _WLAN_ELP_WAKE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLAN_ELP_WAKE_EN;
#[doc = "`read()` method returns [wlan_elp_wake_en::R](wlan_elp_wake_en::R) reader structure"]
impl crate::Readable for WLAN_ELP_WAKE_EN {}
#[doc = "`write(|w| ..)` method takes [wlan_elp_wake_en::W](wlan_elp_wake_en::W) writer structure"]
impl crate::Writable for WLAN_ELP_WAKE_EN {}
#[doc = "WLAN_ELP_WAKE_EN"]
pub mod wlan_elp_wake_en;
#[doc = "DEVINIT_ROM_START_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinit_rom_start_addr](devinit_rom_start_addr) module"]
pub type DEVINIT_ROM_START_ADDR = crate::Reg<u32, _DEVINIT_ROM_START_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINIT_ROM_START_ADDR;
#[doc = "`read()` method returns [devinit_rom_start_addr::R](devinit_rom_start_addr::R) reader structure"]
impl crate::Readable for DEVINIT_ROM_START_ADDR {}
#[doc = "`write(|w| ..)` method takes [devinit_rom_start_addr::W](devinit_rom_start_addr::W) writer structure"]
impl crate::Writable for DEVINIT_ROM_START_ADDR {}
#[doc = "DEVINIT_ROM_START_ADDR"]
pub mod devinit_rom_start_addr;
#[doc = "DEVINIT_ROM_END_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinit_rom_end_addr](devinit_rom_end_addr) module"]
pub type DEVINIT_ROM_END_ADDR = crate::Reg<u32, _DEVINIT_ROM_END_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINIT_ROM_END_ADDR;
#[doc = "`read()` method returns [devinit_rom_end_addr::R](devinit_rom_end_addr::R) reader structure"]
impl crate::Readable for DEVINIT_ROM_END_ADDR {}
#[doc = "`write(|w| ..)` method takes [devinit_rom_end_addr::W](devinit_rom_end_addr::W) writer structure"]
impl crate::Writable for DEVINIT_ROM_END_ADDR {}
#[doc = "DEVINIT_ROM_END_ADDR"]
pub mod devinit_rom_end_addr;
#[doc = "SSBD_SEED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssbd_seed](ssbd_seed) module"]
pub type SSBD_SEED = crate::Reg<u32, _SSBD_SEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSBD_SEED;
#[doc = "`read()` method returns [ssbd_seed::R](ssbd_seed::R) reader structure"]
impl crate::Readable for SSBD_SEED {}
#[doc = "`write(|w| ..)` method takes [ssbd_seed::W](ssbd_seed::W) writer structure"]
impl crate::Writable for SSBD_SEED {}
#[doc = "SSBD_SEED"]
pub mod ssbd_seed;
#[doc = "SSBD_CHK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssbd_chk](ssbd_chk) module"]
pub type SSBD_CHK = crate::Reg<u32, _SSBD_CHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSBD_CHK;
#[doc = "`read()` method returns [ssbd_chk::R](ssbd_chk::R) reader structure"]
impl crate::Readable for SSBD_CHK {}
#[doc = "`write(|w| ..)` method takes [ssbd_chk::W](ssbd_chk::W) writer structure"]
impl crate::Writable for SSBD_CHK {}
#[doc = "SSBD_CHK"]
pub mod ssbd_chk;
#[doc = "SSBD_POLY_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssbd_poly_sel](ssbd_poly_sel) module"]
pub type SSBD_POLY_SEL = crate::Reg<u32, _SSBD_POLY_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSBD_POLY_SEL;
#[doc = "`read()` method returns [ssbd_poly_sel::R](ssbd_poly_sel::R) reader structure"]
impl crate::Readable for SSBD_POLY_SEL {}
#[doc = "`write(|w| ..)` method takes [ssbd_poly_sel::W](ssbd_poly_sel::W) writer structure"]
impl crate::Writable for SSBD_POLY_SEL {}
#[doc = "SSBD_POLY_SEL"]
pub mod ssbd_poly_sel;
#[doc = "SPARE_REG_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_0](spare_reg_0) module"]
pub type SPARE_REG_0 = crate::Reg<u32, _SPARE_REG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_0;
#[doc = "`read()` method returns [spare_reg_0::R](spare_reg_0::R) reader structure"]
impl crate::Readable for SPARE_REG_0 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_0::W](spare_reg_0::W) writer structure"]
impl crate::Writable for SPARE_REG_0 {}
#[doc = "SPARE_REG_0"]
pub mod spare_reg_0;
#[doc = "SPARE_REG_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_1](spare_reg_1) module"]
pub type SPARE_REG_1 = crate::Reg<u32, _SPARE_REG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_1;
#[doc = "`read()` method returns [spare_reg_1::R](spare_reg_1::R) reader structure"]
impl crate::Readable for SPARE_REG_1 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_1::W](spare_reg_1::W) writer structure"]
impl crate::Writable for SPARE_REG_1 {}
#[doc = "SPARE_REG_1"]
pub mod spare_reg_1;
#[doc = "SPARE_REG_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_2](spare_reg_2) module"]
pub type SPARE_REG_2 = crate::Reg<u32, _SPARE_REG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_2;
#[doc = "`read()` method returns [spare_reg_2::R](spare_reg_2::R) reader structure"]
impl crate::Readable for SPARE_REG_2 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_2::W](spare_reg_2::W) writer structure"]
impl crate::Writable for SPARE_REG_2 {}
#[doc = "SPARE_REG_2"]
pub mod spare_reg_2;
#[doc = "SPARE_REG_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_3](spare_reg_3) module"]
pub type SPARE_REG_3 = crate::Reg<u32, _SPARE_REG_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_3;
#[doc = "`read()` method returns [spare_reg_3::R](spare_reg_3::R) reader structure"]
impl crate::Readable for SPARE_REG_3 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_3::W](spare_reg_3::W) writer structure"]
impl crate::Writable for SPARE_REG_3 {}
#[doc = "SPARE_REG_3"]
pub mod spare_reg_3;
#[doc = "GPIO_PAD_CONFIG_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_0](gpio_pad_config_0) module"]
pub type GPIO_PAD_CONFIG_0 = crate::Reg<u32, _GPIO_PAD_CONFIG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_0;
#[doc = "`read()` method returns [gpio_pad_config_0::R](gpio_pad_config_0::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_0 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_0::W](gpio_pad_config_0::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_0 {}
#[doc = "GPIO_PAD_CONFIG_0"]
pub mod gpio_pad_config_0;
#[doc = "GPIO_PAD_CONFIG_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_1](gpio_pad_config_1) module"]
pub type GPIO_PAD_CONFIG_1 = crate::Reg<u32, _GPIO_PAD_CONFIG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_1;
#[doc = "`read()` method returns [gpio_pad_config_1::R](gpio_pad_config_1::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_1 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_1::W](gpio_pad_config_1::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_1 {}
#[doc = "GPIO_PAD_CONFIG_1"]
pub mod gpio_pad_config_1;
#[doc = "GPIO_PAD_CONFIG_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_2](gpio_pad_config_2) module"]
pub type GPIO_PAD_CONFIG_2 = crate::Reg<u32, _GPIO_PAD_CONFIG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_2;
#[doc = "`read()` method returns [gpio_pad_config_2::R](gpio_pad_config_2::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_2 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_2::W](gpio_pad_config_2::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_2 {}
#[doc = "GPIO_PAD_CONFIG_2"]
pub mod gpio_pad_config_2;
#[doc = "GPIO_PAD_CONFIG_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_3](gpio_pad_config_3) module"]
pub type GPIO_PAD_CONFIG_3 = crate::Reg<u32, _GPIO_PAD_CONFIG_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_3;
#[doc = "`read()` method returns [gpio_pad_config_3::R](gpio_pad_config_3::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_3 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_3::W](gpio_pad_config_3::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_3 {}
#[doc = "GPIO_PAD_CONFIG_3"]
pub mod gpio_pad_config_3;
#[doc = "GPIO_PAD_CONFIG_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_4](gpio_pad_config_4) module"]
pub type GPIO_PAD_CONFIG_4 = crate::Reg<u32, _GPIO_PAD_CONFIG_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_4;
#[doc = "`read()` method returns [gpio_pad_config_4::R](gpio_pad_config_4::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_4 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_4::W](gpio_pad_config_4::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_4 {}
#[doc = "GPIO_PAD_CONFIG_4"]
pub mod gpio_pad_config_4;
#[doc = "GPIO_PAD_CONFIG_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_5](gpio_pad_config_5) module"]
pub type GPIO_PAD_CONFIG_5 = crate::Reg<u32, _GPIO_PAD_CONFIG_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_5;
#[doc = "`read()` method returns [gpio_pad_config_5::R](gpio_pad_config_5::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_5 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_5::W](gpio_pad_config_5::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_5 {}
#[doc = "GPIO_PAD_CONFIG_5"]
pub mod gpio_pad_config_5;
#[doc = "GPIO_PAD_CONFIG_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_6](gpio_pad_config_6) module"]
pub type GPIO_PAD_CONFIG_6 = crate::Reg<u32, _GPIO_PAD_CONFIG_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_6;
#[doc = "`read()` method returns [gpio_pad_config_6::R](gpio_pad_config_6::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_6 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_6::W](gpio_pad_config_6::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_6 {}
#[doc = "GPIO_PAD_CONFIG_6"]
pub mod gpio_pad_config_6;
#[doc = "GPIO_PAD_CONFIG_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_7](gpio_pad_config_7) module"]
pub type GPIO_PAD_CONFIG_7 = crate::Reg<u32, _GPIO_PAD_CONFIG_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_7;
#[doc = "`read()` method returns [gpio_pad_config_7::R](gpio_pad_config_7::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_7 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_7::W](gpio_pad_config_7::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_7 {}
#[doc = "GPIO_PAD_CONFIG_7"]
pub mod gpio_pad_config_7;
#[doc = "GPIO_PAD_CONFIG_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_8](gpio_pad_config_8) module"]
pub type GPIO_PAD_CONFIG_8 = crate::Reg<u32, _GPIO_PAD_CONFIG_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_8;
#[doc = "`read()` method returns [gpio_pad_config_8::R](gpio_pad_config_8::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_8 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_8::W](gpio_pad_config_8::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_8 {}
#[doc = "GPIO_PAD_CONFIG_8"]
pub mod gpio_pad_config_8;
#[doc = "GPIO_PAD_CONFIG_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_9](gpio_pad_config_9) module"]
pub type GPIO_PAD_CONFIG_9 = crate::Reg<u32, _GPIO_PAD_CONFIG_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_9;
#[doc = "`read()` method returns [gpio_pad_config_9::R](gpio_pad_config_9::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_9 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_9::W](gpio_pad_config_9::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_9 {}
#[doc = "GPIO_PAD_CONFIG_9"]
pub mod gpio_pad_config_9;
#[doc = "GPIO_PAD_CONFIG_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_10](gpio_pad_config_10) module"]
pub type GPIO_PAD_CONFIG_10 = crate::Reg<u32, _GPIO_PAD_CONFIG_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_10;
#[doc = "`read()` method returns [gpio_pad_config_10::R](gpio_pad_config_10::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_10 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_10::W](gpio_pad_config_10::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_10 {}
#[doc = "GPIO_PAD_CONFIG_10"]
pub mod gpio_pad_config_10;
#[doc = "GPIO_PAD_CONFIG_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_11](gpio_pad_config_11) module"]
pub type GPIO_PAD_CONFIG_11 = crate::Reg<u32, _GPIO_PAD_CONFIG_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_11;
#[doc = "`read()` method returns [gpio_pad_config_11::R](gpio_pad_config_11::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_11 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_11::W](gpio_pad_config_11::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_11 {}
#[doc = "GPIO_PAD_CONFIG_11"]
pub mod gpio_pad_config_11;
#[doc = "GPIO_PAD_CONFIG_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_12](gpio_pad_config_12) module"]
pub type GPIO_PAD_CONFIG_12 = crate::Reg<u32, _GPIO_PAD_CONFIG_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_12;
#[doc = "`read()` method returns [gpio_pad_config_12::R](gpio_pad_config_12::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_12 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_12::W](gpio_pad_config_12::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_12 {}
#[doc = "GPIO_PAD_CONFIG_12"]
pub mod gpio_pad_config_12;
#[doc = "GPIO_PAD_CONFIG_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_13](gpio_pad_config_13) module"]
pub type GPIO_PAD_CONFIG_13 = crate::Reg<u32, _GPIO_PAD_CONFIG_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_13;
#[doc = "`read()` method returns [gpio_pad_config_13::R](gpio_pad_config_13::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_13 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_13::W](gpio_pad_config_13::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_13 {}
#[doc = "GPIO_PAD_CONFIG_13"]
pub mod gpio_pad_config_13;
#[doc = "GPIO_PAD_CONFIG_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_14](gpio_pad_config_14) module"]
pub type GPIO_PAD_CONFIG_14 = crate::Reg<u32, _GPIO_PAD_CONFIG_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_14;
#[doc = "`read()` method returns [gpio_pad_config_14::R](gpio_pad_config_14::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_14 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_14::W](gpio_pad_config_14::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_14 {}
#[doc = "GPIO_PAD_CONFIG_14"]
pub mod gpio_pad_config_14;
#[doc = "GPIO_PAD_CONFIG_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_15](gpio_pad_config_15) module"]
pub type GPIO_PAD_CONFIG_15 = crate::Reg<u32, _GPIO_PAD_CONFIG_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_15;
#[doc = "`read()` method returns [gpio_pad_config_15::R](gpio_pad_config_15::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_15 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_15::W](gpio_pad_config_15::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_15 {}
#[doc = "GPIO_PAD_CONFIG_15"]
pub mod gpio_pad_config_15;
#[doc = "GPIO_PAD_CONFIG_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_16](gpio_pad_config_16) module"]
pub type GPIO_PAD_CONFIG_16 = crate::Reg<u32, _GPIO_PAD_CONFIG_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_16;
#[doc = "`read()` method returns [gpio_pad_config_16::R](gpio_pad_config_16::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_16 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_16::W](gpio_pad_config_16::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_16 {}
#[doc = "GPIO_PAD_CONFIG_16"]
pub mod gpio_pad_config_16;
#[doc = "GPIO_PAD_CONFIG_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_17](gpio_pad_config_17) module"]
pub type GPIO_PAD_CONFIG_17 = crate::Reg<u32, _GPIO_PAD_CONFIG_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_17;
#[doc = "`read()` method returns [gpio_pad_config_17::R](gpio_pad_config_17::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_17 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_17::W](gpio_pad_config_17::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_17 {}
#[doc = "GPIO_PAD_CONFIG_17"]
pub mod gpio_pad_config_17;
#[doc = "GPIO_PAD_CONFIG_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_18](gpio_pad_config_18) module"]
pub type GPIO_PAD_CONFIG_18 = crate::Reg<u32, _GPIO_PAD_CONFIG_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_18;
#[doc = "`read()` method returns [gpio_pad_config_18::R](gpio_pad_config_18::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_18 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_18::W](gpio_pad_config_18::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_18 {}
#[doc = "GPIO_PAD_CONFIG_18"]
pub mod gpio_pad_config_18;
#[doc = "GPIO_PAD_CONFIG_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_19](gpio_pad_config_19) module"]
pub type GPIO_PAD_CONFIG_19 = crate::Reg<u32, _GPIO_PAD_CONFIG_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_19;
#[doc = "`read()` method returns [gpio_pad_config_19::R](gpio_pad_config_19::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_19 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_19::W](gpio_pad_config_19::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_19 {}
#[doc = "GPIO_PAD_CONFIG_19"]
pub mod gpio_pad_config_19;
#[doc = "GPIO_PAD_CONFIG_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_20](gpio_pad_config_20) module"]
pub type GPIO_PAD_CONFIG_20 = crate::Reg<u32, _GPIO_PAD_CONFIG_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_20;
#[doc = "`read()` method returns [gpio_pad_config_20::R](gpio_pad_config_20::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_20 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_20::W](gpio_pad_config_20::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_20 {}
#[doc = "GPIO_PAD_CONFIG_20"]
pub mod gpio_pad_config_20;
#[doc = "GPIO_PAD_CONFIG_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_21](gpio_pad_config_21) module"]
pub type GPIO_PAD_CONFIG_21 = crate::Reg<u32, _GPIO_PAD_CONFIG_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_21;
#[doc = "`read()` method returns [gpio_pad_config_21::R](gpio_pad_config_21::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_21 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_21::W](gpio_pad_config_21::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_21 {}
#[doc = "GPIO_PAD_CONFIG_21"]
pub mod gpio_pad_config_21;
#[doc = "GPIO_PAD_CONFIG_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_22](gpio_pad_config_22) module"]
pub type GPIO_PAD_CONFIG_22 = crate::Reg<u32, _GPIO_PAD_CONFIG_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_22;
#[doc = "`read()` method returns [gpio_pad_config_22::R](gpio_pad_config_22::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_22 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_22::W](gpio_pad_config_22::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_22 {}
#[doc = "GPIO_PAD_CONFIG_22"]
pub mod gpio_pad_config_22;
#[doc = "GPIO_PAD_CONFIG_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_23](gpio_pad_config_23) module"]
pub type GPIO_PAD_CONFIG_23 = crate::Reg<u32, _GPIO_PAD_CONFIG_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_23;
#[doc = "`read()` method returns [gpio_pad_config_23::R](gpio_pad_config_23::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_23 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_23::W](gpio_pad_config_23::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_23 {}
#[doc = "GPIO_PAD_CONFIG_23"]
pub mod gpio_pad_config_23;
#[doc = "GPIO_PAD_CONFIG_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_24](gpio_pad_config_24) module"]
pub type GPIO_PAD_CONFIG_24 = crate::Reg<u32, _GPIO_PAD_CONFIG_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_24;
#[doc = "`read()` method returns [gpio_pad_config_24::R](gpio_pad_config_24::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_24 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_24::W](gpio_pad_config_24::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_24 {}
#[doc = "GPIO_PAD_CONFIG_24"]
pub mod gpio_pad_config_24;
#[doc = "GPIO_PAD_CONFIG_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_25](gpio_pad_config_25) module"]
pub type GPIO_PAD_CONFIG_25 = crate::Reg<u32, _GPIO_PAD_CONFIG_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_25;
#[doc = "`read()` method returns [gpio_pad_config_25::R](gpio_pad_config_25::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_25 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_25::W](gpio_pad_config_25::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_25 {}
#[doc = "GPIO_PAD_CONFIG_25"]
pub mod gpio_pad_config_25;
#[doc = "GPIO_PAD_CONFIG_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_26](gpio_pad_config_26) module"]
pub type GPIO_PAD_CONFIG_26 = crate::Reg<u32, _GPIO_PAD_CONFIG_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_26;
#[doc = "`read()` method returns [gpio_pad_config_26::R](gpio_pad_config_26::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_26 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_26::W](gpio_pad_config_26::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_26 {}
#[doc = "GPIO_PAD_CONFIG_26"]
pub mod gpio_pad_config_26;
#[doc = "GPIO_PAD_CONFIG_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_27](gpio_pad_config_27) module"]
pub type GPIO_PAD_CONFIG_27 = crate::Reg<u32, _GPIO_PAD_CONFIG_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_27;
#[doc = "`read()` method returns [gpio_pad_config_27::R](gpio_pad_config_27::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_27 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_27::W](gpio_pad_config_27::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_27 {}
#[doc = "GPIO_PAD_CONFIG_27"]
pub mod gpio_pad_config_27;
#[doc = "GPIO_PAD_CONFIG_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_28](gpio_pad_config_28) module"]
pub type GPIO_PAD_CONFIG_28 = crate::Reg<u32, _GPIO_PAD_CONFIG_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_28;
#[doc = "`read()` method returns [gpio_pad_config_28::R](gpio_pad_config_28::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_28 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_28::W](gpio_pad_config_28::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_28 {}
#[doc = "GPIO_PAD_CONFIG_28"]
pub mod gpio_pad_config_28;
#[doc = "GPIO_PAD_CONFIG_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_29](gpio_pad_config_29) module"]
pub type GPIO_PAD_CONFIG_29 = crate::Reg<u32, _GPIO_PAD_CONFIG_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_29;
#[doc = "`read()` method returns [gpio_pad_config_29::R](gpio_pad_config_29::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_29 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_29::W](gpio_pad_config_29::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_29 {}
#[doc = "GPIO_PAD_CONFIG_29"]
pub mod gpio_pad_config_29;
#[doc = "GPIO_PAD_CONFIG_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_30](gpio_pad_config_30) module"]
pub type GPIO_PAD_CONFIG_30 = crate::Reg<u32, _GPIO_PAD_CONFIG_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_30;
#[doc = "`read()` method returns [gpio_pad_config_30::R](gpio_pad_config_30::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_30 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_30::W](gpio_pad_config_30::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_30 {}
#[doc = "GPIO_PAD_CONFIG_30"]
pub mod gpio_pad_config_30;
#[doc = "GPIO_PAD_CONFIG_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_31](gpio_pad_config_31) module"]
pub type GPIO_PAD_CONFIG_31 = crate::Reg<u32, _GPIO_PAD_CONFIG_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_31;
#[doc = "`read()` method returns [gpio_pad_config_31::R](gpio_pad_config_31::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_31 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_31::W](gpio_pad_config_31::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_31 {}
#[doc = "GPIO_PAD_CONFIG_31"]
pub mod gpio_pad_config_31;
#[doc = "GPIO_PAD_CONFIG_32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_32](gpio_pad_config_32) module"]
pub type GPIO_PAD_CONFIG_32 = crate::Reg<u32, _GPIO_PAD_CONFIG_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_32;
#[doc = "`read()` method returns [gpio_pad_config_32::R](gpio_pad_config_32::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_32 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_32::W](gpio_pad_config_32::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_32 {}
#[doc = "GPIO_PAD_CONFIG_32"]
pub mod gpio_pad_config_32;
#[doc = "GPIO_PAD_CONFIG_33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_33](gpio_pad_config_33) module"]
pub type GPIO_PAD_CONFIG_33 = crate::Reg<u32, _GPIO_PAD_CONFIG_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_33;
#[doc = "`read()` method returns [gpio_pad_config_33::R](gpio_pad_config_33::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_33 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_33::W](gpio_pad_config_33::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_33 {}
#[doc = "GPIO_PAD_CONFIG_33"]
pub mod gpio_pad_config_33;
#[doc = "GPIO_PAD_CONFIG_34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_34](gpio_pad_config_34) module"]
pub type GPIO_PAD_CONFIG_34 = crate::Reg<u32, _GPIO_PAD_CONFIG_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_34;
#[doc = "`read()` method returns [gpio_pad_config_34::R](gpio_pad_config_34::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_34 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_34::W](gpio_pad_config_34::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_34 {}
#[doc = "GPIO_PAD_CONFIG_34"]
pub mod gpio_pad_config_34;
#[doc = "GPIO_PAD_CONFIG_35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_35](gpio_pad_config_35) module"]
pub type GPIO_PAD_CONFIG_35 = crate::Reg<u32, _GPIO_PAD_CONFIG_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_35;
#[doc = "`read()` method returns [gpio_pad_config_35::R](gpio_pad_config_35::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_35 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_35::W](gpio_pad_config_35::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_35 {}
#[doc = "GPIO_PAD_CONFIG_35"]
pub mod gpio_pad_config_35;
#[doc = "GPIO_PAD_CONFIG_36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_36](gpio_pad_config_36) module"]
pub type GPIO_PAD_CONFIG_36 = crate::Reg<u32, _GPIO_PAD_CONFIG_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_36;
#[doc = "`read()` method returns [gpio_pad_config_36::R](gpio_pad_config_36::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_36 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_36::W](gpio_pad_config_36::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_36 {}
#[doc = "GPIO_PAD_CONFIG_36"]
pub mod gpio_pad_config_36;
#[doc = "GPIO_PAD_CONFIG_37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_37](gpio_pad_config_37) module"]
pub type GPIO_PAD_CONFIG_37 = crate::Reg<u32, _GPIO_PAD_CONFIG_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_37;
#[doc = "`read()` method returns [gpio_pad_config_37::R](gpio_pad_config_37::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_37 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_37::W](gpio_pad_config_37::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_37 {}
#[doc = "GPIO_PAD_CONFIG_37"]
pub mod gpio_pad_config_37;
#[doc = "GPIO_PAD_CONFIG_38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_38](gpio_pad_config_38) module"]
pub type GPIO_PAD_CONFIG_38 = crate::Reg<u32, _GPIO_PAD_CONFIG_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_38;
#[doc = "`read()` method returns [gpio_pad_config_38::R](gpio_pad_config_38::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_38 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_38::W](gpio_pad_config_38::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_38 {}
#[doc = "GPIO_PAD_CONFIG_38"]
pub mod gpio_pad_config_38;
#[doc = "GPIO_PAD_CONFIG_39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_39](gpio_pad_config_39) module"]
pub type GPIO_PAD_CONFIG_39 = crate::Reg<u32, _GPIO_PAD_CONFIG_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_39;
#[doc = "`read()` method returns [gpio_pad_config_39::R](gpio_pad_config_39::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_39 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_39::W](gpio_pad_config_39::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_39 {}
#[doc = "GPIO_PAD_CONFIG_39"]
pub mod gpio_pad_config_39;
#[doc = "GPIO_PAD_CONFIG_40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_config_40](gpio_pad_config_40) module"]
pub type GPIO_PAD_CONFIG_40 = crate::Reg<u32, _GPIO_PAD_CONFIG_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CONFIG_40;
#[doc = "`read()` method returns [gpio_pad_config_40::R](gpio_pad_config_40::R) reader structure"]
impl crate::Readable for GPIO_PAD_CONFIG_40 {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_config_40::W](gpio_pad_config_40::W) writer structure"]
impl crate::Writable for GPIO_PAD_CONFIG_40 {}
#[doc = "GPIO_PAD_CONFIG_40"]
pub mod gpio_pad_config_40;
#[doc = "This register provide control to GPIO_CC3XXV1 IO PAD. Common control signals to all bottom Die IO's are controlled via this.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pad_cmn_config](gpio_pad_cmn_config) module"]
pub type GPIO_PAD_CMN_CONFIG = crate::Reg<u32, _GPIO_PAD_CMN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_PAD_CMN_CONFIG;
#[doc = "`read()` method returns [gpio_pad_cmn_config::R](gpio_pad_cmn_config::R) reader structure"]
impl crate::Readable for GPIO_PAD_CMN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [gpio_pad_cmn_config::W](gpio_pad_cmn_config::W) writer structure"]
impl crate::Writable for GPIO_PAD_CMN_CONFIG {}
#[doc = "This register provide control to GPIO_CC3XXV1 IO PAD. Common control signals to all bottom Die IO's are controlled via this."]
pub mod gpio_pad_cmn_config;
#[doc = "D2D_DEV_PAD_CMN_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2d_dev_pad_cmn_config](d2d_dev_pad_cmn_config) module"]
pub type D2D_DEV_PAD_CMN_CONFIG = crate::Reg<u32, _D2D_DEV_PAD_CMN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2D_DEV_PAD_CMN_CONFIG;
#[doc = "`read()` method returns [d2d_dev_pad_cmn_config::R](d2d_dev_pad_cmn_config::R) reader structure"]
impl crate::Readable for D2D_DEV_PAD_CMN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [d2d_dev_pad_cmn_config::W](d2d_dev_pad_cmn_config::W) writer structure"]
impl crate::Writable for D2D_DEV_PAD_CMN_CONFIG {}
#[doc = "D2D_DEV_PAD_CMN_CONFIG"]
pub mod d2d_dev_pad_cmn_config;
#[doc = "D2D_TOSTACK_PAD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2d_tostack_pad_conf](d2d_tostack_pad_conf) module"]
pub type D2D_TOSTACK_PAD_CONF = crate::Reg<u32, _D2D_TOSTACK_PAD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2D_TOSTACK_PAD_CONF;
#[doc = "`read()` method returns [d2d_tostack_pad_conf::R](d2d_tostack_pad_conf::R) reader structure"]
impl crate::Readable for D2D_TOSTACK_PAD_CONF {}
#[doc = "`write(|w| ..)` method takes [d2d_tostack_pad_conf::W](d2d_tostack_pad_conf::W) writer structure"]
impl crate::Writable for D2D_TOSTACK_PAD_CONF {}
#[doc = "D2D_TOSTACK_PAD_CONF"]
pub mod d2d_tostack_pad_conf;
#[doc = "D2D_MISC_PAD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2d_misc_pad_conf](d2d_misc_pad_conf) module"]
pub type D2D_MISC_PAD_CONF = crate::Reg<u32, _D2D_MISC_PAD_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2D_MISC_PAD_CONF;
#[doc = "`read()` method returns [d2d_misc_pad_conf::R](d2d_misc_pad_conf::R) reader structure"]
impl crate::Readable for D2D_MISC_PAD_CONF {}
#[doc = "`write(|w| ..)` method takes [d2d_misc_pad_conf::W](d2d_misc_pad_conf::W) writer structure"]
impl crate::Writable for D2D_MISC_PAD_CONF {}
#[doc = "D2D_MISC_PAD_CONF"]
pub mod d2d_misc_pad_conf;
#[doc = "SOP_CONF_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sop_conf_override](sop_conf_override) module"]
pub type SOP_CONF_OVERRIDE = crate::Reg<u32, _SOP_CONF_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOP_CONF_OVERRIDE;
#[doc = "`read()` method returns [sop_conf_override::R](sop_conf_override::R) reader structure"]
impl crate::Readable for SOP_CONF_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [sop_conf_override::W](sop_conf_override::W) writer structure"]
impl crate::Writable for SOP_CONF_OVERRIDE {}
#[doc = "SOP_CONF_OVERRIDE"]
pub mod sop_conf_override;
#[doc = "CC3XX_DEBUGSS_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_debugss_status](cc3xx_debugss_status) module"]
pub type CC3XX_DEBUGSS_STATUS = crate::Reg<u32, _CC3XX_DEBUGSS_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_DEBUGSS_STATUS;
#[doc = "`read()` method returns [cc3xx_debugss_status::R](cc3xx_debugss_status::R) reader structure"]
impl crate::Readable for CC3XX_DEBUGSS_STATUS {}
#[doc = "`write(|w| ..)` method takes [cc3xx_debugss_status::W](cc3xx_debugss_status::W) writer structure"]
impl crate::Writable for CC3XX_DEBUGSS_STATUS {}
#[doc = "CC3XX_DEBUGSS_STATUS"]
pub mod cc3xx_debugss_status;
#[doc = "CC3XX_DEBUGMUX_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_debugmux_sel](cc3xx_debugmux_sel) module"]
pub type CC3XX_DEBUGMUX_SEL = crate::Reg<u32, _CC3XX_DEBUGMUX_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_DEBUGMUX_SEL;
#[doc = "`read()` method returns [cc3xx_debugmux_sel::R](cc3xx_debugmux_sel::R) reader structure"]
impl crate::Readable for CC3XX_DEBUGMUX_SEL {}
#[doc = "`write(|w| ..)` method takes [cc3xx_debugmux_sel::W](cc3xx_debugmux_sel::W) writer structure"]
impl crate::Writable for CC3XX_DEBUGMUX_SEL {}
#[doc = "CC3XX_DEBUGMUX_SEL"]
pub mod cc3xx_debugmux_sel;
#[doc = "ALT_PC_VAL_NW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_pc_val_nw](alt_pc_val_nw) module"]
pub type ALT_PC_VAL_NW = crate::Reg<u32, _ALT_PC_VAL_NW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALT_PC_VAL_NW;
#[doc = "`read()` method returns [alt_pc_val_nw::R](alt_pc_val_nw::R) reader structure"]
impl crate::Readable for ALT_PC_VAL_NW {}
#[doc = "`write(|w| ..)` method takes [alt_pc_val_nw::W](alt_pc_val_nw::W) writer structure"]
impl crate::Writable for ALT_PC_VAL_NW {}
#[doc = "ALT_PC_VAL_NW"]
pub mod alt_pc_val_nw;
#[doc = "ALT_PC_VAL_APPS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_pc_val_apps](alt_pc_val_apps) module"]
pub type ALT_PC_VAL_APPS = crate::Reg<u32, _ALT_PC_VAL_APPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALT_PC_VAL_APPS;
#[doc = "`read()` method returns [alt_pc_val_apps::R](alt_pc_val_apps::R) reader structure"]
impl crate::Readable for ALT_PC_VAL_APPS {}
#[doc = "`write(|w| ..)` method takes [alt_pc_val_apps::W](alt_pc_val_apps::W) writer structure"]
impl crate::Writable for ALT_PC_VAL_APPS {}
#[doc = "ALT_PC_VAL_APPS"]
pub mod alt_pc_val_apps;
#[doc = "SPARE_REG_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_4](spare_reg_4) module"]
pub type SPARE_REG_4 = crate::Reg<u32, _SPARE_REG_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_4;
#[doc = "`read()` method returns [spare_reg_4::R](spare_reg_4::R) reader structure"]
impl crate::Readable for SPARE_REG_4 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_4::W](spare_reg_4::W) writer structure"]
impl crate::Writable for SPARE_REG_4 {}
#[doc = "SPARE_REG_4"]
pub mod spare_reg_4;
#[doc = "SPARE_REG_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_5](spare_reg_5) module"]
pub type SPARE_REG_5 = crate::Reg<u32, _SPARE_REG_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_5;
#[doc = "`read()` method returns [spare_reg_5::R](spare_reg_5::R) reader structure"]
impl crate::Readable for SPARE_REG_5 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_5::W](spare_reg_5::W) writer structure"]
impl crate::Writable for SPARE_REG_5 {}
#[doc = "SPARE_REG_5"]
pub mod spare_reg_5;
#[doc = "SH_SPI_CS_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sh_spi_cs_mask](sh_spi_cs_mask) module"]
pub type SH_SPI_CS_MASK = crate::Reg<u32, _SH_SPI_CS_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SH_SPI_CS_MASK;
#[doc = "`read()` method returns [sh_spi_cs_mask::R](sh_spi_cs_mask::R) reader structure"]
impl crate::Readable for SH_SPI_CS_MASK {}
#[doc = "`write(|w| ..)` method takes [sh_spi_cs_mask::W](sh_spi_cs_mask::W) writer structure"]
impl crate::Writable for SH_SPI_CS_MASK {}
#[doc = "SH_SPI_CS_MASK"]
pub mod sh_spi_cs_mask;
#[doc = "CC3XX_DEVICE_TYPE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_device_type](cc3xx_device_type) module"]
pub type CC3XX_DEVICE_TYPE = crate::Reg<u32, _CC3XX_DEVICE_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_DEVICE_TYPE;
#[doc = "`read()` method returns [cc3xx_device_type::R](cc3xx_device_type::R) reader structure"]
impl crate::Readable for CC3XX_DEVICE_TYPE {}
#[doc = "`write(|w| ..)` method takes [cc3xx_device_type::W](cc3xx_device_type::W) writer structure"]
impl crate::Writable for CC3XX_DEVICE_TYPE {}
#[doc = "CC3XX_DEVICE_TYPE"]
pub mod cc3xx_device_type;
#[doc = "MEM_TOPMUXCTRL_IFORCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_topmuxctrl_iforce](mem_topmuxctrl_iforce) module"]
pub type MEM_TOPMUXCTRL_IFORCE = crate::Reg<u32, _MEM_TOPMUXCTRL_IFORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_TOPMUXCTRL_IFORCE;
#[doc = "`read()` method returns [mem_topmuxctrl_iforce::R](mem_topmuxctrl_iforce::R) reader structure"]
impl crate::Readable for MEM_TOPMUXCTRL_IFORCE {}
#[doc = "`write(|w| ..)` method takes [mem_topmuxctrl_iforce::W](mem_topmuxctrl_iforce::W) writer structure"]
impl crate::Writable for MEM_TOPMUXCTRL_IFORCE {}
#[doc = "MEM_TOPMUXCTRL_IFORCE"]
pub mod mem_topmuxctrl_iforce;
#[doc = "CC3XX_DEV_PACKAGE_DETECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_dev_package_detect](cc3xx_dev_package_detect) module"]
pub type CC3XX_DEV_PACKAGE_DETECT = crate::Reg<u32, _CC3XX_DEV_PACKAGE_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_DEV_PACKAGE_DETECT;
#[doc = "`read()` method returns [cc3xx_dev_package_detect::R](cc3xx_dev_package_detect::R) reader structure"]
impl crate::Readable for CC3XX_DEV_PACKAGE_DETECT {}
#[doc = "`write(|w| ..)` method takes [cc3xx_dev_package_detect::W](cc3xx_dev_package_detect::W) writer structure"]
impl crate::Writable for CC3XX_DEV_PACKAGE_DETECT {}
#[doc = "CC3XX_DEV_PACKAGE_DETECT"]
pub mod cc3xx_dev_package_detect;
#[doc = "AUTONMS_SPICLK_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autonms_spiclk_sel](autonms_spiclk_sel) module"]
pub type AUTONMS_SPICLK_SEL = crate::Reg<u32, _AUTONMS_SPICLK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTONMS_SPICLK_SEL;
#[doc = "`read()` method returns [autonms_spiclk_sel::R](autonms_spiclk_sel::R) reader structure"]
impl crate::Readable for AUTONMS_SPICLK_SEL {}
#[doc = "`write(|w| ..)` method takes [autonms_spiclk_sel::W](autonms_spiclk_sel::W) writer structure"]
impl crate::Writable for AUTONMS_SPICLK_SEL {}
#[doc = "AUTONMS_SPICLK_SEL"]
pub mod autonms_spiclk_sel;
#[doc = "CC3XX_DEV_PADCONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3xx_dev_padconf](cc3xx_dev_padconf) module"]
pub type CC3XX_DEV_PADCONF = crate::Reg<u32, _CC3XX_DEV_PADCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC3XX_DEV_PADCONF;
#[doc = "`read()` method returns [cc3xx_dev_padconf::R](cc3xx_dev_padconf::R) reader structure"]
impl crate::Readable for CC3XX_DEV_PADCONF {}
#[doc = "`write(|w| ..)` method takes [cc3xx_dev_padconf::W](cc3xx_dev_padconf::W) writer structure"]
impl crate::Writable for CC3XX_DEV_PADCONF {}
#[doc = "CC3XX_DEV_PADCONF"]
pub mod cc3xx_dev_padconf;
#[doc = "SPARE_REG_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_8](spare_reg_8) module"]
pub type SPARE_REG_8 = crate::Reg<u32, _SPARE_REG_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_8;
#[doc = "`read()` method returns [spare_reg_8::R](spare_reg_8::R) reader structure"]
impl crate::Readable for SPARE_REG_8 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_8::W](spare_reg_8::W) writer structure"]
impl crate::Writable for SPARE_REG_8 {}
#[doc = "SPARE_REG_8"]
pub mod spare_reg_8;
#[doc = "SPARE_REG_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_6](spare_reg_6) module"]
pub type SPARE_REG_6 = crate::Reg<u32, _SPARE_REG_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_6;
#[doc = "`read()` method returns [spare_reg_6::R](spare_reg_6::R) reader structure"]
impl crate::Readable for SPARE_REG_6 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_6::W](spare_reg_6::W) writer structure"]
impl crate::Writable for SPARE_REG_6 {}
#[doc = "SPARE_REG_6"]
pub mod spare_reg_6;
#[doc = "SPARE_REG_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_reg_7](spare_reg_7) module"]
pub type SPARE_REG_7 = crate::Reg<u32, _SPARE_REG_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_REG_7;
#[doc = "`read()` method returns [spare_reg_7::R](spare_reg_7::R) reader structure"]
impl crate::Readable for SPARE_REG_7 {}
#[doc = "`write(|w| ..)` method takes [spare_reg_7::W](spare_reg_7::W) writer structure"]
impl crate::Writable for SPARE_REG_7 {}
#[doc = "SPARE_REG_7"]
pub mod spare_reg_7;
#[doc = "APPS_WLAN_ORBIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_wlan_orbit](apps_wlan_orbit) module"]
pub type APPS_WLAN_ORBIT = crate::Reg<u32, _APPS_WLAN_ORBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_WLAN_ORBIT;
#[doc = "`read()` method returns [apps_wlan_orbit::R](apps_wlan_orbit::R) reader structure"]
impl crate::Readable for APPS_WLAN_ORBIT {}
#[doc = "`write(|w| ..)` method takes [apps_wlan_orbit::W](apps_wlan_orbit::W) writer structure"]
impl crate::Writable for APPS_WLAN_ORBIT {}
#[doc = "APPS_WLAN_ORBIT"]
pub mod apps_wlan_orbit;
#[doc = "APPS_WLAN_SCRATCH_PAD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_wlan_scratch_pad](apps_wlan_scratch_pad) module"]
pub type APPS_WLAN_SCRATCH_PAD = crate::Reg<u32, _APPS_WLAN_SCRATCH_PAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_WLAN_SCRATCH_PAD;
#[doc = "`read()` method returns [apps_wlan_scratch_pad::R](apps_wlan_scratch_pad::R) reader structure"]
impl crate::Readable for APPS_WLAN_SCRATCH_PAD {}
#[doc = "`write(|w| ..)` method takes [apps_wlan_scratch_pad::W](apps_wlan_scratch_pad::W) writer structure"]
impl crate::Writable for APPS_WLAN_SCRATCH_PAD {}
#[doc = "APPS_WLAN_SCRATCH_PAD"]
pub mod apps_wlan_scratch_pad;
