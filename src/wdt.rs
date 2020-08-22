#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load Value Register"]
    pub wdtload: WDTLOAD,
    #[doc = "0x04 - Watchdog Current Value Register"]
    pub wdtvalue: WDTVALUE,
    #[doc = "0x08 - Watchdog Control Register"]
    pub wdtctl: WDTCTL,
    #[doc = "0x0c - Watchdog Interrupt Clear Register"]
    pub wdticr: WDTICR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status Register"]
    pub wdtris: WDTRIS,
    #[doc = "0x14 - WDTMIS"]
    pub wdtmis: WDTMIS,
    _reserved6: [u8; 1024usize],
    #[doc = "0x418 - WDTTEST"]
    pub wdttest: WDTTEST,
    _reserved7: [u8; 2020usize],
    #[doc = "0xc00 - Watchdog Lock Register. A write of the value 0x1ACCE551 unlocks the watchdog registers for write access. All other values lock the watchdog registers."]
    pub wdtlock: WDTLOCK,
}
#[doc = "Watchdog Load Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtload](wdtload) module"]
pub type WDTLOAD = crate::Reg<u32, _WDTLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTLOAD;
#[doc = "`read()` method returns [wdtload::R](wdtload::R) reader structure"]
impl crate::Readable for WDTLOAD {}
#[doc = "`write(|w| ..)` method takes [wdtload::W](wdtload::W) writer structure"]
impl crate::Writable for WDTLOAD {}
#[doc = "Watchdog Load Value Register"]
pub mod wdtload;
#[doc = "Watchdog Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtvalue](wdtvalue) module"]
pub type WDTVALUE = crate::Reg<u32, _WDTVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTVALUE;
#[doc = "`read()` method returns [wdtvalue::R](wdtvalue::R) reader structure"]
impl crate::Readable for WDTVALUE {}
#[doc = "`write(|w| ..)` method takes [wdtvalue::W](wdtvalue::W) writer structure"]
impl crate::Writable for WDTVALUE {}
#[doc = "Watchdog Current Value Register"]
pub mod wdtvalue;
#[doc = "Watchdog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](wdtctl) module"]
pub type WDTCTL = crate::Reg<u32, _WDTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCTL;
#[doc = "`read()` method returns [wdtctl::R](wdtctl::R) reader structure"]
impl crate::Readable for WDTCTL {}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](wdtctl::W) writer structure"]
impl crate::Writable for WDTCTL {}
#[doc = "Watchdog Control Register"]
pub mod wdtctl;
#[doc = "Watchdog Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdticr](wdticr) module"]
pub type WDTICR = crate::Reg<u32, _WDTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTICR;
#[doc = "`read()` method returns [wdticr::R](wdticr::R) reader structure"]
impl crate::Readable for WDTICR {}
#[doc = "`write(|w| ..)` method takes [wdticr::W](wdticr::W) writer structure"]
impl crate::Writable for WDTICR {}
#[doc = "Watchdog Interrupt Clear Register"]
pub mod wdticr;
#[doc = "Watchdog Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtris](wdtris) module"]
pub type WDTRIS = crate::Reg<u32, _WDTRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTRIS;
#[doc = "`read()` method returns [wdtris::R](wdtris::R) reader structure"]
impl crate::Readable for WDTRIS {}
#[doc = "`write(|w| ..)` method takes [wdtris::W](wdtris::W) writer structure"]
impl crate::Writable for WDTRIS {}
#[doc = "Watchdog Raw Interrupt Status Register"]
pub mod wdtris;
#[doc = "WDTMIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtmis](wdtmis) module"]
pub type WDTMIS = crate::Reg<u32, _WDTMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTMIS;
#[doc = "`read()` method returns [wdtmis::R](wdtmis::R) reader structure"]
impl crate::Readable for WDTMIS {}
#[doc = "`write(|w| ..)` method takes [wdtmis::W](wdtmis::W) writer structure"]
impl crate::Writable for WDTMIS {}
#[doc = "WDTMIS"]
pub mod wdtmis;
#[doc = "WDTTEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdttest](wdttest) module"]
pub type WDTTEST = crate::Reg<u32, _WDTTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTTEST;
#[doc = "`read()` method returns [wdttest::R](wdttest::R) reader structure"]
impl crate::Readable for WDTTEST {}
#[doc = "`write(|w| ..)` method takes [wdttest::W](wdttest::W) writer structure"]
impl crate::Writable for WDTTEST {}
#[doc = "WDTTEST"]
pub mod wdttest;
#[doc = "Watchdog Lock Register. A write of the value 0x1ACCE551 unlocks the watchdog registers for write access. All other values lock the watchdog registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtlock](wdtlock) module"]
pub type WDTLOCK = crate::Reg<u32, _WDTLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTLOCK;
#[doc = "`read()` method returns [wdtlock::R](wdtlock::R) reader structure"]
impl crate::Readable for WDTLOCK {}
#[doc = "`write(|w| ..)` method takes [wdtlock::W](wdtlock::W) writer structure"]
impl crate::Writable for WDTLOCK {}
#[doc = "Watchdog Lock Register. A write of the value 0x1ACCE551 unlocks the watchdog registers for write access. All other values lock the watchdog registers."]
pub mod wdtlock;
