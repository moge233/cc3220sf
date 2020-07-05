#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the IP revision code ( Parallel Mode)"]
    pub cc_revision: CC_REVISION,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - This register controls the various parameters of the OCP interface (CCP and Parallel Mode)"]
    pub cc_sysconfig: CC_SYSCONFIG,
    #[doc = "0x14 - This register provides status information about the module excluding the interrupt status information (CCP and Parallel Mode)"]
    pub cc_sysstatus: CC_SYSSTATUS,
    #[doc = "0x18 - The interrupt status regroups all the status of the module internal events that can generate an interrupt (CCP & Parallel Mode)"]
    pub cc_irqstatus: CC_IRQSTATUS,
    #[doc = "0x1c - The interrupt enable register allows to enable/disable the module internal sources of interrupt on an event-by-event basis (CCP & Parallel Mode)"]
    pub cc_irqenable: CC_IRQENABLE,
    _reserved5: [u8; 32usize],
    #[doc = "0x40 - This register controls the various parameters of the Camera Core block (CCP & Parallel Mode)"]
    pub cc_ctrl: CC_CTRL,
    #[doc = "0x44 - This register controls the DMA interface of the Camera Core block (CCP & Parallel Mode)"]
    pub cc_ctrl_dma: CC_CTRL_DMA,
    #[doc = "0x48 - This register control the value of the clock divisor used to generate the external clock (Parallel Mode)"]
    pub cc_ctrl_xclk: CC_CTRL_XCLK,
    #[doc = "0x4c - This register allows to write to the FIFO and read from the FIFO (CCP & Parallel Mode)"]
    pub cc_fifo_data: CC_FIFO_DATA,
    #[doc = "0x50 - This register shows the status of some important variables of the camera core module (CCP & Parallel Mode)"]
    pub cc_test: CC_TEST,
    #[doc = "0x54 - This register shows the values of the generic parameters of the module ****************************************************************************"]
    pub cc_gen_par: CC_GEN_PAR,
}
#[doc = "This register contains the IP revision code ( Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_revision](cc_revision) module"]
pub type CC_REVISION = crate::Reg<u32, _CC_REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_REVISION;
#[doc = "`read()` method returns [cc_revision::R](cc_revision::R) reader structure"]
impl crate::Readable for CC_REVISION {}
#[doc = "`write(|w| ..)` method takes [cc_revision::W](cc_revision::W) writer structure"]
impl crate::Writable for CC_REVISION {}
#[doc = "This register contains the IP revision code ( Parallel Mode)"]
pub mod cc_revision;
#[doc = "This register controls the various parameters of the OCP interface (CCP and Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_sysconfig](cc_sysconfig) module"]
pub type CC_SYSCONFIG = crate::Reg<u32, _CC_SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_SYSCONFIG;
#[doc = "`read()` method returns [cc_sysconfig::R](cc_sysconfig::R) reader structure"]
impl crate::Readable for CC_SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [cc_sysconfig::W](cc_sysconfig::W) writer structure"]
impl crate::Writable for CC_SYSCONFIG {}
#[doc = "This register controls the various parameters of the OCP interface (CCP and Parallel Mode)"]
pub mod cc_sysconfig;
#[doc = "This register provides status information about the module excluding the interrupt status information (CCP and Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_sysstatus](cc_sysstatus) module"]
pub type CC_SYSSTATUS = crate::Reg<u32, _CC_SYSSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_SYSSTATUS;
#[doc = "`read()` method returns [cc_sysstatus::R](cc_sysstatus::R) reader structure"]
impl crate::Readable for CC_SYSSTATUS {}
#[doc = "`write(|w| ..)` method takes [cc_sysstatus::W](cc_sysstatus::W) writer structure"]
impl crate::Writable for CC_SYSSTATUS {}
#[doc = "This register provides status information about the module excluding the interrupt status information (CCP and Parallel Mode)"]
pub mod cc_sysstatus;
#[doc = "The interrupt status regroups all the status of the module internal events that can generate an interrupt (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_irqstatus](cc_irqstatus) module"]
pub type CC_IRQSTATUS = crate::Reg<u32, _CC_IRQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_IRQSTATUS;
#[doc = "`read()` method returns [cc_irqstatus::R](cc_irqstatus::R) reader structure"]
impl crate::Readable for CC_IRQSTATUS {}
#[doc = "`write(|w| ..)` method takes [cc_irqstatus::W](cc_irqstatus::W) writer structure"]
impl crate::Writable for CC_IRQSTATUS {}
#[doc = "The interrupt status regroups all the status of the module internal events that can generate an interrupt (CCP & Parallel Mode)"]
pub mod cc_irqstatus;
#[doc = "The interrupt enable register allows to enable/disable the module internal sources of interrupt on an event-by-event basis (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_irqenable](cc_irqenable) module"]
pub type CC_IRQENABLE = crate::Reg<u32, _CC_IRQENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_IRQENABLE;
#[doc = "`read()` method returns [cc_irqenable::R](cc_irqenable::R) reader structure"]
impl crate::Readable for CC_IRQENABLE {}
#[doc = "`write(|w| ..)` method takes [cc_irqenable::W](cc_irqenable::W) writer structure"]
impl crate::Writable for CC_IRQENABLE {}
#[doc = "The interrupt enable register allows to enable/disable the module internal sources of interrupt on an event-by-event basis (CCP & Parallel Mode)"]
pub mod cc_irqenable;
#[doc = "This register controls the various parameters of the Camera Core block (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_ctrl](cc_ctrl) module"]
pub type CC_CTRL = crate::Reg<u32, _CC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_CTRL;
#[doc = "`read()` method returns [cc_ctrl::R](cc_ctrl::R) reader structure"]
impl crate::Readable for CC_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc_ctrl::W](cc_ctrl::W) writer structure"]
impl crate::Writable for CC_CTRL {}
#[doc = "This register controls the various parameters of the Camera Core block (CCP & Parallel Mode)"]
pub mod cc_ctrl;
#[doc = "This register controls the DMA interface of the Camera Core block (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_ctrl_dma](cc_ctrl_dma) module"]
pub type CC_CTRL_DMA = crate::Reg<u32, _CC_CTRL_DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_CTRL_DMA;
#[doc = "`read()` method returns [cc_ctrl_dma::R](cc_ctrl_dma::R) reader structure"]
impl crate::Readable for CC_CTRL_DMA {}
#[doc = "`write(|w| ..)` method takes [cc_ctrl_dma::W](cc_ctrl_dma::W) writer structure"]
impl crate::Writable for CC_CTRL_DMA {}
#[doc = "This register controls the DMA interface of the Camera Core block (CCP & Parallel Mode)"]
pub mod cc_ctrl_dma;
#[doc = "This register control the value of the clock divisor used to generate the external clock (Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_ctrl_xclk](cc_ctrl_xclk) module"]
pub type CC_CTRL_XCLK = crate::Reg<u32, _CC_CTRL_XCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_CTRL_XCLK;
#[doc = "`read()` method returns [cc_ctrl_xclk::R](cc_ctrl_xclk::R) reader structure"]
impl crate::Readable for CC_CTRL_XCLK {}
#[doc = "`write(|w| ..)` method takes [cc_ctrl_xclk::W](cc_ctrl_xclk::W) writer structure"]
impl crate::Writable for CC_CTRL_XCLK {}
#[doc = "This register control the value of the clock divisor used to generate the external clock (Parallel Mode)"]
pub mod cc_ctrl_xclk;
#[doc = "This register allows to write to the FIFO and read from the FIFO (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_fifo_data](cc_fifo_data) module"]
pub type CC_FIFO_DATA = crate::Reg<u32, _CC_FIFO_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_FIFO_DATA;
#[doc = "`read()` method returns [cc_fifo_data::R](cc_fifo_data::R) reader structure"]
impl crate::Readable for CC_FIFO_DATA {}
#[doc = "`write(|w| ..)` method takes [cc_fifo_data::W](cc_fifo_data::W) writer structure"]
impl crate::Writable for CC_FIFO_DATA {}
#[doc = "This register allows to write to the FIFO and read from the FIFO (CCP & Parallel Mode)"]
pub mod cc_fifo_data;
#[doc = "This register shows the status of some important variables of the camera core module (CCP & Parallel Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_test](cc_test) module"]
pub type CC_TEST = crate::Reg<u32, _CC_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_TEST;
#[doc = "`read()` method returns [cc_test::R](cc_test::R) reader structure"]
impl crate::Readable for CC_TEST {}
#[doc = "`write(|w| ..)` method takes [cc_test::W](cc_test::W) writer structure"]
impl crate::Writable for CC_TEST {}
#[doc = "This register shows the status of some important variables of the camera core module (CCP & Parallel Mode)"]
pub mod cc_test;
#[doc = "This register shows the values of the generic parameters of the module ****************************************************************************\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_gen_par](cc_gen_par) module"]
pub type CC_GEN_PAR = crate::Reg<u32, _CC_GEN_PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_GEN_PAR;
#[doc = "`read()` method returns [cc_gen_par::R](cc_gen_par::R) reader structure"]
impl crate::Readable for CC_GEN_PAR {}
#[doc = "`write(|w| ..)` method takes [cc_gen_par::W](cc_gen_par::W) writer structure"]
impl crate::Writable for CC_GEN_PAR {}
#[doc = "This register shows the values of the generic parameters of the module ****************************************************************************"]
pub mod cc_gen_par;
