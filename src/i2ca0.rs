#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSA"]
    pub msa: MSA,
    #[doc = "0x04 - MCS"]
    pub mcs: MCS,
    #[doc = "0x08 - MDR"]
    pub mdr: MDR,
    #[doc = "0x0c - MTPR"]
    pub mtpr: MTPR,
    #[doc = "0x10 - MIMR"]
    pub mimr: MIMR,
    #[doc = "0x14 - MRIS"]
    pub mris: MRIS,
    #[doc = "0x18 - MMIS"]
    pub mmis: MMIS,
    #[doc = "0x1c - MICR"]
    pub micr: MICR,
    #[doc = "0x20 - MCR"]
    pub mcr: MCR,
    #[doc = "0x24 - MCLKOCNT"]
    pub mclkocnt: MCLKOCNT,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - MBMON"]
    pub mbmon: MBMON,
    #[doc = "0x30 - MBLEN"]
    pub mblen: MBLEN,
    #[doc = "0x34 - MBCNT"]
    pub mbcnt: MBCNT,
    _reserved13: [u8; 1992usize],
    #[doc = "0x800 - SOAR"]
    pub soar: SOAR,
    #[doc = "0x804 - SCSR"]
    pub scsr: SCSR,
    #[doc = "0x808 - SDR"]
    pub sdr: SDR,
    #[doc = "0x80c - SIMR"]
    pub simr: SIMR,
    #[doc = "0x810 - SRIS"]
    pub sris: SRIS,
    #[doc = "0x814 - SMIS"]
    pub smis: SMIS,
    #[doc = "0x818 - SICR"]
    pub sicr: SICR,
    #[doc = "0x81c - SOAR2"]
    pub soar2: SOAR2,
    #[doc = "0x820 - SACKCTL"]
    pub sackctl: SACKCTL,
    _reserved22: [u8; 1756usize],
    #[doc = "0xf00 - FIFODATA"]
    pub fifodata: FIFODATA,
    #[doc = "0xf04 - FIFOCTL"]
    pub fifoctl: FIFOCTL,
    #[doc = "0xf08 - FIFOSTATUS"]
    pub fifostatus: FIFOSTATUS,
    _reserved25: [u8; 116usize],
    #[doc = "0xf80 - OBSMUXSEL0"]
    pub obsmuxsel0: OBSMUXSEL0,
    #[doc = "0xf84 - OBSMUXSEL1"]
    pub obsmuxsel1: OBSMUXSEL1,
    #[doc = "0xf88 - MUXROUTE"]
    pub muxroute: MUXROUTE,
    _reserved28: [u8; 36usize],
    #[doc = "0xfb0 - PV"]
    pub pv: PV,
    _reserved29: [u8; 12usize],
    #[doc = "0xfc0 - PP"]
    pub pp: PP,
    #[doc = "0xfc4 - PC"]
    pub pc: PC,
    #[doc = "0xfc8 - CC"]
    pub cc: CC,
}
#[doc = "MSA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msa](msa) module"]
pub type MSA = crate::Reg<u32, _MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSA;
#[doc = "`read()` method returns [msa::R](msa::R) reader structure"]
impl crate::Readable for MSA {}
#[doc = "`write(|w| ..)` method takes [msa::W](msa::W) writer structure"]
impl crate::Writable for MSA {}
#[doc = "MSA"]
pub mod msa;
#[doc = "MCS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcs](mcs) module"]
pub type MCS = crate::Reg<u32, _MCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCS;
#[doc = "`read()` method returns [mcs::R](mcs::R) reader structure"]
impl crate::Readable for MCS {}
#[doc = "`write(|w| ..)` method takes [mcs::W](mcs::W) writer structure"]
impl crate::Writable for MCS {}
#[doc = "MCS"]
pub mod mcs;
#[doc = "MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "MDR"]
pub mod mdr;
#[doc = "MTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpr](mtpr) module"]
pub type MTPR = crate::Reg<u32, _MTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTPR;
#[doc = "`read()` method returns [mtpr::R](mtpr::R) reader structure"]
impl crate::Readable for MTPR {}
#[doc = "`write(|w| ..)` method takes [mtpr::W](mtpr::W) writer structure"]
impl crate::Writable for MTPR {}
#[doc = "MTPR"]
pub mod mtpr;
#[doc = "MIMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mimr](mimr) module"]
pub type MIMR = crate::Reg<u32, _MIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIMR;
#[doc = "`read()` method returns [mimr::R](mimr::R) reader structure"]
impl crate::Readable for MIMR {}
#[doc = "`write(|w| ..)` method takes [mimr::W](mimr::W) writer structure"]
impl crate::Writable for MIMR {}
#[doc = "MIMR"]
pub mod mimr;
#[doc = "MRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mris](mris) module"]
pub type MRIS = crate::Reg<u32, _MRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRIS;
#[doc = "`read()` method returns [mris::R](mris::R) reader structure"]
impl crate::Readable for MRIS {}
#[doc = "`write(|w| ..)` method takes [mris::W](mris::W) writer structure"]
impl crate::Writable for MRIS {}
#[doc = "MRIS"]
pub mod mris;
#[doc = "MMIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmis](mmis) module"]
pub type MMIS = crate::Reg<u32, _MMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMIS;
#[doc = "`read()` method returns [mmis::R](mmis::R) reader structure"]
impl crate::Readable for MMIS {}
#[doc = "`write(|w| ..)` method takes [mmis::W](mmis::W) writer structure"]
impl crate::Writable for MMIS {}
#[doc = "MMIS"]
pub mod mmis;
#[doc = "MICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](micr) module"]
pub type MICR = crate::Reg<u32, _MICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICR;
#[doc = "`read()` method returns [micr::R](micr::R) reader structure"]
impl crate::Readable for MICR {}
#[doc = "`write(|w| ..)` method takes [micr::W](micr::W) writer structure"]
impl crate::Writable for MICR {}
#[doc = "MICR"]
pub mod micr;
#[doc = "MCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "MCR"]
pub mod mcr;
#[doc = "MCLKOCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkocnt](mclkocnt) module"]
pub type MCLKOCNT = crate::Reg<u32, _MCLKOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKOCNT;
#[doc = "`read()` method returns [mclkocnt::R](mclkocnt::R) reader structure"]
impl crate::Readable for MCLKOCNT {}
#[doc = "`write(|w| ..)` method takes [mclkocnt::W](mclkocnt::W) writer structure"]
impl crate::Writable for MCLKOCNT {}
#[doc = "MCLKOCNT"]
pub mod mclkocnt;
#[doc = "MBMON\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbmon](mbmon) module"]
pub type MBMON = crate::Reg<u32, _MBMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBMON;
#[doc = "`read()` method returns [mbmon::R](mbmon::R) reader structure"]
impl crate::Readable for MBMON {}
#[doc = "`write(|w| ..)` method takes [mbmon::W](mbmon::W) writer structure"]
impl crate::Writable for MBMON {}
#[doc = "MBMON"]
pub mod mbmon;
#[doc = "MBLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mblen](mblen) module"]
pub type MBLEN = crate::Reg<u32, _MBLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBLEN;
#[doc = "`read()` method returns [mblen::R](mblen::R) reader structure"]
impl crate::Readable for MBLEN {}
#[doc = "`write(|w| ..)` method takes [mblen::W](mblen::W) writer structure"]
impl crate::Writable for MBLEN {}
#[doc = "MBLEN"]
pub mod mblen;
#[doc = "MBCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbcnt](mbcnt) module"]
pub type MBCNT = crate::Reg<u32, _MBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBCNT;
#[doc = "`read()` method returns [mbcnt::R](mbcnt::R) reader structure"]
impl crate::Readable for MBCNT {}
#[doc = "`write(|w| ..)` method takes [mbcnt::W](mbcnt::W) writer structure"]
impl crate::Writable for MBCNT {}
#[doc = "MBCNT"]
pub mod mbcnt;
#[doc = "SOAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar](soar) module"]
pub type SOAR = crate::Reg<u32, _SOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR;
#[doc = "`read()` method returns [soar::R](soar::R) reader structure"]
impl crate::Readable for SOAR {}
#[doc = "`write(|w| ..)` method takes [soar::W](soar::W) writer structure"]
impl crate::Writable for SOAR {}
#[doc = "SOAR"]
pub mod soar;
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure"]
impl crate::Writable for SCSR {}
#[doc = "SCSR"]
pub mod scsr;
#[doc = "SDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdr](sdr) module"]
pub type SDR = crate::Reg<u32, _SDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDR;
#[doc = "`read()` method returns [sdr::R](sdr::R) reader structure"]
impl crate::Readable for SDR {}
#[doc = "`write(|w| ..)` method takes [sdr::W](sdr::W) writer structure"]
impl crate::Writable for SDR {}
#[doc = "SDR"]
pub mod sdr;
#[doc = "SIMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr](simr) module"]
pub type SIMR = crate::Reg<u32, _SIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIMR;
#[doc = "`read()` method returns [simr::R](simr::R) reader structure"]
impl crate::Readable for SIMR {}
#[doc = "`write(|w| ..)` method takes [simr::W](simr::W) writer structure"]
impl crate::Writable for SIMR {}
#[doc = "SIMR"]
pub mod simr;
#[doc = "SRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sris](sris) module"]
pub type SRIS = crate::Reg<u32, _SRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIS;
#[doc = "`read()` method returns [sris::R](sris::R) reader structure"]
impl crate::Readable for SRIS {}
#[doc = "`write(|w| ..)` method takes [sris::W](sris::W) writer structure"]
impl crate::Writable for SRIS {}
#[doc = "SRIS"]
pub mod sris;
#[doc = "SMIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](smis) module"]
pub type SMIS = crate::Reg<u32, _SMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMIS;
#[doc = "`read()` method returns [smis::R](smis::R) reader structure"]
impl crate::Readable for SMIS {}
#[doc = "`write(|w| ..)` method takes [smis::W](smis::W) writer structure"]
impl crate::Writable for SMIS {}
#[doc = "SMIS"]
pub mod smis;
#[doc = "SICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](sicr) module"]
pub type SICR = crate::Reg<u32, _SICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SICR;
#[doc = "`read()` method returns [sicr::R](sicr::R) reader structure"]
impl crate::Readable for SICR {}
#[doc = "`write(|w| ..)` method takes [sicr::W](sicr::W) writer structure"]
impl crate::Writable for SICR {}
#[doc = "SICR"]
pub mod sicr;
#[doc = "SOAR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar2](soar2) module"]
pub type SOAR2 = crate::Reg<u32, _SOAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR2;
#[doc = "`read()` method returns [soar2::R](soar2::R) reader structure"]
impl crate::Readable for SOAR2 {}
#[doc = "`write(|w| ..)` method takes [soar2::W](soar2::W) writer structure"]
impl crate::Writable for SOAR2 {}
#[doc = "SOAR2"]
pub mod soar2;
#[doc = "SACKCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sackctl](sackctl) module"]
pub type SACKCTL = crate::Reg<u32, _SACKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SACKCTL;
#[doc = "`read()` method returns [sackctl::R](sackctl::R) reader structure"]
impl crate::Readable for SACKCTL {}
#[doc = "`write(|w| ..)` method takes [sackctl::W](sackctl::W) writer structure"]
impl crate::Writable for SACKCTL {}
#[doc = "SACKCTL"]
pub mod sackctl;
#[doc = "FIFODATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifodata](fifodata) module"]
pub type FIFODATA = crate::Reg<u32, _FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFODATA;
#[doc = "`read()` method returns [fifodata::R](fifodata::R) reader structure"]
impl crate::Readable for FIFODATA {}
#[doc = "`write(|w| ..)` method takes [fifodata::W](fifodata::W) writer structure"]
impl crate::Writable for FIFODATA {}
#[doc = "FIFODATA"]
pub mod fifodata;
#[doc = "FIFOCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctl](fifoctl) module"]
pub type FIFOCTL = crate::Reg<u32, _FIFOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCTL;
#[doc = "`read()` method returns [fifoctl::R](fifoctl::R) reader structure"]
impl crate::Readable for FIFOCTL {}
#[doc = "`write(|w| ..)` method takes [fifoctl::W](fifoctl::W) writer structure"]
impl crate::Writable for FIFOCTL {}
#[doc = "FIFOCTL"]
pub mod fifoctl;
#[doc = "FIFOSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostatus](fifostatus) module"]
pub type FIFOSTATUS = crate::Reg<u32, _FIFOSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOSTATUS;
#[doc = "`read()` method returns [fifostatus::R](fifostatus::R) reader structure"]
impl crate::Readable for FIFOSTATUS {}
#[doc = "`write(|w| ..)` method takes [fifostatus::W](fifostatus::W) writer structure"]
impl crate::Writable for FIFOSTATUS {}
#[doc = "FIFOSTATUS"]
pub mod fifostatus;
#[doc = "OBSMUXSEL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obsmuxsel0](obsmuxsel0) module"]
pub type OBSMUXSEL0 = crate::Reg<u32, _OBSMUXSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSMUXSEL0;
#[doc = "`read()` method returns [obsmuxsel0::R](obsmuxsel0::R) reader structure"]
impl crate::Readable for OBSMUXSEL0 {}
#[doc = "`write(|w| ..)` method takes [obsmuxsel0::W](obsmuxsel0::W) writer structure"]
impl crate::Writable for OBSMUXSEL0 {}
#[doc = "OBSMUXSEL0"]
pub mod obsmuxsel0;
#[doc = "OBSMUXSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obsmuxsel1](obsmuxsel1) module"]
pub type OBSMUXSEL1 = crate::Reg<u32, _OBSMUXSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSMUXSEL1;
#[doc = "`read()` method returns [obsmuxsel1::R](obsmuxsel1::R) reader structure"]
impl crate::Readable for OBSMUXSEL1 {}
#[doc = "`write(|w| ..)` method takes [obsmuxsel1::W](obsmuxsel1::W) writer structure"]
impl crate::Writable for OBSMUXSEL1 {}
#[doc = "OBSMUXSEL1"]
pub mod obsmuxsel1;
#[doc = "MUXROUTE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxroute](muxroute) module"]
pub type MUXROUTE = crate::Reg<u32, _MUXROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUXROUTE;
#[doc = "`read()` method returns [muxroute::R](muxroute::R) reader structure"]
impl crate::Readable for MUXROUTE {}
#[doc = "`write(|w| ..)` method takes [muxroute::W](muxroute::W) writer structure"]
impl crate::Writable for MUXROUTE {}
#[doc = "MUXROUTE"]
pub mod muxroute;
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
#[doc = "PC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "PC"]
pub mod pc;
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
