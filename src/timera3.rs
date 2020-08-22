#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM Configuration"]
    pub gptmcfg: GPTMCFG,
    #[doc = "0x04 - GPTM Timer A Mode"]
    pub gptmtamr: GPTMTAMR,
    #[doc = "0x08 - GPTM Timer B Mode"]
    pub gptmtbmr: GPTMTBMR,
    #[doc = "0x0c - GPTM Control"]
    pub gptmctl: GPTMCTL,
    #[doc = "0x10 - GPTM Synchronize"]
    pub gptmsync: GPTMSYNC,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - GPTM Interrupt Mask"]
    pub gptmimr: GPTMIMR,
    #[doc = "0x1c - GPTM Raw Interrupt Status"]
    pub gptmris: GPTMRIS,
    #[doc = "0x20 - GPTM Masked Interrupt Status"]
    pub gptmmis: GPTMMIS,
    #[doc = "0x24 - GPTM Interrupt Clear"]
    pub gptmicr: GPTMICR,
    #[doc = "0x28 - GPTM Timer A Interval Load"]
    pub gptmtailr: GPTMTAILR,
    #[doc = "0x2c - GPTM Timer B Interval Load"]
    pub gptmtbilr: GPTMTBILR,
    #[doc = "0x30 - GPTM Timer A Match"]
    pub gptmtamatchr: GPTMTAMATCHR,
    #[doc = "0x34 - GPTM Timer B Match"]
    pub gptmtbmatchr: GPTMTBMATCHR,
    #[doc = "0x38 - GPTM Timer A Prescale"]
    pub gptmtapr: GPTMTAPR,
    #[doc = "0x3c - GPTM Timer B Prescale"]
    pub gptmtbpr: GPTMTBPR,
    #[doc = "0x40 - GPTM TimerA Prescale Match"]
    pub gptmtapmr: GPTMTAPMR,
    #[doc = "0x44 - GPTM TimerB Prescale Match"]
    pub gptmtbpmr: GPTMTBPMR,
    #[doc = "0x48 - GPTM Timer A"]
    pub gptmtar: GPTMTAR,
    #[doc = "0x4c - GPTM Timer B"]
    pub gptmtbr: GPTMTBR,
    #[doc = "0x50 - GPTM Timer A Value"]
    pub gptmtav: GPTMTAV,
    #[doc = "0x54 - GPTM Timer B Value"]
    pub gptmtbv: GPTMTBV,
    #[doc = "0x58 - GPTM RTC Predivide"]
    pub gptmrtcpd: GPTMRTCPD,
    #[doc = "0x5c - GPTM Timer A Prescale Snapshot"]
    pub gptmtaps: GPTMTAPS,
    #[doc = "0x60 - GPTM Timer B Prescale Snapshot"]
    pub gptmtbps: GPTMTBPS,
    #[doc = "0x64 - GPTM Timer A Prescale Value"]
    pub gptmtapv: GPTMTAPV,
    #[doc = "0x68 - GPTM Timer B Prescale Value"]
    pub gptmtbpv: GPTMTBPV,
    #[doc = "0x6c - GPTM DMA Event"]
    pub gptmdmaev: GPTMDMAEV,
    _reserved27: [u8; 3920usize],
    #[doc = "0xfc0 - GPTM Peripheral Properties"]
    pub gptmpp: GPTMPP,
}
#[doc = "GPTM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmcfg](gptmcfg) module"]
pub type GPTMCFG = crate::Reg<u32, _GPTMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMCFG;
#[doc = "`read()` method returns [gptmcfg::R](gptmcfg::R) reader structure"]
impl crate::Readable for GPTMCFG {}
#[doc = "`write(|w| ..)` method takes [gptmcfg::W](gptmcfg::W) writer structure"]
impl crate::Writable for GPTMCFG {}
#[doc = "GPTM Configuration"]
pub mod gptmcfg;
#[doc = "GPTM Timer A Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtamr](gptmtamr) module"]
pub type GPTMTAMR = crate::Reg<u32, _GPTMTAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAMR;
#[doc = "`read()` method returns [gptmtamr::R](gptmtamr::R) reader structure"]
impl crate::Readable for GPTMTAMR {}
#[doc = "`write(|w| ..)` method takes [gptmtamr::W](gptmtamr::W) writer structure"]
impl crate::Writable for GPTMTAMR {}
#[doc = "GPTM Timer A Mode"]
pub mod gptmtamr;
#[doc = "GPTM Timer B Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbmr](gptmtbmr) module"]
pub type GPTMTBMR = crate::Reg<u32, _GPTMTBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBMR;
#[doc = "`read()` method returns [gptmtbmr::R](gptmtbmr::R) reader structure"]
impl crate::Readable for GPTMTBMR {}
#[doc = "`write(|w| ..)` method takes [gptmtbmr::W](gptmtbmr::W) writer structure"]
impl crate::Writable for GPTMTBMR {}
#[doc = "GPTM Timer B Mode"]
pub mod gptmtbmr;
#[doc = "GPTM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmctl](gptmctl) module"]
pub type GPTMCTL = crate::Reg<u32, _GPTMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMCTL;
#[doc = "`read()` method returns [gptmctl::R](gptmctl::R) reader structure"]
impl crate::Readable for GPTMCTL {}
#[doc = "`write(|w| ..)` method takes [gptmctl::W](gptmctl::W) writer structure"]
impl crate::Writable for GPTMCTL {}
#[doc = "GPTM Control"]
pub mod gptmctl;
#[doc = "GPTM Synchronize\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmsync](gptmsync) module"]
pub type GPTMSYNC = crate::Reg<u32, _GPTMSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMSYNC;
#[doc = "`read()` method returns [gptmsync::R](gptmsync::R) reader structure"]
impl crate::Readable for GPTMSYNC {}
#[doc = "`write(|w| ..)` method takes [gptmsync::W](gptmsync::W) writer structure"]
impl crate::Writable for GPTMSYNC {}
#[doc = "GPTM Synchronize"]
pub mod gptmsync;
#[doc = "GPTM Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmimr](gptmimr) module"]
pub type GPTMIMR = crate::Reg<u32, _GPTMIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMIMR;
#[doc = "`read()` method returns [gptmimr::R](gptmimr::R) reader structure"]
impl crate::Readable for GPTMIMR {}
#[doc = "`write(|w| ..)` method takes [gptmimr::W](gptmimr::W) writer structure"]
impl crate::Writable for GPTMIMR {}
#[doc = "GPTM Interrupt Mask"]
pub mod gptmimr;
#[doc = "GPTM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmris](gptmris) module"]
pub type GPTMRIS = crate::Reg<u32, _GPTMRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMRIS;
#[doc = "`read()` method returns [gptmris::R](gptmris::R) reader structure"]
impl crate::Readable for GPTMRIS {}
#[doc = "`write(|w| ..)` method takes [gptmris::W](gptmris::W) writer structure"]
impl crate::Writable for GPTMRIS {}
#[doc = "GPTM Raw Interrupt Status"]
pub mod gptmris;
#[doc = "GPTM Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmmis](gptmmis) module"]
pub type GPTMMIS = crate::Reg<u32, _GPTMMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMMIS;
#[doc = "`read()` method returns [gptmmis::R](gptmmis::R) reader structure"]
impl crate::Readable for GPTMMIS {}
#[doc = "`write(|w| ..)` method takes [gptmmis::W](gptmmis::W) writer structure"]
impl crate::Writable for GPTMMIS {}
#[doc = "GPTM Masked Interrupt Status"]
pub mod gptmmis;
#[doc = "GPTM Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmicr](gptmicr) module"]
pub type GPTMICR = crate::Reg<u32, _GPTMICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMICR;
#[doc = "`read()` method returns [gptmicr::R](gptmicr::R) reader structure"]
impl crate::Readable for GPTMICR {}
#[doc = "`write(|w| ..)` method takes [gptmicr::W](gptmicr::W) writer structure"]
impl crate::Writable for GPTMICR {}
#[doc = "GPTM Interrupt Clear"]
pub mod gptmicr;
#[doc = "GPTM Timer A Interval Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtailr](gptmtailr) module"]
pub type GPTMTAILR = crate::Reg<u32, _GPTMTAILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAILR;
#[doc = "`read()` method returns [gptmtailr::R](gptmtailr::R) reader structure"]
impl crate::Readable for GPTMTAILR {}
#[doc = "`write(|w| ..)` method takes [gptmtailr::W](gptmtailr::W) writer structure"]
impl crate::Writable for GPTMTAILR {}
#[doc = "GPTM Timer A Interval Load"]
pub mod gptmtailr;
#[doc = "GPTM Timer B Interval Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbilr](gptmtbilr) module"]
pub type GPTMTBILR = crate::Reg<u32, _GPTMTBILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBILR;
#[doc = "`read()` method returns [gptmtbilr::R](gptmtbilr::R) reader structure"]
impl crate::Readable for GPTMTBILR {}
#[doc = "`write(|w| ..)` method takes [gptmtbilr::W](gptmtbilr::W) writer structure"]
impl crate::Writable for GPTMTBILR {}
#[doc = "GPTM Timer B Interval Load"]
pub mod gptmtbilr;
#[doc = "GPTM Timer A Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtamatchr](gptmtamatchr) module"]
pub type GPTMTAMATCHR = crate::Reg<u32, _GPTMTAMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAMATCHR;
#[doc = "`read()` method returns [gptmtamatchr::R](gptmtamatchr::R) reader structure"]
impl crate::Readable for GPTMTAMATCHR {}
#[doc = "`write(|w| ..)` method takes [gptmtamatchr::W](gptmtamatchr::W) writer structure"]
impl crate::Writable for GPTMTAMATCHR {}
#[doc = "GPTM Timer A Match"]
pub mod gptmtamatchr;
#[doc = "GPTM Timer B Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbmatchr](gptmtbmatchr) module"]
pub type GPTMTBMATCHR = crate::Reg<u32, _GPTMTBMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBMATCHR;
#[doc = "`read()` method returns [gptmtbmatchr::R](gptmtbmatchr::R) reader structure"]
impl crate::Readable for GPTMTBMATCHR {}
#[doc = "`write(|w| ..)` method takes [gptmtbmatchr::W](gptmtbmatchr::W) writer structure"]
impl crate::Writable for GPTMTBMATCHR {}
#[doc = "GPTM Timer B Match"]
pub mod gptmtbmatchr;
#[doc = "GPTM Timer A Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtapr](gptmtapr) module"]
pub type GPTMTAPR = crate::Reg<u32, _GPTMTAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAPR;
#[doc = "`read()` method returns [gptmtapr::R](gptmtapr::R) reader structure"]
impl crate::Readable for GPTMTAPR {}
#[doc = "`write(|w| ..)` method takes [gptmtapr::W](gptmtapr::W) writer structure"]
impl crate::Writable for GPTMTAPR {}
#[doc = "GPTM Timer A Prescale"]
pub mod gptmtapr;
#[doc = "GPTM Timer B Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbpr](gptmtbpr) module"]
pub type GPTMTBPR = crate::Reg<u32, _GPTMTBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBPR;
#[doc = "`read()` method returns [gptmtbpr::R](gptmtbpr::R) reader structure"]
impl crate::Readable for GPTMTBPR {}
#[doc = "`write(|w| ..)` method takes [gptmtbpr::W](gptmtbpr::W) writer structure"]
impl crate::Writable for GPTMTBPR {}
#[doc = "GPTM Timer B Prescale"]
pub mod gptmtbpr;
#[doc = "GPTM TimerA Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtapmr](gptmtapmr) module"]
pub type GPTMTAPMR = crate::Reg<u32, _GPTMTAPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAPMR;
#[doc = "`read()` method returns [gptmtapmr::R](gptmtapmr::R) reader structure"]
impl crate::Readable for GPTMTAPMR {}
#[doc = "`write(|w| ..)` method takes [gptmtapmr::W](gptmtapmr::W) writer structure"]
impl crate::Writable for GPTMTAPMR {}
#[doc = "GPTM TimerA Prescale Match"]
pub mod gptmtapmr;
#[doc = "GPTM TimerB Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbpmr](gptmtbpmr) module"]
pub type GPTMTBPMR = crate::Reg<u32, _GPTMTBPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBPMR;
#[doc = "`read()` method returns [gptmtbpmr::R](gptmtbpmr::R) reader structure"]
impl crate::Readable for GPTMTBPMR {}
#[doc = "`write(|w| ..)` method takes [gptmtbpmr::W](gptmtbpmr::W) writer structure"]
impl crate::Writable for GPTMTBPMR {}
#[doc = "GPTM TimerB Prescale Match"]
pub mod gptmtbpmr;
#[doc = "GPTM Timer A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtar](gptmtar) module"]
pub type GPTMTAR = crate::Reg<u32, _GPTMTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAR;
#[doc = "`read()` method returns [gptmtar::R](gptmtar::R) reader structure"]
impl crate::Readable for GPTMTAR {}
#[doc = "`write(|w| ..)` method takes [gptmtar::W](gptmtar::W) writer structure"]
impl crate::Writable for GPTMTAR {}
#[doc = "GPTM Timer A"]
pub mod gptmtar;
#[doc = "GPTM Timer B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbr](gptmtbr) module"]
pub type GPTMTBR = crate::Reg<u32, _GPTMTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBR;
#[doc = "`read()` method returns [gptmtbr::R](gptmtbr::R) reader structure"]
impl crate::Readable for GPTMTBR {}
#[doc = "`write(|w| ..)` method takes [gptmtbr::W](gptmtbr::W) writer structure"]
impl crate::Writable for GPTMTBR {}
#[doc = "GPTM Timer B"]
pub mod gptmtbr;
#[doc = "GPTM Timer A Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtav](gptmtav) module"]
pub type GPTMTAV = crate::Reg<u32, _GPTMTAV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAV;
#[doc = "`read()` method returns [gptmtav::R](gptmtav::R) reader structure"]
impl crate::Readable for GPTMTAV {}
#[doc = "`write(|w| ..)` method takes [gptmtav::W](gptmtav::W) writer structure"]
impl crate::Writable for GPTMTAV {}
#[doc = "GPTM Timer A Value"]
pub mod gptmtav;
#[doc = "GPTM Timer B Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbv](gptmtbv) module"]
pub type GPTMTBV = crate::Reg<u32, _GPTMTBV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBV;
#[doc = "`read()` method returns [gptmtbv::R](gptmtbv::R) reader structure"]
impl crate::Readable for GPTMTBV {}
#[doc = "`write(|w| ..)` method takes [gptmtbv::W](gptmtbv::W) writer structure"]
impl crate::Writable for GPTMTBV {}
#[doc = "GPTM Timer B Value"]
pub mod gptmtbv;
#[doc = "GPTM RTC Predivide\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmrtcpd](gptmrtcpd) module"]
pub type GPTMRTCPD = crate::Reg<u32, _GPTMRTCPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMRTCPD;
#[doc = "`read()` method returns [gptmrtcpd::R](gptmrtcpd::R) reader structure"]
impl crate::Readable for GPTMRTCPD {}
#[doc = "`write(|w| ..)` method takes [gptmrtcpd::W](gptmrtcpd::W) writer structure"]
impl crate::Writable for GPTMRTCPD {}
#[doc = "GPTM RTC Predivide"]
pub mod gptmrtcpd;
#[doc = "GPTM Timer A Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtaps](gptmtaps) module"]
pub type GPTMTAPS = crate::Reg<u32, _GPTMTAPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAPS;
#[doc = "`read()` method returns [gptmtaps::R](gptmtaps::R) reader structure"]
impl crate::Readable for GPTMTAPS {}
#[doc = "`write(|w| ..)` method takes [gptmtaps::W](gptmtaps::W) writer structure"]
impl crate::Writable for GPTMTAPS {}
#[doc = "GPTM Timer A Prescale Snapshot"]
pub mod gptmtaps;
#[doc = "GPTM Timer B Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbps](gptmtbps) module"]
pub type GPTMTBPS = crate::Reg<u32, _GPTMTBPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBPS;
#[doc = "`read()` method returns [gptmtbps::R](gptmtbps::R) reader structure"]
impl crate::Readable for GPTMTBPS {}
#[doc = "`write(|w| ..)` method takes [gptmtbps::W](gptmtbps::W) writer structure"]
impl crate::Writable for GPTMTBPS {}
#[doc = "GPTM Timer B Prescale Snapshot"]
pub mod gptmtbps;
#[doc = "GPTM Timer A Prescale Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtapv](gptmtapv) module"]
pub type GPTMTAPV = crate::Reg<u32, _GPTMTAPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTAPV;
#[doc = "`read()` method returns [gptmtapv::R](gptmtapv::R) reader structure"]
impl crate::Readable for GPTMTAPV {}
#[doc = "`write(|w| ..)` method takes [gptmtapv::W](gptmtapv::W) writer structure"]
impl crate::Writable for GPTMTAPV {}
#[doc = "GPTM Timer A Prescale Value"]
pub mod gptmtapv;
#[doc = "GPTM Timer B Prescale Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmtbpv](gptmtbpv) module"]
pub type GPTMTBPV = crate::Reg<u32, _GPTMTBPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMTBPV;
#[doc = "`read()` method returns [gptmtbpv::R](gptmtbpv::R) reader structure"]
impl crate::Readable for GPTMTBPV {}
#[doc = "`write(|w| ..)` method takes [gptmtbpv::W](gptmtbpv::W) writer structure"]
impl crate::Writable for GPTMTBPV {}
#[doc = "GPTM Timer B Prescale Value"]
pub mod gptmtbpv;
#[doc = "GPTM DMA Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmdmaev](gptmdmaev) module"]
pub type GPTMDMAEV = crate::Reg<u32, _GPTMDMAEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMDMAEV;
#[doc = "`read()` method returns [gptmdmaev::R](gptmdmaev::R) reader structure"]
impl crate::Readable for GPTMDMAEV {}
#[doc = "`write(|w| ..)` method takes [gptmdmaev::W](gptmdmaev::W) writer structure"]
impl crate::Writable for GPTMDMAEV {}
#[doc = "GPTM DMA Event"]
pub mod gptmdmaev;
#[doc = "GPTM Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptmpp](gptmpp) module"]
pub type GPTMPP = crate::Reg<u32, _GPTMPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTMPP;
#[doc = "`read()` method returns [gptmpp::R](gptmpp::R) reader structure"]
impl crate::Readable for GPTMPP {}
#[doc = "`write(|w| ..)` method takes [gptmpp::W](gptmpp::W) writer structure"]
impl crate::Writable for GPTMPP {}
#[doc = "GPTM Peripheral Properties"]
pub mod gptmpp;
