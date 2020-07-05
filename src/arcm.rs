#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAMCLKCFG"]
    pub camclkcfg: CAMCLKCFG,
    #[doc = "0x04 - CAMCLKEN"]
    pub camclken: CAMCLKEN,
    #[doc = "0x08 - CAMSWRST"]
    pub camswrst: CAMSWRST,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - MCASPCLKEN"]
    pub mcaspclken: MCASPCLKEN,
    #[doc = "0x18 - MCASPSWRST"]
    pub mcaspswrst: MCASPSWRST,
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - SDIOMCLKCFG"]
    pub sdiomclkcfg: SDIOMCLKCFG,
    #[doc = "0x24 - SDIOMCLKEN"]
    pub sdiomclken: SDIOMCLKEN,
    #[doc = "0x28 - SDIOMSWRST"]
    pub sdiomswrst: SDIOMSWRST,
    #[doc = "0x2c - APSPICLKCFG"]
    pub apspiclkcfg: APSPICLKCFG,
    #[doc = "0x30 - APSPICLKEN"]
    pub apspiclken: APSPICLKEN,
    #[doc = "0x34 - APSPISWRST"]
    pub apspiswrst: APSPISWRST,
    _reserved11: [u8; 16usize],
    #[doc = "0x48 - DMACLKEN"]
    pub dmaclken: DMACLKEN,
    #[doc = "0x4c - DMASWRST"]
    pub dmaswrst: DMASWRST,
    #[doc = "0x50 - GPIO0CLKEN"]
    pub gpio0clken: GPIO0CLKEN,
    #[doc = "0x54 - GPIO0SWRST"]
    pub gpio0swrst: GPIO0SWRST,
    #[doc = "0x58 - GPIO1CLKEN"]
    pub gpio1clken: GPIO1CLKEN,
    #[doc = "0x5c - GPIO1SWRST"]
    pub gpio1swrst: GPIO1SWRST,
    #[doc = "0x60 - GPIO2CLKEN"]
    pub gpio2clken: GPIO2CLKEN,
    #[doc = "0x64 - GPIO2SWRST"]
    pub gpio2swrst: GPIO2SWRST,
    #[doc = "0x68 - GPIO3CLKEN"]
    pub gpio3clken: GPIO3CLKEN,
    #[doc = "0x6c - GPIO3SWRST"]
    pub gpio3swrst: GPIO3SWRST,
    #[doc = "0x70 - GPIO4CLKEN"]
    pub gpio4clken: GPIO4CLKEN,
    #[doc = "0x74 - GPIO4SWRST"]
    pub gpio4swrst: GPIO4SWRST,
    #[doc = "0x78 - WDTCLKEN"]
    pub wdtclken: WDTCLKEN,
    #[doc = "0x7c - WDTSWRST"]
    pub wdtswrst: WDTSWRST,
    #[doc = "0x80 - UART0CLKEN"]
    pub uart0clken: UART0CLKEN,
    #[doc = "0x84 - UART0SWRST"]
    pub uart0swrst: UART0SWRST,
    #[doc = "0x88 - UART1CLKEN"]
    pub uart1clken: UART1CLKEN,
    #[doc = "0x8c - UART1SWRST"]
    pub uart1swrst: UART1SWRST,
    #[doc = "0x90 - GPT0CLKEN"]
    pub gpt0clken: GPT0CLKEN,
    #[doc = "0x94 - GPT0SWRST"]
    pub gpt0swrst: GPT0SWRST,
    #[doc = "0x98 - GPT1CLKEN"]
    pub gpt1clken: GPT1CLKEN,
    #[doc = "0x9c - GPT1SWRST"]
    pub gpt1swrst: GPT1SWRST,
    #[doc = "0xa0 - GPT2CLKEN"]
    pub gpt2clken: GPT2CLKEN,
    #[doc = "0xa4 - GPT2SWRST"]
    pub gpt2swrst: GPT2SWRST,
    #[doc = "0xa8 - GPT3CLKEN"]
    pub gpt3clken: GPT3CLKEN,
    #[doc = "0xac - GPT3SWRST"]
    pub gpt3swrst: GPT3SWRST,
    #[doc = "0xb0 - MCASPCLKCFG0"]
    pub mcaspclkcfg0: MCASPCLKCFG0,
    #[doc = "0xb4 - MCASPCLKCFG1"]
    pub mcaspclkcfg1: MCASPCLKCFG1,
    _reserved39: [u8; 32usize],
    #[doc = "0xd8 - I2CCLKEN"]
    pub i2cclken: I2CCLKEN,
    #[doc = "0xdc - I2CSWRST"]
    pub i2cswrst: I2CSWRST,
    _reserved41: [u8; 4usize],
    #[doc = "0xe4 - LPDSREQ"]
    pub lpdsreq: LPDSREQ,
    _reserved42: [u8; 4usize],
    #[doc = "0xec - TURBOREQ"]
    pub turboreq: TURBOREQ,
    _reserved43: [u8; 24usize],
    #[doc = "0x108 - DSLPWAKECFG"]
    pub dslpwakecfg: DSLPWAKECFG,
    #[doc = "0x10c - DSLPTIMRCFG"]
    pub dslptimrcfg: DSLPTIMRCFG,
    #[doc = "0x110 - SLPWAKEEN"]
    pub slpwakeen: SLPWAKEEN,
    #[doc = "0x114 - SLPTMRCFG"]
    pub slptmrcfg: SLPTMRCFG,
    #[doc = "0x118 - WAKENWP"]
    pub wakenwp: WAKENWP,
    _reserved48: [u8; 4usize],
    #[doc = "0x120 - RCM_IS"]
    pub rcm_is: RCM_IS,
    #[doc = "0x124 - RCM_IEN"]
    pub rcm_ien: RCM_IEN,
}
#[doc = "CAMCLKCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [camclkcfg](camclkcfg) module"]
pub type CAMCLKCFG = crate::Reg<u32, _CAMCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAMCLKCFG;
#[doc = "`read()` method returns [camclkcfg::R](camclkcfg::R) reader structure"]
impl crate::Readable for CAMCLKCFG {}
#[doc = "`write(|w| ..)` method takes [camclkcfg::W](camclkcfg::W) writer structure"]
impl crate::Writable for CAMCLKCFG {}
#[doc = "CAMCLKCFG"]
pub mod camclkcfg;
#[doc = "CAMCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [camclken](camclken) module"]
pub type CAMCLKEN = crate::Reg<u32, _CAMCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAMCLKEN;
#[doc = "`read()` method returns [camclken::R](camclken::R) reader structure"]
impl crate::Readable for CAMCLKEN {}
#[doc = "`write(|w| ..)` method takes [camclken::W](camclken::W) writer structure"]
impl crate::Writable for CAMCLKEN {}
#[doc = "CAMCLKEN"]
pub mod camclken;
#[doc = "CAMSWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [camswrst](camswrst) module"]
pub type CAMSWRST = crate::Reg<u32, _CAMSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAMSWRST;
#[doc = "`read()` method returns [camswrst::R](camswrst::R) reader structure"]
impl crate::Readable for CAMSWRST {}
#[doc = "`write(|w| ..)` method takes [camswrst::W](camswrst::W) writer structure"]
impl crate::Writable for CAMSWRST {}
#[doc = "CAMSWRST"]
pub mod camswrst;
#[doc = "MCASPCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcaspclken](mcaspclken) module"]
pub type MCASPCLKEN = crate::Reg<u32, _MCASPCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCASPCLKEN;
#[doc = "`read()` method returns [mcaspclken::R](mcaspclken::R) reader structure"]
impl crate::Readable for MCASPCLKEN {}
#[doc = "`write(|w| ..)` method takes [mcaspclken::W](mcaspclken::W) writer structure"]
impl crate::Writable for MCASPCLKEN {}
#[doc = "MCASPCLKEN"]
pub mod mcaspclken;
#[doc = "MCASPSWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcaspswrst](mcaspswrst) module"]
pub type MCASPSWRST = crate::Reg<u32, _MCASPSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCASPSWRST;
#[doc = "`read()` method returns [mcaspswrst::R](mcaspswrst::R) reader structure"]
impl crate::Readable for MCASPSWRST {}
#[doc = "`write(|w| ..)` method takes [mcaspswrst::W](mcaspswrst::W) writer structure"]
impl crate::Writable for MCASPSWRST {}
#[doc = "MCASPSWRST"]
pub mod mcaspswrst;
#[doc = "SDIOMCLKCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdiomclkcfg](sdiomclkcfg) module"]
pub type SDIOMCLKCFG = crate::Reg<u32, _SDIOMCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOMCLKCFG;
#[doc = "`read()` method returns [sdiomclkcfg::R](sdiomclkcfg::R) reader structure"]
impl crate::Readable for SDIOMCLKCFG {}
#[doc = "`write(|w| ..)` method takes [sdiomclkcfg::W](sdiomclkcfg::W) writer structure"]
impl crate::Writable for SDIOMCLKCFG {}
#[doc = "SDIOMCLKCFG"]
pub mod sdiomclkcfg;
#[doc = "SDIOMCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdiomclken](sdiomclken) module"]
pub type SDIOMCLKEN = crate::Reg<u32, _SDIOMCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOMCLKEN;
#[doc = "`read()` method returns [sdiomclken::R](sdiomclken::R) reader structure"]
impl crate::Readable for SDIOMCLKEN {}
#[doc = "`write(|w| ..)` method takes [sdiomclken::W](sdiomclken::W) writer structure"]
impl crate::Writable for SDIOMCLKEN {}
#[doc = "SDIOMCLKEN"]
pub mod sdiomclken;
#[doc = "SDIOMSWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdiomswrst](sdiomswrst) module"]
pub type SDIOMSWRST = crate::Reg<u32, _SDIOMSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOMSWRST;
#[doc = "`read()` method returns [sdiomswrst::R](sdiomswrst::R) reader structure"]
impl crate::Readable for SDIOMSWRST {}
#[doc = "`write(|w| ..)` method takes [sdiomswrst::W](sdiomswrst::W) writer structure"]
impl crate::Writable for SDIOMSWRST {}
#[doc = "SDIOMSWRST"]
pub mod sdiomswrst;
#[doc = "APSPICLKCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apspiclkcfg](apspiclkcfg) module"]
pub type APSPICLKCFG = crate::Reg<u32, _APSPICLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APSPICLKCFG;
#[doc = "`read()` method returns [apspiclkcfg::R](apspiclkcfg::R) reader structure"]
impl crate::Readable for APSPICLKCFG {}
#[doc = "`write(|w| ..)` method takes [apspiclkcfg::W](apspiclkcfg::W) writer structure"]
impl crate::Writable for APSPICLKCFG {}
#[doc = "APSPICLKCFG"]
pub mod apspiclkcfg;
#[doc = "APSPICLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apspiclken](apspiclken) module"]
pub type APSPICLKEN = crate::Reg<u32, _APSPICLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APSPICLKEN;
#[doc = "`read()` method returns [apspiclken::R](apspiclken::R) reader structure"]
impl crate::Readable for APSPICLKEN {}
#[doc = "`write(|w| ..)` method takes [apspiclken::W](apspiclken::W) writer structure"]
impl crate::Writable for APSPICLKEN {}
#[doc = "APSPICLKEN"]
pub mod apspiclken;
#[doc = "APSPISWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apspiswrst](apspiswrst) module"]
pub type APSPISWRST = crate::Reg<u32, _APSPISWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APSPISWRST;
#[doc = "`read()` method returns [apspiswrst::R](apspiswrst::R) reader structure"]
impl crate::Readable for APSPISWRST {}
#[doc = "`write(|w| ..)` method takes [apspiswrst::W](apspiswrst::W) writer structure"]
impl crate::Writable for APSPISWRST {}
#[doc = "APSPISWRST"]
pub mod apspiswrst;
#[doc = "DMACLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaclken](dmaclken) module"]
pub type DMACLKEN = crate::Reg<u32, _DMACLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACLKEN;
#[doc = "`read()` method returns [dmaclken::R](dmaclken::R) reader structure"]
impl crate::Readable for DMACLKEN {}
#[doc = "`write(|w| ..)` method takes [dmaclken::W](dmaclken::W) writer structure"]
impl crate::Writable for DMACLKEN {}
#[doc = "DMACLKEN"]
pub mod dmaclken;
#[doc = "DMASWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaswrst](dmaswrst) module"]
pub type DMASWRST = crate::Reg<u32, _DMASWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASWRST;
#[doc = "`read()` method returns [dmaswrst::R](dmaswrst::R) reader structure"]
impl crate::Readable for DMASWRST {}
#[doc = "`write(|w| ..)` method takes [dmaswrst::W](dmaswrst::W) writer structure"]
impl crate::Writable for DMASWRST {}
#[doc = "DMASWRST"]
pub mod dmaswrst;
#[doc = "GPIO0CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0clken](gpio0clken) module"]
pub type GPIO0CLKEN = crate::Reg<u32, _GPIO0CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0CLKEN;
#[doc = "`read()` method returns [gpio0clken::R](gpio0clken::R) reader structure"]
impl crate::Readable for GPIO0CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpio0clken::W](gpio0clken::W) writer structure"]
impl crate::Writable for GPIO0CLKEN {}
#[doc = "GPIO0CLKEN"]
pub mod gpio0clken;
#[doc = "GPIO0SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio0swrst](gpio0swrst) module"]
pub type GPIO0SWRST = crate::Reg<u32, _GPIO0SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO0SWRST;
#[doc = "`read()` method returns [gpio0swrst::R](gpio0swrst::R) reader structure"]
impl crate::Readable for GPIO0SWRST {}
#[doc = "`write(|w| ..)` method takes [gpio0swrst::W](gpio0swrst::W) writer structure"]
impl crate::Writable for GPIO0SWRST {}
#[doc = "GPIO0SWRST"]
pub mod gpio0swrst;
#[doc = "GPIO1CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1clken](gpio1clken) module"]
pub type GPIO1CLKEN = crate::Reg<u32, _GPIO1CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1CLKEN;
#[doc = "`read()` method returns [gpio1clken::R](gpio1clken::R) reader structure"]
impl crate::Readable for GPIO1CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpio1clken::W](gpio1clken::W) writer structure"]
impl crate::Writable for GPIO1CLKEN {}
#[doc = "GPIO1CLKEN"]
pub mod gpio1clken;
#[doc = "GPIO1SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio1swrst](gpio1swrst) module"]
pub type GPIO1SWRST = crate::Reg<u32, _GPIO1SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO1SWRST;
#[doc = "`read()` method returns [gpio1swrst::R](gpio1swrst::R) reader structure"]
impl crate::Readable for GPIO1SWRST {}
#[doc = "`write(|w| ..)` method takes [gpio1swrst::W](gpio1swrst::W) writer structure"]
impl crate::Writable for GPIO1SWRST {}
#[doc = "GPIO1SWRST"]
pub mod gpio1swrst;
#[doc = "GPIO2CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2clken](gpio2clken) module"]
pub type GPIO2CLKEN = crate::Reg<u32, _GPIO2CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2CLKEN;
#[doc = "`read()` method returns [gpio2clken::R](gpio2clken::R) reader structure"]
impl crate::Readable for GPIO2CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpio2clken::W](gpio2clken::W) writer structure"]
impl crate::Writable for GPIO2CLKEN {}
#[doc = "GPIO2CLKEN"]
pub mod gpio2clken;
#[doc = "GPIO2SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2swrst](gpio2swrst) module"]
pub type GPIO2SWRST = crate::Reg<u32, _GPIO2SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO2SWRST;
#[doc = "`read()` method returns [gpio2swrst::R](gpio2swrst::R) reader structure"]
impl crate::Readable for GPIO2SWRST {}
#[doc = "`write(|w| ..)` method takes [gpio2swrst::W](gpio2swrst::W) writer structure"]
impl crate::Writable for GPIO2SWRST {}
#[doc = "GPIO2SWRST"]
pub mod gpio2swrst;
#[doc = "GPIO3CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3clken](gpio3clken) module"]
pub type GPIO3CLKEN = crate::Reg<u32, _GPIO3CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3CLKEN;
#[doc = "`read()` method returns [gpio3clken::R](gpio3clken::R) reader structure"]
impl crate::Readable for GPIO3CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpio3clken::W](gpio3clken::W) writer structure"]
impl crate::Writable for GPIO3CLKEN {}
#[doc = "GPIO3CLKEN"]
pub mod gpio3clken;
#[doc = "GPIO3SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio3swrst](gpio3swrst) module"]
pub type GPIO3SWRST = crate::Reg<u32, _GPIO3SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO3SWRST;
#[doc = "`read()` method returns [gpio3swrst::R](gpio3swrst::R) reader structure"]
impl crate::Readable for GPIO3SWRST {}
#[doc = "`write(|w| ..)` method takes [gpio3swrst::W](gpio3swrst::W) writer structure"]
impl crate::Writable for GPIO3SWRST {}
#[doc = "GPIO3SWRST"]
pub mod gpio3swrst;
#[doc = "GPIO4CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio4clken](gpio4clken) module"]
pub type GPIO4CLKEN = crate::Reg<u32, _GPIO4CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO4CLKEN;
#[doc = "`read()` method returns [gpio4clken::R](gpio4clken::R) reader structure"]
impl crate::Readable for GPIO4CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpio4clken::W](gpio4clken::W) writer structure"]
impl crate::Writable for GPIO4CLKEN {}
#[doc = "GPIO4CLKEN"]
pub mod gpio4clken;
#[doc = "GPIO4SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio4swrst](gpio4swrst) module"]
pub type GPIO4SWRST = crate::Reg<u32, _GPIO4SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO4SWRST;
#[doc = "`read()` method returns [gpio4swrst::R](gpio4swrst::R) reader structure"]
impl crate::Readable for GPIO4SWRST {}
#[doc = "`write(|w| ..)` method takes [gpio4swrst::W](gpio4swrst::W) writer structure"]
impl crate::Writable for GPIO4SWRST {}
#[doc = "GPIO4SWRST"]
pub mod gpio4swrst;
#[doc = "WDTCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclken](wdtclken) module"]
pub type WDTCLKEN = crate::Reg<u32, _WDTCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKEN;
#[doc = "`read()` method returns [wdtclken::R](wdtclken::R) reader structure"]
impl crate::Readable for WDTCLKEN {}
#[doc = "`write(|w| ..)` method takes [wdtclken::W](wdtclken::W) writer structure"]
impl crate::Writable for WDTCLKEN {}
#[doc = "WDTCLKEN"]
pub mod wdtclken;
#[doc = "WDTSWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtswrst](wdtswrst) module"]
pub type WDTSWRST = crate::Reg<u32, _WDTSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTSWRST;
#[doc = "`read()` method returns [wdtswrst::R](wdtswrst::R) reader structure"]
impl crate::Readable for WDTSWRST {}
#[doc = "`write(|w| ..)` method takes [wdtswrst::W](wdtswrst::W) writer structure"]
impl crate::Writable for WDTSWRST {}
#[doc = "WDTSWRST"]
pub mod wdtswrst;
#[doc = "UART0CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0clken](uart0clken) module"]
pub type UART0CLKEN = crate::Reg<u32, _UART0CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0CLKEN;
#[doc = "`read()` method returns [uart0clken::R](uart0clken::R) reader structure"]
impl crate::Readable for UART0CLKEN {}
#[doc = "`write(|w| ..)` method takes [uart0clken::W](uart0clken::W) writer structure"]
impl crate::Writable for UART0CLKEN {}
#[doc = "UART0CLKEN"]
pub mod uart0clken;
#[doc = "UART0SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0swrst](uart0swrst) module"]
pub type UART0SWRST = crate::Reg<u32, _UART0SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0SWRST;
#[doc = "`read()` method returns [uart0swrst::R](uart0swrst::R) reader structure"]
impl crate::Readable for UART0SWRST {}
#[doc = "`write(|w| ..)` method takes [uart0swrst::W](uart0swrst::W) writer structure"]
impl crate::Writable for UART0SWRST {}
#[doc = "UART0SWRST"]
pub mod uart0swrst;
#[doc = "UART1CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1clken](uart1clken) module"]
pub type UART1CLKEN = crate::Reg<u32, _UART1CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1CLKEN;
#[doc = "`read()` method returns [uart1clken::R](uart1clken::R) reader structure"]
impl crate::Readable for UART1CLKEN {}
#[doc = "`write(|w| ..)` method takes [uart1clken::W](uart1clken::W) writer structure"]
impl crate::Writable for UART1CLKEN {}
#[doc = "UART1CLKEN"]
pub mod uart1clken;
#[doc = "UART1SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1swrst](uart1swrst) module"]
pub type UART1SWRST = crate::Reg<u32, _UART1SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1SWRST;
#[doc = "`read()` method returns [uart1swrst::R](uart1swrst::R) reader structure"]
impl crate::Readable for UART1SWRST {}
#[doc = "`write(|w| ..)` method takes [uart1swrst::W](uart1swrst::W) writer structure"]
impl crate::Writable for UART1SWRST {}
#[doc = "UART1SWRST"]
pub mod uart1swrst;
#[doc = "GPT0CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt0clken](gpt0clken) module"]
pub type GPT0CLKEN = crate::Reg<u32, _GPT0CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0CLKEN;
#[doc = "`read()` method returns [gpt0clken::R](gpt0clken::R) reader structure"]
impl crate::Readable for GPT0CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpt0clken::W](gpt0clken::W) writer structure"]
impl crate::Writable for GPT0CLKEN {}
#[doc = "GPT0CLKEN"]
pub mod gpt0clken;
#[doc = "GPT0SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt0swrst](gpt0swrst) module"]
pub type GPT0SWRST = crate::Reg<u32, _GPT0SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0SWRST;
#[doc = "`read()` method returns [gpt0swrst::R](gpt0swrst::R) reader structure"]
impl crate::Readable for GPT0SWRST {}
#[doc = "`write(|w| ..)` method takes [gpt0swrst::W](gpt0swrst::W) writer structure"]
impl crate::Writable for GPT0SWRST {}
#[doc = "GPT0SWRST"]
pub mod gpt0swrst;
#[doc = "GPT1CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt1clken](gpt1clken) module"]
pub type GPT1CLKEN = crate::Reg<u32, _GPT1CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1CLKEN;
#[doc = "`read()` method returns [gpt1clken::R](gpt1clken::R) reader structure"]
impl crate::Readable for GPT1CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpt1clken::W](gpt1clken::W) writer structure"]
impl crate::Writable for GPT1CLKEN {}
#[doc = "GPT1CLKEN"]
pub mod gpt1clken;
#[doc = "GPT1SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt1swrst](gpt1swrst) module"]
pub type GPT1SWRST = crate::Reg<u32, _GPT1SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1SWRST;
#[doc = "`read()` method returns [gpt1swrst::R](gpt1swrst::R) reader structure"]
impl crate::Readable for GPT1SWRST {}
#[doc = "`write(|w| ..)` method takes [gpt1swrst::W](gpt1swrst::W) writer structure"]
impl crate::Writable for GPT1SWRST {}
#[doc = "GPT1SWRST"]
pub mod gpt1swrst;
#[doc = "GPT2CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2clken](gpt2clken) module"]
pub type GPT2CLKEN = crate::Reg<u32, _GPT2CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2CLKEN;
#[doc = "`read()` method returns [gpt2clken::R](gpt2clken::R) reader structure"]
impl crate::Readable for GPT2CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpt2clken::W](gpt2clken::W) writer structure"]
impl crate::Writable for GPT2CLKEN {}
#[doc = "GPT2CLKEN"]
pub mod gpt2clken;
#[doc = "GPT2SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2swrst](gpt2swrst) module"]
pub type GPT2SWRST = crate::Reg<u32, _GPT2SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2SWRST;
#[doc = "`read()` method returns [gpt2swrst::R](gpt2swrst::R) reader structure"]
impl crate::Readable for GPT2SWRST {}
#[doc = "`write(|w| ..)` method takes [gpt2swrst::W](gpt2swrst::W) writer structure"]
impl crate::Writable for GPT2SWRST {}
#[doc = "GPT2SWRST"]
pub mod gpt2swrst;
#[doc = "GPT3CLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt3clken](gpt3clken) module"]
pub type GPT3CLKEN = crate::Reg<u32, _GPT3CLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3CLKEN;
#[doc = "`read()` method returns [gpt3clken::R](gpt3clken::R) reader structure"]
impl crate::Readable for GPT3CLKEN {}
#[doc = "`write(|w| ..)` method takes [gpt3clken::W](gpt3clken::W) writer structure"]
impl crate::Writable for GPT3CLKEN {}
#[doc = "GPT3CLKEN"]
pub mod gpt3clken;
#[doc = "GPT3SWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt3swrst](gpt3swrst) module"]
pub type GPT3SWRST = crate::Reg<u32, _GPT3SWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3SWRST;
#[doc = "`read()` method returns [gpt3swrst::R](gpt3swrst::R) reader structure"]
impl crate::Readable for GPT3SWRST {}
#[doc = "`write(|w| ..)` method takes [gpt3swrst::W](gpt3swrst::W) writer structure"]
impl crate::Writable for GPT3SWRST {}
#[doc = "GPT3SWRST"]
pub mod gpt3swrst;
#[doc = "MCASPCLKCFG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcaspclkcfg0](mcaspclkcfg0) module"]
pub type MCASPCLKCFG0 = crate::Reg<u32, _MCASPCLKCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCASPCLKCFG0;
#[doc = "`read()` method returns [mcaspclkcfg0::R](mcaspclkcfg0::R) reader structure"]
impl crate::Readable for MCASPCLKCFG0 {}
#[doc = "`write(|w| ..)` method takes [mcaspclkcfg0::W](mcaspclkcfg0::W) writer structure"]
impl crate::Writable for MCASPCLKCFG0 {}
#[doc = "MCASPCLKCFG0"]
pub mod mcaspclkcfg0;
#[doc = "MCASPCLKCFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcaspclkcfg1](mcaspclkcfg1) module"]
pub type MCASPCLKCFG1 = crate::Reg<u32, _MCASPCLKCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCASPCLKCFG1;
#[doc = "`read()` method returns [mcaspclkcfg1::R](mcaspclkcfg1::R) reader structure"]
impl crate::Readable for MCASPCLKCFG1 {}
#[doc = "`write(|w| ..)` method takes [mcaspclkcfg1::W](mcaspclkcfg1::W) writer structure"]
impl crate::Writable for MCASPCLKCFG1 {}
#[doc = "MCASPCLKCFG1"]
pub mod mcaspclkcfg1;
#[doc = "I2CCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclken](i2cclken) module"]
pub type I2CCLKEN = crate::Reg<u32, _I2CCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CCLKEN;
#[doc = "`read()` method returns [i2cclken::R](i2cclken::R) reader structure"]
impl crate::Readable for I2CCLKEN {}
#[doc = "`write(|w| ..)` method takes [i2cclken::W](i2cclken::W) writer structure"]
impl crate::Writable for I2CCLKEN {}
#[doc = "I2CCLKEN"]
pub mod i2cclken;
#[doc = "I2CSWRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cswrst](i2cswrst) module"]
pub type I2CSWRST = crate::Reg<u32, _I2CSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CSWRST;
#[doc = "`read()` method returns [i2cswrst::R](i2cswrst::R) reader structure"]
impl crate::Readable for I2CSWRST {}
#[doc = "`write(|w| ..)` method takes [i2cswrst::W](i2cswrst::W) writer structure"]
impl crate::Writable for I2CSWRST {}
#[doc = "I2CSWRST"]
pub mod i2cswrst;
#[doc = "LPDSREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpdsreq](lpdsreq) module"]
pub type LPDSREQ = crate::Reg<u32, _LPDSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPDSREQ;
#[doc = "`read()` method returns [lpdsreq::R](lpdsreq::R) reader structure"]
impl crate::Readable for LPDSREQ {}
#[doc = "`write(|w| ..)` method takes [lpdsreq::W](lpdsreq::W) writer structure"]
impl crate::Writable for LPDSREQ {}
#[doc = "LPDSREQ"]
pub mod lpdsreq;
#[doc = "TURBOREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [turboreq](turboreq) module"]
pub type TURBOREQ = crate::Reg<u32, _TURBOREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TURBOREQ;
#[doc = "`read()` method returns [turboreq::R](turboreq::R) reader structure"]
impl crate::Readable for TURBOREQ {}
#[doc = "`write(|w| ..)` method takes [turboreq::W](turboreq::W) writer structure"]
impl crate::Writable for TURBOREQ {}
#[doc = "TURBOREQ"]
pub mod turboreq;
#[doc = "DSLPWAKECFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dslpwakecfg](dslpwakecfg) module"]
pub type DSLPWAKECFG = crate::Reg<u32, _DSLPWAKECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLPWAKECFG;
#[doc = "`read()` method returns [dslpwakecfg::R](dslpwakecfg::R) reader structure"]
impl crate::Readable for DSLPWAKECFG {}
#[doc = "`write(|w| ..)` method takes [dslpwakecfg::W](dslpwakecfg::W) writer structure"]
impl crate::Writable for DSLPWAKECFG {}
#[doc = "DSLPWAKECFG"]
pub mod dslpwakecfg;
#[doc = "DSLPTIMRCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dslptimrcfg](dslptimrcfg) module"]
pub type DSLPTIMRCFG = crate::Reg<u32, _DSLPTIMRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLPTIMRCFG;
#[doc = "`read()` method returns [dslptimrcfg::R](dslptimrcfg::R) reader structure"]
impl crate::Readable for DSLPTIMRCFG {}
#[doc = "`write(|w| ..)` method takes [dslptimrcfg::W](dslptimrcfg::W) writer structure"]
impl crate::Writable for DSLPTIMRCFG {}
#[doc = "DSLPTIMRCFG"]
pub mod dslptimrcfg;
#[doc = "SLPWAKEEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwakeen](slpwakeen) module"]
pub type SLPWAKEEN = crate::Reg<u32, _SLPWAKEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPWAKEEN;
#[doc = "`read()` method returns [slpwakeen::R](slpwakeen::R) reader structure"]
impl crate::Readable for SLPWAKEEN {}
#[doc = "`write(|w| ..)` method takes [slpwakeen::W](slpwakeen::W) writer structure"]
impl crate::Writable for SLPWAKEEN {}
#[doc = "SLPWAKEEN"]
pub mod slpwakeen;
#[doc = "SLPTMRCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slptmrcfg](slptmrcfg) module"]
pub type SLPTMRCFG = crate::Reg<u32, _SLPTMRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPTMRCFG;
#[doc = "`read()` method returns [slptmrcfg::R](slptmrcfg::R) reader structure"]
impl crate::Readable for SLPTMRCFG {}
#[doc = "`write(|w| ..)` method takes [slptmrcfg::W](slptmrcfg::W) writer structure"]
impl crate::Writable for SLPTMRCFG {}
#[doc = "SLPTMRCFG"]
pub mod slptmrcfg;
#[doc = "WAKENWP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakenwp](wakenwp) module"]
pub type WAKENWP = crate::Reg<u32, _WAKENWP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKENWP;
#[doc = "`read()` method returns [wakenwp::R](wakenwp::R) reader structure"]
impl crate::Readable for WAKENWP {}
#[doc = "`write(|w| ..)` method takes [wakenwp::W](wakenwp::W) writer structure"]
impl crate::Writable for WAKENWP {}
#[doc = "WAKENWP"]
pub mod wakenwp;
#[doc = "RCM_IS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcm_is](rcm_is) module"]
pub type RCM_IS = crate::Reg<u32, _RCM_IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCM_IS;
#[doc = "`read()` method returns [rcm_is::R](rcm_is::R) reader structure"]
impl crate::Readable for RCM_IS {}
#[doc = "`write(|w| ..)` method takes [rcm_is::W](rcm_is::W) writer structure"]
impl crate::Writable for RCM_IS {}
#[doc = "RCM_IS"]
pub mod rcm_is;
#[doc = "RCM_IEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcm_ien](rcm_ien) module"]
pub type RCM_IEN = crate::Reg<u32, _RCM_IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCM_IEN;
#[doc = "`read()` method returns [rcm_ien::R](rcm_ien::R) reader structure"]
impl crate::Readable for RCM_IEN {}
#[doc = "`write(|w| ..)` method takes [rcm_ien::W](rcm_ien::W) writer structure"]
impl crate::Writable for RCM_IEN {}
#[doc = "RCM_IEN"]
pub mod rcm_ien;
