#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MEM_HIB_REQ"]
    pub mem_hib_req: MEM_HIB_REQ,
    #[doc = "0x04 - MEM_HIB_RTC_TIMER_ENABLE"]
    pub mem_hib_rtc_timer_enable: MEM_HIB_RTC_TIMER_ENABLE,
    #[doc = "0x08 - MEM_HIB_RTC_TIMER_RESET"]
    pub mem_hib_rtc_timer_reset: MEM_HIB_RTC_TIMER_RESET,
    #[doc = "0x0c - MEM_HIB_RTC_TIMER_READ"]
    pub mem_hib_rtc_timer_read: MEM_HIB_RTC_TIMER_READ,
    #[doc = "0x10 - MEM_HIB_RTC_TIMER_LSW"]
    pub mem_hib_rtc_timer_lsw: MEM_HIB_RTC_TIMER_LSW,
    #[doc = "0x14 - MEM_HIB_RTC_TIMER_MSW"]
    pub mem_hib_rtc_timer_msw: MEM_HIB_RTC_TIMER_MSW,
    #[doc = "0x18 - MEM_HIB_RTC_WAKE_EN"]
    pub mem_hib_rtc_wake_en: MEM_HIB_RTC_WAKE_EN,
    #[doc = "0x1c - MEM_HIB_RTC_WAKE_LSW_CONF"]
    pub mem_hib_rtc_wake_lsw_conf: MEM_HIB_RTC_WAKE_LSW_CONF,
    #[doc = "0x20 - MEM_HIB_RTC_WAKE_MSW_CONF"]
    pub mem_hib_rtc_wake_msw_conf: MEM_HIB_RTC_WAKE_MSW_CONF,
    _reserved9: [u8; 8usize],
    #[doc = "0x2c - MEM_INT_OSC_CONF"]
    pub mem_int_osc_conf: MEM_INT_OSC_CONF,
    _reserved10: [u8; 4usize],
    #[doc = "0x34 - MEM_XTAL_OSC_CONF"]
    pub mem_xtal_osc_conf: MEM_XTAL_OSC_CONF,
    #[doc = "0x38 - MEM_BGAP_PARAMETERS0"]
    pub mem_bgap_parameters0: MEM_BGAP_PARAMETERS0,
    #[doc = "0x3c - MEM_BGAP_PARAMETERS1"]
    pub mem_bgap_parameters1: MEM_BGAP_PARAMETERS1,
    #[doc = "0x40 - MEM_HIB_DETECTION_STATUS"]
    pub mem_hib_detection_status: MEM_HIB_DETECTION_STATUS,
    #[doc = "0x44 - MEM_HIB_MISC_CONTROLS"]
    pub mem_hib_misc_controls: MEM_HIB_MISC_CONTROLS,
    _reserved15: [u8; 8usize],
    #[doc = "0x50 - MEM_HIB_CONFIG"]
    pub mem_hib_config: MEM_HIB_CONFIG,
    #[doc = "0x54 - MEM_HIB_RTC_IRQ_ENABLE"]
    pub mem_hib_rtc_irq_enable: MEM_HIB_RTC_IRQ_ENABLE,
    #[doc = "0x58 - MEM_HIB_RTC_IRQ_LSW_CONF"]
    pub mem_hib_rtc_irq_lsw_conf: MEM_HIB_RTC_IRQ_LSW_CONF,
    #[doc = "0x5c - MEM_HIB_RTC_IRQ_MSW_CONF"]
    pub mem_hib_rtc_irq_msw_conf: MEM_HIB_RTC_IRQ_MSW_CONF,
    _reserved19: [u8; 928usize],
    #[doc = "0x400 - MEM_HIB_UART_CONF"]
    pub mem_hib_uart_conf: MEM_HIB_UART_CONF,
    #[doc = "0x404 - MEM_GPIO_WAKE_EN"]
    pub mem_gpio_wake_en: MEM_GPIO_WAKE_EN,
    #[doc = "0x408 - MEM_GPIO_WAKE_CONF"]
    pub mem_gpio_wake_conf: MEM_GPIO_WAKE_CONF,
    #[doc = "0x40c - MEM_PAD_OEN_RET33_CONF"]
    pub mem_pad_oen_ret33_conf: MEM_PAD_OEN_RET33_CONF,
    #[doc = "0x410 - MEM_UART_RTS_OEN_RET33_CONF"]
    pub mem_uart_rts_oen_ret33_conf: MEM_UART_RTS_OEN_RET33_CONF,
    #[doc = "0x414 - MEM_JTAG_CONF"]
    pub mem_jtag_conf: MEM_JTAG_CONF,
    #[doc = "0x418 - MEM_HIB_REG0"]
    pub mem_hib_reg0: MEM_HIB_REG0,
    #[doc = "0x41c - MEM_HIB_REG1"]
    pub mem_hib_reg1: MEM_HIB_REG1,
    #[doc = "0x420 - MEM_HIB_REG2"]
    pub mem_hib_reg2: MEM_HIB_REG2,
    #[doc = "0x424 - MEM_HIB_REG3"]
    pub mem_hib_reg3: MEM_HIB_REG3,
    _reserved29: [u8; 52usize],
    #[doc = "0x45c - MEM_HIB_SEQUENCER_CFG0"]
    pub mem_hib_sequencer_cfg0: MEM_HIB_SEQUENCER_CFG0,
    #[doc = "0x460 - MEM_HIB_SEQUENCER_CFG1"]
    pub mem_hib_sequencer_cfg1: MEM_HIB_SEQUENCER_CFG1,
    #[doc = "0x464 - MEM_HIB_MISC_CONFIG"]
    pub mem_hib_misc_config: MEM_HIB_MISC_CONFIG,
    #[doc = "0x468 - MEM_HIB_WAKE_STATUS"]
    pub mem_hib_wake_status: MEM_HIB_WAKE_STATUS,
    #[doc = "0x46c - MEM_HIB_LPDS_GPIO_SEL"]
    pub mem_hib_lpds_gpio_sel: MEM_HIB_LPDS_GPIO_SEL,
    #[doc = "0x470 - MEM_HIB_SEQUENCER_CFG2"]
    pub mem_hib_sequencer_cfg2: MEM_HIB_SEQUENCER_CFG2,
    #[doc = "0x474 - HIBANA_SPARE_LOWV"]
    pub hibana_spare_lowv: HIBANA_SPARE_LOWV,
    #[doc = "0x478 - HIB_TMUX_CTRL"]
    pub hib_tmux_ctrl: HIB_TMUX_CTRL,
    #[doc = "0x47c - HIB_1P2_1P8_LDO_TRIM"]
    pub hib_1p2_1p8_ldo_trim: HIB_1P2_1P8_LDO_TRIM,
    #[doc = "0x480 - HIB_COMP_TRIM"]
    pub hib_comp_trim: HIB_COMP_TRIM,
    #[doc = "0x484 - HIB_EN_TS"]
    pub hib_en_ts: HIB_EN_TS,
    #[doc = "0x488 - HIB_1P8V_DET_EN"]
    pub hib_1p8v_det_en: HIB_1P8V_DET_EN,
    #[doc = "0x48c - HIB_VBAT_MON_EN"]
    pub hib_vbat_mon_en: HIB_VBAT_MON_EN,
    #[doc = "0x490 - HIB_NHIB_ENABLE"]
    pub hib_nhib_enable: HIB_NHIB_ENABLE,
    #[doc = "0x494 - HIB_UART_RTS_SW_ENABLE"]
    pub hib_uart_rts_sw_enable: HIB_UART_RTS_SW_ENABLE,
}
#[doc = "MEM_HIB_REQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_req](mem_hib_req) module"]
pub type MEM_HIB_REQ = crate::Reg<u32, _MEM_HIB_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_REQ;
#[doc = "`read()` method returns [mem_hib_req::R](mem_hib_req::R) reader structure"]
impl crate::Readable for MEM_HIB_REQ {}
#[doc = "`write(|w| ..)` method takes [mem_hib_req::W](mem_hib_req::W) writer structure"]
impl crate::Writable for MEM_HIB_REQ {}
#[doc = "MEM_HIB_REQ"]
pub mod mem_hib_req;
#[doc = "MEM_HIB_RTC_TIMER_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_timer_enable](mem_hib_rtc_timer_enable) module"]
pub type MEM_HIB_RTC_TIMER_ENABLE = crate::Reg<u32, _MEM_HIB_RTC_TIMER_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_TIMER_ENABLE;
#[doc = "`read()` method returns [mem_hib_rtc_timer_enable::R](mem_hib_rtc_timer_enable::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_TIMER_ENABLE {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_timer_enable::W](mem_hib_rtc_timer_enable::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_TIMER_ENABLE {}
#[doc = "MEM_HIB_RTC_TIMER_ENABLE"]
pub mod mem_hib_rtc_timer_enable;
#[doc = "MEM_HIB_RTC_TIMER_RESET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_timer_reset](mem_hib_rtc_timer_reset) module"]
pub type MEM_HIB_RTC_TIMER_RESET = crate::Reg<u32, _MEM_HIB_RTC_TIMER_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_TIMER_RESET;
#[doc = "`read()` method returns [mem_hib_rtc_timer_reset::R](mem_hib_rtc_timer_reset::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_TIMER_RESET {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_timer_reset::W](mem_hib_rtc_timer_reset::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_TIMER_RESET {}
#[doc = "MEM_HIB_RTC_TIMER_RESET"]
pub mod mem_hib_rtc_timer_reset;
#[doc = "MEM_HIB_RTC_TIMER_READ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_timer_read](mem_hib_rtc_timer_read) module"]
pub type MEM_HIB_RTC_TIMER_READ = crate::Reg<u32, _MEM_HIB_RTC_TIMER_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_TIMER_READ;
#[doc = "`read()` method returns [mem_hib_rtc_timer_read::R](mem_hib_rtc_timer_read::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_TIMER_READ {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_timer_read::W](mem_hib_rtc_timer_read::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_TIMER_READ {}
#[doc = "MEM_HIB_RTC_TIMER_READ"]
pub mod mem_hib_rtc_timer_read;
#[doc = "MEM_HIB_RTC_TIMER_LSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_timer_lsw](mem_hib_rtc_timer_lsw) module"]
pub type MEM_HIB_RTC_TIMER_LSW = crate::Reg<u32, _MEM_HIB_RTC_TIMER_LSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_TIMER_LSW;
#[doc = "`read()` method returns [mem_hib_rtc_timer_lsw::R](mem_hib_rtc_timer_lsw::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_TIMER_LSW {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_timer_lsw::W](mem_hib_rtc_timer_lsw::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_TIMER_LSW {}
#[doc = "MEM_HIB_RTC_TIMER_LSW"]
pub mod mem_hib_rtc_timer_lsw;
#[doc = "MEM_HIB_RTC_TIMER_MSW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_timer_msw](mem_hib_rtc_timer_msw) module"]
pub type MEM_HIB_RTC_TIMER_MSW = crate::Reg<u32, _MEM_HIB_RTC_TIMER_MSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_TIMER_MSW;
#[doc = "`read()` method returns [mem_hib_rtc_timer_msw::R](mem_hib_rtc_timer_msw::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_TIMER_MSW {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_timer_msw::W](mem_hib_rtc_timer_msw::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_TIMER_MSW {}
#[doc = "MEM_HIB_RTC_TIMER_MSW"]
pub mod mem_hib_rtc_timer_msw;
#[doc = "MEM_HIB_RTC_WAKE_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_wake_en](mem_hib_rtc_wake_en) module"]
pub type MEM_HIB_RTC_WAKE_EN = crate::Reg<u32, _MEM_HIB_RTC_WAKE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_WAKE_EN;
#[doc = "`read()` method returns [mem_hib_rtc_wake_en::R](mem_hib_rtc_wake_en::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_WAKE_EN {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_wake_en::W](mem_hib_rtc_wake_en::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_WAKE_EN {}
#[doc = "MEM_HIB_RTC_WAKE_EN"]
pub mod mem_hib_rtc_wake_en;
#[doc = "MEM_HIB_RTC_WAKE_LSW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_wake_lsw_conf](mem_hib_rtc_wake_lsw_conf) module"]
pub type MEM_HIB_RTC_WAKE_LSW_CONF = crate::Reg<u32, _MEM_HIB_RTC_WAKE_LSW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_WAKE_LSW_CONF;
#[doc = "`read()` method returns [mem_hib_rtc_wake_lsw_conf::R](mem_hib_rtc_wake_lsw_conf::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_WAKE_LSW_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_wake_lsw_conf::W](mem_hib_rtc_wake_lsw_conf::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_WAKE_LSW_CONF {}
#[doc = "MEM_HIB_RTC_WAKE_LSW_CONF"]
pub mod mem_hib_rtc_wake_lsw_conf;
#[doc = "MEM_HIB_RTC_WAKE_MSW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_wake_msw_conf](mem_hib_rtc_wake_msw_conf) module"]
pub type MEM_HIB_RTC_WAKE_MSW_CONF = crate::Reg<u32, _MEM_HIB_RTC_WAKE_MSW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_WAKE_MSW_CONF;
#[doc = "`read()` method returns [mem_hib_rtc_wake_msw_conf::R](mem_hib_rtc_wake_msw_conf::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_WAKE_MSW_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_wake_msw_conf::W](mem_hib_rtc_wake_msw_conf::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_WAKE_MSW_CONF {}
#[doc = "MEM_HIB_RTC_WAKE_MSW_CONF"]
pub mod mem_hib_rtc_wake_msw_conf;
#[doc = "MEM_INT_OSC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_int_osc_conf](mem_int_osc_conf) module"]
pub type MEM_INT_OSC_CONF = crate::Reg<u32, _MEM_INT_OSC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_INT_OSC_CONF;
#[doc = "`read()` method returns [mem_int_osc_conf::R](mem_int_osc_conf::R) reader structure"]
impl crate::Readable for MEM_INT_OSC_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_int_osc_conf::W](mem_int_osc_conf::W) writer structure"]
impl crate::Writable for MEM_INT_OSC_CONF {}
#[doc = "MEM_INT_OSC_CONF"]
pub mod mem_int_osc_conf;
#[doc = "MEM_XTAL_OSC_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_xtal_osc_conf](mem_xtal_osc_conf) module"]
pub type MEM_XTAL_OSC_CONF = crate::Reg<u32, _MEM_XTAL_OSC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_XTAL_OSC_CONF;
#[doc = "`read()` method returns [mem_xtal_osc_conf::R](mem_xtal_osc_conf::R) reader structure"]
impl crate::Readable for MEM_XTAL_OSC_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_xtal_osc_conf::W](mem_xtal_osc_conf::W) writer structure"]
impl crate::Writable for MEM_XTAL_OSC_CONF {}
#[doc = "MEM_XTAL_OSC_CONF"]
pub mod mem_xtal_osc_conf;
#[doc = "MEM_BGAP_PARAMETERS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_bgap_parameters0](mem_bgap_parameters0) module"]
pub type MEM_BGAP_PARAMETERS0 = crate::Reg<u32, _MEM_BGAP_PARAMETERS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_BGAP_PARAMETERS0;
#[doc = "`read()` method returns [mem_bgap_parameters0::R](mem_bgap_parameters0::R) reader structure"]
impl crate::Readable for MEM_BGAP_PARAMETERS0 {}
#[doc = "`write(|w| ..)` method takes [mem_bgap_parameters0::W](mem_bgap_parameters0::W) writer structure"]
impl crate::Writable for MEM_BGAP_PARAMETERS0 {}
#[doc = "MEM_BGAP_PARAMETERS0"]
pub mod mem_bgap_parameters0;
#[doc = "MEM_BGAP_PARAMETERS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_bgap_parameters1](mem_bgap_parameters1) module"]
pub type MEM_BGAP_PARAMETERS1 = crate::Reg<u32, _MEM_BGAP_PARAMETERS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_BGAP_PARAMETERS1;
#[doc = "`read()` method returns [mem_bgap_parameters1::R](mem_bgap_parameters1::R) reader structure"]
impl crate::Readable for MEM_BGAP_PARAMETERS1 {}
#[doc = "`write(|w| ..)` method takes [mem_bgap_parameters1::W](mem_bgap_parameters1::W) writer structure"]
impl crate::Writable for MEM_BGAP_PARAMETERS1 {}
#[doc = "MEM_BGAP_PARAMETERS1"]
pub mod mem_bgap_parameters1;
#[doc = "MEM_HIB_DETECTION_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_detection_status](mem_hib_detection_status) module"]
pub type MEM_HIB_DETECTION_STATUS = crate::Reg<u32, _MEM_HIB_DETECTION_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_DETECTION_STATUS;
#[doc = "`read()` method returns [mem_hib_detection_status::R](mem_hib_detection_status::R) reader structure"]
impl crate::Readable for MEM_HIB_DETECTION_STATUS {}
#[doc = "`write(|w| ..)` method takes [mem_hib_detection_status::W](mem_hib_detection_status::W) writer structure"]
impl crate::Writable for MEM_HIB_DETECTION_STATUS {}
#[doc = "MEM_HIB_DETECTION_STATUS"]
pub mod mem_hib_detection_status;
#[doc = "MEM_HIB_MISC_CONTROLS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_misc_controls](mem_hib_misc_controls) module"]
pub type MEM_HIB_MISC_CONTROLS = crate::Reg<u32, _MEM_HIB_MISC_CONTROLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_MISC_CONTROLS;
#[doc = "`read()` method returns [mem_hib_misc_controls::R](mem_hib_misc_controls::R) reader structure"]
impl crate::Readable for MEM_HIB_MISC_CONTROLS {}
#[doc = "`write(|w| ..)` method takes [mem_hib_misc_controls::W](mem_hib_misc_controls::W) writer structure"]
impl crate::Writable for MEM_HIB_MISC_CONTROLS {}
#[doc = "MEM_HIB_MISC_CONTROLS"]
pub mod mem_hib_misc_controls;
#[doc = "MEM_HIB_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_config](mem_hib_config) module"]
pub type MEM_HIB_CONFIG = crate::Reg<u32, _MEM_HIB_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_CONFIG;
#[doc = "`read()` method returns [mem_hib_config::R](mem_hib_config::R) reader structure"]
impl crate::Readable for MEM_HIB_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_hib_config::W](mem_hib_config::W) writer structure"]
impl crate::Writable for MEM_HIB_CONFIG {}
#[doc = "MEM_HIB_CONFIG"]
pub mod mem_hib_config;
#[doc = "MEM_HIB_RTC_IRQ_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_irq_enable](mem_hib_rtc_irq_enable) module"]
pub type MEM_HIB_RTC_IRQ_ENABLE = crate::Reg<u32, _MEM_HIB_RTC_IRQ_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_IRQ_ENABLE;
#[doc = "`read()` method returns [mem_hib_rtc_irq_enable::R](mem_hib_rtc_irq_enable::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_IRQ_ENABLE {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_irq_enable::W](mem_hib_rtc_irq_enable::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_IRQ_ENABLE {}
#[doc = "MEM_HIB_RTC_IRQ_ENABLE"]
pub mod mem_hib_rtc_irq_enable;
#[doc = "MEM_HIB_RTC_IRQ_LSW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_irq_lsw_conf](mem_hib_rtc_irq_lsw_conf) module"]
pub type MEM_HIB_RTC_IRQ_LSW_CONF = crate::Reg<u32, _MEM_HIB_RTC_IRQ_LSW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_IRQ_LSW_CONF;
#[doc = "`read()` method returns [mem_hib_rtc_irq_lsw_conf::R](mem_hib_rtc_irq_lsw_conf::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_IRQ_LSW_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_irq_lsw_conf::W](mem_hib_rtc_irq_lsw_conf::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_IRQ_LSW_CONF {}
#[doc = "MEM_HIB_RTC_IRQ_LSW_CONF"]
pub mod mem_hib_rtc_irq_lsw_conf;
#[doc = "MEM_HIB_RTC_IRQ_MSW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_rtc_irq_msw_conf](mem_hib_rtc_irq_msw_conf) module"]
pub type MEM_HIB_RTC_IRQ_MSW_CONF = crate::Reg<u32, _MEM_HIB_RTC_IRQ_MSW_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_RTC_IRQ_MSW_CONF;
#[doc = "`read()` method returns [mem_hib_rtc_irq_msw_conf::R](mem_hib_rtc_irq_msw_conf::R) reader structure"]
impl crate::Readable for MEM_HIB_RTC_IRQ_MSW_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_hib_rtc_irq_msw_conf::W](mem_hib_rtc_irq_msw_conf::W) writer structure"]
impl crate::Writable for MEM_HIB_RTC_IRQ_MSW_CONF {}
#[doc = "MEM_HIB_RTC_IRQ_MSW_CONF"]
pub mod mem_hib_rtc_irq_msw_conf;
#[doc = "MEM_HIB_UART_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_uart_conf](mem_hib_uart_conf) module"]
pub type MEM_HIB_UART_CONF = crate::Reg<u32, _MEM_HIB_UART_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_UART_CONF;
#[doc = "`read()` method returns [mem_hib_uart_conf::R](mem_hib_uart_conf::R) reader structure"]
impl crate::Readable for MEM_HIB_UART_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_hib_uart_conf::W](mem_hib_uart_conf::W) writer structure"]
impl crate::Writable for MEM_HIB_UART_CONF {}
#[doc = "MEM_HIB_UART_CONF"]
pub mod mem_hib_uart_conf;
#[doc = "MEM_GPIO_WAKE_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_gpio_wake_en](mem_gpio_wake_en) module"]
pub type MEM_GPIO_WAKE_EN = crate::Reg<u32, _MEM_GPIO_WAKE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_GPIO_WAKE_EN;
#[doc = "`read()` method returns [mem_gpio_wake_en::R](mem_gpio_wake_en::R) reader structure"]
impl crate::Readable for MEM_GPIO_WAKE_EN {}
#[doc = "`write(|w| ..)` method takes [mem_gpio_wake_en::W](mem_gpio_wake_en::W) writer structure"]
impl crate::Writable for MEM_GPIO_WAKE_EN {}
#[doc = "MEM_GPIO_WAKE_EN"]
pub mod mem_gpio_wake_en;
#[doc = "MEM_GPIO_WAKE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_gpio_wake_conf](mem_gpio_wake_conf) module"]
pub type MEM_GPIO_WAKE_CONF = crate::Reg<u32, _MEM_GPIO_WAKE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_GPIO_WAKE_CONF;
#[doc = "`read()` method returns [mem_gpio_wake_conf::R](mem_gpio_wake_conf::R) reader structure"]
impl crate::Readable for MEM_GPIO_WAKE_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_gpio_wake_conf::W](mem_gpio_wake_conf::W) writer structure"]
impl crate::Writable for MEM_GPIO_WAKE_CONF {}
#[doc = "MEM_GPIO_WAKE_CONF"]
pub mod mem_gpio_wake_conf;
#[doc = "MEM_PAD_OEN_RET33_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pad_oen_ret33_conf](mem_pad_oen_ret33_conf) module"]
pub type MEM_PAD_OEN_RET33_CONF = crate::Reg<u32, _MEM_PAD_OEN_RET33_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_PAD_OEN_RET33_CONF;
#[doc = "`read()` method returns [mem_pad_oen_ret33_conf::R](mem_pad_oen_ret33_conf::R) reader structure"]
impl crate::Readable for MEM_PAD_OEN_RET33_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_pad_oen_ret33_conf::W](mem_pad_oen_ret33_conf::W) writer structure"]
impl crate::Writable for MEM_PAD_OEN_RET33_CONF {}
#[doc = "MEM_PAD_OEN_RET33_CONF"]
pub mod mem_pad_oen_ret33_conf;
#[doc = "MEM_UART_RTS_OEN_RET33_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_uart_rts_oen_ret33_conf](mem_uart_rts_oen_ret33_conf) module"]
pub type MEM_UART_RTS_OEN_RET33_CONF = crate::Reg<u32, _MEM_UART_RTS_OEN_RET33_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_UART_RTS_OEN_RET33_CONF;
#[doc = "`read()` method returns [mem_uart_rts_oen_ret33_conf::R](mem_uart_rts_oen_ret33_conf::R) reader structure"]
impl crate::Readable for MEM_UART_RTS_OEN_RET33_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_uart_rts_oen_ret33_conf::W](mem_uart_rts_oen_ret33_conf::W) writer structure"]
impl crate::Writable for MEM_UART_RTS_OEN_RET33_CONF {}
#[doc = "MEM_UART_RTS_OEN_RET33_CONF"]
pub mod mem_uart_rts_oen_ret33_conf;
#[doc = "MEM_JTAG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_jtag_conf](mem_jtag_conf) module"]
pub type MEM_JTAG_CONF = crate::Reg<u32, _MEM_JTAG_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_JTAG_CONF;
#[doc = "`read()` method returns [mem_jtag_conf::R](mem_jtag_conf::R) reader structure"]
impl crate::Readable for MEM_JTAG_CONF {}
#[doc = "`write(|w| ..)` method takes [mem_jtag_conf::W](mem_jtag_conf::W) writer structure"]
impl crate::Writable for MEM_JTAG_CONF {}
#[doc = "MEM_JTAG_CONF"]
pub mod mem_jtag_conf;
#[doc = "MEM_HIB_REG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_reg0](mem_hib_reg0) module"]
pub type MEM_HIB_REG0 = crate::Reg<u32, _MEM_HIB_REG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_REG0;
#[doc = "`read()` method returns [mem_hib_reg0::R](mem_hib_reg0::R) reader structure"]
impl crate::Readable for MEM_HIB_REG0 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_reg0::W](mem_hib_reg0::W) writer structure"]
impl crate::Writable for MEM_HIB_REG0 {}
#[doc = "MEM_HIB_REG0"]
pub mod mem_hib_reg0;
#[doc = "MEM_HIB_REG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_reg1](mem_hib_reg1) module"]
pub type MEM_HIB_REG1 = crate::Reg<u32, _MEM_HIB_REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_REG1;
#[doc = "`read()` method returns [mem_hib_reg1::R](mem_hib_reg1::R) reader structure"]
impl crate::Readable for MEM_HIB_REG1 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_reg1::W](mem_hib_reg1::W) writer structure"]
impl crate::Writable for MEM_HIB_REG1 {}
#[doc = "MEM_HIB_REG1"]
pub mod mem_hib_reg1;
#[doc = "MEM_HIB_REG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_reg2](mem_hib_reg2) module"]
pub type MEM_HIB_REG2 = crate::Reg<u32, _MEM_HIB_REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_REG2;
#[doc = "`read()` method returns [mem_hib_reg2::R](mem_hib_reg2::R) reader structure"]
impl crate::Readable for MEM_HIB_REG2 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_reg2::W](mem_hib_reg2::W) writer structure"]
impl crate::Writable for MEM_HIB_REG2 {}
#[doc = "MEM_HIB_REG2"]
pub mod mem_hib_reg2;
#[doc = "MEM_HIB_REG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_reg3](mem_hib_reg3) module"]
pub type MEM_HIB_REG3 = crate::Reg<u32, _MEM_HIB_REG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_REG3;
#[doc = "`read()` method returns [mem_hib_reg3::R](mem_hib_reg3::R) reader structure"]
impl crate::Readable for MEM_HIB_REG3 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_reg3::W](mem_hib_reg3::W) writer structure"]
impl crate::Writable for MEM_HIB_REG3 {}
#[doc = "MEM_HIB_REG3"]
pub mod mem_hib_reg3;
#[doc = "MEM_HIB_SEQUENCER_CFG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_sequencer_cfg0](mem_hib_sequencer_cfg0) module"]
pub type MEM_HIB_SEQUENCER_CFG0 = crate::Reg<u32, _MEM_HIB_SEQUENCER_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_SEQUENCER_CFG0;
#[doc = "`read()` method returns [mem_hib_sequencer_cfg0::R](mem_hib_sequencer_cfg0::R) reader structure"]
impl crate::Readable for MEM_HIB_SEQUENCER_CFG0 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_sequencer_cfg0::W](mem_hib_sequencer_cfg0::W) writer structure"]
impl crate::Writable for MEM_HIB_SEQUENCER_CFG0 {}
#[doc = "MEM_HIB_SEQUENCER_CFG0"]
pub mod mem_hib_sequencer_cfg0;
#[doc = "MEM_HIB_SEQUENCER_CFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_sequencer_cfg1](mem_hib_sequencer_cfg1) module"]
pub type MEM_HIB_SEQUENCER_CFG1 = crate::Reg<u32, _MEM_HIB_SEQUENCER_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_SEQUENCER_CFG1;
#[doc = "`read()` method returns [mem_hib_sequencer_cfg1::R](mem_hib_sequencer_cfg1::R) reader structure"]
impl crate::Readable for MEM_HIB_SEQUENCER_CFG1 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_sequencer_cfg1::W](mem_hib_sequencer_cfg1::W) writer structure"]
impl crate::Writable for MEM_HIB_SEQUENCER_CFG1 {}
#[doc = "MEM_HIB_SEQUENCER_CFG1"]
pub mod mem_hib_sequencer_cfg1;
#[doc = "MEM_HIB_MISC_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_misc_config](mem_hib_misc_config) module"]
pub type MEM_HIB_MISC_CONFIG = crate::Reg<u32, _MEM_HIB_MISC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_MISC_CONFIG;
#[doc = "`read()` method returns [mem_hib_misc_config::R](mem_hib_misc_config::R) reader structure"]
impl crate::Readable for MEM_HIB_MISC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_hib_misc_config::W](mem_hib_misc_config::W) writer structure"]
impl crate::Writable for MEM_HIB_MISC_CONFIG {}
#[doc = "MEM_HIB_MISC_CONFIG"]
pub mod mem_hib_misc_config;
#[doc = "MEM_HIB_WAKE_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_wake_status](mem_hib_wake_status) module"]
pub type MEM_HIB_WAKE_STATUS = crate::Reg<u32, _MEM_HIB_WAKE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_WAKE_STATUS;
#[doc = "`read()` method returns [mem_hib_wake_status::R](mem_hib_wake_status::R) reader structure"]
impl crate::Readable for MEM_HIB_WAKE_STATUS {}
#[doc = "`write(|w| ..)` method takes [mem_hib_wake_status::W](mem_hib_wake_status::W) writer structure"]
impl crate::Writable for MEM_HIB_WAKE_STATUS {}
#[doc = "MEM_HIB_WAKE_STATUS"]
pub mod mem_hib_wake_status;
#[doc = "MEM_HIB_LPDS_GPIO_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_lpds_gpio_sel](mem_hib_lpds_gpio_sel) module"]
pub type MEM_HIB_LPDS_GPIO_SEL = crate::Reg<u32, _MEM_HIB_LPDS_GPIO_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_LPDS_GPIO_SEL;
#[doc = "`read()` method returns [mem_hib_lpds_gpio_sel::R](mem_hib_lpds_gpio_sel::R) reader structure"]
impl crate::Readable for MEM_HIB_LPDS_GPIO_SEL {}
#[doc = "`write(|w| ..)` method takes [mem_hib_lpds_gpio_sel::W](mem_hib_lpds_gpio_sel::W) writer structure"]
impl crate::Writable for MEM_HIB_LPDS_GPIO_SEL {}
#[doc = "MEM_HIB_LPDS_GPIO_SEL"]
pub mod mem_hib_lpds_gpio_sel;
#[doc = "MEM_HIB_SEQUENCER_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hib_sequencer_cfg2](mem_hib_sequencer_cfg2) module"]
pub type MEM_HIB_SEQUENCER_CFG2 = crate::Reg<u32, _MEM_HIB_SEQUENCER_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HIB_SEQUENCER_CFG2;
#[doc = "`read()` method returns [mem_hib_sequencer_cfg2::R](mem_hib_sequencer_cfg2::R) reader structure"]
impl crate::Readable for MEM_HIB_SEQUENCER_CFG2 {}
#[doc = "`write(|w| ..)` method takes [mem_hib_sequencer_cfg2::W](mem_hib_sequencer_cfg2::W) writer structure"]
impl crate::Writable for MEM_HIB_SEQUENCER_CFG2 {}
#[doc = "MEM_HIB_SEQUENCER_CFG2"]
pub mod mem_hib_sequencer_cfg2;
#[doc = "HIBANA_SPARE_LOWV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hibana_spare_lowv](hibana_spare_lowv) module"]
pub type HIBANA_SPARE_LOWV = crate::Reg<u32, _HIBANA_SPARE_LOWV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIBANA_SPARE_LOWV;
#[doc = "`read()` method returns [hibana_spare_lowv::R](hibana_spare_lowv::R) reader structure"]
impl crate::Readable for HIBANA_SPARE_LOWV {}
#[doc = "`write(|w| ..)` method takes [hibana_spare_lowv::W](hibana_spare_lowv::W) writer structure"]
impl crate::Writable for HIBANA_SPARE_LOWV {}
#[doc = "HIBANA_SPARE_LOWV"]
pub mod hibana_spare_lowv;
#[doc = "HIB_TMUX_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_tmux_ctrl](hib_tmux_ctrl) module"]
pub type HIB_TMUX_CTRL = crate::Reg<u32, _HIB_TMUX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_TMUX_CTRL;
#[doc = "`read()` method returns [hib_tmux_ctrl::R](hib_tmux_ctrl::R) reader structure"]
impl crate::Readable for HIB_TMUX_CTRL {}
#[doc = "`write(|w| ..)` method takes [hib_tmux_ctrl::W](hib_tmux_ctrl::W) writer structure"]
impl crate::Writable for HIB_TMUX_CTRL {}
#[doc = "HIB_TMUX_CTRL"]
pub mod hib_tmux_ctrl;
#[doc = "HIB_1P2_1P8_LDO_TRIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_1p2_1p8_ldo_trim](hib_1p2_1p8_ldo_trim) module"]
pub type HIB_1P2_1P8_LDO_TRIM = crate::Reg<u32, _HIB_1P2_1P8_LDO_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_1P2_1P8_LDO_TRIM;
#[doc = "`read()` method returns [hib_1p2_1p8_ldo_trim::R](hib_1p2_1p8_ldo_trim::R) reader structure"]
impl crate::Readable for HIB_1P2_1P8_LDO_TRIM {}
#[doc = "`write(|w| ..)` method takes [hib_1p2_1p8_ldo_trim::W](hib_1p2_1p8_ldo_trim::W) writer structure"]
impl crate::Writable for HIB_1P2_1P8_LDO_TRIM {}
#[doc = "HIB_1P2_1P8_LDO_TRIM"]
pub mod hib_1p2_1p8_ldo_trim;
#[doc = "HIB_COMP_TRIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_comp_trim](hib_comp_trim) module"]
pub type HIB_COMP_TRIM = crate::Reg<u32, _HIB_COMP_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_COMP_TRIM;
#[doc = "`read()` method returns [hib_comp_trim::R](hib_comp_trim::R) reader structure"]
impl crate::Readable for HIB_COMP_TRIM {}
#[doc = "`write(|w| ..)` method takes [hib_comp_trim::W](hib_comp_trim::W) writer structure"]
impl crate::Writable for HIB_COMP_TRIM {}
#[doc = "HIB_COMP_TRIM"]
pub mod hib_comp_trim;
#[doc = "HIB_EN_TS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_en_ts](hib_en_ts) module"]
pub type HIB_EN_TS = crate::Reg<u32, _HIB_EN_TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_EN_TS;
#[doc = "`read()` method returns [hib_en_ts::R](hib_en_ts::R) reader structure"]
impl crate::Readable for HIB_EN_TS {}
#[doc = "`write(|w| ..)` method takes [hib_en_ts::W](hib_en_ts::W) writer structure"]
impl crate::Writable for HIB_EN_TS {}
#[doc = "HIB_EN_TS"]
pub mod hib_en_ts;
#[doc = "HIB_1P8V_DET_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_1p8v_det_en](hib_1p8v_det_en) module"]
pub type HIB_1P8V_DET_EN = crate::Reg<u32, _HIB_1P8V_DET_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_1P8V_DET_EN;
#[doc = "`read()` method returns [hib_1p8v_det_en::R](hib_1p8v_det_en::R) reader structure"]
impl crate::Readable for HIB_1P8V_DET_EN {}
#[doc = "`write(|w| ..)` method takes [hib_1p8v_det_en::W](hib_1p8v_det_en::W) writer structure"]
impl crate::Writable for HIB_1P8V_DET_EN {}
#[doc = "HIB_1P8V_DET_EN"]
pub mod hib_1p8v_det_en;
#[doc = "HIB_VBAT_MON_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_vbat_mon_en](hib_vbat_mon_en) module"]
pub type HIB_VBAT_MON_EN = crate::Reg<u32, _HIB_VBAT_MON_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_VBAT_MON_EN;
#[doc = "`read()` method returns [hib_vbat_mon_en::R](hib_vbat_mon_en::R) reader structure"]
impl crate::Readable for HIB_VBAT_MON_EN {}
#[doc = "`write(|w| ..)` method takes [hib_vbat_mon_en::W](hib_vbat_mon_en::W) writer structure"]
impl crate::Writable for HIB_VBAT_MON_EN {}
#[doc = "HIB_VBAT_MON_EN"]
pub mod hib_vbat_mon_en;
#[doc = "HIB_NHIB_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_nhib_enable](hib_nhib_enable) module"]
pub type HIB_NHIB_ENABLE = crate::Reg<u32, _HIB_NHIB_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_NHIB_ENABLE;
#[doc = "`read()` method returns [hib_nhib_enable::R](hib_nhib_enable::R) reader structure"]
impl crate::Readable for HIB_NHIB_ENABLE {}
#[doc = "`write(|w| ..)` method takes [hib_nhib_enable::W](hib_nhib_enable::W) writer structure"]
impl crate::Writable for HIB_NHIB_ENABLE {}
#[doc = "HIB_NHIB_ENABLE"]
pub mod hib_nhib_enable;
#[doc = "HIB_UART_RTS_SW_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hib_uart_rts_sw_enable](hib_uart_rts_sw_enable) module"]
pub type HIB_UART_RTS_SW_ENABLE = crate::Reg<u32, _HIB_UART_RTS_SW_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIB_UART_RTS_SW_ENABLE;
#[doc = "`read()` method returns [hib_uart_rts_sw_enable::R](hib_uart_rts_sw_enable::R) reader structure"]
impl crate::Readable for HIB_UART_RTS_SW_ENABLE {}
#[doc = "`write(|w| ..)` method takes [hib_uart_rts_sw_enable::W](hib_uart_rts_sw_enable::W) writer structure"]
impl crate::Writable for HIB_UART_RTS_SW_ENABLE {}
#[doc = "HIB_UART_RTS_SW_ENABLE"]
pub mod hib_uart_rts_sw_enable;
