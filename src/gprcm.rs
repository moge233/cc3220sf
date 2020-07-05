#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APPS_SOFT_RESET"]
    pub apps_soft_reset: APPS_SOFT_RESET,
    #[doc = "0x04 - APPS_LPDS_WAKEUP_CFG"]
    pub apps_lpds_wakeup_cfg: APPS_LPDS_WAKEUP_CFG,
    #[doc = "0x08 - APPS_LPDS_WAKEUP_SRC"]
    pub apps_lpds_wakeup_src: APPS_LPDS_WAKEUP_SRC,
    #[doc = "0x0c - APPS_RESET_CAUSE"]
    pub apps_reset_cause: APPS_RESET_CAUSE,
    #[doc = "0x10 - APPS_LPDS_WAKETIME_OPP_CFG"]
    pub apps_lpds_waketime_opp_cfg: APPS_LPDS_WAKETIME_OPP_CFG,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - APPS_SRAM_DSLP_CFG"]
    pub apps_sram_dslp_cfg: APPS_SRAM_DSLP_CFG,
    #[doc = "0x1c - APPS_SRAM_LPDS_CFG"]
    pub apps_sram_lpds_cfg: APPS_SRAM_LPDS_CFG,
    #[doc = "0x20 - APPS_LPDS_WAKETIME_WAKE_CFG"]
    pub apps_lpds_waketime_wake_cfg: APPS_LPDS_WAKETIME_WAKE_CFG,
    _reserved8: [u8; 220usize],
    #[doc = "0x100 - TOP_DIE_ENABLE"]
    pub top_die_enable: TOP_DIE_ENABLE,
    #[doc = "0x104 - TOP_DIE_ENABLE_PARAMETERS"]
    pub top_die_enable_parameters: TOP_DIE_ENABLE_PARAMETERS,
    #[doc = "0x108 - MCU_GLOBAL_SOFT_RESET"]
    pub mcu_global_soft_reset: MCU_GLOBAL_SOFT_RESET,
    #[doc = "0x10c - ADC_CLK_CONFIG"]
    pub adc_clk_config: ADC_CLK_CONFIG,
    #[doc = "0x110 - APPS_GPIO_WAKE_CONF"]
    pub apps_gpio_wake_conf: APPS_GPIO_WAKE_CONF,
    #[doc = "0x114 - EN_NWP_BOOT_WO_DEVINIT"]
    pub en_nwp_boot_wo_devinit: EN_NWP_BOOT_WO_DEVINIT,
    #[doc = "0x118 - MEM_HCLK_DIV_CFG"]
    pub mem_hclk_div_cfg: MEM_HCLK_DIV_CFG,
    #[doc = "0x11c - MEM_SYSCLK_DIV_CFG"]
    pub mem_sysclk_div_cfg: MEM_SYSCLK_DIV_CFG,
    #[doc = "0x120 - APLLMCS_LOCK_TIME_CONF"]
    pub apllmcs_lock_time_conf: APLLMCS_LOCK_TIME_CONF,
    _reserved17: [u8; 732usize],
    #[doc = "0x400 - NWP_SOFT_RESET"]
    pub nwp_soft_reset: NWP_SOFT_RESET,
    #[doc = "0x404 - NWP_LPDS_WAKEUP_CFG"]
    pub nwp_lpds_wakeup_cfg: NWP_LPDS_WAKEUP_CFG,
    #[doc = "0x408 - NWP_LPDS_WAKEUP_SRC"]
    pub nwp_lpds_wakeup_src: NWP_LPDS_WAKEUP_SRC,
    #[doc = "0x40c - NWP_RESET_CAUSE"]
    pub nwp_reset_cause: NWP_RESET_CAUSE,
    #[doc = "0x410 - NWP_LPDS_WAKETIME_OPP_CFG"]
    pub nwp_lpds_waketime_opp_cfg: NWP_LPDS_WAKETIME_OPP_CFG,
    _reserved22: [u8; 4usize],
    #[doc = "0x418 - NWP_SRAM_DSLP_CFG"]
    pub nwp_sram_dslp_cfg: NWP_SRAM_DSLP_CFG,
    #[doc = "0x41c - NWP_SRAM_LPDS_CFG"]
    pub nwp_sram_lpds_cfg: NWP_SRAM_LPDS_CFG,
    #[doc = "0x420 - NWP_LPDS_WAKETIME_WAKE_CFG"]
    pub nwp_lpds_waketime_wake_cfg: NWP_LPDS_WAKETIME_WAKE_CFG,
    #[doc = "0x424 - NWP_AUTONMS_SPI_MASTER_SEL"]
    pub nwp_autonms_spi_master_sel: NWP_AUTONMS_SPI_MASTER_SEL,
    #[doc = "0x428 - NWP_AUTONMS_SPI_IDLE_REQ"]
    pub nwp_autonms_spi_idle_req: NWP_AUTONMS_SPI_IDLE_REQ,
    #[doc = "0x42c - WLAN_TO_NWP_WAKE_REQUEST"]
    pub wlan_to_nwp_wake_request: WLAN_TO_NWP_WAKE_REQUEST,
    #[doc = "0x430 - NWP_TO_WLAN_WAKE_REQUEST"]
    pub nwp_to_wlan_wake_request: NWP_TO_WLAN_WAKE_REQUEST,
    #[doc = "0x434 - NWP_GPIO_WAKE_CONF"]
    pub nwp_gpio_wake_conf: NWP_GPIO_WAKE_CONF,
    #[doc = "0x438 - EFUSE_READ_REG12"]
    pub efuse_read_reg12: EFUSE_READ_REG12,
    _reserved31: [u8; 12usize],
    #[doc = "0x448 - DIEID_READ_REG5"]
    pub dieid_read_reg5: DIEID_READ_REG5,
    #[doc = "0x44c - DIEID_READ_REG6"]
    pub dieid_read_reg6: DIEID_READ_REG6,
    _reserved33: [u8; 944usize],
    #[doc = "0x800 - REF_FSM_CFG0"]
    pub ref_fsm_cfg0: REF_FSM_CFG0,
    #[doc = "0x804 - REF_FSM_CFG1"]
    pub ref_fsm_cfg1: REF_FSM_CFG1,
    #[doc = "0x808 - APLLMCS_WLAN_CONFIG0_40"]
    pub apllmcs_wlan_config0_40: APLLMCS_WLAN_CONFIG0_40,
    #[doc = "0x80c - APLLMCS_WLAN_CONFIG1_40"]
    pub apllmcs_wlan_config1_40: APLLMCS_WLAN_CONFIG1_40,
    #[doc = "0x810 - APLLMCS_WLAN_CONFIG0_26"]
    pub apllmcs_wlan_config0_26: APLLMCS_WLAN_CONFIG0_26,
    #[doc = "0x814 - APLLMCS_WLAN_CONFIG1_26"]
    pub apllmcs_wlan_config1_26: APLLMCS_WLAN_CONFIG1_26,
    #[doc = "0x818 - APLLMCS_WLAN_OVERRIDES"]
    pub apllmcs_wlan_overrides: APLLMCS_WLAN_OVERRIDES,
    #[doc = "0x81c - APLLMCS_MCU_RUN_CONFIG0_38P4"]
    pub apllmcs_mcu_run_config0_38p4: APLLMCS_MCU_RUN_CONFIG0_38P4,
    #[doc = "0x820 - APLLMCS_MCU_RUN_CONFIG1_38P4"]
    pub apllmcs_mcu_run_config1_38p4: APLLMCS_MCU_RUN_CONFIG1_38P4,
    #[doc = "0x824 - APLLMCS_MCU_RUN_CONFIG0_26"]
    pub apllmcs_mcu_run_config0_26: APLLMCS_MCU_RUN_CONFIG0_26,
    #[doc = "0x828 - APLLMCS_MCU_RUN_CONFIG1_26"]
    pub apllmcs_mcu_run_config1_26: APLLMCS_MCU_RUN_CONFIG1_26,
    #[doc = "0x82c - SPARE_RW0"]
    pub spare_rw0: SPARE_RW0,
    #[doc = "0x830 - SPARE_RW1"]
    pub spare_rw1: SPARE_RW1,
    #[doc = "0x834 - APLLMCS_MCU_OVERRIDES"]
    pub apllmcs_mcu_overrides: APLLMCS_MCU_OVERRIDES,
    #[doc = "0x838 - SYSCLK_SWITCH_STATUS"]
    pub sysclk_switch_status: SYSCLK_SWITCH_STATUS,
    #[doc = "0x83c - REF_LDO_CONTROLS"]
    pub ref_ldo_controls: REF_LDO_CONTROLS,
    #[doc = "0x840 - REF_RTRIM_CONTROL"]
    pub ref_rtrim_control: REF_RTRIM_CONTROL,
    #[doc = "0x844 - REF_SLICER_CONTROLS0"]
    pub ref_slicer_controls0: REF_SLICER_CONTROLS0,
    #[doc = "0x848 - REF_SLICER_CONTROLS1"]
    pub ref_slicer_controls1: REF_SLICER_CONTROLS1,
    #[doc = "0x84c - REF_ANA_BGAP_CONTROLS0"]
    pub ref_ana_bgap_controls0: REF_ANA_BGAP_CONTROLS0,
    #[doc = "0x850 - REF_ANA_BGAP_CONTROLS1"]
    pub ref_ana_bgap_controls1: REF_ANA_BGAP_CONTROLS1,
    #[doc = "0x854 - REF_ANA_SPARE_CONTROLS0"]
    pub ref_ana_spare_controls0: REF_ANA_SPARE_CONTROLS0,
    #[doc = "0x858 - REF_ANA_SPARE_CONTROLS1"]
    pub ref_ana_spare_controls1: REF_ANA_SPARE_CONTROLS1,
    #[doc = "0x85c - MEMSS_PSCON_OVERRIDES0"]
    pub memss_pscon_overrides0: MEMSS_PSCON_OVERRIDES0,
    #[doc = "0x860 - MEMSS_PSCON_OVERRIDES1"]
    pub memss_pscon_overrides1: MEMSS_PSCON_OVERRIDES1,
    #[doc = "0x864 - PLL_REF_LOCK_OVERRIDES"]
    pub pll_ref_lock_overrides: PLL_REF_LOCK_OVERRIDES,
    #[doc = "0x868 - MCU_PSCON_DEBUG"]
    pub mcu_pscon_debug: MCU_PSCON_DEBUG,
    #[doc = "0x86c - MEMSS_PWR_PS"]
    pub memss_pwr_ps: MEMSS_PWR_PS,
    #[doc = "0x870 - REF_FSM_DEBUG"]
    pub ref_fsm_debug: REF_FSM_DEBUG,
    #[doc = "0x874 - MEM_SYS_OPP_REQ_OVERRIDE"]
    pub mem_sys_opp_req_override: MEM_SYS_OPP_REQ_OVERRIDE,
    #[doc = "0x878 - MEM_TESTCTRL_PD_OPP_CONFIG"]
    pub mem_testctrl_pd_opp_config: MEM_TESTCTRL_PD_OPP_CONFIG,
    #[doc = "0x87c - MEM_WL_FAST_CLK_REQ_OVERRIDES"]
    pub mem_wl_fast_clk_req_overrides: MEM_WL_FAST_CLK_REQ_OVERRIDES,
    #[doc = "0x880 - MEM_MCU_PD_MODE_REQ_OVERRIDES"]
    pub mem_mcu_pd_mode_req_overrides: MEM_MCU_PD_MODE_REQ_OVERRIDES,
    #[doc = "0x884 - MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES"]
    pub mem_mcspi_sram_off_req_overrides: MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES,
    #[doc = "0x888 - MEM_WLAN_APLLMCS_OVERRIDES"]
    pub mem_wlan_apllmcs_overrides: MEM_WLAN_APLLMCS_OVERRIDES,
    #[doc = "0x88c - MEM_REF_FSM_CFG2"]
    pub mem_ref_fsm_cfg2: MEM_REF_FSM_CFG2,
    _reserved69: [u8; 896usize],
    #[doc = "0xc10 - TESTCTRL_POWER_CTRL"]
    pub testctrl_power_ctrl: TESTCTRL_POWER_CTRL,
    #[doc = "0xc14 - SSDIO_POWER_CTRL"]
    pub ssdio_power_ctrl: SSDIO_POWER_CTRL,
    #[doc = "0xc18 - MCSPI_N1_POWER_CTRL"]
    pub mcspi_n1_power_ctrl: MCSPI_N1_POWER_CTRL,
    #[doc = "0xc1c - WELP_POWER_CTRL"]
    pub welp_power_ctrl: WELP_POWER_CTRL,
    #[doc = "0xc20 - WL_SDIO_POWER_CTRL"]
    pub wl_sdio_power_ctrl: WL_SDIO_POWER_CTRL,
    #[doc = "0xc24 - WLAN_SRAM_ACTIVE_PWR_CFG"]
    pub wlan_sram_active_pwr_cfg: WLAN_SRAM_ACTIVE_PWR_CFG,
    #[doc = "0xc28 - WLAN_SRAM_SLEEP_PWR_CFG"]
    pub wlan_sram_sleep_pwr_cfg: WLAN_SRAM_SLEEP_PWR_CFG,
    _reserved76: [u8; 4usize],
    #[doc = "0xc30 - APPS_SECURE_INIT_DONE"]
    pub apps_secure_init_done: APPS_SECURE_INIT_DONE,
    #[doc = "0xc34 - APPS_DEV_MODE_INIT_DONE"]
    pub apps_dev_mode_init_done: APPS_DEV_MODE_INIT_DONE,
    #[doc = "0xc38 - EN_APPS_REBOOT"]
    pub en_apps_reboot: EN_APPS_REBOOT,
    #[doc = "0xc3c - MEM_APPS_PERIPH_PRESENT"]
    pub mem_apps_periph_present: MEM_APPS_PERIPH_PRESENT,
    #[doc = "0xc40 - MEM_NWP_PERIPH_PRESENT"]
    pub mem_nwp_periph_present: MEM_NWP_PERIPH_PRESENT,
    #[doc = "0xc44 - MEM_SHARED_PERIPH_PRESENT"]
    pub mem_shared_periph_present: MEM_SHARED_PERIPH_PRESENT,
    #[doc = "0xc48 - NWP_PWR_STATE"]
    pub nwp_pwr_state: NWP_PWR_STATE,
    #[doc = "0xc4c - APPS_PWR_STATE"]
    pub apps_pwr_state: APPS_PWR_STATE,
    #[doc = "0xc50 - MCU_PWR_STATE"]
    pub mcu_pwr_state: MCU_PWR_STATE,
    #[doc = "0xc54 - WTOP_PM_PS"]
    pub wtop_pm_ps: WTOP_PM_PS,
    #[doc = "0xc58 - WTOP_PD_RESETZ_OVERRIDE_REG"]
    pub wtop_pd_resetz_override_reg: WTOP_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc5c - WELP_PD_RESETZ_OVERRIDE_REG"]
    pub welp_pd_resetz_override_reg: WELP_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc60 - WL_SDIO_PD_RESETZ_OVERRIDE_REG"]
    pub wl_sdio_pd_resetz_override_reg: WL_SDIO_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc64 - SSDIO_PD_RESETZ_OVERRIDE_REG"]
    pub ssdio_pd_resetz_override_reg: SSDIO_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc68 - MCSPI_N1_PD_RESETZ_OVERRIDE_REG"]
    pub mcspi_n1_pd_resetz_override_reg: MCSPI_N1_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc6c - TESTCTRL_PD_RESETZ_OVERRIDE_REG"]
    pub testctrl_pd_resetz_override_reg: TESTCTRL_PD_RESETZ_OVERRIDE_REG,
    #[doc = "0xc70 - MCU_PD_RESETZ_OVERRIDE_REG"]
    pub mcu_pd_resetz_override_reg: MCU_PD_RESETZ_OVERRIDE_REG,
    _reserved93: [u8; 4usize],
    #[doc = "0xc78 - EFUSE_READ_REG0"]
    pub efuse_read_reg0: EFUSE_READ_REG0,
    #[doc = "0xc7c - EFUSE_READ_REG1"]
    pub efuse_read_reg1: EFUSE_READ_REG1,
    #[doc = "0xc80 - EFUSE_READ_REG2"]
    pub efuse_read_reg2: EFUSE_READ_REG2,
    #[doc = "0xc84 - EFUSE_READ_REG3"]
    pub efuse_read_reg3: EFUSE_READ_REG3,
    #[doc = "0xc88 - WTOP_MEM_RET_CFG"]
    pub wtop_mem_ret_cfg: WTOP_MEM_RET_CFG,
    #[doc = "0xc8c - COEX_CLK_SWALLOW_CFG0"]
    pub coex_clk_swallow_cfg0: COEX_CLK_SWALLOW_CFG0,
    #[doc = "0xc90 - COEX_CLK_SWALLOW_CFG1"]
    pub coex_clk_swallow_cfg1: COEX_CLK_SWALLOW_CFG1,
    #[doc = "0xc94 - COEX_CLK_SWALLOW_CFG2"]
    pub coex_clk_swallow_cfg2: COEX_CLK_SWALLOW_CFG2,
    #[doc = "0xc98 - COEX_CLK_SWALLOW_ENABLE"]
    pub coex_clk_swallow_enable: COEX_CLK_SWALLOW_ENABLE,
    #[doc = "0xc9c - DCDC_CLK_GEN_CONFIG"]
    pub dcdc_clk_gen_config: DCDC_CLK_GEN_CONFIG,
    #[doc = "0xca0 - EFUSE_READ_REG4"]
    pub efuse_read_reg4: EFUSE_READ_REG4,
    #[doc = "0xca4 - EFUSE_READ_REG5"]
    pub efuse_read_reg5: EFUSE_READ_REG5,
    #[doc = "0xca8 - EFUSE_READ_REG6"]
    pub efuse_read_reg6: EFUSE_READ_REG6,
    #[doc = "0xcac - EFUSE_READ_REG7"]
    pub efuse_read_reg7: EFUSE_READ_REG7,
    #[doc = "0xcb0 - EFUSE_READ_REG8"]
    pub efuse_read_reg8: EFUSE_READ_REG8,
    #[doc = "0xcb4 - EFUSE_READ_REG9"]
    pub efuse_read_reg9: EFUSE_READ_REG9,
    #[doc = "0xcb8 - EFUSE_READ_REG10"]
    pub efuse_read_reg10: EFUSE_READ_REG10,
    #[doc = "0xcbc - EFUSE_READ_REG11"]
    pub efuse_read_reg11: EFUSE_READ_REG11,
    #[doc = "0xcc0 - DIEID_READ_REG0"]
    pub dieid_read_reg0: DIEID_READ_REG0,
    #[doc = "0xcc4 - DIEID_READ_REG1"]
    pub dieid_read_reg1: DIEID_READ_REG1,
    #[doc = "0xcc8 - DIEID_READ_REG2"]
    pub dieid_read_reg2: DIEID_READ_REG2,
    #[doc = "0xccc - DIEID_READ_REG3"]
    pub dieid_read_reg3: DIEID_READ_REG3,
    #[doc = "0xcd0 - DIEID_READ_REG4"]
    pub dieid_read_reg4: DIEID_READ_REG4,
    #[doc = "0xcd4 - APPS_SS_OVERRIDES"]
    pub apps_ss_overrides: APPS_SS_OVERRIDES,
    #[doc = "0xcd8 - NWP_SS_OVERRIDES"]
    pub nwp_ss_overrides: NWP_SS_OVERRIDES,
    #[doc = "0xcdc - SHARED_SS_OVERRIDES"]
    pub shared_ss_overrides: SHARED_SS_OVERRIDES,
    #[doc = "0xce0 - IDMEM_CORE_RST_OVERRIDES"]
    pub idmem_core_rst_overrides: IDMEM_CORE_RST_OVERRIDES,
    #[doc = "0xce4 - TOP_DIE_FSM_OVERRIDES"]
    pub top_die_fsm_overrides: TOP_DIE_FSM_OVERRIDES,
    #[doc = "0xce8 - MCU_PSCON_OVERRIDES"]
    pub mcu_pscon_overrides: MCU_PSCON_OVERRIDES,
    #[doc = "0xcec - WTOP_PSCON_OVERRIDES"]
    pub wtop_pscon_overrides: WTOP_PSCON_OVERRIDES,
    #[doc = "0xcf0 - WELP_PSCON_OVERRIDES"]
    pub welp_pscon_overrides: WELP_PSCON_OVERRIDES,
    #[doc = "0xcf4 - WL_SDIO_PSCON_OVERRIDES"]
    pub wl_sdio_pscon_overrides: WL_SDIO_PSCON_OVERRIDES,
    #[doc = "0xcf8 - MCSPI_PSCON_OVERRIDES"]
    pub mcspi_pscon_overrides: MCSPI_PSCON_OVERRIDES,
    #[doc = "0xcfc - SSDIO_PSCON_OVERRIDES"]
    pub ssdio_pscon_overrides: SSDIO_PSCON_OVERRIDES,
}
#[doc = "APPS_SOFT_RESET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_soft_reset](apps_soft_reset) module"]
pub type APPS_SOFT_RESET = crate::Reg<u32, _APPS_SOFT_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SOFT_RESET;
#[doc = "`read()` method returns [apps_soft_reset::R](apps_soft_reset::R) reader structure"]
impl crate::Readable for APPS_SOFT_RESET {}
#[doc = "`write(|w| ..)` method takes [apps_soft_reset::W](apps_soft_reset::W) writer structure"]
impl crate::Writable for APPS_SOFT_RESET {}
#[doc = "APPS_SOFT_RESET"]
pub mod apps_soft_reset;
#[doc = "APPS_LPDS_WAKEUP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_lpds_wakeup_cfg](apps_lpds_wakeup_cfg) module"]
pub type APPS_LPDS_WAKEUP_CFG = crate::Reg<u32, _APPS_LPDS_WAKEUP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_LPDS_WAKEUP_CFG;
#[doc = "`read()` method returns [apps_lpds_wakeup_cfg::R](apps_lpds_wakeup_cfg::R) reader structure"]
impl crate::Readable for APPS_LPDS_WAKEUP_CFG {}
#[doc = "`write(|w| ..)` method takes [apps_lpds_wakeup_cfg::W](apps_lpds_wakeup_cfg::W) writer structure"]
impl crate::Writable for APPS_LPDS_WAKEUP_CFG {}
#[doc = "APPS_LPDS_WAKEUP_CFG"]
pub mod apps_lpds_wakeup_cfg;
#[doc = "APPS_LPDS_WAKEUP_SRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_lpds_wakeup_src](apps_lpds_wakeup_src) module"]
pub type APPS_LPDS_WAKEUP_SRC = crate::Reg<u32, _APPS_LPDS_WAKEUP_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_LPDS_WAKEUP_SRC;
#[doc = "`read()` method returns [apps_lpds_wakeup_src::R](apps_lpds_wakeup_src::R) reader structure"]
impl crate::Readable for APPS_LPDS_WAKEUP_SRC {}
#[doc = "`write(|w| ..)` method takes [apps_lpds_wakeup_src::W](apps_lpds_wakeup_src::W) writer structure"]
impl crate::Writable for APPS_LPDS_WAKEUP_SRC {}
#[doc = "APPS_LPDS_WAKEUP_SRC"]
pub mod apps_lpds_wakeup_src;
#[doc = "APPS_RESET_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_reset_cause](apps_reset_cause) module"]
pub type APPS_RESET_CAUSE = crate::Reg<u32, _APPS_RESET_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_RESET_CAUSE;
#[doc = "`read()` method returns [apps_reset_cause::R](apps_reset_cause::R) reader structure"]
impl crate::Readable for APPS_RESET_CAUSE {}
#[doc = "`write(|w| ..)` method takes [apps_reset_cause::W](apps_reset_cause::W) writer structure"]
impl crate::Writable for APPS_RESET_CAUSE {}
#[doc = "APPS_RESET_CAUSE"]
pub mod apps_reset_cause;
#[doc = "APPS_LPDS_WAKETIME_OPP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_lpds_waketime_opp_cfg](apps_lpds_waketime_opp_cfg) module"]
pub type APPS_LPDS_WAKETIME_OPP_CFG = crate::Reg<u32, _APPS_LPDS_WAKETIME_OPP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_LPDS_WAKETIME_OPP_CFG;
#[doc = "`read()` method returns [apps_lpds_waketime_opp_cfg::R](apps_lpds_waketime_opp_cfg::R) reader structure"]
impl crate::Readable for APPS_LPDS_WAKETIME_OPP_CFG {}
#[doc = "`write(|w| ..)` method takes [apps_lpds_waketime_opp_cfg::W](apps_lpds_waketime_opp_cfg::W) writer structure"]
impl crate::Writable for APPS_LPDS_WAKETIME_OPP_CFG {}
#[doc = "APPS_LPDS_WAKETIME_OPP_CFG"]
pub mod apps_lpds_waketime_opp_cfg;
#[doc = "APPS_SRAM_DSLP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_sram_dslp_cfg](apps_sram_dslp_cfg) module"]
pub type APPS_SRAM_DSLP_CFG = crate::Reg<u32, _APPS_SRAM_DSLP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SRAM_DSLP_CFG;
#[doc = "`read()` method returns [apps_sram_dslp_cfg::R](apps_sram_dslp_cfg::R) reader structure"]
impl crate::Readable for APPS_SRAM_DSLP_CFG {}
#[doc = "`write(|w| ..)` method takes [apps_sram_dslp_cfg::W](apps_sram_dslp_cfg::W) writer structure"]
impl crate::Writable for APPS_SRAM_DSLP_CFG {}
#[doc = "APPS_SRAM_DSLP_CFG"]
pub mod apps_sram_dslp_cfg;
#[doc = "APPS_SRAM_LPDS_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_sram_lpds_cfg](apps_sram_lpds_cfg) module"]
pub type APPS_SRAM_LPDS_CFG = crate::Reg<u32, _APPS_SRAM_LPDS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SRAM_LPDS_CFG;
#[doc = "`read()` method returns [apps_sram_lpds_cfg::R](apps_sram_lpds_cfg::R) reader structure"]
impl crate::Readable for APPS_SRAM_LPDS_CFG {}
#[doc = "`write(|w| ..)` method takes [apps_sram_lpds_cfg::W](apps_sram_lpds_cfg::W) writer structure"]
impl crate::Writable for APPS_SRAM_LPDS_CFG {}
#[doc = "APPS_SRAM_LPDS_CFG"]
pub mod apps_sram_lpds_cfg;
#[doc = "APPS_LPDS_WAKETIME_WAKE_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_lpds_waketime_wake_cfg](apps_lpds_waketime_wake_cfg) module"]
pub type APPS_LPDS_WAKETIME_WAKE_CFG = crate::Reg<u32, _APPS_LPDS_WAKETIME_WAKE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_LPDS_WAKETIME_WAKE_CFG;
#[doc = "`read()` method returns [apps_lpds_waketime_wake_cfg::R](apps_lpds_waketime_wake_cfg::R) reader structure"]
impl crate::Readable for APPS_LPDS_WAKETIME_WAKE_CFG {}
#[doc = "`write(|w| ..)` method takes [apps_lpds_waketime_wake_cfg::W](apps_lpds_waketime_wake_cfg::W) writer structure"]
impl crate::Writable for APPS_LPDS_WAKETIME_WAKE_CFG {}
#[doc = "APPS_LPDS_WAKETIME_WAKE_CFG"]
pub mod apps_lpds_waketime_wake_cfg;
#[doc = "TOP_DIE_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_die_enable](top_die_enable) module"]
pub type TOP_DIE_ENABLE = crate::Reg<u32, _TOP_DIE_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP_DIE_ENABLE;
#[doc = "`read()` method returns [top_die_enable::R](top_die_enable::R) reader structure"]
impl crate::Readable for TOP_DIE_ENABLE {}
#[doc = "`write(|w| ..)` method takes [top_die_enable::W](top_die_enable::W) writer structure"]
impl crate::Writable for TOP_DIE_ENABLE {}
#[doc = "TOP_DIE_ENABLE"]
pub mod top_die_enable;
#[doc = "TOP_DIE_ENABLE_PARAMETERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_die_enable_parameters](top_die_enable_parameters) module"]
pub type TOP_DIE_ENABLE_PARAMETERS = crate::Reg<u32, _TOP_DIE_ENABLE_PARAMETERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP_DIE_ENABLE_PARAMETERS;
#[doc = "`read()` method returns [top_die_enable_parameters::R](top_die_enable_parameters::R) reader structure"]
impl crate::Readable for TOP_DIE_ENABLE_PARAMETERS {}
#[doc = "`write(|w| ..)` method takes [top_die_enable_parameters::W](top_die_enable_parameters::W) writer structure"]
impl crate::Writable for TOP_DIE_ENABLE_PARAMETERS {}
#[doc = "TOP_DIE_ENABLE_PARAMETERS"]
pub mod top_die_enable_parameters;
#[doc = "MCU_GLOBAL_SOFT_RESET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_global_soft_reset](mcu_global_soft_reset) module"]
pub type MCU_GLOBAL_SOFT_RESET = crate::Reg<u32, _MCU_GLOBAL_SOFT_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_GLOBAL_SOFT_RESET;
#[doc = "`read()` method returns [mcu_global_soft_reset::R](mcu_global_soft_reset::R) reader structure"]
impl crate::Readable for MCU_GLOBAL_SOFT_RESET {}
#[doc = "`write(|w| ..)` method takes [mcu_global_soft_reset::W](mcu_global_soft_reset::W) writer structure"]
impl crate::Writable for MCU_GLOBAL_SOFT_RESET {}
#[doc = "MCU_GLOBAL_SOFT_RESET"]
pub mod mcu_global_soft_reset;
#[doc = "ADC_CLK_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_clk_config](adc_clk_config) module"]
pub type ADC_CLK_CONFIG = crate::Reg<u32, _ADC_CLK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CLK_CONFIG;
#[doc = "`read()` method returns [adc_clk_config::R](adc_clk_config::R) reader structure"]
impl crate::Readable for ADC_CLK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [adc_clk_config::W](adc_clk_config::W) writer structure"]
impl crate::Writable for ADC_CLK_CONFIG {}
#[doc = "ADC_CLK_CONFIG"]
pub mod adc_clk_config;
#[doc = "APPS_GPIO_WAKE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_gpio_wake_conf](apps_gpio_wake_conf) module"]
pub type APPS_GPIO_WAKE_CONF = crate::Reg<u32, _APPS_GPIO_WAKE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_GPIO_WAKE_CONF;
#[doc = "`read()` method returns [apps_gpio_wake_conf::R](apps_gpio_wake_conf::R) reader structure"]
impl crate::Readable for APPS_GPIO_WAKE_CONF {}
#[doc = "`write(|w| ..)` method takes [apps_gpio_wake_conf::W](apps_gpio_wake_conf::W) writer structure"]
impl crate::Writable for APPS_GPIO_WAKE_CONF {}
#[doc = "APPS_GPIO_WAKE_CONF"]
pub mod apps_gpio_wake_conf;
#[doc = "EN_NWP_BOOT_WO_DEVINIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_nwp_boot_wo_devinit](en_nwp_boot_wo_devinit) module"]
pub type EN_NWP_BOOT_WO_DEVINIT = crate::Reg<u32, _EN_NWP_BOOT_WO_DEVINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_NWP_BOOT_WO_DEVINIT;
#[doc = "`read()` method returns [en_nwp_boot_wo_devinit::R](en_nwp_boot_wo_devinit::R) reader structure"]
impl crate::Readable for EN_NWP_BOOT_WO_DEVINIT {}
#[doc = "`write(|w| ..)` method takes [en_nwp_boot_wo_devinit::W](en_nwp_boot_wo_devinit::W) writer structure"]
impl crate::Writable for EN_NWP_BOOT_WO_DEVINIT {}
#[doc = "EN_NWP_BOOT_WO_DEVINIT"]
pub mod en_nwp_boot_wo_devinit;
#[doc = "MEM_HCLK_DIV_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_hclk_div_cfg](mem_hclk_div_cfg) module"]
pub type MEM_HCLK_DIV_CFG = crate::Reg<u32, _MEM_HCLK_DIV_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_HCLK_DIV_CFG;
#[doc = "`read()` method returns [mem_hclk_div_cfg::R](mem_hclk_div_cfg::R) reader structure"]
impl crate::Readable for MEM_HCLK_DIV_CFG {}
#[doc = "`write(|w| ..)` method takes [mem_hclk_div_cfg::W](mem_hclk_div_cfg::W) writer structure"]
impl crate::Writable for MEM_HCLK_DIV_CFG {}
#[doc = "MEM_HCLK_DIV_CFG"]
pub mod mem_hclk_div_cfg;
#[doc = "MEM_SYSCLK_DIV_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sysclk_div_cfg](mem_sysclk_div_cfg) module"]
pub type MEM_SYSCLK_DIV_CFG = crate::Reg<u32, _MEM_SYSCLK_DIV_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SYSCLK_DIV_CFG;
#[doc = "`read()` method returns [mem_sysclk_div_cfg::R](mem_sysclk_div_cfg::R) reader structure"]
impl crate::Readable for MEM_SYSCLK_DIV_CFG {}
#[doc = "`write(|w| ..)` method takes [mem_sysclk_div_cfg::W](mem_sysclk_div_cfg::W) writer structure"]
impl crate::Writable for MEM_SYSCLK_DIV_CFG {}
#[doc = "MEM_SYSCLK_DIV_CFG"]
pub mod mem_sysclk_div_cfg;
#[doc = "APLLMCS_LOCK_TIME_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_lock_time_conf](apllmcs_lock_time_conf) module"]
pub type APLLMCS_LOCK_TIME_CONF = crate::Reg<u32, _APLLMCS_LOCK_TIME_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_LOCK_TIME_CONF;
#[doc = "`read()` method returns [apllmcs_lock_time_conf::R](apllmcs_lock_time_conf::R) reader structure"]
impl crate::Readable for APLLMCS_LOCK_TIME_CONF {}
#[doc = "`write(|w| ..)` method takes [apllmcs_lock_time_conf::W](apllmcs_lock_time_conf::W) writer structure"]
impl crate::Writable for APLLMCS_LOCK_TIME_CONF {}
#[doc = "APLLMCS_LOCK_TIME_CONF"]
pub mod apllmcs_lock_time_conf;
#[doc = "NWP_SOFT_RESET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_soft_reset](nwp_soft_reset) module"]
pub type NWP_SOFT_RESET = crate::Reg<u32, _NWP_SOFT_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SOFT_RESET;
#[doc = "`read()` method returns [nwp_soft_reset::R](nwp_soft_reset::R) reader structure"]
impl crate::Readable for NWP_SOFT_RESET {}
#[doc = "`write(|w| ..)` method takes [nwp_soft_reset::W](nwp_soft_reset::W) writer structure"]
impl crate::Writable for NWP_SOFT_RESET {}
#[doc = "NWP_SOFT_RESET"]
pub mod nwp_soft_reset;
#[doc = "NWP_LPDS_WAKEUP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_lpds_wakeup_cfg](nwp_lpds_wakeup_cfg) module"]
pub type NWP_LPDS_WAKEUP_CFG = crate::Reg<u32, _NWP_LPDS_WAKEUP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_LPDS_WAKEUP_CFG;
#[doc = "`read()` method returns [nwp_lpds_wakeup_cfg::R](nwp_lpds_wakeup_cfg::R) reader structure"]
impl crate::Readable for NWP_LPDS_WAKEUP_CFG {}
#[doc = "`write(|w| ..)` method takes [nwp_lpds_wakeup_cfg::W](nwp_lpds_wakeup_cfg::W) writer structure"]
impl crate::Writable for NWP_LPDS_WAKEUP_CFG {}
#[doc = "NWP_LPDS_WAKEUP_CFG"]
pub mod nwp_lpds_wakeup_cfg;
#[doc = "NWP_LPDS_WAKEUP_SRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_lpds_wakeup_src](nwp_lpds_wakeup_src) module"]
pub type NWP_LPDS_WAKEUP_SRC = crate::Reg<u32, _NWP_LPDS_WAKEUP_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_LPDS_WAKEUP_SRC;
#[doc = "`read()` method returns [nwp_lpds_wakeup_src::R](nwp_lpds_wakeup_src::R) reader structure"]
impl crate::Readable for NWP_LPDS_WAKEUP_SRC {}
#[doc = "`write(|w| ..)` method takes [nwp_lpds_wakeup_src::W](nwp_lpds_wakeup_src::W) writer structure"]
impl crate::Writable for NWP_LPDS_WAKEUP_SRC {}
#[doc = "NWP_LPDS_WAKEUP_SRC"]
pub mod nwp_lpds_wakeup_src;
#[doc = "NWP_RESET_CAUSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_reset_cause](nwp_reset_cause) module"]
pub type NWP_RESET_CAUSE = crate::Reg<u32, _NWP_RESET_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_RESET_CAUSE;
#[doc = "`read()` method returns [nwp_reset_cause::R](nwp_reset_cause::R) reader structure"]
impl crate::Readable for NWP_RESET_CAUSE {}
#[doc = "`write(|w| ..)` method takes [nwp_reset_cause::W](nwp_reset_cause::W) writer structure"]
impl crate::Writable for NWP_RESET_CAUSE {}
#[doc = "NWP_RESET_CAUSE"]
pub mod nwp_reset_cause;
#[doc = "NWP_LPDS_WAKETIME_OPP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_lpds_waketime_opp_cfg](nwp_lpds_waketime_opp_cfg) module"]
pub type NWP_LPDS_WAKETIME_OPP_CFG = crate::Reg<u32, _NWP_LPDS_WAKETIME_OPP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_LPDS_WAKETIME_OPP_CFG;
#[doc = "`read()` method returns [nwp_lpds_waketime_opp_cfg::R](nwp_lpds_waketime_opp_cfg::R) reader structure"]
impl crate::Readable for NWP_LPDS_WAKETIME_OPP_CFG {}
#[doc = "`write(|w| ..)` method takes [nwp_lpds_waketime_opp_cfg::W](nwp_lpds_waketime_opp_cfg::W) writer structure"]
impl crate::Writable for NWP_LPDS_WAKETIME_OPP_CFG {}
#[doc = "NWP_LPDS_WAKETIME_OPP_CFG"]
pub mod nwp_lpds_waketime_opp_cfg;
#[doc = "NWP_SRAM_DSLP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_sram_dslp_cfg](nwp_sram_dslp_cfg) module"]
pub type NWP_SRAM_DSLP_CFG = crate::Reg<u32, _NWP_SRAM_DSLP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SRAM_DSLP_CFG;
#[doc = "`read()` method returns [nwp_sram_dslp_cfg::R](nwp_sram_dslp_cfg::R) reader structure"]
impl crate::Readable for NWP_SRAM_DSLP_CFG {}
#[doc = "`write(|w| ..)` method takes [nwp_sram_dslp_cfg::W](nwp_sram_dslp_cfg::W) writer structure"]
impl crate::Writable for NWP_SRAM_DSLP_CFG {}
#[doc = "NWP_SRAM_DSLP_CFG"]
pub mod nwp_sram_dslp_cfg;
#[doc = "NWP_SRAM_LPDS_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_sram_lpds_cfg](nwp_sram_lpds_cfg) module"]
pub type NWP_SRAM_LPDS_CFG = crate::Reg<u32, _NWP_SRAM_LPDS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SRAM_LPDS_CFG;
#[doc = "`read()` method returns [nwp_sram_lpds_cfg::R](nwp_sram_lpds_cfg::R) reader structure"]
impl crate::Readable for NWP_SRAM_LPDS_CFG {}
#[doc = "`write(|w| ..)` method takes [nwp_sram_lpds_cfg::W](nwp_sram_lpds_cfg::W) writer structure"]
impl crate::Writable for NWP_SRAM_LPDS_CFG {}
#[doc = "NWP_SRAM_LPDS_CFG"]
pub mod nwp_sram_lpds_cfg;
#[doc = "NWP_LPDS_WAKETIME_WAKE_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_lpds_waketime_wake_cfg](nwp_lpds_waketime_wake_cfg) module"]
pub type NWP_LPDS_WAKETIME_WAKE_CFG = crate::Reg<u32, _NWP_LPDS_WAKETIME_WAKE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_LPDS_WAKETIME_WAKE_CFG;
#[doc = "`read()` method returns [nwp_lpds_waketime_wake_cfg::R](nwp_lpds_waketime_wake_cfg::R) reader structure"]
impl crate::Readable for NWP_LPDS_WAKETIME_WAKE_CFG {}
#[doc = "`write(|w| ..)` method takes [nwp_lpds_waketime_wake_cfg::W](nwp_lpds_waketime_wake_cfg::W) writer structure"]
impl crate::Writable for NWP_LPDS_WAKETIME_WAKE_CFG {}
#[doc = "NWP_LPDS_WAKETIME_WAKE_CFG"]
pub mod nwp_lpds_waketime_wake_cfg;
#[doc = "NWP_AUTONMS_SPI_MASTER_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_autonms_spi_master_sel](nwp_autonms_spi_master_sel) module"]
pub type NWP_AUTONMS_SPI_MASTER_SEL = crate::Reg<u32, _NWP_AUTONMS_SPI_MASTER_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_AUTONMS_SPI_MASTER_SEL;
#[doc = "`read()` method returns [nwp_autonms_spi_master_sel::R](nwp_autonms_spi_master_sel::R) reader structure"]
impl crate::Readable for NWP_AUTONMS_SPI_MASTER_SEL {}
#[doc = "`write(|w| ..)` method takes [nwp_autonms_spi_master_sel::W](nwp_autonms_spi_master_sel::W) writer structure"]
impl crate::Writable for NWP_AUTONMS_SPI_MASTER_SEL {}
#[doc = "NWP_AUTONMS_SPI_MASTER_SEL"]
pub mod nwp_autonms_spi_master_sel;
#[doc = "NWP_AUTONMS_SPI_IDLE_REQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_autonms_spi_idle_req](nwp_autonms_spi_idle_req) module"]
pub type NWP_AUTONMS_SPI_IDLE_REQ = crate::Reg<u32, _NWP_AUTONMS_SPI_IDLE_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_AUTONMS_SPI_IDLE_REQ;
#[doc = "`read()` method returns [nwp_autonms_spi_idle_req::R](nwp_autonms_spi_idle_req::R) reader structure"]
impl crate::Readable for NWP_AUTONMS_SPI_IDLE_REQ {}
#[doc = "`write(|w| ..)` method takes [nwp_autonms_spi_idle_req::W](nwp_autonms_spi_idle_req::W) writer structure"]
impl crate::Writable for NWP_AUTONMS_SPI_IDLE_REQ {}
#[doc = "NWP_AUTONMS_SPI_IDLE_REQ"]
pub mod nwp_autonms_spi_idle_req;
#[doc = "WLAN_TO_NWP_WAKE_REQUEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlan_to_nwp_wake_request](wlan_to_nwp_wake_request) module"]
pub type WLAN_TO_NWP_WAKE_REQUEST = crate::Reg<u32, _WLAN_TO_NWP_WAKE_REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLAN_TO_NWP_WAKE_REQUEST;
#[doc = "`read()` method returns [wlan_to_nwp_wake_request::R](wlan_to_nwp_wake_request::R) reader structure"]
impl crate::Readable for WLAN_TO_NWP_WAKE_REQUEST {}
#[doc = "`write(|w| ..)` method takes [wlan_to_nwp_wake_request::W](wlan_to_nwp_wake_request::W) writer structure"]
impl crate::Writable for WLAN_TO_NWP_WAKE_REQUEST {}
#[doc = "WLAN_TO_NWP_WAKE_REQUEST"]
pub mod wlan_to_nwp_wake_request;
#[doc = "NWP_TO_WLAN_WAKE_REQUEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_to_wlan_wake_request](nwp_to_wlan_wake_request) module"]
pub type NWP_TO_WLAN_WAKE_REQUEST = crate::Reg<u32, _NWP_TO_WLAN_WAKE_REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_TO_WLAN_WAKE_REQUEST;
#[doc = "`read()` method returns [nwp_to_wlan_wake_request::R](nwp_to_wlan_wake_request::R) reader structure"]
impl crate::Readable for NWP_TO_WLAN_WAKE_REQUEST {}
#[doc = "`write(|w| ..)` method takes [nwp_to_wlan_wake_request::W](nwp_to_wlan_wake_request::W) writer structure"]
impl crate::Writable for NWP_TO_WLAN_WAKE_REQUEST {}
#[doc = "NWP_TO_WLAN_WAKE_REQUEST"]
pub mod nwp_to_wlan_wake_request;
#[doc = "NWP_GPIO_WAKE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_gpio_wake_conf](nwp_gpio_wake_conf) module"]
pub type NWP_GPIO_WAKE_CONF = crate::Reg<u32, _NWP_GPIO_WAKE_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_GPIO_WAKE_CONF;
#[doc = "`read()` method returns [nwp_gpio_wake_conf::R](nwp_gpio_wake_conf::R) reader structure"]
impl crate::Readable for NWP_GPIO_WAKE_CONF {}
#[doc = "`write(|w| ..)` method takes [nwp_gpio_wake_conf::W](nwp_gpio_wake_conf::W) writer structure"]
impl crate::Writable for NWP_GPIO_WAKE_CONF {}
#[doc = "NWP_GPIO_WAKE_CONF"]
pub mod nwp_gpio_wake_conf;
#[doc = "EFUSE_READ_REG12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg12](efuse_read_reg12) module"]
pub type EFUSE_READ_REG12 = crate::Reg<u32, _EFUSE_READ_REG12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG12;
#[doc = "`read()` method returns [efuse_read_reg12::R](efuse_read_reg12::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG12 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg12::W](efuse_read_reg12::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG12 {}
#[doc = "EFUSE_READ_REG12"]
pub mod efuse_read_reg12;
#[doc = "DIEID_READ_REG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg5](dieid_read_reg5) module"]
pub type DIEID_READ_REG5 = crate::Reg<u32, _DIEID_READ_REG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG5;
#[doc = "`read()` method returns [dieid_read_reg5::R](dieid_read_reg5::R) reader structure"]
impl crate::Readable for DIEID_READ_REG5 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg5::W](dieid_read_reg5::W) writer structure"]
impl crate::Writable for DIEID_READ_REG5 {}
#[doc = "DIEID_READ_REG5"]
pub mod dieid_read_reg5;
#[doc = "DIEID_READ_REG6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg6](dieid_read_reg6) module"]
pub type DIEID_READ_REG6 = crate::Reg<u32, _DIEID_READ_REG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG6;
#[doc = "`read()` method returns [dieid_read_reg6::R](dieid_read_reg6::R) reader structure"]
impl crate::Readable for DIEID_READ_REG6 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg6::W](dieid_read_reg6::W) writer structure"]
impl crate::Writable for DIEID_READ_REG6 {}
#[doc = "DIEID_READ_REG6"]
pub mod dieid_read_reg6;
#[doc = "REF_FSM_CFG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_fsm_cfg0](ref_fsm_cfg0) module"]
pub type REF_FSM_CFG0 = crate::Reg<u32, _REF_FSM_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_FSM_CFG0;
#[doc = "`read()` method returns [ref_fsm_cfg0::R](ref_fsm_cfg0::R) reader structure"]
impl crate::Readable for REF_FSM_CFG0 {}
#[doc = "`write(|w| ..)` method takes [ref_fsm_cfg0::W](ref_fsm_cfg0::W) writer structure"]
impl crate::Writable for REF_FSM_CFG0 {}
#[doc = "REF_FSM_CFG0"]
pub mod ref_fsm_cfg0;
#[doc = "REF_FSM_CFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_fsm_cfg1](ref_fsm_cfg1) module"]
pub type REF_FSM_CFG1 = crate::Reg<u32, _REF_FSM_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_FSM_CFG1;
#[doc = "`read()` method returns [ref_fsm_cfg1::R](ref_fsm_cfg1::R) reader structure"]
impl crate::Readable for REF_FSM_CFG1 {}
#[doc = "`write(|w| ..)` method takes [ref_fsm_cfg1::W](ref_fsm_cfg1::W) writer structure"]
impl crate::Writable for REF_FSM_CFG1 {}
#[doc = "REF_FSM_CFG1"]
pub mod ref_fsm_cfg1;
#[doc = "APLLMCS_WLAN_CONFIG0_40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_wlan_config0_40](apllmcs_wlan_config0_40) module"]
pub type APLLMCS_WLAN_CONFIG0_40 = crate::Reg<u32, _APLLMCS_WLAN_CONFIG0_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_WLAN_CONFIG0_40;
#[doc = "`read()` method returns [apllmcs_wlan_config0_40::R](apllmcs_wlan_config0_40::R) reader structure"]
impl crate::Readable for APLLMCS_WLAN_CONFIG0_40 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_wlan_config0_40::W](apllmcs_wlan_config0_40::W) writer structure"]
impl crate::Writable for APLLMCS_WLAN_CONFIG0_40 {}
#[doc = "APLLMCS_WLAN_CONFIG0_40"]
pub mod apllmcs_wlan_config0_40;
#[doc = "APLLMCS_WLAN_CONFIG1_40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_wlan_config1_40](apllmcs_wlan_config1_40) module"]
pub type APLLMCS_WLAN_CONFIG1_40 = crate::Reg<u32, _APLLMCS_WLAN_CONFIG1_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_WLAN_CONFIG1_40;
#[doc = "`read()` method returns [apllmcs_wlan_config1_40::R](apllmcs_wlan_config1_40::R) reader structure"]
impl crate::Readable for APLLMCS_WLAN_CONFIG1_40 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_wlan_config1_40::W](apllmcs_wlan_config1_40::W) writer structure"]
impl crate::Writable for APLLMCS_WLAN_CONFIG1_40 {}
#[doc = "APLLMCS_WLAN_CONFIG1_40"]
pub mod apllmcs_wlan_config1_40;
#[doc = "APLLMCS_WLAN_CONFIG0_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_wlan_config0_26](apllmcs_wlan_config0_26) module"]
pub type APLLMCS_WLAN_CONFIG0_26 = crate::Reg<u32, _APLLMCS_WLAN_CONFIG0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_WLAN_CONFIG0_26;
#[doc = "`read()` method returns [apllmcs_wlan_config0_26::R](apllmcs_wlan_config0_26::R) reader structure"]
impl crate::Readable for APLLMCS_WLAN_CONFIG0_26 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_wlan_config0_26::W](apllmcs_wlan_config0_26::W) writer structure"]
impl crate::Writable for APLLMCS_WLAN_CONFIG0_26 {}
#[doc = "APLLMCS_WLAN_CONFIG0_26"]
pub mod apllmcs_wlan_config0_26;
#[doc = "APLLMCS_WLAN_CONFIG1_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_wlan_config1_26](apllmcs_wlan_config1_26) module"]
pub type APLLMCS_WLAN_CONFIG1_26 = crate::Reg<u32, _APLLMCS_WLAN_CONFIG1_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_WLAN_CONFIG1_26;
#[doc = "`read()` method returns [apllmcs_wlan_config1_26::R](apllmcs_wlan_config1_26::R) reader structure"]
impl crate::Readable for APLLMCS_WLAN_CONFIG1_26 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_wlan_config1_26::W](apllmcs_wlan_config1_26::W) writer structure"]
impl crate::Writable for APLLMCS_WLAN_CONFIG1_26 {}
#[doc = "APLLMCS_WLAN_CONFIG1_26"]
pub mod apllmcs_wlan_config1_26;
#[doc = "APLLMCS_WLAN_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_wlan_overrides](apllmcs_wlan_overrides) module"]
pub type APLLMCS_WLAN_OVERRIDES = crate::Reg<u32, _APLLMCS_WLAN_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_WLAN_OVERRIDES;
#[doc = "`read()` method returns [apllmcs_wlan_overrides::R](apllmcs_wlan_overrides::R) reader structure"]
impl crate::Readable for APLLMCS_WLAN_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [apllmcs_wlan_overrides::W](apllmcs_wlan_overrides::W) writer structure"]
impl crate::Writable for APLLMCS_WLAN_OVERRIDES {}
#[doc = "APLLMCS_WLAN_OVERRIDES"]
pub mod apllmcs_wlan_overrides;
#[doc = "APLLMCS_MCU_RUN_CONFIG0_38P4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_mcu_run_config0_38p4](apllmcs_mcu_run_config0_38p4) module"]
pub type APLLMCS_MCU_RUN_CONFIG0_38P4 = crate::Reg<u32, _APLLMCS_MCU_RUN_CONFIG0_38P4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_MCU_RUN_CONFIG0_38P4;
#[doc = "`read()` method returns [apllmcs_mcu_run_config0_38p4::R](apllmcs_mcu_run_config0_38p4::R) reader structure"]
impl crate::Readable for APLLMCS_MCU_RUN_CONFIG0_38P4 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_mcu_run_config0_38p4::W](apllmcs_mcu_run_config0_38p4::W) writer structure"]
impl crate::Writable for APLLMCS_MCU_RUN_CONFIG0_38P4 {}
#[doc = "APLLMCS_MCU_RUN_CONFIG0_38P4"]
pub mod apllmcs_mcu_run_config0_38p4;
#[doc = "APLLMCS_MCU_RUN_CONFIG1_38P4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_mcu_run_config1_38p4](apllmcs_mcu_run_config1_38p4) module"]
pub type APLLMCS_MCU_RUN_CONFIG1_38P4 = crate::Reg<u32, _APLLMCS_MCU_RUN_CONFIG1_38P4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_MCU_RUN_CONFIG1_38P4;
#[doc = "`read()` method returns [apllmcs_mcu_run_config1_38p4::R](apllmcs_mcu_run_config1_38p4::R) reader structure"]
impl crate::Readable for APLLMCS_MCU_RUN_CONFIG1_38P4 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_mcu_run_config1_38p4::W](apllmcs_mcu_run_config1_38p4::W) writer structure"]
impl crate::Writable for APLLMCS_MCU_RUN_CONFIG1_38P4 {}
#[doc = "APLLMCS_MCU_RUN_CONFIG1_38P4"]
pub mod apllmcs_mcu_run_config1_38p4;
#[doc = "APLLMCS_MCU_RUN_CONFIG0_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_mcu_run_config0_26](apllmcs_mcu_run_config0_26) module"]
pub type APLLMCS_MCU_RUN_CONFIG0_26 = crate::Reg<u32, _APLLMCS_MCU_RUN_CONFIG0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_MCU_RUN_CONFIG0_26;
#[doc = "`read()` method returns [apllmcs_mcu_run_config0_26::R](apllmcs_mcu_run_config0_26::R) reader structure"]
impl crate::Readable for APLLMCS_MCU_RUN_CONFIG0_26 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_mcu_run_config0_26::W](apllmcs_mcu_run_config0_26::W) writer structure"]
impl crate::Writable for APLLMCS_MCU_RUN_CONFIG0_26 {}
#[doc = "APLLMCS_MCU_RUN_CONFIG0_26"]
pub mod apllmcs_mcu_run_config0_26;
#[doc = "APLLMCS_MCU_RUN_CONFIG1_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_mcu_run_config1_26](apllmcs_mcu_run_config1_26) module"]
pub type APLLMCS_MCU_RUN_CONFIG1_26 = crate::Reg<u32, _APLLMCS_MCU_RUN_CONFIG1_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_MCU_RUN_CONFIG1_26;
#[doc = "`read()` method returns [apllmcs_mcu_run_config1_26::R](apllmcs_mcu_run_config1_26::R) reader structure"]
impl crate::Readable for APLLMCS_MCU_RUN_CONFIG1_26 {}
#[doc = "`write(|w| ..)` method takes [apllmcs_mcu_run_config1_26::W](apllmcs_mcu_run_config1_26::W) writer structure"]
impl crate::Writable for APLLMCS_MCU_RUN_CONFIG1_26 {}
#[doc = "APLLMCS_MCU_RUN_CONFIG1_26"]
pub mod apllmcs_mcu_run_config1_26;
#[doc = "SPARE_RW0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_rw0](spare_rw0) module"]
pub type SPARE_RW0 = crate::Reg<u32, _SPARE_RW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_RW0;
#[doc = "`read()` method returns [spare_rw0::R](spare_rw0::R) reader structure"]
impl crate::Readable for SPARE_RW0 {}
#[doc = "`write(|w| ..)` method takes [spare_rw0::W](spare_rw0::W) writer structure"]
impl crate::Writable for SPARE_RW0 {}
#[doc = "SPARE_RW0"]
pub mod spare_rw0;
#[doc = "SPARE_RW1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare_rw1](spare_rw1) module"]
pub type SPARE_RW1 = crate::Reg<u32, _SPARE_RW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE_RW1;
#[doc = "`read()` method returns [spare_rw1::R](spare_rw1::R) reader structure"]
impl crate::Readable for SPARE_RW1 {}
#[doc = "`write(|w| ..)` method takes [spare_rw1::W](spare_rw1::W) writer structure"]
impl crate::Writable for SPARE_RW1 {}
#[doc = "SPARE_RW1"]
pub mod spare_rw1;
#[doc = "APLLMCS_MCU_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllmcs_mcu_overrides](apllmcs_mcu_overrides) module"]
pub type APLLMCS_MCU_OVERRIDES = crate::Reg<u32, _APLLMCS_MCU_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APLLMCS_MCU_OVERRIDES;
#[doc = "`read()` method returns [apllmcs_mcu_overrides::R](apllmcs_mcu_overrides::R) reader structure"]
impl crate::Readable for APLLMCS_MCU_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [apllmcs_mcu_overrides::W](apllmcs_mcu_overrides::W) writer structure"]
impl crate::Writable for APLLMCS_MCU_OVERRIDES {}
#[doc = "APLLMCS_MCU_OVERRIDES"]
pub mod apllmcs_mcu_overrides;
#[doc = "SYSCLK_SWITCH_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclk_switch_status](sysclk_switch_status) module"]
pub type SYSCLK_SWITCH_STATUS = crate::Reg<u32, _SYSCLK_SWITCH_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLK_SWITCH_STATUS;
#[doc = "`read()` method returns [sysclk_switch_status::R](sysclk_switch_status::R) reader structure"]
impl crate::Readable for SYSCLK_SWITCH_STATUS {}
#[doc = "`write(|w| ..)` method takes [sysclk_switch_status::W](sysclk_switch_status::W) writer structure"]
impl crate::Writable for SYSCLK_SWITCH_STATUS {}
#[doc = "SYSCLK_SWITCH_STATUS"]
pub mod sysclk_switch_status;
#[doc = "REF_LDO_CONTROLS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ldo_controls](ref_ldo_controls) module"]
pub type REF_LDO_CONTROLS = crate::Reg<u32, _REF_LDO_CONTROLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_LDO_CONTROLS;
#[doc = "`read()` method returns [ref_ldo_controls::R](ref_ldo_controls::R) reader structure"]
impl crate::Readable for REF_LDO_CONTROLS {}
#[doc = "`write(|w| ..)` method takes [ref_ldo_controls::W](ref_ldo_controls::W) writer structure"]
impl crate::Writable for REF_LDO_CONTROLS {}
#[doc = "REF_LDO_CONTROLS"]
pub mod ref_ldo_controls;
#[doc = "REF_RTRIM_CONTROL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_rtrim_control](ref_rtrim_control) module"]
pub type REF_RTRIM_CONTROL = crate::Reg<u32, _REF_RTRIM_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_RTRIM_CONTROL;
#[doc = "`read()` method returns [ref_rtrim_control::R](ref_rtrim_control::R) reader structure"]
impl crate::Readable for REF_RTRIM_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ref_rtrim_control::W](ref_rtrim_control::W) writer structure"]
impl crate::Writable for REF_RTRIM_CONTROL {}
#[doc = "REF_RTRIM_CONTROL"]
pub mod ref_rtrim_control;
#[doc = "REF_SLICER_CONTROLS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_slicer_controls0](ref_slicer_controls0) module"]
pub type REF_SLICER_CONTROLS0 = crate::Reg<u32, _REF_SLICER_CONTROLS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_SLICER_CONTROLS0;
#[doc = "`read()` method returns [ref_slicer_controls0::R](ref_slicer_controls0::R) reader structure"]
impl crate::Readable for REF_SLICER_CONTROLS0 {}
#[doc = "`write(|w| ..)` method takes [ref_slicer_controls0::W](ref_slicer_controls0::W) writer structure"]
impl crate::Writable for REF_SLICER_CONTROLS0 {}
#[doc = "REF_SLICER_CONTROLS0"]
pub mod ref_slicer_controls0;
#[doc = "REF_SLICER_CONTROLS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_slicer_controls1](ref_slicer_controls1) module"]
pub type REF_SLICER_CONTROLS1 = crate::Reg<u32, _REF_SLICER_CONTROLS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_SLICER_CONTROLS1;
#[doc = "`read()` method returns [ref_slicer_controls1::R](ref_slicer_controls1::R) reader structure"]
impl crate::Readable for REF_SLICER_CONTROLS1 {}
#[doc = "`write(|w| ..)` method takes [ref_slicer_controls1::W](ref_slicer_controls1::W) writer structure"]
impl crate::Writable for REF_SLICER_CONTROLS1 {}
#[doc = "REF_SLICER_CONTROLS1"]
pub mod ref_slicer_controls1;
#[doc = "REF_ANA_BGAP_CONTROLS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ana_bgap_controls0](ref_ana_bgap_controls0) module"]
pub type REF_ANA_BGAP_CONTROLS0 = crate::Reg<u32, _REF_ANA_BGAP_CONTROLS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_ANA_BGAP_CONTROLS0;
#[doc = "`read()` method returns [ref_ana_bgap_controls0::R](ref_ana_bgap_controls0::R) reader structure"]
impl crate::Readable for REF_ANA_BGAP_CONTROLS0 {}
#[doc = "`write(|w| ..)` method takes [ref_ana_bgap_controls0::W](ref_ana_bgap_controls0::W) writer structure"]
impl crate::Writable for REF_ANA_BGAP_CONTROLS0 {}
#[doc = "REF_ANA_BGAP_CONTROLS0"]
pub mod ref_ana_bgap_controls0;
#[doc = "REF_ANA_BGAP_CONTROLS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ana_bgap_controls1](ref_ana_bgap_controls1) module"]
pub type REF_ANA_BGAP_CONTROLS1 = crate::Reg<u32, _REF_ANA_BGAP_CONTROLS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_ANA_BGAP_CONTROLS1;
#[doc = "`read()` method returns [ref_ana_bgap_controls1::R](ref_ana_bgap_controls1::R) reader structure"]
impl crate::Readable for REF_ANA_BGAP_CONTROLS1 {}
#[doc = "`write(|w| ..)` method takes [ref_ana_bgap_controls1::W](ref_ana_bgap_controls1::W) writer structure"]
impl crate::Writable for REF_ANA_BGAP_CONTROLS1 {}
#[doc = "REF_ANA_BGAP_CONTROLS1"]
pub mod ref_ana_bgap_controls1;
#[doc = "REF_ANA_SPARE_CONTROLS0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ana_spare_controls0](ref_ana_spare_controls0) module"]
pub type REF_ANA_SPARE_CONTROLS0 = crate::Reg<u32, _REF_ANA_SPARE_CONTROLS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_ANA_SPARE_CONTROLS0;
#[doc = "`read()` method returns [ref_ana_spare_controls0::R](ref_ana_spare_controls0::R) reader structure"]
impl crate::Readable for REF_ANA_SPARE_CONTROLS0 {}
#[doc = "`write(|w| ..)` method takes [ref_ana_spare_controls0::W](ref_ana_spare_controls0::W) writer structure"]
impl crate::Writable for REF_ANA_SPARE_CONTROLS0 {}
#[doc = "REF_ANA_SPARE_CONTROLS0"]
pub mod ref_ana_spare_controls0;
#[doc = "REF_ANA_SPARE_CONTROLS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_ana_spare_controls1](ref_ana_spare_controls1) module"]
pub type REF_ANA_SPARE_CONTROLS1 = crate::Reg<u32, _REF_ANA_SPARE_CONTROLS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_ANA_SPARE_CONTROLS1;
#[doc = "`read()` method returns [ref_ana_spare_controls1::R](ref_ana_spare_controls1::R) reader structure"]
impl crate::Readable for REF_ANA_SPARE_CONTROLS1 {}
#[doc = "`write(|w| ..)` method takes [ref_ana_spare_controls1::W](ref_ana_spare_controls1::W) writer structure"]
impl crate::Writable for REF_ANA_SPARE_CONTROLS1 {}
#[doc = "REF_ANA_SPARE_CONTROLS1"]
pub mod ref_ana_spare_controls1;
#[doc = "MEMSS_PSCON_OVERRIDES0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memss_pscon_overrides0](memss_pscon_overrides0) module"]
pub type MEMSS_PSCON_OVERRIDES0 = crate::Reg<u32, _MEMSS_PSCON_OVERRIDES0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSS_PSCON_OVERRIDES0;
#[doc = "`read()` method returns [memss_pscon_overrides0::R](memss_pscon_overrides0::R) reader structure"]
impl crate::Readable for MEMSS_PSCON_OVERRIDES0 {}
#[doc = "`write(|w| ..)` method takes [memss_pscon_overrides0::W](memss_pscon_overrides0::W) writer structure"]
impl crate::Writable for MEMSS_PSCON_OVERRIDES0 {}
#[doc = "MEMSS_PSCON_OVERRIDES0"]
pub mod memss_pscon_overrides0;
#[doc = "MEMSS_PSCON_OVERRIDES1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memss_pscon_overrides1](memss_pscon_overrides1) module"]
pub type MEMSS_PSCON_OVERRIDES1 = crate::Reg<u32, _MEMSS_PSCON_OVERRIDES1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSS_PSCON_OVERRIDES1;
#[doc = "`read()` method returns [memss_pscon_overrides1::R](memss_pscon_overrides1::R) reader structure"]
impl crate::Readable for MEMSS_PSCON_OVERRIDES1 {}
#[doc = "`write(|w| ..)` method takes [memss_pscon_overrides1::W](memss_pscon_overrides1::W) writer structure"]
impl crate::Writable for MEMSS_PSCON_OVERRIDES1 {}
#[doc = "MEMSS_PSCON_OVERRIDES1"]
pub mod memss_pscon_overrides1;
#[doc = "PLL_REF_LOCK_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_ref_lock_overrides](pll_ref_lock_overrides) module"]
pub type PLL_REF_LOCK_OVERRIDES = crate::Reg<u32, _PLL_REF_LOCK_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_REF_LOCK_OVERRIDES;
#[doc = "`read()` method returns [pll_ref_lock_overrides::R](pll_ref_lock_overrides::R) reader structure"]
impl crate::Readable for PLL_REF_LOCK_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [pll_ref_lock_overrides::W](pll_ref_lock_overrides::W) writer structure"]
impl crate::Writable for PLL_REF_LOCK_OVERRIDES {}
#[doc = "PLL_REF_LOCK_OVERRIDES"]
pub mod pll_ref_lock_overrides;
#[doc = "MCU_PSCON_DEBUG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_pscon_debug](mcu_pscon_debug) module"]
pub type MCU_PSCON_DEBUG = crate::Reg<u32, _MCU_PSCON_DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_PSCON_DEBUG;
#[doc = "`read()` method returns [mcu_pscon_debug::R](mcu_pscon_debug::R) reader structure"]
impl crate::Readable for MCU_PSCON_DEBUG {}
#[doc = "`write(|w| ..)` method takes [mcu_pscon_debug::W](mcu_pscon_debug::W) writer structure"]
impl crate::Writable for MCU_PSCON_DEBUG {}
#[doc = "MCU_PSCON_DEBUG"]
pub mod mcu_pscon_debug;
#[doc = "MEMSS_PWR_PS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memss_pwr_ps](memss_pwr_ps) module"]
pub type MEMSS_PWR_PS = crate::Reg<u32, _MEMSS_PWR_PS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSS_PWR_PS;
#[doc = "`read()` method returns [memss_pwr_ps::R](memss_pwr_ps::R) reader structure"]
impl crate::Readable for MEMSS_PWR_PS {}
#[doc = "`write(|w| ..)` method takes [memss_pwr_ps::W](memss_pwr_ps::W) writer structure"]
impl crate::Writable for MEMSS_PWR_PS {}
#[doc = "MEMSS_PWR_PS"]
pub mod memss_pwr_ps;
#[doc = "REF_FSM_DEBUG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_fsm_debug](ref_fsm_debug) module"]
pub type REF_FSM_DEBUG = crate::Reg<u32, _REF_FSM_DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REF_FSM_DEBUG;
#[doc = "`read()` method returns [ref_fsm_debug::R](ref_fsm_debug::R) reader structure"]
impl crate::Readable for REF_FSM_DEBUG {}
#[doc = "`write(|w| ..)` method takes [ref_fsm_debug::W](ref_fsm_debug::W) writer structure"]
impl crate::Writable for REF_FSM_DEBUG {}
#[doc = "REF_FSM_DEBUG"]
pub mod ref_fsm_debug;
#[doc = "MEM_SYS_OPP_REQ_OVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_sys_opp_req_override](mem_sys_opp_req_override) module"]
pub type MEM_SYS_OPP_REQ_OVERRIDE = crate::Reg<u32, _MEM_SYS_OPP_REQ_OVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SYS_OPP_REQ_OVERRIDE;
#[doc = "`read()` method returns [mem_sys_opp_req_override::R](mem_sys_opp_req_override::R) reader structure"]
impl crate::Readable for MEM_SYS_OPP_REQ_OVERRIDE {}
#[doc = "`write(|w| ..)` method takes [mem_sys_opp_req_override::W](mem_sys_opp_req_override::W) writer structure"]
impl crate::Writable for MEM_SYS_OPP_REQ_OVERRIDE {}
#[doc = "MEM_SYS_OPP_REQ_OVERRIDE"]
pub mod mem_sys_opp_req_override;
#[doc = "MEM_TESTCTRL_PD_OPP_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_testctrl_pd_opp_config](mem_testctrl_pd_opp_config) module"]
pub type MEM_TESTCTRL_PD_OPP_CONFIG = crate::Reg<u32, _MEM_TESTCTRL_PD_OPP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_TESTCTRL_PD_OPP_CONFIG;
#[doc = "`read()` method returns [mem_testctrl_pd_opp_config::R](mem_testctrl_pd_opp_config::R) reader structure"]
impl crate::Readable for MEM_TESTCTRL_PD_OPP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mem_testctrl_pd_opp_config::W](mem_testctrl_pd_opp_config::W) writer structure"]
impl crate::Writable for MEM_TESTCTRL_PD_OPP_CONFIG {}
#[doc = "MEM_TESTCTRL_PD_OPP_CONFIG"]
pub mod mem_testctrl_pd_opp_config;
#[doc = "MEM_WL_FAST_CLK_REQ_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_wl_fast_clk_req_overrides](mem_wl_fast_clk_req_overrides) module"]
pub type MEM_WL_FAST_CLK_REQ_OVERRIDES = crate::Reg<u32, _MEM_WL_FAST_CLK_REQ_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_WL_FAST_CLK_REQ_OVERRIDES;
#[doc = "`read()` method returns [mem_wl_fast_clk_req_overrides::R](mem_wl_fast_clk_req_overrides::R) reader structure"]
impl crate::Readable for MEM_WL_FAST_CLK_REQ_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mem_wl_fast_clk_req_overrides::W](mem_wl_fast_clk_req_overrides::W) writer structure"]
impl crate::Writable for MEM_WL_FAST_CLK_REQ_OVERRIDES {}
#[doc = "MEM_WL_FAST_CLK_REQ_OVERRIDES"]
pub mod mem_wl_fast_clk_req_overrides;
#[doc = "MEM_MCU_PD_MODE_REQ_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_mcu_pd_mode_req_overrides](mem_mcu_pd_mode_req_overrides) module"]
pub type MEM_MCU_PD_MODE_REQ_OVERRIDES = crate::Reg<u32, _MEM_MCU_PD_MODE_REQ_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_MCU_PD_MODE_REQ_OVERRIDES;
#[doc = "`read()` method returns [mem_mcu_pd_mode_req_overrides::R](mem_mcu_pd_mode_req_overrides::R) reader structure"]
impl crate::Readable for MEM_MCU_PD_MODE_REQ_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mem_mcu_pd_mode_req_overrides::W](mem_mcu_pd_mode_req_overrides::W) writer structure"]
impl crate::Writable for MEM_MCU_PD_MODE_REQ_OVERRIDES {}
#[doc = "MEM_MCU_PD_MODE_REQ_OVERRIDES"]
pub mod mem_mcu_pd_mode_req_overrides;
#[doc = "MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_mcspi_sram_off_req_overrides](mem_mcspi_sram_off_req_overrides) module"]
pub type MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES = crate::Reg<u32, _MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES;
#[doc = "`read()` method returns [mem_mcspi_sram_off_req_overrides::R](mem_mcspi_sram_off_req_overrides::R) reader structure"]
impl crate::Readable for MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mem_mcspi_sram_off_req_overrides::W](mem_mcspi_sram_off_req_overrides::W) writer structure"]
impl crate::Writable for MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES {}
#[doc = "MEM_MCSPI_SRAM_OFF_REQ_OVERRIDES"]
pub mod mem_mcspi_sram_off_req_overrides;
#[doc = "MEM_WLAN_APLLMCS_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_wlan_apllmcs_overrides](mem_wlan_apllmcs_overrides) module"]
pub type MEM_WLAN_APLLMCS_OVERRIDES = crate::Reg<u32, _MEM_WLAN_APLLMCS_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_WLAN_APLLMCS_OVERRIDES;
#[doc = "`read()` method returns [mem_wlan_apllmcs_overrides::R](mem_wlan_apllmcs_overrides::R) reader structure"]
impl crate::Readable for MEM_WLAN_APLLMCS_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mem_wlan_apllmcs_overrides::W](mem_wlan_apllmcs_overrides::W) writer structure"]
impl crate::Writable for MEM_WLAN_APLLMCS_OVERRIDES {}
#[doc = "MEM_WLAN_APLLMCS_OVERRIDES"]
pub mod mem_wlan_apllmcs_overrides;
#[doc = "MEM_REF_FSM_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_ref_fsm_cfg2](mem_ref_fsm_cfg2) module"]
pub type MEM_REF_FSM_CFG2 = crate::Reg<u32, _MEM_REF_FSM_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_REF_FSM_CFG2;
#[doc = "`read()` method returns [mem_ref_fsm_cfg2::R](mem_ref_fsm_cfg2::R) reader structure"]
impl crate::Readable for MEM_REF_FSM_CFG2 {}
#[doc = "`write(|w| ..)` method takes [mem_ref_fsm_cfg2::W](mem_ref_fsm_cfg2::W) writer structure"]
impl crate::Writable for MEM_REF_FSM_CFG2 {}
#[doc = "MEM_REF_FSM_CFG2"]
pub mod mem_ref_fsm_cfg2;
#[doc = "TESTCTRL_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testctrl_power_ctrl](testctrl_power_ctrl) module"]
pub type TESTCTRL_POWER_CTRL = crate::Reg<u32, _TESTCTRL_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESTCTRL_POWER_CTRL;
#[doc = "`read()` method returns [testctrl_power_ctrl::R](testctrl_power_ctrl::R) reader structure"]
impl crate::Readable for TESTCTRL_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [testctrl_power_ctrl::W](testctrl_power_ctrl::W) writer structure"]
impl crate::Writable for TESTCTRL_POWER_CTRL {}
#[doc = "TESTCTRL_POWER_CTRL"]
pub mod testctrl_power_ctrl;
#[doc = "SSDIO_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdio_power_ctrl](ssdio_power_ctrl) module"]
pub type SSDIO_POWER_CTRL = crate::Reg<u32, _SSDIO_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDIO_POWER_CTRL;
#[doc = "`read()` method returns [ssdio_power_ctrl::R](ssdio_power_ctrl::R) reader structure"]
impl crate::Readable for SSDIO_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [ssdio_power_ctrl::W](ssdio_power_ctrl::W) writer structure"]
impl crate::Writable for SSDIO_POWER_CTRL {}
#[doc = "SSDIO_POWER_CTRL"]
pub mod ssdio_power_ctrl;
#[doc = "MCSPI_N1_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcspi_n1_power_ctrl](mcspi_n1_power_ctrl) module"]
pub type MCSPI_N1_POWER_CTRL = crate::Reg<u32, _MCSPI_N1_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCSPI_N1_POWER_CTRL;
#[doc = "`read()` method returns [mcspi_n1_power_ctrl::R](mcspi_n1_power_ctrl::R) reader structure"]
impl crate::Readable for MCSPI_N1_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [mcspi_n1_power_ctrl::W](mcspi_n1_power_ctrl::W) writer structure"]
impl crate::Writable for MCSPI_N1_POWER_CTRL {}
#[doc = "MCSPI_N1_POWER_CTRL"]
pub mod mcspi_n1_power_ctrl;
#[doc = "WELP_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [welp_power_ctrl](welp_power_ctrl) module"]
pub type WELP_POWER_CTRL = crate::Reg<u32, _WELP_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WELP_POWER_CTRL;
#[doc = "`read()` method returns [welp_power_ctrl::R](welp_power_ctrl::R) reader structure"]
impl crate::Readable for WELP_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [welp_power_ctrl::W](welp_power_ctrl::W) writer structure"]
impl crate::Writable for WELP_POWER_CTRL {}
#[doc = "WELP_POWER_CTRL"]
pub mod welp_power_ctrl;
#[doc = "WL_SDIO_POWER_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_sdio_power_ctrl](wl_sdio_power_ctrl) module"]
pub type WL_SDIO_POWER_CTRL = crate::Reg<u32, _WL_SDIO_POWER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_SDIO_POWER_CTRL;
#[doc = "`read()` method returns [wl_sdio_power_ctrl::R](wl_sdio_power_ctrl::R) reader structure"]
impl crate::Readable for WL_SDIO_POWER_CTRL {}
#[doc = "`write(|w| ..)` method takes [wl_sdio_power_ctrl::W](wl_sdio_power_ctrl::W) writer structure"]
impl crate::Writable for WL_SDIO_POWER_CTRL {}
#[doc = "WL_SDIO_POWER_CTRL"]
pub mod wl_sdio_power_ctrl;
#[doc = "WLAN_SRAM_ACTIVE_PWR_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlan_sram_active_pwr_cfg](wlan_sram_active_pwr_cfg) module"]
pub type WLAN_SRAM_ACTIVE_PWR_CFG = crate::Reg<u32, _WLAN_SRAM_ACTIVE_PWR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLAN_SRAM_ACTIVE_PWR_CFG;
#[doc = "`read()` method returns [wlan_sram_active_pwr_cfg::R](wlan_sram_active_pwr_cfg::R) reader structure"]
impl crate::Readable for WLAN_SRAM_ACTIVE_PWR_CFG {}
#[doc = "`write(|w| ..)` method takes [wlan_sram_active_pwr_cfg::W](wlan_sram_active_pwr_cfg::W) writer structure"]
impl crate::Writable for WLAN_SRAM_ACTIVE_PWR_CFG {}
#[doc = "WLAN_SRAM_ACTIVE_PWR_CFG"]
pub mod wlan_sram_active_pwr_cfg;
#[doc = "WLAN_SRAM_SLEEP_PWR_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlan_sram_sleep_pwr_cfg](wlan_sram_sleep_pwr_cfg) module"]
pub type WLAN_SRAM_SLEEP_PWR_CFG = crate::Reg<u32, _WLAN_SRAM_SLEEP_PWR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLAN_SRAM_SLEEP_PWR_CFG;
#[doc = "`read()` method returns [wlan_sram_sleep_pwr_cfg::R](wlan_sram_sleep_pwr_cfg::R) reader structure"]
impl crate::Readable for WLAN_SRAM_SLEEP_PWR_CFG {}
#[doc = "`write(|w| ..)` method takes [wlan_sram_sleep_pwr_cfg::W](wlan_sram_sleep_pwr_cfg::W) writer structure"]
impl crate::Writable for WLAN_SRAM_SLEEP_PWR_CFG {}
#[doc = "WLAN_SRAM_SLEEP_PWR_CFG"]
pub mod wlan_sram_sleep_pwr_cfg;
#[doc = "APPS_SECURE_INIT_DONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_secure_init_done](apps_secure_init_done) module"]
pub type APPS_SECURE_INIT_DONE = crate::Reg<u32, _APPS_SECURE_INIT_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SECURE_INIT_DONE;
#[doc = "`read()` method returns [apps_secure_init_done::R](apps_secure_init_done::R) reader structure"]
impl crate::Readable for APPS_SECURE_INIT_DONE {}
#[doc = "`write(|w| ..)` method takes [apps_secure_init_done::W](apps_secure_init_done::W) writer structure"]
impl crate::Writable for APPS_SECURE_INIT_DONE {}
#[doc = "APPS_SECURE_INIT_DONE"]
pub mod apps_secure_init_done;
#[doc = "APPS_DEV_MODE_INIT_DONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_dev_mode_init_done](apps_dev_mode_init_done) module"]
pub type APPS_DEV_MODE_INIT_DONE = crate::Reg<u32, _APPS_DEV_MODE_INIT_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_DEV_MODE_INIT_DONE;
#[doc = "`read()` method returns [apps_dev_mode_init_done::R](apps_dev_mode_init_done::R) reader structure"]
impl crate::Readable for APPS_DEV_MODE_INIT_DONE {}
#[doc = "`write(|w| ..)` method takes [apps_dev_mode_init_done::W](apps_dev_mode_init_done::W) writer structure"]
impl crate::Writable for APPS_DEV_MODE_INIT_DONE {}
#[doc = "APPS_DEV_MODE_INIT_DONE"]
pub mod apps_dev_mode_init_done;
#[doc = "EN_APPS_REBOOT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_apps_reboot](en_apps_reboot) module"]
pub type EN_APPS_REBOOT = crate::Reg<u32, _EN_APPS_REBOOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_APPS_REBOOT;
#[doc = "`read()` method returns [en_apps_reboot::R](en_apps_reboot::R) reader structure"]
impl crate::Readable for EN_APPS_REBOOT {}
#[doc = "`write(|w| ..)` method takes [en_apps_reboot::W](en_apps_reboot::W) writer structure"]
impl crate::Writable for EN_APPS_REBOOT {}
#[doc = "EN_APPS_REBOOT"]
pub mod en_apps_reboot;
#[doc = "MEM_APPS_PERIPH_PRESENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_apps_periph_present](mem_apps_periph_present) module"]
pub type MEM_APPS_PERIPH_PRESENT = crate::Reg<u32, _MEM_APPS_PERIPH_PRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_APPS_PERIPH_PRESENT;
#[doc = "`read()` method returns [mem_apps_periph_present::R](mem_apps_periph_present::R) reader structure"]
impl crate::Readable for MEM_APPS_PERIPH_PRESENT {}
#[doc = "`write(|w| ..)` method takes [mem_apps_periph_present::W](mem_apps_periph_present::W) writer structure"]
impl crate::Writable for MEM_APPS_PERIPH_PRESENT {}
#[doc = "MEM_APPS_PERIPH_PRESENT"]
pub mod mem_apps_periph_present;
#[doc = "MEM_NWP_PERIPH_PRESENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_nwp_periph_present](mem_nwp_periph_present) module"]
pub type MEM_NWP_PERIPH_PRESENT = crate::Reg<u32, _MEM_NWP_PERIPH_PRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_NWP_PERIPH_PRESENT;
#[doc = "`read()` method returns [mem_nwp_periph_present::R](mem_nwp_periph_present::R) reader structure"]
impl crate::Readable for MEM_NWP_PERIPH_PRESENT {}
#[doc = "`write(|w| ..)` method takes [mem_nwp_periph_present::W](mem_nwp_periph_present::W) writer structure"]
impl crate::Writable for MEM_NWP_PERIPH_PRESENT {}
#[doc = "MEM_NWP_PERIPH_PRESENT"]
pub mod mem_nwp_periph_present;
#[doc = "MEM_SHARED_PERIPH_PRESENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_shared_periph_present](mem_shared_periph_present) module"]
pub type MEM_SHARED_PERIPH_PRESENT = crate::Reg<u32, _MEM_SHARED_PERIPH_PRESENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM_SHARED_PERIPH_PRESENT;
#[doc = "`read()` method returns [mem_shared_periph_present::R](mem_shared_periph_present::R) reader structure"]
impl crate::Readable for MEM_SHARED_PERIPH_PRESENT {}
#[doc = "`write(|w| ..)` method takes [mem_shared_periph_present::W](mem_shared_periph_present::W) writer structure"]
impl crate::Writable for MEM_SHARED_PERIPH_PRESENT {}
#[doc = "MEM_SHARED_PERIPH_PRESENT"]
pub mod mem_shared_periph_present;
#[doc = "NWP_PWR_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_pwr_state](nwp_pwr_state) module"]
pub type NWP_PWR_STATE = crate::Reg<u32, _NWP_PWR_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_PWR_STATE;
#[doc = "`read()` method returns [nwp_pwr_state::R](nwp_pwr_state::R) reader structure"]
impl crate::Readable for NWP_PWR_STATE {}
#[doc = "`write(|w| ..)` method takes [nwp_pwr_state::W](nwp_pwr_state::W) writer structure"]
impl crate::Writable for NWP_PWR_STATE {}
#[doc = "NWP_PWR_STATE"]
pub mod nwp_pwr_state;
#[doc = "APPS_PWR_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_pwr_state](apps_pwr_state) module"]
pub type APPS_PWR_STATE = crate::Reg<u32, _APPS_PWR_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_PWR_STATE;
#[doc = "`read()` method returns [apps_pwr_state::R](apps_pwr_state::R) reader structure"]
impl crate::Readable for APPS_PWR_STATE {}
#[doc = "`write(|w| ..)` method takes [apps_pwr_state::W](apps_pwr_state::W) writer structure"]
impl crate::Writable for APPS_PWR_STATE {}
#[doc = "APPS_PWR_STATE"]
pub mod apps_pwr_state;
#[doc = "MCU_PWR_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_pwr_state](mcu_pwr_state) module"]
pub type MCU_PWR_STATE = crate::Reg<u32, _MCU_PWR_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_PWR_STATE;
#[doc = "`read()` method returns [mcu_pwr_state::R](mcu_pwr_state::R) reader structure"]
impl crate::Readable for MCU_PWR_STATE {}
#[doc = "`write(|w| ..)` method takes [mcu_pwr_state::W](mcu_pwr_state::W) writer structure"]
impl crate::Writable for MCU_PWR_STATE {}
#[doc = "MCU_PWR_STATE"]
pub mod mcu_pwr_state;
#[doc = "WTOP_PM_PS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtop_pm_ps](wtop_pm_ps) module"]
pub type WTOP_PM_PS = crate::Reg<u32, _WTOP_PM_PS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTOP_PM_PS;
#[doc = "`read()` method returns [wtop_pm_ps::R](wtop_pm_ps::R) reader structure"]
impl crate::Readable for WTOP_PM_PS {}
#[doc = "`write(|w| ..)` method takes [wtop_pm_ps::W](wtop_pm_ps::W) writer structure"]
impl crate::Writable for WTOP_PM_PS {}
#[doc = "WTOP_PM_PS"]
pub mod wtop_pm_ps;
#[doc = "WTOP_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtop_pd_resetz_override_reg](wtop_pd_resetz_override_reg) module"]
pub type WTOP_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _WTOP_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTOP_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [wtop_pd_resetz_override_reg::R](wtop_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for WTOP_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [wtop_pd_resetz_override_reg::W](wtop_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for WTOP_PD_RESETZ_OVERRIDE_REG {}
#[doc = "WTOP_PD_RESETZ_OVERRIDE_REG"]
pub mod wtop_pd_resetz_override_reg;
#[doc = "WELP_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [welp_pd_resetz_override_reg](welp_pd_resetz_override_reg) module"]
pub type WELP_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _WELP_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WELP_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [welp_pd_resetz_override_reg::R](welp_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for WELP_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [welp_pd_resetz_override_reg::W](welp_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for WELP_PD_RESETZ_OVERRIDE_REG {}
#[doc = "WELP_PD_RESETZ_OVERRIDE_REG"]
pub mod welp_pd_resetz_override_reg;
#[doc = "WL_SDIO_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_sdio_pd_resetz_override_reg](wl_sdio_pd_resetz_override_reg) module"]
pub type WL_SDIO_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _WL_SDIO_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_SDIO_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [wl_sdio_pd_resetz_override_reg::R](wl_sdio_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for WL_SDIO_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [wl_sdio_pd_resetz_override_reg::W](wl_sdio_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for WL_SDIO_PD_RESETZ_OVERRIDE_REG {}
#[doc = "WL_SDIO_PD_RESETZ_OVERRIDE_REG"]
pub mod wl_sdio_pd_resetz_override_reg;
#[doc = "SSDIO_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdio_pd_resetz_override_reg](ssdio_pd_resetz_override_reg) module"]
pub type SSDIO_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _SSDIO_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDIO_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [ssdio_pd_resetz_override_reg::R](ssdio_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for SSDIO_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [ssdio_pd_resetz_override_reg::W](ssdio_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for SSDIO_PD_RESETZ_OVERRIDE_REG {}
#[doc = "SSDIO_PD_RESETZ_OVERRIDE_REG"]
pub mod ssdio_pd_resetz_override_reg;
#[doc = "MCSPI_N1_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcspi_n1_pd_resetz_override_reg](mcspi_n1_pd_resetz_override_reg) module"]
pub type MCSPI_N1_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _MCSPI_N1_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCSPI_N1_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [mcspi_n1_pd_resetz_override_reg::R](mcspi_n1_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for MCSPI_N1_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [mcspi_n1_pd_resetz_override_reg::W](mcspi_n1_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for MCSPI_N1_PD_RESETZ_OVERRIDE_REG {}
#[doc = "MCSPI_N1_PD_RESETZ_OVERRIDE_REG"]
pub mod mcspi_n1_pd_resetz_override_reg;
#[doc = "TESTCTRL_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testctrl_pd_resetz_override_reg](testctrl_pd_resetz_override_reg) module"]
pub type TESTCTRL_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _TESTCTRL_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESTCTRL_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [testctrl_pd_resetz_override_reg::R](testctrl_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for TESTCTRL_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [testctrl_pd_resetz_override_reg::W](testctrl_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for TESTCTRL_PD_RESETZ_OVERRIDE_REG {}
#[doc = "TESTCTRL_PD_RESETZ_OVERRIDE_REG"]
pub mod testctrl_pd_resetz_override_reg;
#[doc = "MCU_PD_RESETZ_OVERRIDE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_pd_resetz_override_reg](mcu_pd_resetz_override_reg) module"]
pub type MCU_PD_RESETZ_OVERRIDE_REG = crate::Reg<u32, _MCU_PD_RESETZ_OVERRIDE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_PD_RESETZ_OVERRIDE_REG;
#[doc = "`read()` method returns [mcu_pd_resetz_override_reg::R](mcu_pd_resetz_override_reg::R) reader structure"]
impl crate::Readable for MCU_PD_RESETZ_OVERRIDE_REG {}
#[doc = "`write(|w| ..)` method takes [mcu_pd_resetz_override_reg::W](mcu_pd_resetz_override_reg::W) writer structure"]
impl crate::Writable for MCU_PD_RESETZ_OVERRIDE_REG {}
#[doc = "MCU_PD_RESETZ_OVERRIDE_REG"]
pub mod mcu_pd_resetz_override_reg;
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
#[doc = "EFUSE_READ_REG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg2](efuse_read_reg2) module"]
pub type EFUSE_READ_REG2 = crate::Reg<u32, _EFUSE_READ_REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG2;
#[doc = "`read()` method returns [efuse_read_reg2::R](efuse_read_reg2::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG2 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg2::W](efuse_read_reg2::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG2 {}
#[doc = "EFUSE_READ_REG2"]
pub mod efuse_read_reg2;
#[doc = "EFUSE_READ_REG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg3](efuse_read_reg3) module"]
pub type EFUSE_READ_REG3 = crate::Reg<u32, _EFUSE_READ_REG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG3;
#[doc = "`read()` method returns [efuse_read_reg3::R](efuse_read_reg3::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG3 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg3::W](efuse_read_reg3::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG3 {}
#[doc = "EFUSE_READ_REG3"]
pub mod efuse_read_reg3;
#[doc = "WTOP_MEM_RET_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtop_mem_ret_cfg](wtop_mem_ret_cfg) module"]
pub type WTOP_MEM_RET_CFG = crate::Reg<u32, _WTOP_MEM_RET_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTOP_MEM_RET_CFG;
#[doc = "`read()` method returns [wtop_mem_ret_cfg::R](wtop_mem_ret_cfg::R) reader structure"]
impl crate::Readable for WTOP_MEM_RET_CFG {}
#[doc = "`write(|w| ..)` method takes [wtop_mem_ret_cfg::W](wtop_mem_ret_cfg::W) writer structure"]
impl crate::Writable for WTOP_MEM_RET_CFG {}
#[doc = "WTOP_MEM_RET_CFG"]
pub mod wtop_mem_ret_cfg;
#[doc = "COEX_CLK_SWALLOW_CFG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_clk_swallow_cfg0](coex_clk_swallow_cfg0) module"]
pub type COEX_CLK_SWALLOW_CFG0 = crate::Reg<u32, _COEX_CLK_SWALLOW_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COEX_CLK_SWALLOW_CFG0;
#[doc = "`read()` method returns [coex_clk_swallow_cfg0::R](coex_clk_swallow_cfg0::R) reader structure"]
impl crate::Readable for COEX_CLK_SWALLOW_CFG0 {}
#[doc = "`write(|w| ..)` method takes [coex_clk_swallow_cfg0::W](coex_clk_swallow_cfg0::W) writer structure"]
impl crate::Writable for COEX_CLK_SWALLOW_CFG0 {}
#[doc = "COEX_CLK_SWALLOW_CFG0"]
pub mod coex_clk_swallow_cfg0;
#[doc = "COEX_CLK_SWALLOW_CFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_clk_swallow_cfg1](coex_clk_swallow_cfg1) module"]
pub type COEX_CLK_SWALLOW_CFG1 = crate::Reg<u32, _COEX_CLK_SWALLOW_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COEX_CLK_SWALLOW_CFG1;
#[doc = "`read()` method returns [coex_clk_swallow_cfg1::R](coex_clk_swallow_cfg1::R) reader structure"]
impl crate::Readable for COEX_CLK_SWALLOW_CFG1 {}
#[doc = "`write(|w| ..)` method takes [coex_clk_swallow_cfg1::W](coex_clk_swallow_cfg1::W) writer structure"]
impl crate::Writable for COEX_CLK_SWALLOW_CFG1 {}
#[doc = "COEX_CLK_SWALLOW_CFG1"]
pub mod coex_clk_swallow_cfg1;
#[doc = "COEX_CLK_SWALLOW_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_clk_swallow_cfg2](coex_clk_swallow_cfg2) module"]
pub type COEX_CLK_SWALLOW_CFG2 = crate::Reg<u32, _COEX_CLK_SWALLOW_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COEX_CLK_SWALLOW_CFG2;
#[doc = "`read()` method returns [coex_clk_swallow_cfg2::R](coex_clk_swallow_cfg2::R) reader structure"]
impl crate::Readable for COEX_CLK_SWALLOW_CFG2 {}
#[doc = "`write(|w| ..)` method takes [coex_clk_swallow_cfg2::W](coex_clk_swallow_cfg2::W) writer structure"]
impl crate::Writable for COEX_CLK_SWALLOW_CFG2 {}
#[doc = "COEX_CLK_SWALLOW_CFG2"]
pub mod coex_clk_swallow_cfg2;
#[doc = "COEX_CLK_SWALLOW_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coex_clk_swallow_enable](coex_clk_swallow_enable) module"]
pub type COEX_CLK_SWALLOW_ENABLE = crate::Reg<u32, _COEX_CLK_SWALLOW_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COEX_CLK_SWALLOW_ENABLE;
#[doc = "`read()` method returns [coex_clk_swallow_enable::R](coex_clk_swallow_enable::R) reader structure"]
impl crate::Readable for COEX_CLK_SWALLOW_ENABLE {}
#[doc = "`write(|w| ..)` method takes [coex_clk_swallow_enable::W](coex_clk_swallow_enable::W) writer structure"]
impl crate::Writable for COEX_CLK_SWALLOW_ENABLE {}
#[doc = "COEX_CLK_SWALLOW_ENABLE"]
pub mod coex_clk_swallow_enable;
#[doc = "DCDC_CLK_GEN_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_clk_gen_config](dcdc_clk_gen_config) module"]
pub type DCDC_CLK_GEN_CONFIG = crate::Reg<u32, _DCDC_CLK_GEN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDC_CLK_GEN_CONFIG;
#[doc = "`read()` method returns [dcdc_clk_gen_config::R](dcdc_clk_gen_config::R) reader structure"]
impl crate::Readable for DCDC_CLK_GEN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [dcdc_clk_gen_config::W](dcdc_clk_gen_config::W) writer structure"]
impl crate::Writable for DCDC_CLK_GEN_CONFIG {}
#[doc = "DCDC_CLK_GEN_CONFIG"]
pub mod dcdc_clk_gen_config;
#[doc = "EFUSE_READ_REG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg4](efuse_read_reg4) module"]
pub type EFUSE_READ_REG4 = crate::Reg<u32, _EFUSE_READ_REG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG4;
#[doc = "`read()` method returns [efuse_read_reg4::R](efuse_read_reg4::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG4 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg4::W](efuse_read_reg4::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG4 {}
#[doc = "EFUSE_READ_REG4"]
pub mod efuse_read_reg4;
#[doc = "EFUSE_READ_REG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg5](efuse_read_reg5) module"]
pub type EFUSE_READ_REG5 = crate::Reg<u32, _EFUSE_READ_REG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG5;
#[doc = "`read()` method returns [efuse_read_reg5::R](efuse_read_reg5::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG5 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg5::W](efuse_read_reg5::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG5 {}
#[doc = "EFUSE_READ_REG5"]
pub mod efuse_read_reg5;
#[doc = "EFUSE_READ_REG6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg6](efuse_read_reg6) module"]
pub type EFUSE_READ_REG6 = crate::Reg<u32, _EFUSE_READ_REG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG6;
#[doc = "`read()` method returns [efuse_read_reg6::R](efuse_read_reg6::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG6 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg6::W](efuse_read_reg6::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG6 {}
#[doc = "EFUSE_READ_REG6"]
pub mod efuse_read_reg6;
#[doc = "EFUSE_READ_REG7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg7](efuse_read_reg7) module"]
pub type EFUSE_READ_REG7 = crate::Reg<u32, _EFUSE_READ_REG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG7;
#[doc = "`read()` method returns [efuse_read_reg7::R](efuse_read_reg7::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG7 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg7::W](efuse_read_reg7::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG7 {}
#[doc = "EFUSE_READ_REG7"]
pub mod efuse_read_reg7;
#[doc = "EFUSE_READ_REG8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg8](efuse_read_reg8) module"]
pub type EFUSE_READ_REG8 = crate::Reg<u32, _EFUSE_READ_REG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG8;
#[doc = "`read()` method returns [efuse_read_reg8::R](efuse_read_reg8::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG8 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg8::W](efuse_read_reg8::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG8 {}
#[doc = "EFUSE_READ_REG8"]
pub mod efuse_read_reg8;
#[doc = "EFUSE_READ_REG9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg9](efuse_read_reg9) module"]
pub type EFUSE_READ_REG9 = crate::Reg<u32, _EFUSE_READ_REG9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG9;
#[doc = "`read()` method returns [efuse_read_reg9::R](efuse_read_reg9::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG9 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg9::W](efuse_read_reg9::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG9 {}
#[doc = "EFUSE_READ_REG9"]
pub mod efuse_read_reg9;
#[doc = "EFUSE_READ_REG10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg10](efuse_read_reg10) module"]
pub type EFUSE_READ_REG10 = crate::Reg<u32, _EFUSE_READ_REG10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG10;
#[doc = "`read()` method returns [efuse_read_reg10::R](efuse_read_reg10::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG10 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg10::W](efuse_read_reg10::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG10 {}
#[doc = "EFUSE_READ_REG10"]
pub mod efuse_read_reg10;
#[doc = "EFUSE_READ_REG11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_read_reg11](efuse_read_reg11) module"]
pub type EFUSE_READ_REG11 = crate::Reg<u32, _EFUSE_READ_REG11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_READ_REG11;
#[doc = "`read()` method returns [efuse_read_reg11::R](efuse_read_reg11::R) reader structure"]
impl crate::Readable for EFUSE_READ_REG11 {}
#[doc = "`write(|w| ..)` method takes [efuse_read_reg11::W](efuse_read_reg11::W) writer structure"]
impl crate::Writable for EFUSE_READ_REG11 {}
#[doc = "EFUSE_READ_REG11"]
pub mod efuse_read_reg11;
#[doc = "DIEID_READ_REG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg0](dieid_read_reg0) module"]
pub type DIEID_READ_REG0 = crate::Reg<u32, _DIEID_READ_REG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG0;
#[doc = "`read()` method returns [dieid_read_reg0::R](dieid_read_reg0::R) reader structure"]
impl crate::Readable for DIEID_READ_REG0 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg0::W](dieid_read_reg0::W) writer structure"]
impl crate::Writable for DIEID_READ_REG0 {}
#[doc = "DIEID_READ_REG0"]
pub mod dieid_read_reg0;
#[doc = "DIEID_READ_REG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg1](dieid_read_reg1) module"]
pub type DIEID_READ_REG1 = crate::Reg<u32, _DIEID_READ_REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG1;
#[doc = "`read()` method returns [dieid_read_reg1::R](dieid_read_reg1::R) reader structure"]
impl crate::Readable for DIEID_READ_REG1 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg1::W](dieid_read_reg1::W) writer structure"]
impl crate::Writable for DIEID_READ_REG1 {}
#[doc = "DIEID_READ_REG1"]
pub mod dieid_read_reg1;
#[doc = "DIEID_READ_REG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg2](dieid_read_reg2) module"]
pub type DIEID_READ_REG2 = crate::Reg<u32, _DIEID_READ_REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG2;
#[doc = "`read()` method returns [dieid_read_reg2::R](dieid_read_reg2::R) reader structure"]
impl crate::Readable for DIEID_READ_REG2 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg2::W](dieid_read_reg2::W) writer structure"]
impl crate::Writable for DIEID_READ_REG2 {}
#[doc = "DIEID_READ_REG2"]
pub mod dieid_read_reg2;
#[doc = "DIEID_READ_REG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg3](dieid_read_reg3) module"]
pub type DIEID_READ_REG3 = crate::Reg<u32, _DIEID_READ_REG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG3;
#[doc = "`read()` method returns [dieid_read_reg3::R](dieid_read_reg3::R) reader structure"]
impl crate::Readable for DIEID_READ_REG3 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg3::W](dieid_read_reg3::W) writer structure"]
impl crate::Writable for DIEID_READ_REG3 {}
#[doc = "DIEID_READ_REG3"]
pub mod dieid_read_reg3;
#[doc = "DIEID_READ_REG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid_read_reg4](dieid_read_reg4) module"]
pub type DIEID_READ_REG4 = crate::Reg<u32, _DIEID_READ_REG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID_READ_REG4;
#[doc = "`read()` method returns [dieid_read_reg4::R](dieid_read_reg4::R) reader structure"]
impl crate::Readable for DIEID_READ_REG4 {}
#[doc = "`write(|w| ..)` method takes [dieid_read_reg4::W](dieid_read_reg4::W) writer structure"]
impl crate::Writable for DIEID_READ_REG4 {}
#[doc = "DIEID_READ_REG4"]
pub mod dieid_read_reg4;
#[doc = "APPS_SS_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apps_ss_overrides](apps_ss_overrides) module"]
pub type APPS_SS_OVERRIDES = crate::Reg<u32, _APPS_SS_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPS_SS_OVERRIDES;
#[doc = "`read()` method returns [apps_ss_overrides::R](apps_ss_overrides::R) reader structure"]
impl crate::Readable for APPS_SS_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [apps_ss_overrides::W](apps_ss_overrides::W) writer structure"]
impl crate::Writable for APPS_SS_OVERRIDES {}
#[doc = "APPS_SS_OVERRIDES"]
pub mod apps_ss_overrides;
#[doc = "NWP_SS_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwp_ss_overrides](nwp_ss_overrides) module"]
pub type NWP_SS_OVERRIDES = crate::Reg<u32, _NWP_SS_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWP_SS_OVERRIDES;
#[doc = "`read()` method returns [nwp_ss_overrides::R](nwp_ss_overrides::R) reader structure"]
impl crate::Readable for NWP_SS_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [nwp_ss_overrides::W](nwp_ss_overrides::W) writer structure"]
impl crate::Writable for NWP_SS_OVERRIDES {}
#[doc = "NWP_SS_OVERRIDES"]
pub mod nwp_ss_overrides;
#[doc = "SHARED_SS_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shared_ss_overrides](shared_ss_overrides) module"]
pub type SHARED_SS_OVERRIDES = crate::Reg<u32, _SHARED_SS_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHARED_SS_OVERRIDES;
#[doc = "`read()` method returns [shared_ss_overrides::R](shared_ss_overrides::R) reader structure"]
impl crate::Readable for SHARED_SS_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [shared_ss_overrides::W](shared_ss_overrides::W) writer structure"]
impl crate::Writable for SHARED_SS_OVERRIDES {}
#[doc = "SHARED_SS_OVERRIDES"]
pub mod shared_ss_overrides;
#[doc = "IDMEM_CORE_RST_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmem_core_rst_overrides](idmem_core_rst_overrides) module"]
pub type IDMEM_CORE_RST_OVERRIDES = crate::Reg<u32, _IDMEM_CORE_RST_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMEM_CORE_RST_OVERRIDES;
#[doc = "`read()` method returns [idmem_core_rst_overrides::R](idmem_core_rst_overrides::R) reader structure"]
impl crate::Readable for IDMEM_CORE_RST_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [idmem_core_rst_overrides::W](idmem_core_rst_overrides::W) writer structure"]
impl crate::Writable for IDMEM_CORE_RST_OVERRIDES {}
#[doc = "IDMEM_CORE_RST_OVERRIDES"]
pub mod idmem_core_rst_overrides;
#[doc = "TOP_DIE_FSM_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_die_fsm_overrides](top_die_fsm_overrides) module"]
pub type TOP_DIE_FSM_OVERRIDES = crate::Reg<u32, _TOP_DIE_FSM_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP_DIE_FSM_OVERRIDES;
#[doc = "`read()` method returns [top_die_fsm_overrides::R](top_die_fsm_overrides::R) reader structure"]
impl crate::Readable for TOP_DIE_FSM_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [top_die_fsm_overrides::W](top_die_fsm_overrides::W) writer structure"]
impl crate::Writable for TOP_DIE_FSM_OVERRIDES {}
#[doc = "TOP_DIE_FSM_OVERRIDES"]
pub mod top_die_fsm_overrides;
#[doc = "MCU_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_pscon_overrides](mcu_pscon_overrides) module"]
pub type MCU_PSCON_OVERRIDES = crate::Reg<u32, _MCU_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCU_PSCON_OVERRIDES;
#[doc = "`read()` method returns [mcu_pscon_overrides::R](mcu_pscon_overrides::R) reader structure"]
impl crate::Readable for MCU_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mcu_pscon_overrides::W](mcu_pscon_overrides::W) writer structure"]
impl crate::Writable for MCU_PSCON_OVERRIDES {}
#[doc = "MCU_PSCON_OVERRIDES"]
pub mod mcu_pscon_overrides;
#[doc = "WTOP_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtop_pscon_overrides](wtop_pscon_overrides) module"]
pub type WTOP_PSCON_OVERRIDES = crate::Reg<u32, _WTOP_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTOP_PSCON_OVERRIDES;
#[doc = "`read()` method returns [wtop_pscon_overrides::R](wtop_pscon_overrides::R) reader structure"]
impl crate::Readable for WTOP_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [wtop_pscon_overrides::W](wtop_pscon_overrides::W) writer structure"]
impl crate::Writable for WTOP_PSCON_OVERRIDES {}
#[doc = "WTOP_PSCON_OVERRIDES"]
pub mod wtop_pscon_overrides;
#[doc = "WELP_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [welp_pscon_overrides](welp_pscon_overrides) module"]
pub type WELP_PSCON_OVERRIDES = crate::Reg<u32, _WELP_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WELP_PSCON_OVERRIDES;
#[doc = "`read()` method returns [welp_pscon_overrides::R](welp_pscon_overrides::R) reader structure"]
impl crate::Readable for WELP_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [welp_pscon_overrides::W](welp_pscon_overrides::W) writer structure"]
impl crate::Writable for WELP_PSCON_OVERRIDES {}
#[doc = "WELP_PSCON_OVERRIDES"]
pub mod welp_pscon_overrides;
#[doc = "WL_SDIO_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_sdio_pscon_overrides](wl_sdio_pscon_overrides) module"]
pub type WL_SDIO_PSCON_OVERRIDES = crate::Reg<u32, _WL_SDIO_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_SDIO_PSCON_OVERRIDES;
#[doc = "`read()` method returns [wl_sdio_pscon_overrides::R](wl_sdio_pscon_overrides::R) reader structure"]
impl crate::Readable for WL_SDIO_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [wl_sdio_pscon_overrides::W](wl_sdio_pscon_overrides::W) writer structure"]
impl crate::Writable for WL_SDIO_PSCON_OVERRIDES {}
#[doc = "WL_SDIO_PSCON_OVERRIDES"]
pub mod wl_sdio_pscon_overrides;
#[doc = "MCSPI_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcspi_pscon_overrides](mcspi_pscon_overrides) module"]
pub type MCSPI_PSCON_OVERRIDES = crate::Reg<u32, _MCSPI_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCSPI_PSCON_OVERRIDES;
#[doc = "`read()` method returns [mcspi_pscon_overrides::R](mcspi_pscon_overrides::R) reader structure"]
impl crate::Readable for MCSPI_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [mcspi_pscon_overrides::W](mcspi_pscon_overrides::W) writer structure"]
impl crate::Writable for MCSPI_PSCON_OVERRIDES {}
#[doc = "MCSPI_PSCON_OVERRIDES"]
pub mod mcspi_pscon_overrides;
#[doc = "SSDIO_PSCON_OVERRIDES\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdio_pscon_overrides](ssdio_pscon_overrides) module"]
pub type SSDIO_PSCON_OVERRIDES = crate::Reg<u32, _SSDIO_PSCON_OVERRIDES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSDIO_PSCON_OVERRIDES;
#[doc = "`read()` method returns [ssdio_pscon_overrides::R](ssdio_pscon_overrides::R) reader structure"]
impl crate::Readable for SSDIO_PSCON_OVERRIDES {}
#[doc = "`write(|w| ..)` method takes [ssdio_pscon_overrides::W](ssdio_pscon_overrides::W) writer structure"]
impl crate::Writable for SSDIO_PSCON_OVERRIDES {}
#[doc = "SSDIO_PSCON_OVERRIDES"]
pub mod ssdio_pscon_overrides;
