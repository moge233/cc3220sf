#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Can be written only by Base Processor. Writing to this register will reset the stack processor reset will be de-asserted upon clearing this register."]
    pub stk_up_reset: STK_UP_RESET,
    #[doc = "0x04 - This register defines who among base processor and stack processor have highest priority for Sram Access. Can be written only by Base Processor."]
    pub sr_master_priority: SR_MASTER_PRIORITY,
    #[doc = "0x08 - In Spinlock mode this Register defines who among base processor and stack processor have access to Sram Bank2 right now. In Handshake mode this Register defines who among base processor and stack processor have access to Sram Bank2 and Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor."]
    pub stk_sr_acc_ctl_bk2: STK_SR_ACC_CTL_BK2,
    #[doc = "0x0c - In Spinlock mode whenever Base processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted."]
    pub base_up_acc_req_bk2: BASE_UP_ACC_REQ_BK2,
    #[doc = "0x10 - In Spinlock mode Whenever Stack processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted."]
    pub stk_up_acc_req_bk2: STK_UP_ACC_REQ_BK2,
    #[doc = "0x14 - Register defines who among base processor and stack processor have access to Sram Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor."]
    pub stk_sr_acc_ctl_bk3: STK_SR_ACC_CTL_BK3,
    #[doc = "0x18 - In Spinlock mode whenever Base processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted."]
    pub base_up_acc_req_bk3: BASE_UP_ACC_REQ_BK3,
    #[doc = "0x1c - In Spinlock mode Whenever Stack processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted."]
    pub stk_up_acc_req_bk3: STK_UP_ACC_REQ_BK3,
    #[doc = "0x20 - Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz."]
    pub rdsm_cfg_cpu: RDSM_CFG_CPU,
    #[doc = "0x24 - Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz."]
    pub rdsm_cfg_ee: RDSM_CFG_EE,
    #[doc = "0x28 - Reading this register Base procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Base processor and can be cleared only by BASE processor."]
    pub base_up_irq_log: BASE_UP_IRQ_LOG,
    #[doc = "0x2c - Reading this register Stack procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Stack processor and can be cleared only by Stack processor."]
    pub stk_up_irq_log: STK_UP_IRQ_LOG,
    #[doc = "0x30 - Can be written only by base processor. Controls the enable pin of the cgcs for the clocks going to CM3 dft ctrl block and Sram."]
    pub stk_clk_en: STK_CLK_EN,
    #[doc = "0x34 - Can be written only by the base processor. Decides the ram sharing mode :: handshake or Spinlock mode."]
    pub spin_lock_mode: SPIN_LOCK_MODE,
    #[doc = "0x38 - Stores the last bus fault address."]
    pub bus_fault_addr: BUS_FAULT_ADDR,
    #[doc = "0x3c - write only registers on read returns 0.W Write 1 to clear the bust fault to store the new bus fault address"]
    pub bus_fault_clr: BUS_FAULT_CLR,
    #[doc = "0x40 - Reset cause value captured from the ICR_CLKRST block."]
    pub reset_cause: RESET_CAUSE,
    #[doc = "0x44 - Watchdog timer event value captured from the ICR_CLKRST block"]
    pub wdog_timer_event: WDOG_TIMER_EVENT,
    #[doc = "0x48 - To send Dma Request to bottom die."]
    pub dma_req: DMA_REQ,
    #[doc = "0x4c - Address offset within SRAM to which CM3 should jump after reset."]
    pub sram_jump_offset_addr: SRAM_JUMP_OFFSET_ADDR,
    #[doc = "0x50 - These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored."]
    pub sw_reg1: SW_REG1,
    #[doc = "0x54 - These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored."]
    pub sw_reg2: SW_REG2,
    #[doc = "0x58 - By posting the request Flash can be put into low-power mode (Sleep) without powering down the Flash. Earlier (in Garnet) this was fully h/w controlled and the control for this was coming from SysCtl while entering into Cortex Deep-sleep mode. But for our device the D2D i/f doesnt support this. The Firmware has to program the register in the top-die for entering into this mode and wait for an interrupt."]
    pub fmc_sleep_ctl: FMC_SLEEP_CTL,
    #[doc = "0x5c - Miscellanious control register."]
    pub misc_ctl: MISC_CTL,
    _reserved24: [u8; 156usize],
    #[doc = "0xfc - DFT control and status bits"]
    pub sw_dft_ctl: SW_DFT_CTL,
    #[doc = "0x100 - Mainly for For controlling the pads OEN pins. There are total 60 pads and hence 60 control registe i.e n value varies from 0 to 59. Here is the mapping for the pad_ctl register number and the functionality : 0 D2DPAD_DMAREQ1 1 D2DPAD_DMAREQ0 2 D2DPAD_INT2BASE 3 D2DPAD_PIOSC 4 D2DPAD_RST_N 5 D2DPAD_POR_RST_N 6 D2DPAD_HCLK 7 D2DPAD_JTAG_TDO 8 D2DPAD_JTAG_TCK 9 D2DPAD_JTAG_TMS 10 D2DPAD_JTAG_TDI 11-27 D2DPAD_FROMSTACK\\[D2D_FROMSTACK_SIZE -1:0\\]
28-56 D2DPAD_TOSTACK \\[D2D_TOSTACK_SIZE -1:0\\]
57-59 D2DPAD_SPARE \\[D2D_SPARE_PAD_SIZE -1:0\\]
0:00 ****************************************************************************"]
    pub padn_ctl_0: PADN_CTL_0,
}
#[doc = "Can be written only by Base Processor. Writing to this register will reset the stack processor reset will be de-asserted upon clearing this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_up_reset](stk_up_reset) module"]
pub type STK_UP_RESET = crate::Reg<u32, _STK_UP_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_UP_RESET;
#[doc = "`read()` method returns [stk_up_reset::R](stk_up_reset::R) reader structure"]
impl crate::Readable for STK_UP_RESET {}
#[doc = "`write(|w| ..)` method takes [stk_up_reset::W](stk_up_reset::W) writer structure"]
impl crate::Writable for STK_UP_RESET {}
#[doc = "Can be written only by Base Processor. Writing to this register will reset the stack processor reset will be de-asserted upon clearing this register."]
pub mod stk_up_reset;
#[doc = "This register defines who among base processor and stack processor have highest priority for Sram Access. Can be written only by Base Processor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr_master_priority](sr_master_priority) module"]
pub type SR_MASTER_PRIORITY = crate::Reg<u32, _SR_MASTER_PRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR_MASTER_PRIORITY;
#[doc = "`read()` method returns [sr_master_priority::R](sr_master_priority::R) reader structure"]
impl crate::Readable for SR_MASTER_PRIORITY {}
#[doc = "`write(|w| ..)` method takes [sr_master_priority::W](sr_master_priority::W) writer structure"]
impl crate::Writable for SR_MASTER_PRIORITY {}
#[doc = "This register defines who among base processor and stack processor have highest priority for Sram Access. Can be written only by Base Processor."]
pub mod sr_master_priority;
#[doc = "In Spinlock mode this Register defines who among base processor and stack processor have access to Sram Bank2 right now. In Handshake mode this Register defines who among base processor and stack processor have access to Sram Bank2 and Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_sr_acc_ctl_bk2](stk_sr_acc_ctl_bk2) module"]
pub type STK_SR_ACC_CTL_BK2 = crate::Reg<u32, _STK_SR_ACC_CTL_BK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_SR_ACC_CTL_BK2;
#[doc = "`read()` method returns [stk_sr_acc_ctl_bk2::R](stk_sr_acc_ctl_bk2::R) reader structure"]
impl crate::Readable for STK_SR_ACC_CTL_BK2 {}
#[doc = "`write(|w| ..)` method takes [stk_sr_acc_ctl_bk2::W](stk_sr_acc_ctl_bk2::W) writer structure"]
impl crate::Writable for STK_SR_ACC_CTL_BK2 {}
#[doc = "In Spinlock mode this Register defines who among base processor and stack processor have access to Sram Bank2 right now. In Handshake mode this Register defines who among base processor and stack processor have access to Sram Bank2 and Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor."]
pub mod stk_sr_acc_ctl_bk2;
#[doc = "In Spinlock mode whenever Base processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_up_acc_req_bk2](base_up_acc_req_bk2) module"]
pub type BASE_UP_ACC_REQ_BK2 = crate::Reg<u32, _BASE_UP_ACC_REQ_BK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_UP_ACC_REQ_BK2;
#[doc = "`read()` method returns [base_up_acc_req_bk2::R](base_up_acc_req_bk2::R) reader structure"]
impl crate::Readable for BASE_UP_ACC_REQ_BK2 {}
#[doc = "`write(|w| ..)` method takes [base_up_acc_req_bk2::W](base_up_acc_req_bk2::W) writer structure"]
impl crate::Writable for BASE_UP_ACC_REQ_BK2 {}
#[doc = "In Spinlock mode whenever Base processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted."]
pub mod base_up_acc_req_bk2;
#[doc = "In Spinlock mode Whenever Stack processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_up_acc_req_bk2](stk_up_acc_req_bk2) module"]
pub type STK_UP_ACC_REQ_BK2 = crate::Reg<u32, _STK_UP_ACC_REQ_BK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_UP_ACC_REQ_BK2;
#[doc = "`read()` method returns [stk_up_acc_req_bk2::R](stk_up_acc_req_bk2::R) reader structure"]
impl crate::Readable for STK_UP_ACC_REQ_BK2 {}
#[doc = "`write(|w| ..)` method takes [stk_up_acc_req_bk2::W](stk_up_acc_req_bk2::W) writer structure"]
impl crate::Writable for STK_UP_ACC_REQ_BK2 {}
#[doc = "In Spinlock mode Whenever Stack processor wants the access to Sram Bank2 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted."]
pub mod stk_up_acc_req_bk2;
#[doc = "Register defines who among base processor and stack processor have access to Sram Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_sr_acc_ctl_bk3](stk_sr_acc_ctl_bk3) module"]
pub type STK_SR_ACC_CTL_BK3 = crate::Reg<u32, _STK_SR_ACC_CTL_BK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_SR_ACC_CTL_BK3;
#[doc = "`read()` method returns [stk_sr_acc_ctl_bk3::R](stk_sr_acc_ctl_bk3::R) reader structure"]
impl crate::Readable for STK_SR_ACC_CTL_BK3 {}
#[doc = "`write(|w| ..)` method takes [stk_sr_acc_ctl_bk3::W](stk_sr_acc_ctl_bk3::W) writer structure"]
impl crate::Writable for STK_SR_ACC_CTL_BK3 {}
#[doc = "Register defines who among base processor and stack processor have access to Sram Bank3 right now. Its Clear only register and is set by hardware. Lower bit can be cleared only by Base Processor and Upper bit Cleared only by the Stack processor."]
pub mod stk_sr_acc_ctl_bk3;
#[doc = "In Spinlock mode whenever Base processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_up_acc_req_bk3](base_up_acc_req_bk3) module"]
pub type BASE_UP_ACC_REQ_BK3 = crate::Reg<u32, _BASE_UP_ACC_REQ_BK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_UP_ACC_REQ_BK3;
#[doc = "`read()` method returns [base_up_acc_req_bk3::R](base_up_acc_req_bk3::R) reader structure"]
impl crate::Readable for BASE_UP_ACC_REQ_BK3 {}
#[doc = "`write(|w| ..)` method takes [base_up_acc_req_bk3::W](base_up_acc_req_bk3::W) writer structure"]
impl crate::Writable for BASE_UP_ACC_REQ_BK3 {}
#[doc = "In Spinlock mode whenever Base processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by Stack processor. Its a set only bit and is cleared by HW when the request is granted."]
pub mod base_up_acc_req_bk3;
#[doc = "In Spinlock mode Whenever Stack processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_up_acc_req_bk3](stk_up_acc_req_bk3) module"]
pub type STK_UP_ACC_REQ_BK3 = crate::Reg<u32, _STK_UP_ACC_REQ_BK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_UP_ACC_REQ_BK3;
#[doc = "`read()` method returns [stk_up_acc_req_bk3::R](stk_up_acc_req_bk3::R) reader structure"]
impl crate::Readable for STK_UP_ACC_REQ_BK3 {}
#[doc = "`write(|w| ..)` method takes [stk_up_acc_req_bk3::W](stk_up_acc_req_bk3::W) writer structure"]
impl crate::Writable for STK_UP_ACC_REQ_BK3 {}
#[doc = "In Spinlock mode Whenever Stack processor wants the access to Sram Bank3 it should request for it by writing into this register. It'll get interrupt whenever it is granted. In Handshake mode this bit will be set by the Base processor. Its a set only bit and is cleared by HW when the request is granted."]
pub mod stk_up_acc_req_bk3;
#[doc = "Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdsm_cfg_cpu](rdsm_cfg_cpu) module"]
pub type RDSM_CFG_CPU = crate::Reg<u32, _RDSM_CFG_CPU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDSM_CFG_CPU;
#[doc = "`read()` method returns [rdsm_cfg_cpu::R](rdsm_cfg_cpu::R) reader structure"]
impl crate::Readable for RDSM_CFG_CPU {}
#[doc = "`write(|w| ..)` method takes [rdsm_cfg_cpu::W](rdsm_cfg_cpu::W) writer structure"]
impl crate::Writable for RDSM_CFG_CPU {}
#[doc = "Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz."]
pub mod rdsm_cfg_cpu;
#[doc = "Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdsm_cfg_ee](rdsm_cfg_ee) module"]
pub type RDSM_CFG_EE = crate::Reg<u32, _RDSM_CFG_EE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDSM_CFG_EE;
#[doc = "`read()` method returns [rdsm_cfg_ee::R](rdsm_cfg_ee::R) reader structure"]
impl crate::Readable for RDSM_CFG_EE {}
#[doc = "`write(|w| ..)` method takes [rdsm_cfg_ee::W](rdsm_cfg_ee::W) writer structure"]
impl crate::Writable for RDSM_CFG_EE {}
#[doc = "Read State Machine timing configuration register. Generally Bit 4 and 3 will be identical. For stacked die always 43 are 0 and 6:5 == 1 for 120Mhz."]
pub mod rdsm_cfg_ee;
#[doc = "Reading this register Base procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Base processor and can be cleared only by BASE processor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_up_irq_log](base_up_irq_log) module"]
pub type BASE_UP_IRQ_LOG = crate::Reg<u32, _BASE_UP_IRQ_LOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_UP_IRQ_LOG;
#[doc = "`read()` method returns [base_up_irq_log::R](base_up_irq_log::R) reader structure"]
impl crate::Readable for BASE_UP_IRQ_LOG {}
#[doc = "`write(|w| ..)` method takes [base_up_irq_log::W](base_up_irq_log::W) writer structure"]
impl crate::Writable for BASE_UP_IRQ_LOG {}
#[doc = "Reading this register Base procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Base processor and can be cleared only by BASE processor."]
pub mod base_up_irq_log;
#[doc = "Reading this register Stack procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Stack processor and can be cleared only by Stack processor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_up_irq_log](stk_up_irq_log) module"]
pub type STK_UP_IRQ_LOG = crate::Reg<u32, _STK_UP_IRQ_LOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_UP_IRQ_LOG;
#[doc = "`read()` method returns [stk_up_irq_log::R](stk_up_irq_log::R) reader structure"]
impl crate::Readable for STK_UP_IRQ_LOG {}
#[doc = "`write(|w| ..)` method takes [stk_up_irq_log::W](stk_up_irq_log::W) writer structure"]
impl crate::Writable for STK_UP_IRQ_LOG {}
#[doc = "Reading this register Stack procesor will able to know the reason for the interrupt. This is clear only register - set by HW upon an interrupt to Stack processor and can be cleared only by Stack processor."]
pub mod stk_up_irq_log;
#[doc = "Can be written only by base processor. Controls the enable pin of the cgcs for the clocks going to CM3 dft ctrl block and Sram.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_clk_en](stk_clk_en) module"]
pub type STK_CLK_EN = crate::Reg<u32, _STK_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STK_CLK_EN;
#[doc = "`read()` method returns [stk_clk_en::R](stk_clk_en::R) reader structure"]
impl crate::Readable for STK_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [stk_clk_en::W](stk_clk_en::W) writer structure"]
impl crate::Writable for STK_CLK_EN {}
#[doc = "Can be written only by base processor. Controls the enable pin of the cgcs for the clocks going to CM3 dft ctrl block and Sram."]
pub mod stk_clk_en;
#[doc = "Can be written only by the base processor. Decides the ram sharing mode :: handshake or Spinlock mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spin_lock_mode](spin_lock_mode) module"]
pub type SPIN_LOCK_MODE = crate::Reg<u32, _SPIN_LOCK_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIN_LOCK_MODE;
#[doc = "`read()` method returns [spin_lock_mode::R](spin_lock_mode::R) reader structure"]
impl crate::Readable for SPIN_LOCK_MODE {}
#[doc = "`write(|w| ..)` method takes [spin_lock_mode::W](spin_lock_mode::W) writer structure"]
impl crate::Writable for SPIN_LOCK_MODE {}
#[doc = "Can be written only by the base processor. Decides the ram sharing mode :: handshake or Spinlock mode."]
pub mod spin_lock_mode;
#[doc = "Stores the last bus fault address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_fault_addr](bus_fault_addr) module"]
pub type BUS_FAULT_ADDR = crate::Reg<u32, _BUS_FAULT_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_FAULT_ADDR;
#[doc = "`read()` method returns [bus_fault_addr::R](bus_fault_addr::R) reader structure"]
impl crate::Readable for BUS_FAULT_ADDR {}
#[doc = "`write(|w| ..)` method takes [bus_fault_addr::W](bus_fault_addr::W) writer structure"]
impl crate::Writable for BUS_FAULT_ADDR {}
#[doc = "Stores the last bus fault address."]
pub mod bus_fault_addr;
#[doc = "write only registers on read returns 0.W Write 1 to clear the bust fault to store the new bus fault address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_fault_clr](bus_fault_clr) module"]
pub type BUS_FAULT_CLR = crate::Reg<u32, _BUS_FAULT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_FAULT_CLR;
#[doc = "`read()` method returns [bus_fault_clr::R](bus_fault_clr::R) reader structure"]
impl crate::Readable for BUS_FAULT_CLR {}
#[doc = "`write(|w| ..)` method takes [bus_fault_clr::W](bus_fault_clr::W) writer structure"]
impl crate::Writable for BUS_FAULT_CLR {}
#[doc = "write only registers on read returns 0.W Write 1 to clear the bust fault to store the new bus fault address"]
pub mod bus_fault_clr;
#[doc = "Reset cause value captured from the ICR_CLKRST block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_cause](reset_cause) module"]
pub type RESET_CAUSE = crate::Reg<u32, _RESET_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_CAUSE;
#[doc = "`read()` method returns [reset_cause::R](reset_cause::R) reader structure"]
impl crate::Readable for RESET_CAUSE {}
#[doc = "`write(|w| ..)` method takes [reset_cause::W](reset_cause::W) writer structure"]
impl crate::Writable for RESET_CAUSE {}
#[doc = "Reset cause value captured from the ICR_CLKRST block."]
pub mod reset_cause;
#[doc = "Watchdog timer event value captured from the ICR_CLKRST block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_timer_event](wdog_timer_event) module"]
pub type WDOG_TIMER_EVENT = crate::Reg<u32, _WDOG_TIMER_EVENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDOG_TIMER_EVENT;
#[doc = "`read()` method returns [wdog_timer_event::R](wdog_timer_event::R) reader structure"]
impl crate::Readable for WDOG_TIMER_EVENT {}
#[doc = "`write(|w| ..)` method takes [wdog_timer_event::W](wdog_timer_event::W) writer structure"]
impl crate::Writable for WDOG_TIMER_EVENT {}
#[doc = "Watchdog timer event value captured from the ICR_CLKRST block"]
pub mod wdog_timer_event;
#[doc = "To send Dma Request to bottom die.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_req](dma_req) module"]
pub type DMA_REQ = crate::Reg<u32, _DMA_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_REQ;
#[doc = "`read()` method returns [dma_req::R](dma_req::R) reader structure"]
impl crate::Readable for DMA_REQ {}
#[doc = "`write(|w| ..)` method takes [dma_req::W](dma_req::W) writer structure"]
impl crate::Writable for DMA_REQ {}
#[doc = "To send Dma Request to bottom die."]
pub mod dma_req;
#[doc = "Address offset within SRAM to which CM3 should jump after reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_jump_offset_addr](sram_jump_offset_addr) module"]
pub type SRAM_JUMP_OFFSET_ADDR = crate::Reg<u32, _SRAM_JUMP_OFFSET_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_JUMP_OFFSET_ADDR;
#[doc = "`read()` method returns [sram_jump_offset_addr::R](sram_jump_offset_addr::R) reader structure"]
impl crate::Readable for SRAM_JUMP_OFFSET_ADDR {}
#[doc = "`write(|w| ..)` method takes [sram_jump_offset_addr::W](sram_jump_offset_addr::W) writer structure"]
impl crate::Writable for SRAM_JUMP_OFFSET_ADDR {}
#[doc = "Address offset within SRAM to which CM3 should jump after reset."]
pub mod sram_jump_offset_addr;
#[doc = "These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_reg1](sw_reg1) module"]
pub type SW_REG1 = crate::Reg<u32, _SW_REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_REG1;
#[doc = "`read()` method returns [sw_reg1::R](sw_reg1::R) reader structure"]
impl crate::Readable for SW_REG1 {}
#[doc = "`write(|w| ..)` method takes [sw_reg1::W](sw_reg1::W) writer structure"]
impl crate::Writable for SW_REG1 {}
#[doc = "These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored."]
pub mod sw_reg1;
#[doc = "These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_reg2](sw_reg2) module"]
pub type SW_REG2 = crate::Reg<u32, _SW_REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_REG2;
#[doc = "`read()` method returns [sw_reg2::R](sw_reg2::R) reader structure"]
impl crate::Readable for SW_REG2 {}
#[doc = "`write(|w| ..)` method takes [sw_reg2::W](sw_reg2::W) writer structure"]
impl crate::Writable for SW_REG2 {}
#[doc = "These are sw registers for topdie processor and bottom die processor to communicate. Both can set and read these registers. In case of write clash bottom die's processor wins and top die processor access is ignored."]
pub mod sw_reg2;
#[doc = "By posting the request Flash can be put into low-power mode (Sleep) without powering down the Flash. Earlier (in Garnet) this was fully h/w controlled and the control for this was coming from SysCtl while entering into Cortex Deep-sleep mode. But for our device the D2D i/f doesnt support this. The Firmware has to program the register in the top-die for entering into this mode and wait for an interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sleep_ctl](fmc_sleep_ctl) module"]
pub type FMC_SLEEP_CTL = crate::Reg<u32, _FMC_SLEEP_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_SLEEP_CTL;
#[doc = "`read()` method returns [fmc_sleep_ctl::R](fmc_sleep_ctl::R) reader structure"]
impl crate::Readable for FMC_SLEEP_CTL {}
#[doc = "`write(|w| ..)` method takes [fmc_sleep_ctl::W](fmc_sleep_ctl::W) writer structure"]
impl crate::Writable for FMC_SLEEP_CTL {}
#[doc = "By posting the request Flash can be put into low-power mode (Sleep) without powering down the Flash. Earlier (in Garnet) this was fully h/w controlled and the control for this was coming from SysCtl while entering into Cortex Deep-sleep mode. But for our device the D2D i/f doesnt support this. The Firmware has to program the register in the top-die for entering into this mode and wait for an interrupt."]
pub mod fmc_sleep_ctl;
#[doc = "Miscellanious control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctl](misc_ctl) module"]
pub type MISC_CTL = crate::Reg<u32, _MISC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CTL;
#[doc = "`read()` method returns [misc_ctl::R](misc_ctl::R) reader structure"]
impl crate::Readable for MISC_CTL {}
#[doc = "`write(|w| ..)` method takes [misc_ctl::W](misc_ctl::W) writer structure"]
impl crate::Writable for MISC_CTL {}
#[doc = "Miscellanious control register."]
pub mod misc_ctl;
#[doc = "DFT control and status bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_dft_ctl](sw_dft_ctl) module"]
pub type SW_DFT_CTL = crate::Reg<u32, _SW_DFT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_DFT_CTL;
#[doc = "`read()` method returns [sw_dft_ctl::R](sw_dft_ctl::R) reader structure"]
impl crate::Readable for SW_DFT_CTL {}
#[doc = "`write(|w| ..)` method takes [sw_dft_ctl::W](sw_dft_ctl::W) writer structure"]
impl crate::Writable for SW_DFT_CTL {}
#[doc = "DFT control and status bits"]
pub mod sw_dft_ctl;
#[doc = "Mainly for For controlling the pads OEN pins. There are total 60 pads and hence 60 control registe i.e n value varies from 0 to 59. Here is the mapping for the pad_ctl register number and the functionality : 0 D2DPAD_DMAREQ1 1 D2DPAD_DMAREQ0 2 D2DPAD_INT2BASE 3 D2DPAD_PIOSC 4 D2DPAD_RST_N 5 D2DPAD_POR_RST_N 6 D2DPAD_HCLK 7 D2DPAD_JTAG_TDO 8 D2DPAD_JTAG_TCK 9 D2DPAD_JTAG_TMS 10 D2DPAD_JTAG_TDI 11-27 D2DPAD_FROMSTACK\\[D2D_FROMSTACK_SIZE -1:0\\]
28-56 D2DPAD_TOSTACK \\[D2D_TOSTACK_SIZE -1:0\\]
57-59 D2DPAD_SPARE \\[D2D_SPARE_PAD_SIZE -1:0\\]
0:00 ****************************************************************************\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padn_ctl_0](padn_ctl_0) module"]
pub type PADN_CTL_0 = crate::Reg<u32, _PADN_CTL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADN_CTL_0;
#[doc = "`read()` method returns [padn_ctl_0::R](padn_ctl_0::R) reader structure"]
impl crate::Readable for PADN_CTL_0 {}
#[doc = "`write(|w| ..)` method takes [padn_ctl_0::W](padn_ctl_0::W) writer structure"]
impl crate::Writable for PADN_CTL_0 {}
#[doc = "Mainly for For controlling the pads OEN pins. There are total 60 pads and hence 60 control registe i.e n value varies from 0 to 59. Here is the mapping for the pad_ctl register number and the functionality : 0 D2DPAD_DMAREQ1 1 D2DPAD_DMAREQ0 2 D2DPAD_INT2BASE 3 D2DPAD_PIOSC 4 D2DPAD_RST_N 5 D2DPAD_POR_RST_N 6 D2DPAD_HCLK 7 D2DPAD_JTAG_TDO 8 D2DPAD_JTAG_TCK 9 D2DPAD_JTAG_TMS 10 D2DPAD_JTAG_TDI 11-27 D2DPAD_FROMSTACK\\[D2D_FROMSTACK_SIZE -1:0\\]
28-56 D2DPAD_TOSTACK \\[D2D_TOSTACK_SIZE -1:0\\]
57-59 D2DPAD_SPARE \\[D2D_SPARE_PAD_SIZE -1:0\\]
0:00 ****************************************************************************"]
pub mod padn_ctl_0;
