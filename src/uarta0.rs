#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART data register"]
    pub uartdr: UARTDR,
    #[doc = "0x04 - UART receive status register/error clear register"]
    pub uartrsr_uartecr: UARTRSR_UARTECR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - UART flag register"]
    pub uartfr: UARTFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - UARTILPR"]
    pub uartilpr: UARTILPR,
    #[doc = "0x24 - UART integer part of the baud-rate divisor value"]
    pub uartibrd: UARTIBRD,
    #[doc = "0x28 - UART fractional part of the baud-rate divisor value"]
    pub uartfbrd: UARTFBRD,
    #[doc = "0x2c - UART line control register"]
    pub uartlcrh: UARTLCRH,
    #[doc = "0x30 - UARTCTL"]
    pub uartctl: UARTCTL,
    #[doc = "0x34 - UART interrupt FIFO level select"]
    pub uartifls: UARTIFLS,
    #[doc = "0x38 - UART interrupt mask register"]
    pub uartim: UARTIM,
    #[doc = "0x3c - UART raw interrupt status register"]
    pub uartris: UARTRIS,
    #[doc = "0x40 - UART masked interrupt status register"]
    pub uartmis: UARTMIS,
    #[doc = "0x44 - UART interrupt clear register"]
    pub uarticr: UARTICR,
    #[doc = "0x48 - UART DMA control register"]
    pub uartdmactl: UARTDMACTL,
    _reserved14: [u8; 68usize],
    #[doc = "0x90 - UARTLCTL"]
    pub uartlctl: UARTLCTL,
    #[doc = "0x94 - UARTLSS"]
    pub uartlss: UARTLSS,
    #[doc = "0x98 - UARTLTIM"]
    pub uartltim: UARTLTIM,
    _reserved17: [u8; 8usize],
    #[doc = "0xa4 - UART_9BITADDR"]
    pub uart_9bitaddr: UART_9BITADDR,
    #[doc = "0xa8 - UART_9BITAMASK"]
    pub uart_9bitamask: UART_9BITAMASK,
    _reserved19: [u8; 3860usize],
    #[doc = "0xfc0 - UARTPP"]
    pub uartpp: UARTPP,
    _reserved20: [u8; 4usize],
    #[doc = "0xfc8 - UARTCC"]
    pub uartcc: UARTCC,
}
#[doc = "UART data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartdr](uartdr) module"]
pub type UARTDR = crate::Reg<u32, _UARTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDR;
#[doc = "`read()` method returns [uartdr::R](uartdr::R) reader structure"]
impl crate::Readable for UARTDR {}
#[doc = "`write(|w| ..)` method takes [uartdr::W](uartdr::W) writer structure"]
impl crate::Writable for UARTDR {}
#[doc = "UART data register"]
pub mod uartdr;
#[doc = "UART receive status register/error clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartrsr_uartecr](uartrsr_uartecr) module"]
pub type UARTRSR_UARTECR = crate::Reg<u32, _UARTRSR_UARTECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRSR_UARTECR;
#[doc = "`read()` method returns [uartrsr_uartecr::R](uartrsr_uartecr::R) reader structure"]
impl crate::Readable for UARTRSR_UARTECR {}
#[doc = "`write(|w| ..)` method takes [uartrsr_uartecr::W](uartrsr_uartecr::W) writer structure"]
impl crate::Writable for UARTRSR_UARTECR {}
#[doc = "UART receive status register/error clear register"]
pub mod uartrsr_uartecr;
#[doc = "UART flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartfr](uartfr) module"]
pub type UARTFR = crate::Reg<u32, _UARTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTFR;
#[doc = "`read()` method returns [uartfr::R](uartfr::R) reader structure"]
impl crate::Readable for UARTFR {}
#[doc = "`write(|w| ..)` method takes [uartfr::W](uartfr::W) writer structure"]
impl crate::Writable for UARTFR {}
#[doc = "UART flag register"]
pub mod uartfr;
#[doc = "UARTILPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartilpr](uartilpr) module"]
pub type UARTILPR = crate::Reg<u32, _UARTILPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTILPR;
#[doc = "`read()` method returns [uartilpr::R](uartilpr::R) reader structure"]
impl crate::Readable for UARTILPR {}
#[doc = "`write(|w| ..)` method takes [uartilpr::W](uartilpr::W) writer structure"]
impl crate::Writable for UARTILPR {}
#[doc = "UARTILPR"]
pub mod uartilpr;
#[doc = "UART integer part of the baud-rate divisor value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartibrd](uartibrd) module"]
pub type UARTIBRD = crate::Reg<u32, _UARTIBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIBRD;
#[doc = "`read()` method returns [uartibrd::R](uartibrd::R) reader structure"]
impl crate::Readable for UARTIBRD {}
#[doc = "`write(|w| ..)` method takes [uartibrd::W](uartibrd::W) writer structure"]
impl crate::Writable for UARTIBRD {}
#[doc = "UART integer part of the baud-rate divisor value"]
pub mod uartibrd;
#[doc = "UART fractional part of the baud-rate divisor value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartfbrd](uartfbrd) module"]
pub type UARTFBRD = crate::Reg<u32, _UARTFBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTFBRD;
#[doc = "`read()` method returns [uartfbrd::R](uartfbrd::R) reader structure"]
impl crate::Readable for UARTFBRD {}
#[doc = "`write(|w| ..)` method takes [uartfbrd::W](uartfbrd::W) writer structure"]
impl crate::Writable for UARTFBRD {}
#[doc = "UART fractional part of the baud-rate divisor value"]
pub mod uartfbrd;
#[doc = "UART line control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartlcrh](uartlcrh) module"]
pub type UARTLCRH = crate::Reg<u32, _UARTLCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLCRH;
#[doc = "`read()` method returns [uartlcrh::R](uartlcrh::R) reader structure"]
impl crate::Readable for UARTLCRH {}
#[doc = "`write(|w| ..)` method takes [uartlcrh::W](uartlcrh::W) writer structure"]
impl crate::Writable for UARTLCRH {}
#[doc = "UART line control register"]
pub mod uartlcrh;
#[doc = "UARTCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartctl](uartctl) module"]
pub type UARTCTL = crate::Reg<u32, _UARTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCTL;
#[doc = "`read()` method returns [uartctl::R](uartctl::R) reader structure"]
impl crate::Readable for UARTCTL {}
#[doc = "`write(|w| ..)` method takes [uartctl::W](uartctl::W) writer structure"]
impl crate::Writable for UARTCTL {}
#[doc = "UARTCTL"]
pub mod uartctl;
#[doc = "UART interrupt FIFO level select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartifls](uartifls) module"]
pub type UARTIFLS = crate::Reg<u32, _UARTIFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIFLS;
#[doc = "`read()` method returns [uartifls::R](uartifls::R) reader structure"]
impl crate::Readable for UARTIFLS {}
#[doc = "`write(|w| ..)` method takes [uartifls::W](uartifls::W) writer structure"]
impl crate::Writable for UARTIFLS {}
#[doc = "UART interrupt FIFO level select"]
pub mod uartifls;
#[doc = "UART interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartim](uartim) module"]
pub type UARTIM = crate::Reg<u32, _UARTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTIM;
#[doc = "`read()` method returns [uartim::R](uartim::R) reader structure"]
impl crate::Readable for UARTIM {}
#[doc = "`write(|w| ..)` method takes [uartim::W](uartim::W) writer structure"]
impl crate::Writable for UARTIM {}
#[doc = "UART interrupt mask register"]
pub mod uartim;
#[doc = "UART raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartris](uartris) module"]
pub type UARTRIS = crate::Reg<u32, _UARTRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRIS;
#[doc = "`read()` method returns [uartris::R](uartris::R) reader structure"]
impl crate::Readable for UARTRIS {}
#[doc = "`write(|w| ..)` method takes [uartris::W](uartris::W) writer structure"]
impl crate::Writable for UARTRIS {}
#[doc = "UART raw interrupt status register"]
pub mod uartris;
#[doc = "UART masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartmis](uartmis) module"]
pub type UARTMIS = crate::Reg<u32, _UARTMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTMIS;
#[doc = "`read()` method returns [uartmis::R](uartmis::R) reader structure"]
impl crate::Readable for UARTMIS {}
#[doc = "`write(|w| ..)` method takes [uartmis::W](uartmis::W) writer structure"]
impl crate::Writable for UARTMIS {}
#[doc = "UART masked interrupt status register"]
pub mod uartmis;
#[doc = "UART interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uarticr](uarticr) module"]
pub type UARTICR = crate::Reg<u32, _UARTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTICR;
#[doc = "`read()` method returns [uarticr::R](uarticr::R) reader structure"]
impl crate::Readable for UARTICR {}
#[doc = "`write(|w| ..)` method takes [uarticr::W](uarticr::W) writer structure"]
impl crate::Writable for UARTICR {}
#[doc = "UART interrupt clear register"]
pub mod uarticr;
#[doc = "UART DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartdmactl](uartdmactl) module"]
pub type UARTDMACTL = crate::Reg<u32, _UARTDMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTDMACTL;
#[doc = "`read()` method returns [uartdmactl::R](uartdmactl::R) reader structure"]
impl crate::Readable for UARTDMACTL {}
#[doc = "`write(|w| ..)` method takes [uartdmactl::W](uartdmactl::W) writer structure"]
impl crate::Writable for UARTDMACTL {}
#[doc = "UART DMA control register"]
pub mod uartdmactl;
#[doc = "UARTLCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartlctl](uartlctl) module"]
pub type UARTLCTL = crate::Reg<u32, _UARTLCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLCTL;
#[doc = "`read()` method returns [uartlctl::R](uartlctl::R) reader structure"]
impl crate::Readable for UARTLCTL {}
#[doc = "`write(|w| ..)` method takes [uartlctl::W](uartlctl::W) writer structure"]
impl crate::Writable for UARTLCTL {}
#[doc = "UARTLCTL"]
pub mod uartlctl;
#[doc = "UARTLSS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartlss](uartlss) module"]
pub type UARTLSS = crate::Reg<u32, _UARTLSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLSS;
#[doc = "`read()` method returns [uartlss::R](uartlss::R) reader structure"]
impl crate::Readable for UARTLSS {}
#[doc = "`write(|w| ..)` method takes [uartlss::W](uartlss::W) writer structure"]
impl crate::Writable for UARTLSS {}
#[doc = "UARTLSS"]
pub mod uartlss;
#[doc = "UARTLTIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartltim](uartltim) module"]
pub type UARTLTIM = crate::Reg<u32, _UARTLTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTLTIM;
#[doc = "`read()` method returns [uartltim::R](uartltim::R) reader structure"]
impl crate::Readable for UARTLTIM {}
#[doc = "`write(|w| ..)` method takes [uartltim::W](uartltim::W) writer structure"]
impl crate::Writable for UARTLTIM {}
#[doc = "UARTLTIM"]
pub mod uartltim;
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
#[doc = "UARTPP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartpp](uartpp) module"]
pub type UARTPP = crate::Reg<u32, _UARTPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTPP;
#[doc = "`read()` method returns [uartpp::R](uartpp::R) reader structure"]
impl crate::Readable for UARTPP {}
#[doc = "`write(|w| ..)` method takes [uartpp::W](uartpp::W) writer structure"]
impl crate::Writable for UARTPP {}
#[doc = "UARTPP"]
pub mod uartpp;
#[doc = "UARTCC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartcc](uartcc) module"]
pub type UARTCC = crate::Reg<u32, _UARTCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCC;
#[doc = "`read()` method returns [uartcc::R](uartcc::R) reader structure"]
impl crate::Readable for UARTCC {}
#[doc = "`write(|w| ..)` method takes [uartcc::W](uartcc::W) writer structure"]
impl crate::Writable for UARTCC {}
#[doc = "UARTCC"]
pub mod uartcc;
