#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2064usize],
    #[doc = "0x810 - SHA_IM"]
    pub sha_im: SHA_IM,
    #[doc = "0x814 - SHA_RIS"]
    pub sha_ris: SHA_RIS,
    #[doc = "0x818 - SHA_MIS"]
    pub sha_mis: SHA_MIS,
    #[doc = "0x81c - SHA_IC"]
    pub sha_ic: SHA_IC,
    #[doc = "0x820 - AES_IM"]
    pub aes_im: AES_IM,
    #[doc = "0x824 - AES_RIS"]
    pub aes_ris: AES_RIS,
    #[doc = "0x828 - AES_MIS"]
    pub aes_mis: AES_MIS,
    #[doc = "0x82c - AES_IC"]
    pub aes_ic: AES_IC,
    #[doc = "0x830 - DES_IM"]
    pub des_im: DES_IM,
    #[doc = "0x834 - DES_RIS"]
    pub des_ris: DES_RIS,
    #[doc = "0x838 - DES_MIS"]
    pub des_mis: DES_MIS,
    #[doc = "0x83c - DES_IC"]
    pub des_ic: DES_IC,
    _reserved12: [u8; 448usize],
    #[doc = "0xa00 - EIP_CGCFG"]
    pub eip_cgcfg: EIP_CGCFG,
    #[doc = "0xa04 - EIP_CGREQ"]
    pub eip_cgreq: EIP_CGREQ,
    _reserved14: [u8; 504usize],
    #[doc = "0xc00 - CRC_CTRL"]
    pub crc_ctrl: CRC_CTRL,
    _reserved15: [u8; 12usize],
    #[doc = "0xc10 - CRC_SEED"]
    pub crc_seed: CRC_SEED,
    #[doc = "0xc14 - CRC_DIN"]
    pub crc_din: CRC_DIN,
    #[doc = "0xc18 - CRC_RSLT_PP"]
    pub crc_rslt_pp: CRC_RSLT_PP,
    _reserved18: [u8; 740usize],
    #[doc = "0xf00 - RAND_KEY0"]
    pub rand_key0: RAND_KEY0,
    #[doc = "0xf04 - RAND_KEY1"]
    pub rand_key1: RAND_KEY1,
    #[doc = "0xf08 - RAND_KEY2"]
    pub rand_key2: RAND_KEY2,
    #[doc = "0xf0c - RAND_KEY3"]
    pub rand_key3: RAND_KEY3,
}
#[doc = "SHA_IM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_im](sha_im) module"]
pub type SHA_IM = crate::Reg<u32, _SHA_IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA_IM;
#[doc = "`read()` method returns [sha_im::R](sha_im::R) reader structure"]
impl crate::Readable for SHA_IM {}
#[doc = "`write(|w| ..)` method takes [sha_im::W](sha_im::W) writer structure"]
impl crate::Writable for SHA_IM {}
#[doc = "SHA_IM"]
pub mod sha_im;
#[doc = "SHA_RIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_ris](sha_ris) module"]
pub type SHA_RIS = crate::Reg<u32, _SHA_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA_RIS;
#[doc = "`read()` method returns [sha_ris::R](sha_ris::R) reader structure"]
impl crate::Readable for SHA_RIS {}
#[doc = "`write(|w| ..)` method takes [sha_ris::W](sha_ris::W) writer structure"]
impl crate::Writable for SHA_RIS {}
#[doc = "SHA_RIS"]
pub mod sha_ris;
#[doc = "SHA_MIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_mis](sha_mis) module"]
pub type SHA_MIS = crate::Reg<u32, _SHA_MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA_MIS;
#[doc = "`read()` method returns [sha_mis::R](sha_mis::R) reader structure"]
impl crate::Readable for SHA_MIS {}
#[doc = "`write(|w| ..)` method takes [sha_mis::W](sha_mis::W) writer structure"]
impl crate::Writable for SHA_MIS {}
#[doc = "SHA_MIS"]
pub mod sha_mis;
#[doc = "SHA_IC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_ic](sha_ic) module"]
pub type SHA_IC = crate::Reg<u32, _SHA_IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA_IC;
#[doc = "`read()` method returns [sha_ic::R](sha_ic::R) reader structure"]
impl crate::Readable for SHA_IC {}
#[doc = "`write(|w| ..)` method takes [sha_ic::W](sha_ic::W) writer structure"]
impl crate::Writable for SHA_IC {}
#[doc = "SHA_IC"]
pub mod sha_ic;
#[doc = "AES_IM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_im](aes_im) module"]
pub type AES_IM = crate::Reg<u32, _AES_IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IM;
#[doc = "`read()` method returns [aes_im::R](aes_im::R) reader structure"]
impl crate::Readable for AES_IM {}
#[doc = "`write(|w| ..)` method takes [aes_im::W](aes_im::W) writer structure"]
impl crate::Writable for AES_IM {}
#[doc = "AES_IM"]
pub mod aes_im;
#[doc = "AES_RIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ris](aes_ris) module"]
pub type AES_RIS = crate::Reg<u32, _AES_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_RIS;
#[doc = "`read()` method returns [aes_ris::R](aes_ris::R) reader structure"]
impl crate::Readable for AES_RIS {}
#[doc = "`write(|w| ..)` method takes [aes_ris::W](aes_ris::W) writer structure"]
impl crate::Writable for AES_RIS {}
#[doc = "AES_RIS"]
pub mod aes_ris;
#[doc = "AES_MIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_mis](aes_mis) module"]
pub type AES_MIS = crate::Reg<u32, _AES_MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_MIS;
#[doc = "`read()` method returns [aes_mis::R](aes_mis::R) reader structure"]
impl crate::Readable for AES_MIS {}
#[doc = "`write(|w| ..)` method takes [aes_mis::W](aes_mis::W) writer structure"]
impl crate::Writable for AES_MIS {}
#[doc = "AES_MIS"]
pub mod aes_mis;
#[doc = "AES_IC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ic](aes_ic) module"]
pub type AES_IC = crate::Reg<u32, _AES_IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IC;
#[doc = "`read()` method returns [aes_ic::R](aes_ic::R) reader structure"]
impl crate::Readable for AES_IC {}
#[doc = "`write(|w| ..)` method takes [aes_ic::W](aes_ic::W) writer structure"]
impl crate::Writable for AES_IC {}
#[doc = "AES_IC"]
pub mod aes_ic;
#[doc = "DES_IM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [des_im](des_im) module"]
pub type DES_IM = crate::Reg<u32, _DES_IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DES_IM;
#[doc = "`read()` method returns [des_im::R](des_im::R) reader structure"]
impl crate::Readable for DES_IM {}
#[doc = "`write(|w| ..)` method takes [des_im::W](des_im::W) writer structure"]
impl crate::Writable for DES_IM {}
#[doc = "DES_IM"]
pub mod des_im;
#[doc = "DES_RIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [des_ris](des_ris) module"]
pub type DES_RIS = crate::Reg<u32, _DES_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DES_RIS;
#[doc = "`read()` method returns [des_ris::R](des_ris::R) reader structure"]
impl crate::Readable for DES_RIS {}
#[doc = "`write(|w| ..)` method takes [des_ris::W](des_ris::W) writer structure"]
impl crate::Writable for DES_RIS {}
#[doc = "DES_RIS"]
pub mod des_ris;
#[doc = "DES_MIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [des_mis](des_mis) module"]
pub type DES_MIS = crate::Reg<u32, _DES_MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DES_MIS;
#[doc = "`read()` method returns [des_mis::R](des_mis::R) reader structure"]
impl crate::Readable for DES_MIS {}
#[doc = "`write(|w| ..)` method takes [des_mis::W](des_mis::W) writer structure"]
impl crate::Writable for DES_MIS {}
#[doc = "DES_MIS"]
pub mod des_mis;
#[doc = "DES_IC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [des_ic](des_ic) module"]
pub type DES_IC = crate::Reg<u32, _DES_IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DES_IC;
#[doc = "`read()` method returns [des_ic::R](des_ic::R) reader structure"]
impl crate::Readable for DES_IC {}
#[doc = "`write(|w| ..)` method takes [des_ic::W](des_ic::W) writer structure"]
impl crate::Writable for DES_IC {}
#[doc = "DES_IC"]
pub mod des_ic;
#[doc = "EIP_CGCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eip_cgcfg](eip_cgcfg) module"]
pub type EIP_CGCFG = crate::Reg<u32, _EIP_CGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIP_CGCFG;
#[doc = "`read()` method returns [eip_cgcfg::R](eip_cgcfg::R) reader structure"]
impl crate::Readable for EIP_CGCFG {}
#[doc = "`write(|w| ..)` method takes [eip_cgcfg::W](eip_cgcfg::W) writer structure"]
impl crate::Writable for EIP_CGCFG {}
#[doc = "EIP_CGCFG"]
pub mod eip_cgcfg;
#[doc = "EIP_CGREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eip_cgreq](eip_cgreq) module"]
pub type EIP_CGREQ = crate::Reg<u32, _EIP_CGREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIP_CGREQ;
#[doc = "`read()` method returns [eip_cgreq::R](eip_cgreq::R) reader structure"]
impl crate::Readable for EIP_CGREQ {}
#[doc = "`write(|w| ..)` method takes [eip_cgreq::W](eip_cgreq::W) writer structure"]
impl crate::Writable for EIP_CGREQ {}
#[doc = "EIP_CGREQ"]
pub mod eip_cgreq;
#[doc = "CRC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctrl](crc_ctrl) module"]
pub type CRC_CTRL = crate::Reg<u32, _CRC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CTRL;
#[doc = "`read()` method returns [crc_ctrl::R](crc_ctrl::R) reader structure"]
impl crate::Readable for CRC_CTRL {}
#[doc = "`write(|w| ..)` method takes [crc_ctrl::W](crc_ctrl::W) writer structure"]
impl crate::Writable for CRC_CTRL {}
#[doc = "CRC_CTRL"]
pub mod crc_ctrl;
#[doc = "CRC_SEED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_seed](crc_seed) module"]
pub type CRC_SEED = crate::Reg<u32, _CRC_SEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_SEED;
#[doc = "`read()` method returns [crc_seed::R](crc_seed::R) reader structure"]
impl crate::Readable for CRC_SEED {}
#[doc = "`write(|w| ..)` method takes [crc_seed::W](crc_seed::W) writer structure"]
impl crate::Writable for CRC_SEED {}
#[doc = "CRC_SEED"]
pub mod crc_seed;
#[doc = "CRC_DIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_din](crc_din) module"]
pub type CRC_DIN = crate::Reg<u32, _CRC_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_DIN;
#[doc = "`read()` method returns [crc_din::R](crc_din::R) reader structure"]
impl crate::Readable for CRC_DIN {}
#[doc = "`write(|w| ..)` method takes [crc_din::W](crc_din::W) writer structure"]
impl crate::Writable for CRC_DIN {}
#[doc = "CRC_DIN"]
pub mod crc_din;
#[doc = "CRC_RSLT_PP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_rslt_pp](crc_rslt_pp) module"]
pub type CRC_RSLT_PP = crate::Reg<u32, _CRC_RSLT_PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_RSLT_PP;
#[doc = "`read()` method returns [crc_rslt_pp::R](crc_rslt_pp::R) reader structure"]
impl crate::Readable for CRC_RSLT_PP {}
#[doc = "`write(|w| ..)` method takes [crc_rslt_pp::W](crc_rslt_pp::W) writer structure"]
impl crate::Writable for CRC_RSLT_PP {}
#[doc = "CRC_RSLT_PP"]
pub mod crc_rslt_pp;
#[doc = "RAND_KEY0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rand_key0](rand_key0) module"]
pub type RAND_KEY0 = crate::Reg<u32, _RAND_KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAND_KEY0;
#[doc = "`read()` method returns [rand_key0::R](rand_key0::R) reader structure"]
impl crate::Readable for RAND_KEY0 {}
#[doc = "`write(|w| ..)` method takes [rand_key0::W](rand_key0::W) writer structure"]
impl crate::Writable for RAND_KEY0 {}
#[doc = "RAND_KEY0"]
pub mod rand_key0;
#[doc = "RAND_KEY1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rand_key1](rand_key1) module"]
pub type RAND_KEY1 = crate::Reg<u32, _RAND_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAND_KEY1;
#[doc = "`read()` method returns [rand_key1::R](rand_key1::R) reader structure"]
impl crate::Readable for RAND_KEY1 {}
#[doc = "`write(|w| ..)` method takes [rand_key1::W](rand_key1::W) writer structure"]
impl crate::Writable for RAND_KEY1 {}
#[doc = "RAND_KEY1"]
pub mod rand_key1;
#[doc = "RAND_KEY2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rand_key2](rand_key2) module"]
pub type RAND_KEY2 = crate::Reg<u32, _RAND_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAND_KEY2;
#[doc = "`read()` method returns [rand_key2::R](rand_key2::R) reader structure"]
impl crate::Readable for RAND_KEY2 {}
#[doc = "`write(|w| ..)` method takes [rand_key2::W](rand_key2::W) writer structure"]
impl crate::Writable for RAND_KEY2 {}
#[doc = "RAND_KEY2"]
pub mod rand_key2;
#[doc = "RAND_KEY3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rand_key3](rand_key3) module"]
pub type RAND_KEY3 = crate::Reg<u32, _RAND_KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAND_KEY3;
#[doc = "`read()` method returns [rand_key3::R](rand_key3::R) reader structure"]
impl crate::Readable for RAND_KEY3 {}
#[doc = "`write(|w| ..)` method takes [rand_key3::W](rand_key3::W) writer structure"]
impl crate::Writable for RAND_KEY3 {}
#[doc = "RAND_KEY3"]
pub mod rand_key3;
