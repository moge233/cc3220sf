#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DR"]
    pub dr: DR,
    #[doc = "0x04 - RSR_ECR"]
    pub rsr_ecr: RSR_ECR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - FR"]
    pub fr: FR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - ILPR"]
    pub ilpr: ILPR,
    #[doc = "0x24 - IBRD"]
    pub ibrd: IBRD,
    #[doc = "0x28 - FBRD"]
    pub fbrd: FBRD,
    #[doc = "0x2c - LCRH"]
    pub lcrh: LCRH,
    #[doc = "0x30 - CTL"]
    pub ctl: CTL,
    #[doc = "0x34 - IFLS"]
    pub ifls: IFLS,
    #[doc = "0x38 - IM"]
    pub im: IM,
    #[doc = "0x3c - RIS"]
    pub ris: RIS,
    #[doc = "0x40 - MIS"]
    pub mis: MIS,
    #[doc = "0x44 - ICR"]
    pub icr: ICR,
    #[doc = "0x48 - DMACTL"]
    pub dmactl: DMACTL,
    _reserved14: [u8; 68usize],
    #[doc = "0x90 - LCTL"]
    pub lctl: LCTL,
    #[doc = "0x94 - LSS"]
    pub lss: LSS,
    #[doc = "0x98 - LTIM"]
    pub ltim: LTIM,
    _reserved17: [u8; 8usize],
    #[doc = "0xa4 - UART_9BITADDR"]
    pub uart_9bitaddr: UART_9BITADDR,
    #[doc = "0xa8 - UART_9BITAMASK"]
    pub uart_9bitamask: UART_9BITAMASK,
    _reserved19: [u8; 3860usize],
    #[doc = "0xfc0 - PP"]
    pub pp: PP,
    _reserved20: [u8; 4usize],
    #[doc = "0xfc8 - CC"]
    pub cc: CC,
}
#[doc = "DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "DR"]
pub mod dr;
#[doc = "RSR_ECR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr_ecr](rsr_ecr) module"]
pub type RSR_ECR = crate::Reg<u32, _RSR_ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR_ECR;
#[doc = "`read()` method returns [rsr_ecr::R](rsr_ecr::R) reader structure"]
impl crate::Readable for RSR_ECR {}
#[doc = "`write(|w| ..)` method takes [rsr_ecr::W](rsr_ecr::W) writer structure"]
impl crate::Writable for RSR_ECR {}
#[doc = "RSR_ECR"]
pub mod rsr_ecr;
#[doc = "FR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "FR"]
pub mod fr;
#[doc = "ILPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilpr](ilpr) module"]
pub type ILPR = crate::Reg<u32, _ILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILPR;
#[doc = "`read()` method returns [ilpr::R](ilpr::R) reader structure"]
impl crate::Readable for ILPR {}
#[doc = "`write(|w| ..)` method takes [ilpr::W](ilpr::W) writer structure"]
impl crate::Writable for ILPR {}
#[doc = "ILPR"]
pub mod ilpr;
#[doc = "IBRD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](ibrd) module"]
pub type IBRD = crate::Reg<u32, _IBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBRD;
#[doc = "`read()` method returns [ibrd::R](ibrd::R) reader structure"]
impl crate::Readable for IBRD {}
#[doc = "`write(|w| ..)` method takes [ibrd::W](ibrd::W) writer structure"]
impl crate::Writable for IBRD {}
#[doc = "IBRD"]
pub mod ibrd;
#[doc = "FBRD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](fbrd) module"]
pub type FBRD = crate::Reg<u32, _FBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBRD;
#[doc = "`read()` method returns [fbrd::R](fbrd::R) reader structure"]
impl crate::Readable for FBRD {}
#[doc = "`write(|w| ..)` method takes [fbrd::W](fbrd::W) writer structure"]
impl crate::Writable for FBRD {}
#[doc = "FBRD"]
pub mod fbrd;
#[doc = "LCRH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](lcrh) module"]
pub type LCRH = crate::Reg<u32, _LCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCRH;
#[doc = "`read()` method returns [lcrh::R](lcrh::R) reader structure"]
impl crate::Readable for LCRH {}
#[doc = "`write(|w| ..)` method takes [lcrh::W](lcrh::W) writer structure"]
impl crate::Writable for LCRH {}
#[doc = "LCRH"]
pub mod lcrh;
#[doc = "CTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "CTL"]
pub mod ctl;
#[doc = "IFLS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](ifls) module"]
pub type IFLS = crate::Reg<u32, _IFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLS;
#[doc = "`read()` method returns [ifls::R](ifls::R) reader structure"]
impl crate::Readable for IFLS {}
#[doc = "`write(|w| ..)` method takes [ifls::W](ifls::W) writer structure"]
impl crate::Writable for IFLS {}
#[doc = "IFLS"]
pub mod ifls;
#[doc = "IM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "IM"]
pub mod im;
#[doc = "RIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "RIS"]
pub mod ris;
#[doc = "MIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "MIS"]
pub mod mis;
#[doc = "ICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "ICR"]
pub mod icr;
#[doc = "DMACTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "DMACTL"]
pub mod dmactl;
#[doc = "LCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lctl](lctl) module"]
pub type LCTL = crate::Reg<u32, _LCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCTL;
#[doc = "`read()` method returns [lctl::R](lctl::R) reader structure"]
impl crate::Readable for LCTL {}
#[doc = "`write(|w| ..)` method takes [lctl::W](lctl::W) writer structure"]
impl crate::Writable for LCTL {}
#[doc = "LCTL"]
pub mod lctl;
#[doc = "LSS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lss](lss) module"]
pub type LSS = crate::Reg<u32, _LSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSS;
#[doc = "`read()` method returns [lss::R](lss::R) reader structure"]
impl crate::Readable for LSS {}
#[doc = "`write(|w| ..)` method takes [lss::W](lss::W) writer structure"]
impl crate::Writable for LSS {}
#[doc = "LSS"]
pub mod lss;
#[doc = "LTIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltim](ltim) module"]
pub type LTIM = crate::Reg<u32, _LTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTIM;
#[doc = "`read()` method returns [ltim::R](ltim::R) reader structure"]
impl crate::Readable for LTIM {}
#[doc = "`write(|w| ..)` method takes [ltim::W](ltim::W) writer structure"]
impl crate::Writable for LTIM {}
#[doc = "LTIM"]
pub mod ltim;
#[doc = "UART_9BITADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_9bitaddr](uart_9bitaddr) module"]
pub type UART_9BITADDR = crate::Reg<u32, _UART_9BITADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_9BITADDR;
#[doc = "`read()` method returns [uart_9bitaddr::R](uart_9bitaddr::R) reader structure"]
impl crate::Readable for UART_9BITADDR {}
#[doc = "`write(|w| ..)` method takes [uart_9bitaddr::W](uart_9bitaddr::W) writer structure"]
impl crate::Writable for UART_9BITADDR {}
#[doc = "UART_9BITADDR"]
pub mod uart_9bitaddr;
#[doc = "UART_9BITAMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_9bitamask](uart_9bitamask) module"]
pub type UART_9BITAMASK = crate::Reg<u32, _UART_9BITAMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_9BITAMASK;
#[doc = "`read()` method returns [uart_9bitamask::R](uart_9bitamask::R) reader structure"]
impl crate::Readable for UART_9BITAMASK {}
#[doc = "`write(|w| ..)` method takes [uart_9bitamask::W](uart_9bitamask::W) writer structure"]
impl crate::Writable for UART_9BITAMASK {}
#[doc = "UART_9BITAMASK"]
pub mod uart_9bitamask;
#[doc = "PP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "`write(|w| ..)` method takes [pp::W](pp::W) writer structure"]
impl crate::Writable for PP {}
#[doc = "PP"]
pub mod pp;
#[doc = "CC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "CC"]
pub mod cc;
