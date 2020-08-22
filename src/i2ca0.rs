#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C master slave address"]
    pub i2cmsa: I2CMSA,
    #[doc = "0x04 - I2Caster control status"]
    pub i2cmcs: I2CMCS,
    #[doc = "0x08 - I2C master data register"]
    pub i2cmdr: I2CMDR,
    #[doc = "0x0c - I2C time period register"]
    pub i2cmtpr: I2CMTPR,
    #[doc = "0x10 - This register controls whether a raw interrupt is promoted to a controller interrupt"]
    pub i2cmimr: I2CMIMR,
    #[doc = "0x14 - I2CMRIS"]
    pub i2cmris: I2CMRIS,
    #[doc = "0x18 - I2CMMIS"]
    pub i2cmmis: I2CMMIS,
    #[doc = "0x1c - I2C interrupt clear register"]
    pub i2cmicr: I2CMICR,
    #[doc = "0x20 - I2C mode control register"]
    pub i2cmcr: I2CMCR,
    #[doc = "0x24 - I2C Master clock count"]
    pub i2cmclkocnt: I2CMCLKOCNT,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - I2C master bus monitor"]
    pub i2cmbmon: I2CMBMON,
    #[doc = "0x30 - I2C Master burst length"]
    pub i2cmblen: I2CMBLEN,
    #[doc = "0x34 - I2C Master burst count"]
    pub i2cmbcnt: I2CMBCNT,
    _reserved13: [u8; 1992usize],
    #[doc = "0x800 - I2C Slave own address"]
    pub i2csoar: I2CSOAR,
    #[doc = "0x804 - I2C Slave control/status register"]
    pub i2cscsr: I2CSCSR,
    #[doc = "0x808 - I2C Slave data register"]
    pub i2csdr: I2CSDR,
    #[doc = "0x80c - I2C Slave interrupt mask register"]
    pub i2csimr: I2CSIMR,
    #[doc = "0x810 - I2C Slave raw interrupt status register"]
    pub i2csris: I2CSRIS,
    #[doc = "0x814 - I2C Slave masked interrupt status register"]
    pub i2csmis: I2CSMIS,
    #[doc = "0x818 - I2C Slave interrupt clear register"]
    pub i2csicr: I2CSICR,
    #[doc = "0x81c - I2C Slave own address 2"]
    pub i2csoar2: I2CSOAR2,
    #[doc = "0x820 - I2C Slave ACK control"]
    pub i2csackctl: I2CSACKCTL,
    _reserved22: [u8; 1756usize],
    #[doc = "0xf00 - I2C FIFO data"]
    pub i2cfifodata: I2CFIFODATA,
    #[doc = "0xf04 - I2C FIFO control"]
    pub i2cfifoctl: I2CFIFOCTL,
    #[doc = "0xf08 - I2C FIFO status"]
    pub i2cfifostatus: I2CFIFOSTATUS,
    _reserved25: [u8; 116usize],
    #[doc = "0xf80 - I2COBSMUXSEL0"]
    pub i2cobsmuxsel0: I2COBSMUXSEL0,
    #[doc = "0xf84 - I2COBSMUXSEL1"]
    pub i2cobsmuxsel1: I2COBSMUXSEL1,
    #[doc = "0xf88 - I2CMUXROUTE"]
    pub i2cmuxroute: I2CMUXROUTE,
    _reserved28: [u8; 36usize],
    #[doc = "0xfb0 - I2CPV"]
    pub i2cpv: I2CPV,
    _reserved29: [u8; 12usize],
    #[doc = "0xfc0 - I2C Peripheral properties"]
    pub i2cpp: I2CPP,
    #[doc = "0xfc4 - I2C Peripheral configuration"]
    pub i2cpc: I2CPC,
    #[doc = "0xfc8 - I2CCC"]
    pub i2ccc: I2CCC,
}
#[doc = "I2C master slave address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmsa](i2cmsa) module"]
pub type I2CMSA = crate::Reg<u32, _I2CMSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMSA;
#[doc = "`read()` method returns [i2cmsa::R](i2cmsa::R) reader structure"]
impl crate::Readable for I2CMSA {}
#[doc = "`write(|w| ..)` method takes [i2cmsa::W](i2cmsa::W) writer structure"]
impl crate::Writable for I2CMSA {}
#[doc = "I2C master slave address"]
pub mod i2cmsa;
#[doc = "I2Caster control status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmcs](i2cmcs) module"]
pub type I2CMCS = crate::Reg<u32, _I2CMCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMCS;
#[doc = "`read()` method returns [i2cmcs::R](i2cmcs::R) reader structure"]
impl crate::Readable for I2CMCS {}
#[doc = "`write(|w| ..)` method takes [i2cmcs::W](i2cmcs::W) writer structure"]
impl crate::Writable for I2CMCS {}
#[doc = "I2Caster control status"]
pub mod i2cmcs;
#[doc = "I2C master data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmdr](i2cmdr) module"]
pub type I2CMDR = crate::Reg<u32, _I2CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMDR;
#[doc = "`read()` method returns [i2cmdr::R](i2cmdr::R) reader structure"]
impl crate::Readable for I2CMDR {}
#[doc = "`write(|w| ..)` method takes [i2cmdr::W](i2cmdr::W) writer structure"]
impl crate::Writable for I2CMDR {}
#[doc = "I2C master data register"]
pub mod i2cmdr;
#[doc = "I2C time period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmtpr](i2cmtpr) module"]
pub type I2CMTPR = crate::Reg<u32, _I2CMTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMTPR;
#[doc = "`read()` method returns [i2cmtpr::R](i2cmtpr::R) reader structure"]
impl crate::Readable for I2CMTPR {}
#[doc = "`write(|w| ..)` method takes [i2cmtpr::W](i2cmtpr::W) writer structure"]
impl crate::Writable for I2CMTPR {}
#[doc = "I2C time period register"]
pub mod i2cmtpr;
#[doc = "This register controls whether a raw interrupt is promoted to a controller interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmimr](i2cmimr) module"]
pub type I2CMIMR = crate::Reg<u32, _I2CMIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMIMR;
#[doc = "`read()` method returns [i2cmimr::R](i2cmimr::R) reader structure"]
impl crate::Readable for I2CMIMR {}
#[doc = "`write(|w| ..)` method takes [i2cmimr::W](i2cmimr::W) writer structure"]
impl crate::Writable for I2CMIMR {}
#[doc = "This register controls whether a raw interrupt is promoted to a controller interrupt"]
pub mod i2cmimr;
#[doc = "I2CMRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmris](i2cmris) module"]
pub type I2CMRIS = crate::Reg<u32, _I2CMRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMRIS;
#[doc = "`read()` method returns [i2cmris::R](i2cmris::R) reader structure"]
impl crate::Readable for I2CMRIS {}
#[doc = "`write(|w| ..)` method takes [i2cmris::W](i2cmris::W) writer structure"]
impl crate::Writable for I2CMRIS {}
#[doc = "I2CMRIS"]
pub mod i2cmris;
#[doc = "I2CMMIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmmis](i2cmmis) module"]
pub type I2CMMIS = crate::Reg<u32, _I2CMMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMMIS;
#[doc = "`read()` method returns [i2cmmis::R](i2cmmis::R) reader structure"]
impl crate::Readable for I2CMMIS {}
#[doc = "`write(|w| ..)` method takes [i2cmmis::W](i2cmmis::W) writer structure"]
impl crate::Writable for I2CMMIS {}
#[doc = "I2CMMIS"]
pub mod i2cmmis;
#[doc = "I2C interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmicr](i2cmicr) module"]
pub type I2CMICR = crate::Reg<u32, _I2CMICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMICR;
#[doc = "`read()` method returns [i2cmicr::R](i2cmicr::R) reader structure"]
impl crate::Readable for I2CMICR {}
#[doc = "`write(|w| ..)` method takes [i2cmicr::W](i2cmicr::W) writer structure"]
impl crate::Writable for I2CMICR {}
#[doc = "I2C interrupt clear register"]
pub mod i2cmicr;
#[doc = "I2C mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmcr](i2cmcr) module"]
pub type I2CMCR = crate::Reg<u32, _I2CMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMCR;
#[doc = "`read()` method returns [i2cmcr::R](i2cmcr::R) reader structure"]
impl crate::Readable for I2CMCR {}
#[doc = "`write(|w| ..)` method takes [i2cmcr::W](i2cmcr::W) writer structure"]
impl crate::Writable for I2CMCR {}
#[doc = "I2C mode control register"]
pub mod i2cmcr;
#[doc = "I2C Master clock count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmclkocnt](i2cmclkocnt) module"]
pub type I2CMCLKOCNT = crate::Reg<u32, _I2CMCLKOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMCLKOCNT;
#[doc = "`read()` method returns [i2cmclkocnt::R](i2cmclkocnt::R) reader structure"]
impl crate::Readable for I2CMCLKOCNT {}
#[doc = "`write(|w| ..)` method takes [i2cmclkocnt::W](i2cmclkocnt::W) writer structure"]
impl crate::Writable for I2CMCLKOCNT {}
#[doc = "I2C Master clock count"]
pub mod i2cmclkocnt;
#[doc = "I2C master bus monitor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmbmon](i2cmbmon) module"]
pub type I2CMBMON = crate::Reg<u32, _I2CMBMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMBMON;
#[doc = "`read()` method returns [i2cmbmon::R](i2cmbmon::R) reader structure"]
impl crate::Readable for I2CMBMON {}
#[doc = "`write(|w| ..)` method takes [i2cmbmon::W](i2cmbmon::W) writer structure"]
impl crate::Writable for I2CMBMON {}
#[doc = "I2C master bus monitor"]
pub mod i2cmbmon;
#[doc = "I2C Master burst length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmblen](i2cmblen) module"]
pub type I2CMBLEN = crate::Reg<u32, _I2CMBLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMBLEN;
#[doc = "`read()` method returns [i2cmblen::R](i2cmblen::R) reader structure"]
impl crate::Readable for I2CMBLEN {}
#[doc = "`write(|w| ..)` method takes [i2cmblen::W](i2cmblen::W) writer structure"]
impl crate::Writable for I2CMBLEN {}
#[doc = "I2C Master burst length"]
pub mod i2cmblen;
#[doc = "I2C Master burst count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmbcnt](i2cmbcnt) module"]
pub type I2CMBCNT = crate::Reg<u32, _I2CMBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMBCNT;
#[doc = "`read()` method returns [i2cmbcnt::R](i2cmbcnt::R) reader structure"]
impl crate::Readable for I2CMBCNT {}
#[doc = "`write(|w| ..)` method takes [i2cmbcnt::W](i2cmbcnt::W) writer structure"]
impl crate::Writable for I2CMBCNT {}
#[doc = "I2C Master burst count"]
pub mod i2cmbcnt;
#[doc = "I2C Slave own address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csoar](i2csoar) module"]
pub type I2CSOAR = crate::Reg<u32, _I2CSOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSOAR;
#[doc = "`read()` method returns [i2csoar::R](i2csoar::R) reader structure"]
impl crate::Readable for I2CSOAR {}
#[doc = "`write(|w| ..)` method takes [i2csoar::W](i2csoar::W) writer structure"]
impl crate::Writable for I2CSOAR {}
#[doc = "I2C Slave own address"]
pub mod i2csoar;
#[doc = "I2C Slave control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cscsr](i2cscsr) module"]
pub type I2CSCSR = crate::Reg<u32, _I2CSCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSCSR;
#[doc = "`read()` method returns [i2cscsr::R](i2cscsr::R) reader structure"]
impl crate::Readable for I2CSCSR {}
#[doc = "`write(|w| ..)` method takes [i2cscsr::W](i2cscsr::W) writer structure"]
impl crate::Writable for I2CSCSR {}
#[doc = "I2C Slave control/status register"]
pub mod i2cscsr;
#[doc = "I2C Slave data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csdr](i2csdr) module"]
pub type I2CSDR = crate::Reg<u32, _I2CSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSDR;
#[doc = "`read()` method returns [i2csdr::R](i2csdr::R) reader structure"]
impl crate::Readable for I2CSDR {}
#[doc = "`write(|w| ..)` method takes [i2csdr::W](i2csdr::W) writer structure"]
impl crate::Writable for I2CSDR {}
#[doc = "I2C Slave data register"]
pub mod i2csdr;
#[doc = "I2C Slave interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csimr](i2csimr) module"]
pub type I2CSIMR = crate::Reg<u32, _I2CSIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSIMR;
#[doc = "`read()` method returns [i2csimr::R](i2csimr::R) reader structure"]
impl crate::Readable for I2CSIMR {}
#[doc = "`write(|w| ..)` method takes [i2csimr::W](i2csimr::W) writer structure"]
impl crate::Writable for I2CSIMR {}
#[doc = "I2C Slave interrupt mask register"]
pub mod i2csimr;
#[doc = "I2C Slave raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csris](i2csris) module"]
pub type I2CSRIS = crate::Reg<u32, _I2CSRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSRIS;
#[doc = "`read()` method returns [i2csris::R](i2csris::R) reader structure"]
impl crate::Readable for I2CSRIS {}
#[doc = "`write(|w| ..)` method takes [i2csris::W](i2csris::W) writer structure"]
impl crate::Writable for I2CSRIS {}
#[doc = "I2C Slave raw interrupt status register"]
pub mod i2csris;
#[doc = "I2C Slave masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csmis](i2csmis) module"]
pub type I2CSMIS = crate::Reg<u32, _I2CSMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSMIS;
#[doc = "`read()` method returns [i2csmis::R](i2csmis::R) reader structure"]
impl crate::Readable for I2CSMIS {}
#[doc = "`write(|w| ..)` method takes [i2csmis::W](i2csmis::W) writer structure"]
impl crate::Writable for I2CSMIS {}
#[doc = "I2C Slave masked interrupt status register"]
pub mod i2csmis;
#[doc = "I2C Slave interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csicr](i2csicr) module"]
pub type I2CSICR = crate::Reg<u32, _I2CSICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSICR;
#[doc = "`read()` method returns [i2csicr::R](i2csicr::R) reader structure"]
impl crate::Readable for I2CSICR {}
#[doc = "`write(|w| ..)` method takes [i2csicr::W](i2csicr::W) writer structure"]
impl crate::Writable for I2CSICR {}
#[doc = "I2C Slave interrupt clear register"]
pub mod i2csicr;
#[doc = "I2C Slave own address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csoar2](i2csoar2) module"]
pub type I2CSOAR2 = crate::Reg<u32, _I2CSOAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSOAR2;
#[doc = "`read()` method returns [i2csoar2::R](i2csoar2::R) reader structure"]
impl crate::Readable for I2CSOAR2 {}
#[doc = "`write(|w| ..)` method takes [i2csoar2::W](i2csoar2::W) writer structure"]
impl crate::Writable for I2CSOAR2 {}
#[doc = "I2C Slave own address 2"]
pub mod i2csoar2;
#[doc = "I2C Slave ACK control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2csackctl](i2csackctl) module"]
pub type I2CSACKCTL = crate::Reg<u32, _I2CSACKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSACKCTL;
#[doc = "`read()` method returns [i2csackctl::R](i2csackctl::R) reader structure"]
impl crate::Readable for I2CSACKCTL {}
#[doc = "`write(|w| ..)` method takes [i2csackctl::W](i2csackctl::W) writer structure"]
impl crate::Writable for I2CSACKCTL {}
#[doc = "I2C Slave ACK control"]
pub mod i2csackctl;
#[doc = "I2C FIFO data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cfifodata](i2cfifodata) module"]
pub type I2CFIFODATA = crate::Reg<u32, _I2CFIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CFIFODATA;
#[doc = "`read()` method returns [i2cfifodata::R](i2cfifodata::R) reader structure"]
impl crate::Readable for I2CFIFODATA {}
#[doc = "`write(|w| ..)` method takes [i2cfifodata::W](i2cfifodata::W) writer structure"]
impl crate::Writable for I2CFIFODATA {}
#[doc = "I2C FIFO data"]
pub mod i2cfifodata;
#[doc = "I2C FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cfifoctl](i2cfifoctl) module"]
pub type I2CFIFOCTL = crate::Reg<u32, _I2CFIFOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CFIFOCTL;
#[doc = "`read()` method returns [i2cfifoctl::R](i2cfifoctl::R) reader structure"]
impl crate::Readable for I2CFIFOCTL {}
#[doc = "`write(|w| ..)` method takes [i2cfifoctl::W](i2cfifoctl::W) writer structure"]
impl crate::Writable for I2CFIFOCTL {}
#[doc = "I2C FIFO control"]
pub mod i2cfifoctl;
#[doc = "I2C FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cfifostatus](i2cfifostatus) module"]
pub type I2CFIFOSTATUS = crate::Reg<u32, _I2CFIFOSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CFIFOSTATUS;
#[doc = "`read()` method returns [i2cfifostatus::R](i2cfifostatus::R) reader structure"]
impl crate::Readable for I2CFIFOSTATUS {}
#[doc = "`write(|w| ..)` method takes [i2cfifostatus::W](i2cfifostatus::W) writer structure"]
impl crate::Writable for I2CFIFOSTATUS {}
#[doc = "I2C FIFO status"]
pub mod i2cfifostatus;
#[doc = "I2COBSMUXSEL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cobsmuxsel0](i2cobsmuxsel0) module"]
pub type I2COBSMUXSEL0 = crate::Reg<u32, _I2COBSMUXSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2COBSMUXSEL0;
#[doc = "`read()` method returns [i2cobsmuxsel0::R](i2cobsmuxsel0::R) reader structure"]
impl crate::Readable for I2COBSMUXSEL0 {}
#[doc = "`write(|w| ..)` method takes [i2cobsmuxsel0::W](i2cobsmuxsel0::W) writer structure"]
impl crate::Writable for I2COBSMUXSEL0 {}
#[doc = "I2COBSMUXSEL0"]
pub mod i2cobsmuxsel0;
#[doc = "I2COBSMUXSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cobsmuxsel1](i2cobsmuxsel1) module"]
pub type I2COBSMUXSEL1 = crate::Reg<u32, _I2COBSMUXSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2COBSMUXSEL1;
#[doc = "`read()` method returns [i2cobsmuxsel1::R](i2cobsmuxsel1::R) reader structure"]
impl crate::Readable for I2COBSMUXSEL1 {}
#[doc = "`write(|w| ..)` method takes [i2cobsmuxsel1::W](i2cobsmuxsel1::W) writer structure"]
impl crate::Writable for I2COBSMUXSEL1 {}
#[doc = "I2COBSMUXSEL1"]
pub mod i2cobsmuxsel1;
#[doc = "I2CMUXROUTE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmuxroute](i2cmuxroute) module"]
pub type I2CMUXROUTE = crate::Reg<u32, _I2CMUXROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMUXROUTE;
#[doc = "`read()` method returns [i2cmuxroute::R](i2cmuxroute::R) reader structure"]
impl crate::Readable for I2CMUXROUTE {}
#[doc = "`write(|w| ..)` method takes [i2cmuxroute::W](i2cmuxroute::W) writer structure"]
impl crate::Writable for I2CMUXROUTE {}
#[doc = "I2CMUXROUTE"]
pub mod i2cmuxroute;
#[doc = "I2CPV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cpv](i2cpv) module"]
pub type I2CPV = crate::Reg<u32, _I2CPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CPV;
#[doc = "`read()` method returns [i2cpv::R](i2cpv::R) reader structure"]
impl crate::Readable for I2CPV {}
#[doc = "`write(|w| ..)` method takes [i2cpv::W](i2cpv::W) writer structure"]
impl crate::Writable for I2CPV {}
#[doc = "I2CPV"]
pub mod i2cpv;
#[doc = "I2C Peripheral properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cpp](i2cpp) module"]
pub type I2CPP = crate::Reg<u32, _I2CPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CPP;
#[doc = "`read()` method returns [i2cpp::R](i2cpp::R) reader structure"]
impl crate::Readable for I2CPP {}
#[doc = "`write(|w| ..)` method takes [i2cpp::W](i2cpp::W) writer structure"]
impl crate::Writable for I2CPP {}
#[doc = "I2C Peripheral properties"]
pub mod i2cpp;
#[doc = "I2C Peripheral configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cpc](i2cpc) module"]
pub type I2CPC = crate::Reg<u32, _I2CPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CPC;
#[doc = "`read()` method returns [i2cpc::R](i2cpc::R) reader structure"]
impl crate::Readable for I2CPC {}
#[doc = "`write(|w| ..)` method takes [i2cpc::W](i2cpc::W) writer structure"]
impl crate::Writable for I2CPC {}
#[doc = "I2C Peripheral configuration"]
pub mod i2cpc;
#[doc = "I2CCC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2ccc](i2ccc) module"]
pub type I2CCC = crate::Reg<u32, _I2CCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CCC;
#[doc = "`read()` method returns [i2ccc::R](i2ccc::R) reader structure"]
impl crate::Readable for I2CCC {}
#[doc = "`write(|w| ..)` method takes [i2ccc::W](i2ccc::W) writer structure"]
impl crate::Writable for I2CCC {}
#[doc = "I2CCC"]
pub mod i2ccc;
