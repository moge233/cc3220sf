#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - STAT"]
    pub stat: STAT,
    #[doc = "0x04 - CFG"]
    pub cfg: CFG,
    #[doc = "0x08 - CTLBASE"]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - ALTBASE"]
    pub altbase: ALTBASE,
    #[doc = "0x10 - WAITSTAT"]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - SWREQ"]
    pub swreq: SWREQ,
    #[doc = "0x18 - USEBURSTSET"]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - USEBURSTCLR"]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - REQMASKSET"]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - REQMASKCLR"]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - ENASET"]
    pub enaset: ENASET,
    #[doc = "0x2c - ENACLR"]
    pub enaclr: ENACLR,
    #[doc = "0x30 - ALTSET"]
    pub altset: ALTSET,
    #[doc = "0x34 - ALTCLR"]
    pub altclr: ALTCLR,
    #[doc = "0x38 - PRIOSET"]
    pub prioset: PRIOSET,
    #[doc = "0x3c - PRIOCLR"]
    pub prioclr: PRIOCLR,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - ERRCLR"]
    pub errclr: ERRCLR,
    _reserved17: [u8; 1200usize],
    #[doc = "0x500 - CHASGN"]
    pub chasgn: CHASGN,
    #[doc = "0x504 - CHIS"]
    pub chis: CHIS,
    _reserved19: [u8; 8usize],
    #[doc = "0x510 - CHMAP0"]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - CHMAP1"]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - CHMAP2"]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - CHMAP3"]
    pub chmap3: CHMAP3,
    _reserved23: [u8; 2704usize],
    #[doc = "0xfb0 - PV"]
    pub pv: PV,
}
#[doc = "STAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "STAT"]
pub mod stat;
#[doc = "CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "CFG"]
pub mod cfg;
#[doc = "CTLBASE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlbase](ctlbase) module"]
pub type CTLBASE = crate::Reg<u32, _CTLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLBASE;
#[doc = "`read()` method returns [ctlbase::R](ctlbase::R) reader structure"]
impl crate::Readable for CTLBASE {}
#[doc = "`write(|w| ..)` method takes [ctlbase::W](ctlbase::W) writer structure"]
impl crate::Writable for CTLBASE {}
#[doc = "CTLBASE"]
pub mod ctlbase;
#[doc = "ALTBASE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altbase](altbase) module"]
pub type ALTBASE = crate::Reg<u32, _ALTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTBASE;
#[doc = "`read()` method returns [altbase::R](altbase::R) reader structure"]
impl crate::Readable for ALTBASE {}
#[doc = "`write(|w| ..)` method takes [altbase::W](altbase::W) writer structure"]
impl crate::Writable for ALTBASE {}
#[doc = "ALTBASE"]
pub mod altbase;
#[doc = "WAITSTAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitstat](waitstat) module"]
pub type WAITSTAT = crate::Reg<u32, _WAITSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAITSTAT;
#[doc = "`read()` method returns [waitstat::R](waitstat::R) reader structure"]
impl crate::Readable for WAITSTAT {}
#[doc = "`write(|w| ..)` method takes [waitstat::W](waitstat::W) writer structure"]
impl crate::Writable for WAITSTAT {}
#[doc = "WAITSTAT"]
pub mod waitstat;
#[doc = "SWREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](swreq) module"]
pub type SWREQ = crate::Reg<u32, _SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWREQ;
#[doc = "`read()` method returns [swreq::R](swreq::R) reader structure"]
impl crate::Readable for SWREQ {}
#[doc = "`write(|w| ..)` method takes [swreq::W](swreq::W) writer structure"]
impl crate::Writable for SWREQ {}
#[doc = "SWREQ"]
pub mod swreq;
#[doc = "USEBURSTSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstset](useburstset) module"]
pub type USEBURSTSET = crate::Reg<u32, _USEBURSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTSET;
#[doc = "`read()` method returns [useburstset::R](useburstset::R) reader structure"]
impl crate::Readable for USEBURSTSET {}
#[doc = "`write(|w| ..)` method takes [useburstset::W](useburstset::W) writer structure"]
impl crate::Writable for USEBURSTSET {}
#[doc = "USEBURSTSET"]
pub mod useburstset;
#[doc = "USEBURSTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstclr](useburstclr) module"]
pub type USEBURSTCLR = crate::Reg<u32, _USEBURSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTCLR;
#[doc = "`read()` method returns [useburstclr::R](useburstclr::R) reader structure"]
impl crate::Readable for USEBURSTCLR {}
#[doc = "`write(|w| ..)` method takes [useburstclr::W](useburstclr::W) writer structure"]
impl crate::Writable for USEBURSTCLR {}
#[doc = "USEBURSTCLR"]
pub mod useburstclr;
#[doc = "REQMASKSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskset](reqmaskset) module"]
pub type REQMASKSET = crate::Reg<u32, _REQMASKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKSET;
#[doc = "`read()` method returns [reqmaskset::R](reqmaskset::R) reader structure"]
impl crate::Readable for REQMASKSET {}
#[doc = "`write(|w| ..)` method takes [reqmaskset::W](reqmaskset::W) writer structure"]
impl crate::Writable for REQMASKSET {}
#[doc = "REQMASKSET"]
pub mod reqmaskset;
#[doc = "REQMASKCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskclr](reqmaskclr) module"]
pub type REQMASKCLR = crate::Reg<u32, _REQMASKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKCLR;
#[doc = "`read()` method returns [reqmaskclr::R](reqmaskclr::R) reader structure"]
impl crate::Readable for REQMASKCLR {}
#[doc = "`write(|w| ..)` method takes [reqmaskclr::W](reqmaskclr::W) writer structure"]
impl crate::Writable for REQMASKCLR {}
#[doc = "REQMASKCLR"]
pub mod reqmaskclr;
#[doc = "ENASET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaset](enaset) module"]
pub type ENASET = crate::Reg<u32, _ENASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENASET;
#[doc = "`read()` method returns [enaset::R](enaset::R) reader structure"]
impl crate::Readable for ENASET {}
#[doc = "`write(|w| ..)` method takes [enaset::W](enaset::W) writer structure"]
impl crate::Writable for ENASET {}
#[doc = "ENASET"]
pub mod enaset;
#[doc = "ENACLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaclr](enaclr) module"]
pub type ENACLR = crate::Reg<u32, _ENACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENACLR;
#[doc = "`read()` method returns [enaclr::R](enaclr::R) reader structure"]
impl crate::Readable for ENACLR {}
#[doc = "`write(|w| ..)` method takes [enaclr::W](enaclr::W) writer structure"]
impl crate::Writable for ENACLR {}
#[doc = "ENACLR"]
pub mod enaclr;
#[doc = "ALTSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altset](altset) module"]
pub type ALTSET = crate::Reg<u32, _ALTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTSET;
#[doc = "`read()` method returns [altset::R](altset::R) reader structure"]
impl crate::Readable for ALTSET {}
#[doc = "`write(|w| ..)` method takes [altset::W](altset::W) writer structure"]
impl crate::Writable for ALTSET {}
#[doc = "ALTSET"]
pub mod altset;
#[doc = "ALTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altclr](altclr) module"]
pub type ALTCLR = crate::Reg<u32, _ALTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCLR;
#[doc = "`read()` method returns [altclr::R](altclr::R) reader structure"]
impl crate::Readable for ALTCLR {}
#[doc = "`write(|w| ..)` method takes [altclr::W](altclr::W) writer structure"]
impl crate::Writable for ALTCLR {}
#[doc = "ALTCLR"]
pub mod altclr;
#[doc = "PRIOSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioset](prioset) module"]
pub type PRIOSET = crate::Reg<u32, _PRIOSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOSET;
#[doc = "`read()` method returns [prioset::R](prioset::R) reader structure"]
impl crate::Readable for PRIOSET {}
#[doc = "`write(|w| ..)` method takes [prioset::W](prioset::W) writer structure"]
impl crate::Writable for PRIOSET {}
#[doc = "PRIOSET"]
pub mod prioset;
#[doc = "PRIOCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioclr](prioclr) module"]
pub type PRIOCLR = crate::Reg<u32, _PRIOCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOCLR;
#[doc = "`read()` method returns [prioclr::R](prioclr::R) reader structure"]
impl crate::Readable for PRIOCLR {}
#[doc = "`write(|w| ..)` method takes [prioclr::W](prioclr::W) writer structure"]
impl crate::Writable for PRIOCLR {}
#[doc = "PRIOCLR"]
pub mod prioclr;
#[doc = "ERRCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errclr](errclr) module"]
pub type ERRCLR = crate::Reg<u32, _ERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRCLR;
#[doc = "`read()` method returns [errclr::R](errclr::R) reader structure"]
impl crate::Readable for ERRCLR {}
#[doc = "`write(|w| ..)` method takes [errclr::W](errclr::W) writer structure"]
impl crate::Writable for ERRCLR {}
#[doc = "ERRCLR"]
pub mod errclr;
#[doc = "CHASGN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chasgn](chasgn) module"]
pub type CHASGN = crate::Reg<u32, _CHASGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHASGN;
#[doc = "`read()` method returns [chasgn::R](chasgn::R) reader structure"]
impl crate::Readable for CHASGN {}
#[doc = "`write(|w| ..)` method takes [chasgn::W](chasgn::W) writer structure"]
impl crate::Writable for CHASGN {}
#[doc = "CHASGN"]
pub mod chasgn;
#[doc = "CHIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chis](chis) module"]
pub type CHIS = crate::Reg<u32, _CHIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIS;
#[doc = "`read()` method returns [chis::R](chis::R) reader structure"]
impl crate::Readable for CHIS {}
#[doc = "`write(|w| ..)` method takes [chis::W](chis::W) writer structure"]
impl crate::Writable for CHIS {}
#[doc = "CHIS"]
pub mod chis;
#[doc = "CHMAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap0](chmap0) module"]
pub type CHMAP0 = crate::Reg<u32, _CHMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP0;
#[doc = "`read()` method returns [chmap0::R](chmap0::R) reader structure"]
impl crate::Readable for CHMAP0 {}
#[doc = "`write(|w| ..)` method takes [chmap0::W](chmap0::W) writer structure"]
impl crate::Writable for CHMAP0 {}
#[doc = "CHMAP0"]
pub mod chmap0;
#[doc = "CHMAP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap1](chmap1) module"]
pub type CHMAP1 = crate::Reg<u32, _CHMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP1;
#[doc = "`read()` method returns [chmap1::R](chmap1::R) reader structure"]
impl crate::Readable for CHMAP1 {}
#[doc = "`write(|w| ..)` method takes [chmap1::W](chmap1::W) writer structure"]
impl crate::Writable for CHMAP1 {}
#[doc = "CHMAP1"]
pub mod chmap1;
#[doc = "CHMAP2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap2](chmap2) module"]
pub type CHMAP2 = crate::Reg<u32, _CHMAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP2;
#[doc = "`read()` method returns [chmap2::R](chmap2::R) reader structure"]
impl crate::Readable for CHMAP2 {}
#[doc = "`write(|w| ..)` method takes [chmap2::W](chmap2::W) writer structure"]
impl crate::Writable for CHMAP2 {}
#[doc = "CHMAP2"]
pub mod chmap2;
#[doc = "CHMAP3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap3](chmap3) module"]
pub type CHMAP3 = crate::Reg<u32, _CHMAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP3;
#[doc = "`read()` method returns [chmap3::R](chmap3::R) reader structure"]
impl crate::Readable for CHMAP3 {}
#[doc = "`write(|w| ..)` method takes [chmap3::W](chmap3::W) writer structure"]
impl crate::Writable for CHMAP3 {}
#[doc = "CHMAP3"]
pub mod chmap3;
#[doc = "PV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pv](pv) module"]
pub type PV = crate::Reg<u32, _PV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PV;
#[doc = "`read()` method returns [pv::R](pv::R) reader structure"]
impl crate::Readable for PV {}
#[doc = "`write(|w| ..)` method takes [pv::W](pv::W) writer structure"]
impl crate::Writable for PV {}
#[doc = "PV"]
pub mod pv;
