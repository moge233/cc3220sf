#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO data register"]
    pub gpiodata: GPIODATA,
    _reserved1: [u8; 1020usize],
    #[doc = "0x400 - GPIO data direction register"]
    pub gpiodir: GPIODIR,
    #[doc = "0x404 - GPIO interrupt sense register"]
    pub gpiois: GPIOIS,
    #[doc = "0x408 - GPIO interrupt on both edges register"]
    pub gpioibe: GPIOIBE,
    #[doc = "0x40c - GPIO interrupt event register"]
    pub gpioiev: GPIOIEV,
    #[doc = "0x410 - GPIO interrupt mask register"]
    pub gpioim: GPIOIM,
    #[doc = "0x414 - GPIO raw interrupt status"]
    pub gpioris: GPIORIS,
    #[doc = "0x418 - GPIO masked interrupt status"]
    pub gpiomis: GPIOMIS,
    #[doc = "0x41c - GPIO interrupt clear register"]
    pub gpioicr: GPIOICR,
    #[doc = "0x420 - GPIOAFSEL"]
    pub gpioafsel: GPIOAFSEL,
    _reserved10: [u8; 220usize],
    #[doc = "0x500 - GPIODR2R"]
    pub gpiodr2r: GPIODR2R,
    #[doc = "0x504 - GPIODR4R"]
    pub gpiodr4r: GPIODR4R,
    #[doc = "0x508 - GPIODR8R"]
    pub gpiodr8r: GPIODR8R,
    #[doc = "0x50c - GPIOODR"]
    pub gpioodr: GPIOODR,
    #[doc = "0x510 - GPIOPUR"]
    pub gpiopur: GPIOPUR,
    #[doc = "0x514 - GPIOPDR"]
    pub gpiopdr: GPIOPDR,
    #[doc = "0x518 - GPIOSLR"]
    pub gpioslr: GPIOSLR,
    #[doc = "0x51c - GPIODEN"]
    pub gpioden: GPIODEN,
    #[doc = "0x520 - GPIOLOCK"]
    pub gpiolock: GPIOLOCK,
    #[doc = "0x524 - GPIOCR"]
    pub gpiocr: GPIOCR,
    #[doc = "0x528 - GPIOAMSEL"]
    pub gpioamsel: GPIOAMSEL,
    #[doc = "0x52c - GPIOPCTL"]
    pub gpiopctl: GPIOPCTL,
    #[doc = "0x530 - GPIOADCCTL"]
    pub gpioadcctl: GPIOADCCTL,
    #[doc = "0x534 - GPIODMACTL"]
    pub gpiodmactl: GPIODMACTL,
    #[doc = "0x538 - GPIOSI"]
    pub gpiosi: GPIOSI,
    _reserved25: [u8; 2708usize],
    #[doc = "0xfd0 - GPIOPERIPHID4"]
    pub gpioperiphid4: GPIOPERIPHID4,
    #[doc = "0xfd4 - GPIOPERIPHID5"]
    pub gpioperiphid5: GPIOPERIPHID5,
    #[doc = "0xfd8 - GPIOPERIPHID6"]
    pub gpioperiphid6: GPIOPERIPHID6,
    #[doc = "0xfdc - GPIOPERIPHID7"]
    pub gpioperiphid7: GPIOPERIPHID7,
    #[doc = "0xfe0 - GPIOPERIPHID0"]
    pub gpioperiphid0: GPIOPERIPHID0,
    #[doc = "0xfe4 - GPIOPERIPHID1"]
    pub gpioperiphid1: GPIOPERIPHID1,
    #[doc = "0xfe8 - GPIOPERIPHID2"]
    pub gpioperiphid2: GPIOPERIPHID2,
    #[doc = "0xfec - GPIOPERIPHID3"]
    pub gpioperiphid3: GPIOPERIPHID3,
    #[doc = "0xff0 - GPIOPCELLID0"]
    pub gpiopcellid0: GPIOPCELLID0,
    #[doc = "0xff4 - GPIOPCELLID1"]
    pub gpiopcellid1: GPIOPCELLID1,
    #[doc = "0xff8 - GPIOPCELLID2"]
    pub gpiopcellid2: GPIOPCELLID2,
    #[doc = "0xffc - GPIOPCELLID3"]
    pub gpiopcellid3: GPIOPCELLID3,
}
#[doc = "GPIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodata](gpiodata) module"]
pub type GPIODATA = crate::Reg<u32, _GPIODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODATA;
#[doc = "`read()` method returns [gpiodata::R](gpiodata::R) reader structure"]
impl crate::Readable for GPIODATA {}
#[doc = "`write(|w| ..)` method takes [gpiodata::W](gpiodata::W) writer structure"]
impl crate::Writable for GPIODATA {}
#[doc = "GPIO data register"]
pub mod gpiodata;
#[doc = "GPIO data direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodir](gpiodir) module"]
pub type GPIODIR = crate::Reg<u32, _GPIODIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODIR;
#[doc = "`read()` method returns [gpiodir::R](gpiodir::R) reader structure"]
impl crate::Readable for GPIODIR {}
#[doc = "`write(|w| ..)` method takes [gpiodir::W](gpiodir::W) writer structure"]
impl crate::Writable for GPIODIR {}
#[doc = "GPIO data direction register"]
pub mod gpiodir;
#[doc = "GPIO interrupt sense register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiois](gpiois) module"]
pub type GPIOIS = crate::Reg<u32, _GPIOIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOIS;
#[doc = "`read()` method returns [gpiois::R](gpiois::R) reader structure"]
impl crate::Readable for GPIOIS {}
#[doc = "`write(|w| ..)` method takes [gpiois::W](gpiois::W) writer structure"]
impl crate::Writable for GPIOIS {}
#[doc = "GPIO interrupt sense register"]
pub mod gpiois;
#[doc = "GPIO interrupt on both edges register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioibe](gpioibe) module"]
pub type GPIOIBE = crate::Reg<u32, _GPIOIBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOIBE;
#[doc = "`read()` method returns [gpioibe::R](gpioibe::R) reader structure"]
impl crate::Readable for GPIOIBE {}
#[doc = "`write(|w| ..)` method takes [gpioibe::W](gpioibe::W) writer structure"]
impl crate::Writable for GPIOIBE {}
#[doc = "GPIO interrupt on both edges register"]
pub mod gpioibe;
#[doc = "GPIO interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioiev](gpioiev) module"]
pub type GPIOIEV = crate::Reg<u32, _GPIOIEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOIEV;
#[doc = "`read()` method returns [gpioiev::R](gpioiev::R) reader structure"]
impl crate::Readable for GPIOIEV {}
#[doc = "`write(|w| ..)` method takes [gpioiev::W](gpioiev::W) writer structure"]
impl crate::Writable for GPIOIEV {}
#[doc = "GPIO interrupt event register"]
pub mod gpioiev;
#[doc = "GPIO interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioim](gpioim) module"]
pub type GPIOIM = crate::Reg<u32, _GPIOIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOIM;
#[doc = "`read()` method returns [gpioim::R](gpioim::R) reader structure"]
impl crate::Readable for GPIOIM {}
#[doc = "`write(|w| ..)` method takes [gpioim::W](gpioim::W) writer structure"]
impl crate::Writable for GPIOIM {}
#[doc = "GPIO interrupt mask register"]
pub mod gpioim;
#[doc = "GPIO raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioris](gpioris) module"]
pub type GPIORIS = crate::Reg<u32, _GPIORIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIORIS;
#[doc = "`read()` method returns [gpioris::R](gpioris::R) reader structure"]
impl crate::Readable for GPIORIS {}
#[doc = "`write(|w| ..)` method takes [gpioris::W](gpioris::W) writer structure"]
impl crate::Writable for GPIORIS {}
#[doc = "GPIO raw interrupt status"]
pub mod gpioris;
#[doc = "GPIO masked interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiomis](gpiomis) module"]
pub type GPIOMIS = crate::Reg<u32, _GPIOMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOMIS;
#[doc = "`read()` method returns [gpiomis::R](gpiomis::R) reader structure"]
impl crate::Readable for GPIOMIS {}
#[doc = "`write(|w| ..)` method takes [gpiomis::W](gpiomis::W) writer structure"]
impl crate::Writable for GPIOMIS {}
#[doc = "GPIO masked interrupt status"]
pub mod gpiomis;
#[doc = "GPIO interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioicr](gpioicr) module"]
pub type GPIOICR = crate::Reg<u32, _GPIOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOICR;
#[doc = "`read()` method returns [gpioicr::R](gpioicr::R) reader structure"]
impl crate::Readable for GPIOICR {}
#[doc = "`write(|w| ..)` method takes [gpioicr::W](gpioicr::W) writer structure"]
impl crate::Writable for GPIOICR {}
#[doc = "GPIO interrupt clear register"]
pub mod gpioicr;
#[doc = "GPIOAFSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioafsel](gpioafsel) module"]
pub type GPIOAFSEL = crate::Reg<u32, _GPIOAFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOAFSEL;
#[doc = "`read()` method returns [gpioafsel::R](gpioafsel::R) reader structure"]
impl crate::Readable for GPIOAFSEL {}
#[doc = "`write(|w| ..)` method takes [gpioafsel::W](gpioafsel::W) writer structure"]
impl crate::Writable for GPIOAFSEL {}
#[doc = "GPIOAFSEL"]
pub mod gpioafsel;
#[doc = "GPIODR2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodr2r](gpiodr2r) module"]
pub type GPIODR2R = crate::Reg<u32, _GPIODR2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODR2R;
#[doc = "`read()` method returns [gpiodr2r::R](gpiodr2r::R) reader structure"]
impl crate::Readable for GPIODR2R {}
#[doc = "`write(|w| ..)` method takes [gpiodr2r::W](gpiodr2r::W) writer structure"]
impl crate::Writable for GPIODR2R {}
#[doc = "GPIODR2R"]
pub mod gpiodr2r;
#[doc = "GPIODR4R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodr4r](gpiodr4r) module"]
pub type GPIODR4R = crate::Reg<u32, _GPIODR4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODR4R;
#[doc = "`read()` method returns [gpiodr4r::R](gpiodr4r::R) reader structure"]
impl crate::Readable for GPIODR4R {}
#[doc = "`write(|w| ..)` method takes [gpiodr4r::W](gpiodr4r::W) writer structure"]
impl crate::Writable for GPIODR4R {}
#[doc = "GPIODR4R"]
pub mod gpiodr4r;
#[doc = "GPIODR8R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodr8r](gpiodr8r) module"]
pub type GPIODR8R = crate::Reg<u32, _GPIODR8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODR8R;
#[doc = "`read()` method returns [gpiodr8r::R](gpiodr8r::R) reader structure"]
impl crate::Readable for GPIODR8R {}
#[doc = "`write(|w| ..)` method takes [gpiodr8r::W](gpiodr8r::W) writer structure"]
impl crate::Writable for GPIODR8R {}
#[doc = "GPIODR8R"]
pub mod gpiodr8r;
#[doc = "GPIOODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioodr](gpioodr) module"]
pub type GPIOODR = crate::Reg<u32, _GPIOODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOODR;
#[doc = "`read()` method returns [gpioodr::R](gpioodr::R) reader structure"]
impl crate::Readable for GPIOODR {}
#[doc = "`write(|w| ..)` method takes [gpioodr::W](gpioodr::W) writer structure"]
impl crate::Writable for GPIOODR {}
#[doc = "GPIOODR"]
pub mod gpioodr;
#[doc = "GPIOPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopur](gpiopur) module"]
pub type GPIOPUR = crate::Reg<u32, _GPIOPUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPUR;
#[doc = "`read()` method returns [gpiopur::R](gpiopur::R) reader structure"]
impl crate::Readable for GPIOPUR {}
#[doc = "`write(|w| ..)` method takes [gpiopur::W](gpiopur::W) writer structure"]
impl crate::Writable for GPIOPUR {}
#[doc = "GPIOPUR"]
pub mod gpiopur;
#[doc = "GPIOPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopdr](gpiopdr) module"]
pub type GPIOPDR = crate::Reg<u32, _GPIOPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPDR;
#[doc = "`read()` method returns [gpiopdr::R](gpiopdr::R) reader structure"]
impl crate::Readable for GPIOPDR {}
#[doc = "`write(|w| ..)` method takes [gpiopdr::W](gpiopdr::W) writer structure"]
impl crate::Writable for GPIOPDR {}
#[doc = "GPIOPDR"]
pub mod gpiopdr;
#[doc = "GPIOSLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioslr](gpioslr) module"]
pub type GPIOSLR = crate::Reg<u32, _GPIOSLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSLR;
#[doc = "`read()` method returns [gpioslr::R](gpioslr::R) reader structure"]
impl crate::Readable for GPIOSLR {}
#[doc = "`write(|w| ..)` method takes [gpioslr::W](gpioslr::W) writer structure"]
impl crate::Writable for GPIOSLR {}
#[doc = "GPIOSLR"]
pub mod gpioslr;
#[doc = "GPIODEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioden](gpioden) module"]
pub type GPIODEN = crate::Reg<u32, _GPIODEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODEN;
#[doc = "`read()` method returns [gpioden::R](gpioden::R) reader structure"]
impl crate::Readable for GPIODEN {}
#[doc = "`write(|w| ..)` method takes [gpioden::W](gpioden::W) writer structure"]
impl crate::Writable for GPIODEN {}
#[doc = "GPIODEN"]
pub mod gpioden;
#[doc = "GPIOLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiolock](gpiolock) module"]
pub type GPIOLOCK = crate::Reg<u32, _GPIOLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOLOCK;
#[doc = "`read()` method returns [gpiolock::R](gpiolock::R) reader structure"]
impl crate::Readable for GPIOLOCK {}
#[doc = "`write(|w| ..)` method takes [gpiolock::W](gpiolock::W) writer structure"]
impl crate::Writable for GPIOLOCK {}
#[doc = "GPIOLOCK"]
pub mod gpiolock;
#[doc = "GPIOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiocr](gpiocr) module"]
pub type GPIOCR = crate::Reg<u32, _GPIOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOCR;
#[doc = "`read()` method returns [gpiocr::R](gpiocr::R) reader structure"]
impl crate::Readable for GPIOCR {}
#[doc = "`write(|w| ..)` method takes [gpiocr::W](gpiocr::W) writer structure"]
impl crate::Writable for GPIOCR {}
#[doc = "GPIOCR"]
pub mod gpiocr;
#[doc = "GPIOAMSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioamsel](gpioamsel) module"]
pub type GPIOAMSEL = crate::Reg<u32, _GPIOAMSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOAMSEL;
#[doc = "`read()` method returns [gpioamsel::R](gpioamsel::R) reader structure"]
impl crate::Readable for GPIOAMSEL {}
#[doc = "`write(|w| ..)` method takes [gpioamsel::W](gpioamsel::W) writer structure"]
impl crate::Writable for GPIOAMSEL {}
#[doc = "GPIOAMSEL"]
pub mod gpioamsel;
#[doc = "GPIOPCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopctl](gpiopctl) module"]
pub type GPIOPCTL = crate::Reg<u32, _GPIOPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCTL;
#[doc = "`read()` method returns [gpiopctl::R](gpiopctl::R) reader structure"]
impl crate::Readable for GPIOPCTL {}
#[doc = "`write(|w| ..)` method takes [gpiopctl::W](gpiopctl::W) writer structure"]
impl crate::Writable for GPIOPCTL {}
#[doc = "GPIOPCTL"]
pub mod gpiopctl;
#[doc = "GPIOADCCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioadcctl](gpioadcctl) module"]
pub type GPIOADCCTL = crate::Reg<u32, _GPIOADCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOADCCTL;
#[doc = "`read()` method returns [gpioadcctl::R](gpioadcctl::R) reader structure"]
impl crate::Readable for GPIOADCCTL {}
#[doc = "`write(|w| ..)` method takes [gpioadcctl::W](gpioadcctl::W) writer structure"]
impl crate::Writable for GPIOADCCTL {}
#[doc = "GPIOADCCTL"]
pub mod gpioadcctl;
#[doc = "GPIODMACTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodmactl](gpiodmactl) module"]
pub type GPIODMACTL = crate::Reg<u32, _GPIODMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODMACTL;
#[doc = "`read()` method returns [gpiodmactl::R](gpiodmactl::R) reader structure"]
impl crate::Readable for GPIODMACTL {}
#[doc = "`write(|w| ..)` method takes [gpiodmactl::W](gpiodmactl::W) writer structure"]
impl crate::Writable for GPIODMACTL {}
#[doc = "GPIODMACTL"]
pub mod gpiodmactl;
#[doc = "GPIOSI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiosi](gpiosi) module"]
pub type GPIOSI = crate::Reg<u32, _GPIOSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOSI;
#[doc = "`read()` method returns [gpiosi::R](gpiosi::R) reader structure"]
impl crate::Readable for GPIOSI {}
#[doc = "`write(|w| ..)` method takes [gpiosi::W](gpiosi::W) writer structure"]
impl crate::Writable for GPIOSI {}
#[doc = "GPIOSI"]
pub mod gpiosi;
#[doc = "GPIOPERIPHID4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid4](gpioperiphid4) module"]
pub type GPIOPERIPHID4 = crate::Reg<u32, _GPIOPERIPHID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID4;
#[doc = "`read()` method returns [gpioperiphid4::R](gpioperiphid4::R) reader structure"]
impl crate::Readable for GPIOPERIPHID4 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid4::W](gpioperiphid4::W) writer structure"]
impl crate::Writable for GPIOPERIPHID4 {}
#[doc = "GPIOPERIPHID4"]
pub mod gpioperiphid4;
#[doc = "GPIOPERIPHID5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid5](gpioperiphid5) module"]
pub type GPIOPERIPHID5 = crate::Reg<u32, _GPIOPERIPHID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID5;
#[doc = "`read()` method returns [gpioperiphid5::R](gpioperiphid5::R) reader structure"]
impl crate::Readable for GPIOPERIPHID5 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid5::W](gpioperiphid5::W) writer structure"]
impl crate::Writable for GPIOPERIPHID5 {}
#[doc = "GPIOPERIPHID5"]
pub mod gpioperiphid5;
#[doc = "GPIOPERIPHID6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid6](gpioperiphid6) module"]
pub type GPIOPERIPHID6 = crate::Reg<u32, _GPIOPERIPHID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID6;
#[doc = "`read()` method returns [gpioperiphid6::R](gpioperiphid6::R) reader structure"]
impl crate::Readable for GPIOPERIPHID6 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid6::W](gpioperiphid6::W) writer structure"]
impl crate::Writable for GPIOPERIPHID6 {}
#[doc = "GPIOPERIPHID6"]
pub mod gpioperiphid6;
#[doc = "GPIOPERIPHID7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid7](gpioperiphid7) module"]
pub type GPIOPERIPHID7 = crate::Reg<u32, _GPIOPERIPHID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID7;
#[doc = "`read()` method returns [gpioperiphid7::R](gpioperiphid7::R) reader structure"]
impl crate::Readable for GPIOPERIPHID7 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid7::W](gpioperiphid7::W) writer structure"]
impl crate::Writable for GPIOPERIPHID7 {}
#[doc = "GPIOPERIPHID7"]
pub mod gpioperiphid7;
#[doc = "GPIOPERIPHID0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid0](gpioperiphid0) module"]
pub type GPIOPERIPHID0 = crate::Reg<u32, _GPIOPERIPHID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID0;
#[doc = "`read()` method returns [gpioperiphid0::R](gpioperiphid0::R) reader structure"]
impl crate::Readable for GPIOPERIPHID0 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid0::W](gpioperiphid0::W) writer structure"]
impl crate::Writable for GPIOPERIPHID0 {}
#[doc = "GPIOPERIPHID0"]
pub mod gpioperiphid0;
#[doc = "GPIOPERIPHID1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid1](gpioperiphid1) module"]
pub type GPIOPERIPHID1 = crate::Reg<u32, _GPIOPERIPHID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID1;
#[doc = "`read()` method returns [gpioperiphid1::R](gpioperiphid1::R) reader structure"]
impl crate::Readable for GPIOPERIPHID1 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid1::W](gpioperiphid1::W) writer structure"]
impl crate::Writable for GPIOPERIPHID1 {}
#[doc = "GPIOPERIPHID1"]
pub mod gpioperiphid1;
#[doc = "GPIOPERIPHID2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid2](gpioperiphid2) module"]
pub type GPIOPERIPHID2 = crate::Reg<u32, _GPIOPERIPHID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID2;
#[doc = "`read()` method returns [gpioperiphid2::R](gpioperiphid2::R) reader structure"]
impl crate::Readable for GPIOPERIPHID2 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid2::W](gpioperiphid2::W) writer structure"]
impl crate::Writable for GPIOPERIPHID2 {}
#[doc = "GPIOPERIPHID2"]
pub mod gpioperiphid2;
#[doc = "GPIOPERIPHID3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioperiphid3](gpioperiphid3) module"]
pub type GPIOPERIPHID3 = crate::Reg<u32, _GPIOPERIPHID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPERIPHID3;
#[doc = "`read()` method returns [gpioperiphid3::R](gpioperiphid3::R) reader structure"]
impl crate::Readable for GPIOPERIPHID3 {}
#[doc = "`write(|w| ..)` method takes [gpioperiphid3::W](gpioperiphid3::W) writer structure"]
impl crate::Writable for GPIOPERIPHID3 {}
#[doc = "GPIOPERIPHID3"]
pub mod gpioperiphid3;
#[doc = "GPIOPCELLID0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopcellid0](gpiopcellid0) module"]
pub type GPIOPCELLID0 = crate::Reg<u32, _GPIOPCELLID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCELLID0;
#[doc = "`read()` method returns [gpiopcellid0::R](gpiopcellid0::R) reader structure"]
impl crate::Readable for GPIOPCELLID0 {}
#[doc = "`write(|w| ..)` method takes [gpiopcellid0::W](gpiopcellid0::W) writer structure"]
impl crate::Writable for GPIOPCELLID0 {}
#[doc = "GPIOPCELLID0"]
pub mod gpiopcellid0;
#[doc = "GPIOPCELLID1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopcellid1](gpiopcellid1) module"]
pub type GPIOPCELLID1 = crate::Reg<u32, _GPIOPCELLID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCELLID1;
#[doc = "`read()` method returns [gpiopcellid1::R](gpiopcellid1::R) reader structure"]
impl crate::Readable for GPIOPCELLID1 {}
#[doc = "`write(|w| ..)` method takes [gpiopcellid1::W](gpiopcellid1::W) writer structure"]
impl crate::Writable for GPIOPCELLID1 {}
#[doc = "GPIOPCELLID1"]
pub mod gpiopcellid1;
#[doc = "GPIOPCELLID2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopcellid2](gpiopcellid2) module"]
pub type GPIOPCELLID2 = crate::Reg<u32, _GPIOPCELLID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCELLID2;
#[doc = "`read()` method returns [gpiopcellid2::R](gpiopcellid2::R) reader structure"]
impl crate::Readable for GPIOPCELLID2 {}
#[doc = "`write(|w| ..)` method takes [gpiopcellid2::W](gpiopcellid2::W) writer structure"]
impl crate::Writable for GPIOPCELLID2 {}
#[doc = "GPIOPCELLID2"]
pub mod gpiopcellid2;
#[doc = "GPIOPCELLID3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopcellid3](gpiopcellid3) module"]
pub type GPIOPCELLID3 = crate::Reg<u32, _GPIOPCELLID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPCELLID3;
#[doc = "`read()` method returns [gpiopcellid3::R](gpiopcellid3::R) reader structure"]
impl crate::Readable for GPIOPCELLID3 {}
#[doc = "`write(|w| ..)` method takes [gpiopcellid3::W](gpiopcellid3::W) writer structure"]
impl crate::Writable for GPIOPCELLID3 {}
#[doc = "GPIOPCELLID3"]
pub mod gpiopcellid3;
