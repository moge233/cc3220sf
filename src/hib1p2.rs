#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM_SKA_LDO_PARAMETERS0"]
    pub sram_ska_ldo_parameters0: SRAM_SKA_LDO_PARAMETERS0,
    #[doc = "0x04 - SRAM_SKA_LDO_PARAMETERS1"]
    pub sram_ska_ldo_parameters1: SRAM_SKA_LDO_PARAMETERS1,
    #[doc = "0x08 - DIG_DCDC_PARAMETERS0"]
    pub dig_dcdc_parameters0: DIG_DCDC_PARAMETERS0,
    #[doc = "0x0c - DIG_DCDC_PARAMETERS1"]
    pub dig_dcdc_parameters1: DIG_DCDC_PARAMETERS1,
    #[doc = "0x10 - DIG_DCDC_PARAMETERS2"]
    pub dig_dcdc_parameters2: DIG_DCDC_PARAMETERS2,
    #[doc = "0x14 - DIG_DCDC_PARAMETERS3"]
    pub dig_dcdc_parameters3: DIG_DCDC_PARAMETERS3,
    #[doc = "0x18 - DIG_DCDC_PARAMETERS4"]
    pub dig_dcdc_parameters4: DIG_DCDC_PARAMETERS4,
    #[doc = "0x1c - DIG_DCDC_PARAMETERS5"]
    pub dig_dcdc_parameters5: DIG_DCDC_PARAMETERS5,
    #[doc = "0x20 - DIG_DCDC_PARAMETERS6"]
    pub dig_dcdc_parameters6: DIG_DCDC_PARAMETERS6,
    #[doc = "0x24 - ANA_DCDC_PARAMETERS0"]
    pub ana_dcdc_parameters0: ANA_DCDC_PARAMETERS0,
    #[doc = "0x28 - ANA_DCDC_PARAMETERS1"]
    pub ana_dcdc_parameters1: ANA_DCDC_PARAMETERS1,
    _reserved11: [u8; 56usize],
    #[doc = "0x64 - ANA_DCDC_PARAMETERS16"]
    pub ana_dcdc_parameters16: ANA_DCDC_PARAMETERS16,
    #[doc = "0x68 - ANA_DCDC_PARAMETERS17"]
    pub ana_dcdc_parameters17: ANA_DCDC_PARAMETERS17,
    #[doc = "0x6c - ANA_DCDC_PARAMETERS18"]
    pub ana_dcdc_parameters18: ANA_DCDC_PARAMETERS18,
    #[doc = "0x70 - ANA_DCDC_PARAMETERS19"]
    pub ana_dcdc_parameters19: ANA_DCDC_PARAMETERS19,
    #[doc = "0x74 - FLASH_DCDC_PARAMETERS0"]
    pub flash_dcdc_parameters0: FLASH_DCDC_PARAMETERS0,
    #[doc = "0x78 - FLASH_DCDC_PARAMETERS1"]
    pub flash_dcdc_parameters1: FLASH_DCDC_PARAMETERS1,
    #[doc = "0x7c - FLASH_DCDC_PARAMETERS2"]
    pub flash_dcdc_parameters2: FLASH_DCDC_PARAMETERS2,
    #[doc = "0x80 - FLASH_DCDC_PARAMETERS3"]
    pub flash_dcdc_parameters3: FLASH_DCDC_PARAMETERS3,
    #[doc = "0x84 - FLASH_DCDC_PARAMETERS4"]
    pub flash_dcdc_parameters4: FLASH_DCDC_PARAMETERS4,
    #[doc = "0x88 - FLASH_DCDC_PARAMETERS5"]
    pub flash_dcdc_parameters5: FLASH_DCDC_PARAMETERS5,
    #[doc = "0x8c - FLASH_DCDC_PARAMETERS6"]
    pub flash_dcdc_parameters6: FLASH_DCDC_PARAMETERS6,
    _reserved22: [u8; 4usize],
    #[doc = "0x94 - PMBIST_PARAMETERS0"]
    pub pmbist_parameters0: PMBIST_PARAMETERS0,
    #[doc = "0x98 - PMBIST_PARAMETERS1"]
    pub pmbist_parameters1: PMBIST_PARAMETERS1,
    #[doc = "0x9c - PMBIST_PARAMETERS2"]
    pub pmbist_parameters2: PMBIST_PARAMETERS2,
    #[doc = "0xa0 - PMBIST_PARAMETERS3"]
    pub pmbist_parameters3: PMBIST_PARAMETERS3,
    #[doc = "0xa4 - FLASH_DCDC_PARAMETERS8"]
    pub flash_dcdc_parameters8: FLASH_DCDC_PARAMETERS8,
    #[doc = "0xa8 - ANA_DCDC_PARAMETERS_OVERRIDE"]
    pub ana_dcdc_parameters_override: ANA_DCDC_PARAMETERS_OVERRIDE,
    #[doc = "0xac - FLASH_DCDC_PARAMETERS_OVERRIDE"]
    pub flash_dcdc_parameters_override: FLASH_DCDC_PARAMETERS_OVERRIDE,
    #[doc = "0xb0 - DIG_DCDC_VTRIM_CFG"]
    pub dig_dcdc_vtrim_cfg: DIG_DCDC_VTRIM_CFG,
    #[doc = "0xb4 - DIG_DCDC_FSM_PARAMETERS"]
    pub dig_dcdc_fsm_parameters: DIG_DCDC_FSM_PARAMETERS,
    #[doc = "0xb8 - ANA_DCDC_FSM_PARAMETERS"]
    pub ana_dcdc_fsm_parameters: ANA_DCDC_FSM_PARAMETERS,
    #[doc = "0xbc - SRAM_SKA_LDO_FSM_PARAMETERS"]
    pub sram_ska_ldo_fsm_parameters: SRAM_SKA_LDO_FSM_PARAMETERS,
    #[doc = "0xc0 - BGAP_DUTY_CYCLING_EXIT_CFG"]
    pub bgap_duty_cycling_exit_cfg: BGAP_DUTY_CYCLING_EXIT_CFG,
    #[doc = "0xc4 - CM_OSC_16M_CONFIG"]
    pub cm_osc_16m_config: CM_OSC_16M_CONFIG,
    #[doc = "0xc8 - SOP_SENSE_VALUE"]
    pub sop_sense_value: SOP_SENSE_VALUE,
    #[doc = "0xcc - HIB_RTC_TIMER_LSW_1P2"]
    pub hib_rtc_timer_lsw_1p2: HIB_RTC_TIMER_LSW_1P2,
    #[doc = "0xd0 - HIB_RTC_TIMER_MSW_1P2"]
    pub hib_rtc_timer_msw_1p2: HIB_RTC_TIMER_MSW_1P2,
    #[doc = "0xd4 - BGAP_TRIM_OVERRIDES"]
    pub bgap_trim_overrides: BGAP_TRIM_OVERRIDES,
    #[doc = "0xd8 - EFUSE_READ_REG0"]
    pub efuse_read_reg0: EFUSE_READ_REG0,
    #[doc = "0xdc - EFUSE_READ_REG1"]
    pub efuse_read_reg1: EFUSE_READ_REG1,
    #[doc = "0xe0 - POR_TEST_CTRL"]
    pub por_test_ctrl: POR_TEST_CTRL,
    #[doc = "0xe4 - HIB_TIMER_SYNC_CALIB_CFG0"]
    pub hib_timer_sync_calib_cfg0: HIB_TIMER_SYNC_CALIB_CFG0,
    #[doc = "0xe8 - HIB_TIMER_SYNC_CALIB_CFG1"]
    pub hib_timer_sync_calib_cfg1: HIB_TIMER_SYNC_CALIB_CFG1,
    #[doc = "0xec - HIB_TIMER_SYNC_CFG2"]
    pub hib_timer_sync_cfg2: HIB_TIMER_SYNC_CFG2,
    #[doc = "0xf0 - HIB_TIMER_SYNC_TSF_ADJ_VAL"]
    pub hib_timer_sync_tsf_adj_val: HIB_TIMER_SYNC_TSF_ADJ_VAL,
    #[doc = "0xf4 - HIB_TIMER_RTC_GTS_TIMESTAMP_LSW"]
    pub hib_timer_rtc_gts_timestamp_lsw: HIB_TIMER_RTC_GTS_TIMESTAMP_LSW,
    #[doc = "0xf8 - HIB_TIMER_RTC_GTS_TIMESTAMP_MSW"]
    pub hib_timer_rtc_gts_timestamp_msw: HIB_TIMER_RTC_GTS_TIMESTAMP_MSW,
    #[doc = "0xfc - HIB_TIMER_RTC_WUP_TIMESTAMP_LSW"]
    pub hib_timer_rtc_wup_timestamp_lsw: HIB_TIMER_RTC_WUP_TIMESTAMP_LSW,
    #[doc = "0x100 - HIB_TIMER_RTC_WUP_TIMESTAMP_MSW"]
    pub hib_timer_rtc_wup_timestamp_msw: HIB_TIMER_RTC_WUP_TIMESTAMP_MSW,
    #[doc = "0x104 - HIB_TIMER_SYNC_WAKE_OFFSET_ERR"]
    pub hib_timer_sync_wake_offset_err: HIB_TIMER_SYNC_WAKE_OFFSET_ERR,
    #[doc = "0x108 - HIB_TIMER_SYNC_TSF_CURR_VAL_LSW"]
    pub hib_timer_sync_tsf_curr_val_lsw: HIB_TIMER_SYNC_TSF_CURR_VAL_LSW,
    #[doc = "0x10c - HIB_TIMER_SYNC_TSF_CURR_VAL_MSW"]
    pub hib_timer_sync_tsf_curr_val_msw: HIB_TIMER_SYNC_TSF_CURR_VAL_MSW,
    #[doc = "0x110 - CM_SPARE"]
    pub cm_spare: CM_SPARE,
    #[doc = "0x114 - PORPOL_SPARE"]
    pub porpol_spare: PORPOL_SPARE,
    #[doc = "0x118 - MEM_DIG_DCDC_CLK_CONFIG"]
    pub mem_dig_dcdc_clk_config: MEM_DIG_DCDC_CLK_CONFIG,
    #[doc = "0x11c - MEM_ANA_DCDC_CLK_CONFIG"]
    pub mem_ana_dcdc_clk_config: MEM_ANA_DCDC_CLK_CONFIG,
    #[doc = "0x120 - MEM_FLASH_DCDC_CLK_CONFIG"]
    pub mem_flash_dcdc_clk_config: MEM_FLASH_DCDC_CLK_CONFIG,
    #[doc = "0x124 - MEM_PA_DCDC_CLK_CONFIG"]
    pub mem_pa_dcdc_clk_config: MEM_PA_DCDC_CLK_CONFIG,
    #[doc = "0x128 - MEM_SLDO_VNWA_OVERRIDE"]
    pub mem_sldo_vnwa_override: MEM_SLDO_VNWA_OVERRIDE,
    #[doc = "0x12c - MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE"]
    pub mem_bgap_duty_cycling_enable_override: MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE,
    #[doc = "0x130 - MEM_HIB_FSM_DEBUG"]
    pub mem_hib_fsm_debug: MEM_HIB_FSM_DEBUG,
    #[doc = "0x134 - MEM_SLDO_VNWA_SW_CTRL"]
    pub mem_sldo_vnwa_sw_ctrl: MEM_SLDO_VNWA_SW_CTRL,
    #[doc = "0x138 - MEM_SLDO_WEAK_PROCESS"]
    pub mem_sldo_weak_process: MEM_SLDO_WEAK_PROCESS,
    #[doc = "0x13c - MEM_PA_DCDC_OV_UV_STATUS"]
    pub mem_pa_dcdc_ov_uv_status: MEM_PA_DCDC_OV_UV_STATUS,
    #[doc = "0x140 - MEM_CM_TEST_MODE"]
    pub mem_cm_test_mode: MEM_CM_TEST_MODE,
}
#[doc = "SRAM_SKA_LDO_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ska_ldo_parameters0](sram_ska_ldo_parameters0) module"]
pub type SRAM_SKA_LDO_PARAMETERS0 = crate::Reg<u32, _SRAM_SKA_LDO_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_SKA_LDO_PARAMETERS0;
#[doc = "`read()` method returns [sram_ska_ldo_parameters0::R](sram_ska_ldo_parameters0::R) reader structure"]
impl crate::Readable for SRAM_SKA_LDO_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [sram_ska_ldo_parameters0::W](sram_ska_ldo_parameters0::W) writer structure"]
impl crate::Writable for SRAM_SKA_LDO_PARAMETERS0 {}
#[doc = "SRAM_SKA_LDO_PARAMETERS0"]
pub mod sram_ska_ldo_parameters0;
#[doc = "SRAM_SKA_LDO_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ska_ldo_parameters1](sram_ska_ldo_parameters1) module"]
pub type SRAM_SKA_LDO_PARAMETERS1 = crate::Reg<u32, _SRAM_SKA_LDO_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_SKA_LDO_PARAMETERS1;
#[doc = "`read()` method returns [sram_ska_ldo_parameters1::R](sram_ska_ldo_parameters1::R) reader structure"]
impl crate::Readable for SRAM_SKA_LDO_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [sram_ska_ldo_parameters1::W](sram_ska_ldo_parameters1::W) writer structure"]
impl crate::Writable for SRAM_SKA_LDO_PARAMETERS1 {}
#[doc = "SRAM_SKA_LDO_PARAMETERS1"]
pub mod sram_ska_ldo_parameters1;
#[doc = "DIG_DCDC_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters0](dig_dcdc_parameters0) module"]
pub type DIG_DCDC_PARAMETERS0 = crate::Reg<u32, _DIG_DCDC_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS0;
#[doc = "`read()` method returns [dig_dcdc_parameters0::R](dig_dcdc_parameters0::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters0::W](dig_dcdc_parameters0::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS0 {}
#[doc = "DIG_DCDC_PARAMETERS0"]
pub mod dig_dcdc_parameters0;
#[doc = "DIG_DCDC_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters1](dig_dcdc_parameters1) module"]
pub type DIG_DCDC_PARAMETERS1 = crate::Reg<u32, _DIG_DCDC_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS1;
#[doc = "`read()` method returns [dig_dcdc_parameters1::R](dig_dcdc_parameters1::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters1::W](dig_dcdc_parameters1::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS1 {}
#[doc = "DIG_DCDC_PARAMETERS1"]
pub mod dig_dcdc_parameters1;
#[doc = "DIG_DCDC_PARAMETERS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters2](dig_dcdc_parameters2) module"]
pub type DIG_DCDC_PARAMETERS2 = crate::Reg<u32, _DIG_DCDC_PARAMETERS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS2;
#[doc = "`read()` method returns [dig_dcdc_parameters2::R](dig_dcdc_parameters2::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS2 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters2::W](dig_dcdc_parameters2::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS2 {}
#[doc = "DIG_DCDC_PARAMETERS2"]
pub mod dig_dcdc_parameters2;
#[doc = "DIG_DCDC_PARAMETERS3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters3](dig_dcdc_parameters3) module"]
pub type DIG_DCDC_PARAMETERS3 = crate::Reg<u32, _DIG_DCDC_PARAMETERS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS3;
#[doc = "`read()` method returns [dig_dcdc_parameters3::R](dig_dcdc_parameters3::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS3 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters3::W](dig_dcdc_parameters3::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS3 {}
#[doc = "DIG_DCDC_PARAMETERS3"]
pub mod dig_dcdc_parameters3;
#[doc = "DIG_DCDC_PARAMETERS4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters4](dig_dcdc_parameters4) module"]
pub type DIG_DCDC_PARAMETERS4 = crate::Reg<u32, _DIG_DCDC_PARAMETERS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS4;
#[doc = "`read()` method returns [dig_dcdc_parameters4::R](dig_dcdc_parameters4::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS4 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters4::W](dig_dcdc_parameters4::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS4 {}
#[doc = "DIG_DCDC_PARAMETERS4"]
pub mod dig_dcdc_parameters4;
#[doc = "DIG_DCDC_PARAMETERS5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters5](dig_dcdc_parameters5) module"]
pub type DIG_DCDC_PARAMETERS5 = crate::Reg<u32, _DIG_DCDC_PARAMETERS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS5;
#[doc = "`read()` method returns [dig_dcdc_parameters5::R](dig_dcdc_parameters5::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS5 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters5::W](dig_dcdc_parameters5::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS5 {}
#[doc = "DIG_DCDC_PARAMETERS5"]
pub mod dig_dcdc_parameters5;
#[doc = "DIG_DCDC_PARAMETERS6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_parameters6](dig_dcdc_parameters6) module"]
pub type DIG_DCDC_PARAMETERS6 = crate::Reg<u32, _DIG_DCDC_PARAMETERS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_PARAMETERS6;
#[doc = "`read()` method returns [dig_dcdc_parameters6::R](dig_dcdc_parameters6::R) reader structure"]
impl crate::Readable for DIG_DCDC_PARAMETERS6 {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_parameters6::W](dig_dcdc_parameters6::W) writer structure"]
impl crate::Writable for DIG_DCDC_PARAMETERS6 {}
#[doc = "DIG_DCDC_PARAMETERS6"]
pub mod dig_dcdc_parameters6;
#[doc = "ANA_DCDC_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters0](ana_dcdc_parameters0) module"]
pub type ANA_DCDC_PARAMETERS0 = crate::Reg<u32, _ANA_DCDC_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS0;
#[doc = "`read()` method returns [ana_dcdc_parameters0::R](ana_dcdc_parameters0::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters0::W](ana_dcdc_parameters0::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS0 {}
#[doc = "ANA_DCDC_PARAMETERS0"]
pub mod ana_dcdc_parameters0;
#[doc = "ANA_DCDC_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters1](ana_dcdc_parameters1) module"]
pub type ANA_DCDC_PARAMETERS1 = crate::Reg<u32, _ANA_DCDC_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS1;
#[doc = "`read()` method returns [ana_dcdc_parameters1::R](ana_dcdc_parameters1::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters1::W](ana_dcdc_parameters1::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS1 {}
#[doc = "ANA_DCDC_PARAMETERS1"]
pub mod ana_dcdc_parameters1;
#[doc = "ANA_DCDC_PARAMETERS16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters16](ana_dcdc_parameters16) module"]
pub type ANA_DCDC_PARAMETERS16 = crate::Reg<u32, _ANA_DCDC_PARAMETERS16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS16;
#[doc = "`read()` method returns [ana_dcdc_parameters16::R](ana_dcdc_parameters16::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS16 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters16::W](ana_dcdc_parameters16::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS16 {}
#[doc = "ANA_DCDC_PARAMETERS16"]
pub mod ana_dcdc_parameters16;
#[doc = "ANA_DCDC_PARAMETERS17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters17](ana_dcdc_parameters17) module"]
pub type ANA_DCDC_PARAMETERS17 = crate::Reg<u32, _ANA_DCDC_PARAMETERS17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS17;
#[doc = "`read()` method returns [ana_dcdc_parameters17::R](ana_dcdc_parameters17::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS17 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters17::W](ana_dcdc_parameters17::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS17 {}
#[doc = "ANA_DCDC_PARAMETERS17"]
pub mod ana_dcdc_parameters17;
#[doc = "ANA_DCDC_PARAMETERS18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters18](ana_dcdc_parameters18) module"]
pub type ANA_DCDC_PARAMETERS18 = crate::Reg<u32, _ANA_DCDC_PARAMETERS18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS18;
#[doc = "`read()` method returns [ana_dcdc_parameters18::R](ana_dcdc_parameters18::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS18 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters18::W](ana_dcdc_parameters18::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS18 {}
#[doc = "ANA_DCDC_PARAMETERS18"]
pub mod ana_dcdc_parameters18;
#[doc = "ANA_DCDC_PARAMETERS19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters19](ana_dcdc_parameters19) module"]
pub type ANA_DCDC_PARAMETERS19 = crate::Reg<u32, _ANA_DCDC_PARAMETERS19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS19;
#[doc = "`read()` method returns [ana_dcdc_parameters19::R](ana_dcdc_parameters19::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS19 {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters19::W](ana_dcdc_parameters19::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS19 {}
#[doc = "ANA_DCDC_PARAMETERS19"]
pub mod ana_dcdc_parameters19;
#[doc = "FLASH_DCDC_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters0](flash_dcdc_parameters0) module"]
pub type FLASH_DCDC_PARAMETERS0 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS0;
#[doc = "`read()` method returns [flash_dcdc_parameters0::R](flash_dcdc_parameters0::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters0::W](flash_dcdc_parameters0::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS0 {}
#[doc = "FLASH_DCDC_PARAMETERS0"]
pub mod flash_dcdc_parameters0;
#[doc = "FLASH_DCDC_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters1](flash_dcdc_parameters1) module"]
pub type FLASH_DCDC_PARAMETERS1 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS1;
#[doc = "`read()` method returns [flash_dcdc_parameters1::R](flash_dcdc_parameters1::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters1::W](flash_dcdc_parameters1::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS1 {}
#[doc = "FLASH_DCDC_PARAMETERS1"]
pub mod flash_dcdc_parameters1;
#[doc = "FLASH_DCDC_PARAMETERS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters2](flash_dcdc_parameters2) module"]
pub type FLASH_DCDC_PARAMETERS2 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS2;
#[doc = "`read()` method returns [flash_dcdc_parameters2::R](flash_dcdc_parameters2::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS2 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters2::W](flash_dcdc_parameters2::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS2 {}
#[doc = "FLASH_DCDC_PARAMETERS2"]
pub mod flash_dcdc_parameters2;
#[doc = "FLASH_DCDC_PARAMETERS3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters3](flash_dcdc_parameters3) module"]
pub type FLASH_DCDC_PARAMETERS3 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS3;
#[doc = "`read()` method returns [flash_dcdc_parameters3::R](flash_dcdc_parameters3::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS3 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters3::W](flash_dcdc_parameters3::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS3 {}
#[doc = "FLASH_DCDC_PARAMETERS3"]
pub mod flash_dcdc_parameters3;
#[doc = "FLASH_DCDC_PARAMETERS4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters4](flash_dcdc_parameters4) module"]
pub type FLASH_DCDC_PARAMETERS4 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS4;
#[doc = "`read()` method returns [flash_dcdc_parameters4::R](flash_dcdc_parameters4::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS4 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters4::W](flash_dcdc_parameters4::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS4 {}
#[doc = "FLASH_DCDC_PARAMETERS4"]
pub mod flash_dcdc_parameters4;
#[doc = "FLASH_DCDC_PARAMETERS5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters5](flash_dcdc_parameters5) module"]
pub type FLASH_DCDC_PARAMETERS5 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS5;
#[doc = "`read()` method returns [flash_dcdc_parameters5::R](flash_dcdc_parameters5::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS5 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters5::W](flash_dcdc_parameters5::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS5 {}
#[doc = "FLASH_DCDC_PARAMETERS5"]
pub mod flash_dcdc_parameters5;
#[doc = "FLASH_DCDC_PARAMETERS6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters6](flash_dcdc_parameters6) module"]
pub type FLASH_DCDC_PARAMETERS6 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS6;
#[doc = "`read()` method returns [flash_dcdc_parameters6::R](flash_dcdc_parameters6::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS6 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters6::W](flash_dcdc_parameters6::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS6 {}
#[doc = "FLASH_DCDC_PARAMETERS6"]
pub mod flash_dcdc_parameters6;
#[doc = "PMBIST_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmbist_parameters0](pmbist_parameters0) module"]
pub type PMBIST_PARAMETERS0 = crate::Reg<u32, _PMBIST_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMBIST_PARAMETERS0;
#[doc = "`read()` method returns [pmbist_parameters0::R](pmbist_parameters0::R) reader structure"]
impl crate::Readable for PMBIST_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [pmbist_parameters0::W](pmbist_parameters0::W) writer structure"]
impl crate::Writable for PMBIST_PARAMETERS0 {}
#[doc = "PMBIST_PARAMETERS0"]
pub mod pmbist_parameters0;
#[doc = "PMBIST_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmbist_parameters1](pmbist_parameters1) module"]
pub type PMBIST_PARAMETERS1 = crate::Reg<u32, _PMBIST_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMBIST_PARAMETERS1;
#[doc = "`read()` method returns [pmbist_parameters1::R](pmbist_parameters1::R) reader structure"]
impl crate::Readable for PMBIST_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [pmbist_parameters1::W](pmbist_parameters1::W) writer structure"]
impl crate::Writable for PMBIST_PARAMETERS1 {}
#[doc = "PMBIST_PARAMETERS1"]
pub mod pmbist_parameters1;
#[doc = "PMBIST_PARAMETERS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmbist_parameters2](pmbist_parameters2) module"]
pub type PMBIST_PARAMETERS2 = crate::Reg<u32, _PMBIST_PARAMETERS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMBIST_PARAMETERS2;
#[doc = "`read()` method returns [pmbist_parameters2::R](pmbist_parameters2::R) reader structure"]
impl crate::Readable for PMBIST_PARAMETERS2 {}
#[doc = "`write(|w| ..)` method takes [pmbist_parameters2::W](pmbist_parameters2::W) writer structure"]
impl crate::Writable for PMBIST_PARAMETERS2 {}
#[doc = "PMBIST_PARAMETERS2"]
pub mod pmbist_parameters2;
#[doc = "PMBIST_PARAMETERS3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmbist_parameters3](pmbist_parameters3) module"]
pub type PMBIST_PARAMETERS3 = crate::Reg<u32, _PMBIST_PARAMETERS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMBIST_PARAMETERS3;
#[doc = "`read()` method returns [pmbist_parameters3::R](pmbist_parameters3::R) reader structure"]
impl crate::Readable for PMBIST_PARAMETERS3 {}
#[doc = "`write(|w| ..)` method takes [pmbist_parameters3::W](pmbist_parameters3::W) writer structure"]
impl crate::Writable for PMBIST_PARAMETERS3 {}
#[doc = "PMBIST_PARAMETERS3"]
pub mod pmbist_parameters3;
#[doc = "FLASH_DCDC_PARAMETERS8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters8](flash_dcdc_parameters8) module"]
pub type FLASH_DCDC_PARAMETERS8 = crate::Reg<u32, _FLASH_DCDC_PARAMETERS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS8;
#[doc = "`read()` method returns [flash_dcdc_parameters8::R](flash_dcdc_parameters8::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS8 {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters8::W](flash_dcdc_parameters8::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS8 {}
#[doc = "FLASH_DCDC_PARAMETERS8"]
pub mod flash_dcdc_parameters8;
#[doc = "ANA_DCDC_PARAMETERS_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_parameters_override](ana_dcdc_parameters_override) module"]
pub type ANA_DCDC_PARAMETERS_OVERRIDE = crate::Reg<u32, _ANA_DCDC_PARAMETERS_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_PARAMETERS_OVERRIDE;
#[doc = "`read()` method returns [ana_dcdc_parameters_override::R](ana_dcdc_parameters_override::R) reader structure"]
impl crate::Readable for ANA_DCDC_PARAMETERS_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_parameters_override::W](ana_dcdc_parameters_override::W) writer structure"]
impl crate::Writable for ANA_DCDC_PARAMETERS_OVERRIDE {}
#[doc = "ANA_DCDC_PARAMETERS_OVERRIDE"]
pub mod ana_dcdc_parameters_override;
#[doc = "FLASH_DCDC_PARAMETERS_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_dcdc_parameters_override](flash_dcdc_parameters_override) module"]
pub type FLASH_DCDC_PARAMETERS_OVERRIDE = crate::Reg<u32, _FLASH_DCDC_PARAMETERS_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DCDC_PARAMETERS_OVERRIDE;
#[doc = "`read()` method returns [flash_dcdc_parameters_override::R](flash_dcdc_parameters_override::R) reader structure"]
impl crate::Readable for FLASH_DCDC_PARAMETERS_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [flash_dcdc_parameters_override::W](flash_dcdc_parameters_override::W) writer structure"]
impl crate::Writable for FLASH_DCDC_PARAMETERS_OVERRIDE {}
#[doc = "FLASH_DCDC_PARAMETERS_OVERRIDE"]
pub mod flash_dcdc_parameters_override;
#[doc = "DIG_DCDC_VTRIM_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_vtrim_cfg](dig_dcdc_vtrim_cfg) module"]
pub type DIG_DCDC_VTRIM_CFG = crate::Reg<u32, _DIG_DCDC_VTRIM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_VTRIM_CFG;
#[doc = "`read()` method returns [dig_dcdc_vtrim_cfg::R](dig_dcdc_vtrim_cfg::R) reader structure"]
impl crate::Readable for DIG_DCDC_VTRIM_CFG {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_vtrim_cfg::W](dig_dcdc_vtrim_cfg::W) writer structure"]
impl crate::Writable for DIG_DCDC_VTRIM_CFG {}
#[doc = "DIG_DCDC_VTRIM_CFG"]
pub mod dig_dcdc_vtrim_cfg;
#[doc = "DIG_DCDC_FSM_PARAMETERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_dcdc_fsm_parameters](dig_dcdc_fsm_parameters) module"]
pub type DIG_DCDC_FSM_PARAMETERS = crate::Reg<u32, _DIG_DCDC_FSM_PARAMETERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_DCDC_FSM_PARAMETERS;
#[doc = "`read()` method returns [dig_dcdc_fsm_parameters::R](dig_dcdc_fsm_parameters::R) reader structure"]
impl crate::Readable for DIG_DCDC_FSM_PARAMETERS {}
#[doc = "`write(|w| ..)` method takes [dig_dcdc_fsm_parameters::W](dig_dcdc_fsm_parameters::W) writer structure"]
impl crate::Writable for DIG_DCDC_FSM_PARAMETERS {}
#[doc = "DIG_DCDC_FSM_PARAMETERS"]
pub mod dig_dcdc_fsm_parameters;
#[doc = "ANA_DCDC_FSM_PARAMETERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_dcdc_fsm_parameters](ana_dcdc_fsm_parameters) module"]
pub type ANA_DCDC_FSM_PARAMETERS = crate::Reg<u32, _ANA_DCDC_FSM_PARAMETERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_DCDC_FSM_PARAMETERS;
#[doc = "`read()` method returns [ana_dcdc_fsm_parameters::R](ana_dcdc_fsm_parameters::R) reader structure"]
impl crate::Readable for ANA_DCDC_FSM_PARAMETERS {}
#[doc = "`write(|w| ..)` method takes [ana_dcdc_fsm_parameters::W](ana_dcdc_fsm_parameters::W) writer structure"]
impl crate::Writable for ANA_DCDC_FSM_PARAMETERS {}
#[doc = "ANA_DCDC_FSM_PARAMETERS"]
pub mod ana_dcdc_fsm_parameters;
#[doc = "SRAM_SKA_LDO_FSM_PARAMETERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ska_ldo_fsm_parameters](sram_ska_ldo_fsm_parameters) module"]
pub type SRAM_SKA_LDO_FSM_PARAMETERS = crate::Reg<u32, _SRAM_SKA_LDO_FSM_PARAMETERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_SKA_LDO_FSM_PARAMETERS;
#[doc = "`read()` method returns [sram_ska_ldo_fsm_parameters::R](sram_ska_ldo_fsm_parameters::R) reader structure"]
impl crate::Readable for SRAM_SKA_LDO_FSM_PARAMETERS {}
#[doc = "`write(|w| ..)` method takes [sram_ska_ldo_fsm_parameters::W](sram_ska_ldo_fsm_parameters::W) writer structure"]
impl crate::Writable for SRAM_SKA_LDO_FSM_PARAMETERS {}
#[doc = "SRAM_SKA_LDO_FSM_PARAMETERS"]
pub mod sram_ska_ldo_fsm_parameters;
#[doc = "BGAP_DUTY_CYCLING_EXIT_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgap_duty_cycling_exit_cfg](bgap_duty_cycling_exit_cfg) module"]
pub type BGAP_DUTY_CYCLING_EXIT_CFG = crate::Reg<u32, _BGAP_DUTY_CYCLING_EXIT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGAP_DUTY_CYCLING_EXIT_CFG;
#[doc = "`read()` method returns [bgap_duty_cycling_exit_cfg::R](bgap_duty_cycling_exit_cfg::R) reader structure"]
impl crate::Readable for BGAP_DUTY_CYCLING_EXIT_CFG {}
#[doc = "`write(|w| ..)` method takes [bgap_duty_cycling_exit_cfg::W](bgap_duty_cycling_exit_cfg::W) writer structure"]
impl crate::Writable for BGAP_DUTY_CYCLING_EXIT_CFG {}
#[doc = "BGAP_DUTY_CYCLING_EXIT_CFG"]
pub mod bgap_duty_cycling_exit_cfg;
#[doc = "CM_OSC_16M_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm_osc_16m_config](cm_osc_16m_config) module"]
pub type CM_OSC_16M_CONFIG = crate::Reg<u32, _CM_OSC_16M_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM_OSC_16M_CONFIG;
#[doc = "`read()` method returns [cm_osc_16m_config::R](cm_osc_16m_config::R) reader structure"]
impl crate::Readable for CM_OSC_16M_CONFIG {}
#[doc = "`write(|w| ..)` method takes [cm_osc_16m_config::W](cm_osc_16m_config::W) writer structure"]
impl crate::Writable for CM_OSC_16M_CONFIG {}
#[doc = "CM_OSC_16M_CONFIG"]
pub mod cm_osc_16m_config;
#[doc = "SOP_SENSE_VALUE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sop_sense_value](sop_sense_value) module"]
pub type SOP_SENSE_VALUE = crate::Reg<u32, _SOP_SENSE_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOP_SENSE_VALUE;
#[doc = "`read()` method returns [sop_sense_value::R](sop_sense_value::R) reader structure"]
impl crate::Readable for SOP_SENSE_VALUE {}
#[doc = "`write(|w| ..)` method takes [sop_sense_value::W](sop_sense_value::W) writer structure"]
impl crate::Writable for SOP_SENSE_VALUE {}
#[doc = "SOP_SENSE_VALUE"]
pub mod sop_sense_value;
#[doc = "HIB_RTC_TIMER_LSW_1P2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_rtc_timer_lsw_1p2](hib_rtc_timer_lsw_1p2) module"]
pub type HIB_RTC_TIMER_LSW_1P2 = crate::Reg<u32, _HIB_RTC_TIMER_LSW_1P2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_RTC_TIMER_LSW_1P2;
#[doc = "`read()` method returns [hib_rtc_timer_lsw_1p2::R](hib_rtc_timer_lsw_1p2::R) reader structure"]
impl crate::Readable for HIB_RTC_TIMER_LSW_1P2 {}
#[doc = "`write(|w| ..)` method takes [hib_rtc_timer_lsw_1p2::W](hib_rtc_timer_lsw_1p2::W) writer structure"]
impl crate::Writable for HIB_RTC_TIMER_LSW_1P2 {}
#[doc = "HIB_RTC_TIMER_LSW_1P2"]
pub mod hib_rtc_timer_lsw_1p2;
#[doc = "HIB_RTC_TIMER_MSW_1P2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_rtc_timer_msw_1p2](hib_rtc_timer_msw_1p2) module"]
pub type HIB_RTC_TIMER_MSW_1P2 = crate::Reg<u32, _HIB_RTC_TIMER_MSW_1P2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_RTC_TIMER_MSW_1P2;
#[doc = "`read()` method returns [hib_rtc_timer_msw_1p2::R](hib_rtc_timer_msw_1p2::R) reader structure"]
impl crate::Readable for HIB_RTC_TIMER_MSW_1P2 {}
#[doc = "`write(|w| ..)` method takes [hib_rtc_timer_msw_1p2::W](hib_rtc_timer_msw_1p2::W) writer structure"]
impl crate::Writable for HIB_RTC_TIMER_MSW_1P2 {}
#[doc = "HIB_RTC_TIMER_MSW_1P2"]
pub mod hib_rtc_timer_msw_1p2;
#[doc = "BGAP_TRIM_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgap_trim_overrides](bgap_trim_overrides) module"]
pub type BGAP_TRIM_OVERRIDES = crate::Reg<u32, _BGAP_TRIM_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGAP_TRIM_OVERRIDES;
#[doc = "`read()` method returns [bgap_trim_overrides::R](bgap_trim_overrides::R) reader structure"]
impl crate::Readable for BGAP_TRIM_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [bgap_trim_overrides::W](bgap_trim_overrides::W) writer structure"]
impl crate::Writable for BGAP_TRIM_OVERRIDES {}
#[doc = "BGAP_TRIM_OVERRIDES"]
pub mod bgap_trim_overrides;
#[doc = "EFUSE_READ_REG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg0](efuse_read_reg0) module"]
pub type EFUSE_READ_REG0 = crate::Reg<u32, _EFUSE_READ_REG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG0;
#[doc = "`read()` method returns [efuse_read_reg0::R](efuse_read_reg0::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG0 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg0::W](efuse_read_reg0::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG0 {}
#[doc = "EFUSE_READ_REG0"]
pub mod efuse_read_reg0;
#[doc = "EFUSE_READ_REG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg1](efuse_read_reg1) module"]
pub type EFUSE_READ_REG1 = crate::Reg<u32, _EFUSE_READ_REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG1;
#[doc = "`read()` method returns [efuse_read_reg1::R](efuse_read_reg1::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG1 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg1::W](efuse_read_reg1::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG1 {}
#[doc = "EFUSE_READ_REG1"]
pub mod efuse_read_reg1;
#[doc = "POR_TEST_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_test_ctrl](por_test_ctrl) module"]
pub type POR_TEST_CTRL = crate::Reg<u32, _POR_TEST_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POR_TEST_CTRL;
#[doc = "`read()` method returns [por_test_ctrl::R](por_test_ctrl::R) reader structure"]
impl crate::Readable for POR_TEST_CTRL {}
#[doc = "`write(|w| ..)` method takes [por_test_ctrl::W](por_test_ctrl::W) writer structure"]
impl crate::Writable for POR_TEST_CTRL {}
#[doc = "POR_TEST_CTRL"]
pub mod por_test_ctrl;
#[doc = "HIB_TIMER_SYNC_CALIB_CFG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_calib_cfg0](hib_timer_sync_calib_cfg0) module"]
pub type HIB_TIMER_SYNC_CALIB_CFG0 = crate::Reg<u32, _HIB_TIMER_SYNC_CALIB_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_CALIB_CFG0;
#[doc = "`read()` method returns [hib_timer_sync_calib_cfg0::R](hib_timer_sync_calib_cfg0::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_CALIB_CFG0 {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_calib_cfg0::W](hib_timer_sync_calib_cfg0::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_CALIB_CFG0 {}
#[doc = "HIB_TIMER_SYNC_CALIB_CFG0"]
pub mod hib_timer_sync_calib_cfg0;
#[doc = "HIB_TIMER_SYNC_CALIB_CFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_calib_cfg1](hib_timer_sync_calib_cfg1) module"]
pub type HIB_TIMER_SYNC_CALIB_CFG1 = crate::Reg<u32, _HIB_TIMER_SYNC_CALIB_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_CALIB_CFG1;
#[doc = "`read()` method returns [hib_timer_sync_calib_cfg1::R](hib_timer_sync_calib_cfg1::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_CALIB_CFG1 {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_calib_cfg1::W](hib_timer_sync_calib_cfg1::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_CALIB_CFG1 {}
#[doc = "HIB_TIMER_SYNC_CALIB_CFG1"]
pub mod hib_timer_sync_calib_cfg1;
#[doc = "HIB_TIMER_SYNC_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_cfg2](hib_timer_sync_cfg2) module"]
pub type HIB_TIMER_SYNC_CFG2 = crate::Reg<u32, _HIB_TIMER_SYNC_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_CFG2;
#[doc = "`read()` method returns [hib_timer_sync_cfg2::R](hib_timer_sync_cfg2::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_CFG2 {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_cfg2::W](hib_timer_sync_cfg2::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_CFG2 {}
#[doc = "HIB_TIMER_SYNC_CFG2"]
pub mod hib_timer_sync_cfg2;
#[doc = "HIB_TIMER_SYNC_TSF_ADJ_VAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_tsf_adj_val](hib_timer_sync_tsf_adj_val) module"]
pub type HIB_TIMER_SYNC_TSF_ADJ_VAL = crate::Reg<u32, _HIB_TIMER_SYNC_TSF_ADJ_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_TSF_ADJ_VAL;
#[doc = "`read()` method returns [hib_timer_sync_tsf_adj_val::R](hib_timer_sync_tsf_adj_val::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_TSF_ADJ_VAL {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_tsf_adj_val::W](hib_timer_sync_tsf_adj_val::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_TSF_ADJ_VAL {}
#[doc = "HIB_TIMER_SYNC_TSF_ADJ_VAL"]
pub mod hib_timer_sync_tsf_adj_val;
#[doc = "HIB_TIMER_RTC_GTS_TIMESTAMP_LSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_rtc_gts_timestamp_lsw](hib_timer_rtc_gts_timestamp_lsw) module"]
pub type HIB_TIMER_RTC_GTS_TIMESTAMP_LSW = crate::Reg<u32, _HIB_TIMER_RTC_GTS_TIMESTAMP_LSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_RTC_GTS_TIMESTAMP_LSW;
#[doc = "`read()` method returns [hib_timer_rtc_gts_timestamp_lsw::R](hib_timer_rtc_gts_timestamp_lsw::R) reader structure"]
impl crate::Readable for HIB_TIMER_RTC_GTS_TIMESTAMP_LSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_rtc_gts_timestamp_lsw::W](hib_timer_rtc_gts_timestamp_lsw::W) writer structure"]
impl crate::Writable for HIB_TIMER_RTC_GTS_TIMESTAMP_LSW {}
#[doc = "HIB_TIMER_RTC_GTS_TIMESTAMP_LSW"]
pub mod hib_timer_rtc_gts_timestamp_lsw;
#[doc = "HIB_TIMER_RTC_GTS_TIMESTAMP_MSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_rtc_gts_timestamp_msw](hib_timer_rtc_gts_timestamp_msw) module"]
pub type HIB_TIMER_RTC_GTS_TIMESTAMP_MSW = crate::Reg<u32, _HIB_TIMER_RTC_GTS_TIMESTAMP_MSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_RTC_GTS_TIMESTAMP_MSW;
#[doc = "`read()` method returns [hib_timer_rtc_gts_timestamp_msw::R](hib_timer_rtc_gts_timestamp_msw::R) reader structure"]
impl crate::Readable for HIB_TIMER_RTC_GTS_TIMESTAMP_MSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_rtc_gts_timestamp_msw::W](hib_timer_rtc_gts_timestamp_msw::W) writer structure"]
impl crate::Writable for HIB_TIMER_RTC_GTS_TIMESTAMP_MSW {}
#[doc = "HIB_TIMER_RTC_GTS_TIMESTAMP_MSW"]
pub mod hib_timer_rtc_gts_timestamp_msw;
#[doc = "HIB_TIMER_RTC_WUP_TIMESTAMP_LSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_rtc_wup_timestamp_lsw](hib_timer_rtc_wup_timestamp_lsw) module"]
pub type HIB_TIMER_RTC_WUP_TIMESTAMP_LSW = crate::Reg<u32, _HIB_TIMER_RTC_WUP_TIMESTAMP_LSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_RTC_WUP_TIMESTAMP_LSW;
#[doc = "`read()` method returns [hib_timer_rtc_wup_timestamp_lsw::R](hib_timer_rtc_wup_timestamp_lsw::R) reader structure"]
impl crate::Readable for HIB_TIMER_RTC_WUP_TIMESTAMP_LSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_rtc_wup_timestamp_lsw::W](hib_timer_rtc_wup_timestamp_lsw::W) writer structure"]
impl crate::Writable for HIB_TIMER_RTC_WUP_TIMESTAMP_LSW {}
#[doc = "HIB_TIMER_RTC_WUP_TIMESTAMP_LSW"]
pub mod hib_timer_rtc_wup_timestamp_lsw;
#[doc = "HIB_TIMER_RTC_WUP_TIMESTAMP_MSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_rtc_wup_timestamp_msw](hib_timer_rtc_wup_timestamp_msw) module"]
pub type HIB_TIMER_RTC_WUP_TIMESTAMP_MSW = crate::Reg<u32, _HIB_TIMER_RTC_WUP_TIMESTAMP_MSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_RTC_WUP_TIMESTAMP_MSW;
#[doc = "`read()` method returns [hib_timer_rtc_wup_timestamp_msw::R](hib_timer_rtc_wup_timestamp_msw::R) reader structure"]
impl crate::Readable for HIB_TIMER_RTC_WUP_TIMESTAMP_MSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_rtc_wup_timestamp_msw::W](hib_timer_rtc_wup_timestamp_msw::W) writer structure"]
impl crate::Writable for HIB_TIMER_RTC_WUP_TIMESTAMP_MSW {}
#[doc = "HIB_TIMER_RTC_WUP_TIMESTAMP_MSW"]
pub mod hib_timer_rtc_wup_timestamp_msw;
#[doc = "HIB_TIMER_SYNC_WAKE_OFFSET_ERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_wake_offset_err](hib_timer_sync_wake_offset_err) module"]
pub type HIB_TIMER_SYNC_WAKE_OFFSET_ERR = crate::Reg<u32, _HIB_TIMER_SYNC_WAKE_OFFSET_ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_WAKE_OFFSET_ERR;
#[doc = "`read()` method returns [hib_timer_sync_wake_offset_err::R](hib_timer_sync_wake_offset_err::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_WAKE_OFFSET_ERR {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_wake_offset_err::W](hib_timer_sync_wake_offset_err::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_WAKE_OFFSET_ERR {}
#[doc = "HIB_TIMER_SYNC_WAKE_OFFSET_ERR"]
pub mod hib_timer_sync_wake_offset_err;
#[doc = "HIB_TIMER_SYNC_TSF_CURR_VAL_LSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_tsf_curr_val_lsw](hib_timer_sync_tsf_curr_val_lsw) module"]
pub type HIB_TIMER_SYNC_TSF_CURR_VAL_LSW = crate::Reg<u32, _HIB_TIMER_SYNC_TSF_CURR_VAL_LSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_TSF_CURR_VAL_LSW;
#[doc = "`read()` method returns [hib_timer_sync_tsf_curr_val_lsw::R](hib_timer_sync_tsf_curr_val_lsw::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_TSF_CURR_VAL_LSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_tsf_curr_val_lsw::W](hib_timer_sync_tsf_curr_val_lsw::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_TSF_CURR_VAL_LSW {}
#[doc = "HIB_TIMER_SYNC_TSF_CURR_VAL_LSW"]
pub mod hib_timer_sync_tsf_curr_val_lsw;
#[doc = "HIB_TIMER_SYNC_TSF_CURR_VAL_MSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_timer_sync_tsf_curr_val_msw](hib_timer_sync_tsf_curr_val_msw) module"]
pub type HIB_TIMER_SYNC_TSF_CURR_VAL_MSW = crate::Reg<u32, _HIB_TIMER_SYNC_TSF_CURR_VAL_MSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TIMER_SYNC_TSF_CURR_VAL_MSW;
#[doc = "`read()` method returns [hib_timer_sync_tsf_curr_val_msw::R](hib_timer_sync_tsf_curr_val_msw::R) reader structure"]
impl crate::Readable for HIB_TIMER_SYNC_TSF_CURR_VAL_MSW {}
#[doc = "`write(|w| ..)` method takes [hib_timer_sync_tsf_curr_val_msw::W](hib_timer_sync_tsf_curr_val_msw::W) writer structure"]
impl crate::Writable for HIB_TIMER_SYNC_TSF_CURR_VAL_MSW {}
#[doc = "HIB_TIMER_SYNC_TSF_CURR_VAL_MSW"]
pub mod hib_timer_sync_tsf_curr_val_msw;
#[doc = "CM_SPARE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm_spare](cm_spare) module"]
pub type CM_SPARE = crate::Reg<u32, _CM_SPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM_SPARE;
#[doc = "`read()` method returns [cm_spare::R](cm_spare::R) reader structure"]
impl crate::Readable for CM_SPARE {}
#[doc = "`write(|w| ..)` method takes [cm_spare::W](cm_spare::W) writer structure"]
impl crate::Writable for CM_SPARE {}
#[doc = "CM_SPARE"]
pub mod cm_spare;
#[doc = "PORPOL_SPARE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porpol_spare](porpol_spare) module"]
pub type PORPOL_SPARE = crate::Reg<u32, _PORPOL_SPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORPOL_SPARE;
#[doc = "`read()` method returns [porpol_spare::R](porpol_spare::R) reader structure"]
impl crate::Readable for PORPOL_SPARE {}
#[doc = "`write(|w| ..)` method takes [porpol_spare::W](porpol_spare::W) writer structure"]
impl crate::Writable for PORPOL_SPARE {}
#[doc = "PORPOL_SPARE"]
pub mod porpol_spare;
#[doc = "MEM_DIG_DCDC_CLK_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_dig_dcdc_clk_config](mem_dig_dcdc_clk_config) module"]
pub type MEM_DIG_DCDC_CLK_CONFIG = crate::Reg<u32, _MEM_DIG_DCDC_CLK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_DIG_DCDC_CLK_CONFIG;
#[doc = "`read()` method returns [mem_dig_dcdc_clk_config::R](mem_dig_dcdc_clk_config::R) reader structure"]
impl crate::Readable for MEM_DIG_DCDC_CLK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_dig_dcdc_clk_config::W](mem_dig_dcdc_clk_config::W) writer structure"]
impl crate::Writable for MEM_DIG_DCDC_CLK_CONFIG {}
#[doc = "MEM_DIG_DCDC_CLK_CONFIG"]
pub mod mem_dig_dcdc_clk_config;
#[doc = "MEM_ANA_DCDC_CLK_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ana_dcdc_clk_config](mem_ana_dcdc_clk_config) module"]
pub type MEM_ANA_DCDC_CLK_CONFIG = crate::Reg<u32, _MEM_ANA_DCDC_CLK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_ANA_DCDC_CLK_CONFIG;
#[doc = "`read()` method returns [mem_ana_dcdc_clk_config::R](mem_ana_dcdc_clk_config::R) reader structure"]
impl crate::Readable for MEM_ANA_DCDC_CLK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_ana_dcdc_clk_config::W](mem_ana_dcdc_clk_config::W) writer structure"]
impl crate::Writable for MEM_ANA_DCDC_CLK_CONFIG {}
#[doc = "MEM_ANA_DCDC_CLK_CONFIG"]
pub mod mem_ana_dcdc_clk_config;
#[doc = "MEM_FLASH_DCDC_CLK_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_flash_dcdc_clk_config](mem_flash_dcdc_clk_config) module"]
pub type MEM_FLASH_DCDC_CLK_CONFIG = crate::Reg<u32, _MEM_FLASH_DCDC_CLK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_FLASH_DCDC_CLK_CONFIG;
#[doc = "`read()` method returns [mem_flash_dcdc_clk_config::R](mem_flash_dcdc_clk_config::R) reader structure"]
impl crate::Readable for MEM_FLASH_DCDC_CLK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_flash_dcdc_clk_config::W](mem_flash_dcdc_clk_config::W) writer structure"]
impl crate::Writable for MEM_FLASH_DCDC_CLK_CONFIG {}
#[doc = "MEM_FLASH_DCDC_CLK_CONFIG"]
pub mod mem_flash_dcdc_clk_config;
#[doc = "MEM_PA_DCDC_CLK_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pa_dcdc_clk_config](mem_pa_dcdc_clk_config) module"]
pub type MEM_PA_DCDC_CLK_CONFIG = crate::Reg<u32, _MEM_PA_DCDC_CLK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_PA_DCDC_CLK_CONFIG;
#[doc = "`read()` method returns [mem_pa_dcdc_clk_config::R](mem_pa_dcdc_clk_config::R) reader structure"]
impl crate::Readable for MEM_PA_DCDC_CLK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_pa_dcdc_clk_config::W](mem_pa_dcdc_clk_config::W) writer structure"]
impl crate::Writable for MEM_PA_DCDC_CLK_CONFIG {}
#[doc = "MEM_PA_DCDC_CLK_CONFIG"]
pub mod mem_pa_dcdc_clk_config;
#[doc = "MEM_SLDO_VNWA_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sldo_vnwa_override](mem_sldo_vnwa_override) module"]
pub type MEM_SLDO_VNWA_OVERRIDE = crate::Reg<u32, _MEM_SLDO_VNWA_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SLDO_VNWA_OVERRIDE;
#[doc = "`read()` method returns [mem_sldo_vnwa_override::R](mem_sldo_vnwa_override::R) reader structure"]
impl crate::Readable for MEM_SLDO_VNWA_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [mem_sldo_vnwa_override::W](mem_sldo_vnwa_override::W) writer structure"]
impl crate::Writable for MEM_SLDO_VNWA_OVERRIDE {}
#[doc = "MEM_SLDO_VNWA_OVERRIDE"]
pub mod mem_sldo_vnwa_override;
#[doc = "MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_bgap_duty_cycling_enable_override](mem_bgap_duty_cycling_enable_override) module"]
pub type MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE =
    crate::Reg<u32, _MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE;
#[doc = "`read()` method returns [mem_bgap_duty_cycling_enable_override::R](mem_bgap_duty_cycling_enable_override::R) reader structure"]
impl crate::Readable for MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [mem_bgap_duty_cycling_enable_override::W](mem_bgap_duty_cycling_enable_override::W) writer structure"]
impl crate::Writable for MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE {}
#[doc = "MEM_BGAP_DUTY_CYCLING_ENABLE_OVERRIDE"]
pub mod mem_bgap_duty_cycling_enable_override;
#[doc = "MEM_HIB_FSM_DEBUG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_fsm_debug](mem_hib_fsm_debug) module"]
pub type MEM_HIB_FSM_DEBUG = crate::Reg<u32, _MEM_HIB_FSM_DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_FSM_DEBUG;
#[doc = "`read()` method returns [mem_hib_fsm_debug::R](mem_hib_fsm_debug::R) reader structure"]
impl crate::Readable for MEM_HIB_FSM_DEBUG {}
#[doc = "`write(|w| ..)` method takes [mem_hib_fsm_debug::W](mem_hib_fsm_debug::W) writer structure"]
impl crate::Writable for MEM_HIB_FSM_DEBUG {}
#[doc = "MEM_HIB_FSM_DEBUG"]
pub mod mem_hib_fsm_debug;
#[doc = "MEM_SLDO_VNWA_SW_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sldo_vnwa_sw_ctrl](mem_sldo_vnwa_sw_ctrl) module"]
pub type MEM_SLDO_VNWA_SW_CTRL = crate::Reg<u32, _MEM_SLDO_VNWA_SW_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SLDO_VNWA_SW_CTRL;
#[doc = "`read()` method returns [mem_sldo_vnwa_sw_ctrl::R](mem_sldo_vnwa_sw_ctrl::R) reader structure"]
impl crate::Readable for MEM_SLDO_VNWA_SW_CTRL {}
#[doc = "`write(|w| ..)` method takes [mem_sldo_vnwa_sw_ctrl::W](mem_sldo_vnwa_sw_ctrl::W) writer structure"]
impl crate::Writable for MEM_SLDO_VNWA_SW_CTRL {}
#[doc = "MEM_SLDO_VNWA_SW_CTRL"]
pub mod mem_sldo_vnwa_sw_ctrl;
#[doc = "MEM_SLDO_WEAK_PROCESS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sldo_weak_process](mem_sldo_weak_process) module"]
pub type MEM_SLDO_WEAK_PROCESS = crate::Reg<u32, _MEM_SLDO_WEAK_PROCESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SLDO_WEAK_PROCESS;
#[doc = "`read()` method returns [mem_sldo_weak_process::R](mem_sldo_weak_process::R) reader structure"]
impl crate::Readable for MEM_SLDO_WEAK_PROCESS {}
#[doc = "`write(|w| ..)` method takes [mem_sldo_weak_process::W](mem_sldo_weak_process::W) writer structure"]
impl crate::Writable for MEM_SLDO_WEAK_PROCESS {}
#[doc = "MEM_SLDO_WEAK_PROCESS"]
pub mod mem_sldo_weak_process;
#[doc = "MEM_PA_DCDC_OV_UV_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pa_dcdc_ov_uv_status](mem_pa_dcdc_ov_uv_status) module"]
pub type MEM_PA_DCDC_OV_UV_STATUS = crate::Reg<u32, _MEM_PA_DCDC_OV_UV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_PA_DCDC_OV_UV_STATUS;
#[doc = "`read()` method returns [mem_pa_dcdc_ov_uv_status::R](mem_pa_dcdc_ov_uv_status::R) reader structure"]
impl crate::Readable for MEM_PA_DCDC_OV_UV_STATUS {}
#[doc = "`write(|w| ..)` method takes [mem_pa_dcdc_ov_uv_status::W](mem_pa_dcdc_ov_uv_status::W) writer structure"]
impl crate::Writable for MEM_PA_DCDC_OV_UV_STATUS {}
#[doc = "MEM_PA_DCDC_OV_UV_STATUS"]
pub mod mem_pa_dcdc_ov_uv_status;
#[doc = "MEM_CM_TEST_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_cm_test_mode](mem_cm_test_mode) module"]
pub type MEM_CM_TEST_MODE = crate::Reg<u32, _MEM_CM_TEST_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_CM_TEST_MODE;
#[doc = "`read()` method returns [mem_cm_test_mode::R](mem_cm_test_mode::R) reader structure"]
impl crate::Readable for MEM_CM_TEST_MODE {}
#[doc = "`write(|w| ..)` method takes [mem_cm_test_mode::W](mem_cm_test_mode::W) writer structure"]
impl crate::Writable for MEM_CM_TEST_MODE {}
#[doc = "MEM_CM_TEST_MODE"]
pub mod mem_cm_test_mode;
