#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PID"]
    pub pid: PID,
    #[doc = "0x04 - Power Idle SYSCONFIG register."]
    pub esysconfig: ESYSCONFIG,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - PFUNC"]
    pub pfunc: PFUNC,
    #[doc = "0x14 - PDIR"]
    pub pdir: PDIR,
    #[doc = "0x18 - PDOUT"]
    pub pdout: PDOUT,
    #[doc = "0x1c - The pin data input register (PDIN) holds the I/O pin state of each of the McASP pins. PDIN allows the actual value of the pin to be read regardless of the state of PFUNC and PDIR. The pin data set register (PDSET) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDSET bit sets the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic high on the pin."]
    pub pdin_pdset: PDIN_PDSET,
    #[doc = "0x20 - The pin data clear register (PDCLR) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDCLR bit clears the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic low on the pin."]
    pub pdclr: PDCLR,
    _reserved7: [u8; 12usize],
    #[doc = "0x30 - for IODFT"]
    pub tlgc: TLGC,
    #[doc = "0x34 - for IODFT"]
    pub tlmr: TLMR,
    #[doc = "0x38 - for IODFT"]
    pub tlec: TLEC,
    _reserved10: [u8; 8usize],
    #[doc = "0x44 - GBLCTL"]
    pub gblctl: GBLCTL,
    #[doc = "0x48 - AMUTE"]
    pub amute: AMUTE,
    #[doc = "0x4c - LBCTL"]
    pub lbctl: LBCTL,
    #[doc = "0x50 - TXDITCTL"]
    pub txditctl: TXDITCTL,
    _reserved14: [u8; 12usize],
    #[doc = "0x60 - GBLCTLR"]
    pub gblctlr: GBLCTLR,
    #[doc = "0x64 - RXMASK"]
    pub rxmask: RXMASK,
    #[doc = "0x68 - RXFMT"]
    pub rxfmt: RXFMT,
    #[doc = "0x6c - RXFMCTL"]
    pub rxfmctl: RXFMCTL,
    #[doc = "0x70 - ACLKRCTL"]
    pub aclkrctl: ACLKRCTL,
    #[doc = "0x74 - AHCLKRCTL"]
    pub ahclkrctl: AHCLKRCTL,
    #[doc = "0x78 - RXTDM"]
    pub rxtdm: RXTDM,
    #[doc = "0x7c - EVTCTLR"]
    pub evtctlr: EVTCTLR,
    #[doc = "0x80 - RXSTAT"]
    pub rxstat: RXSTAT,
    #[doc = "0x84 - RXTDMSLOT"]
    pub rxtdmslot: RXTDMSLOT,
    #[doc = "0x88 - RXCLKCHK"]
    pub rxclkchk: RXCLKCHK,
    #[doc = "0x8c - REVTCTL"]
    pub revtctl: REVTCTL,
    _reserved26: [u8; 16usize],
    #[doc = "0xa0 - GBLCTLX"]
    pub gblctlx: GBLCTLX,
    #[doc = "0xa4 - TXMASK"]
    pub txmask: TXMASK,
    #[doc = "0xa8 - TXFMT"]
    pub txfmt: TXFMT,
    #[doc = "0xac - TXFMCTL"]
    pub txfmctl: TXFMCTL,
    #[doc = "0xb0 - ACLKXCTL"]
    pub aclkxctl: ACLKXCTL,
    #[doc = "0xb4 - AHCLKXCTL"]
    pub ahclkxctl: AHCLKXCTL,
    #[doc = "0xb8 - TXTDM"]
    pub txtdm: TXTDM,
    #[doc = "0xbc - EVTCTLX"]
    pub evtctlx: EVTCTLX,
    #[doc = "0xc0 - TXSTAT"]
    pub txstat: TXSTAT,
    #[doc = "0xc4 - TXTDMSLOT"]
    pub txtdmslot: TXTDMSLOT,
    #[doc = "0xc8 - TXCLKCHK"]
    pub txclkchk: TXCLKCHK,
    #[doc = "0xcc - XEVTCTL"]
    pub xevtctl: XEVTCTL,
    #[doc = "0xd0 - CLKADJEN"]
    pub clkadjen: CLKADJEN,
    _reserved39: [u8; 44usize],
    #[doc = "0x100 - DITCSRA0"]
    pub ditcsra0: DITCSRA0,
    #[doc = "0x104 - DITCSRA1"]
    pub ditcsra1: DITCSRA1,
    #[doc = "0x108 - DITCSRA2"]
    pub ditcsra2: DITCSRA2,
    #[doc = "0x10c - DITCSRA3"]
    pub ditcsra3: DITCSRA3,
    #[doc = "0x110 - DITCSRA4"]
    pub ditcsra4: DITCSRA4,
    #[doc = "0x114 - DITCSRA5"]
    pub ditcsra5: DITCSRA5,
    #[doc = "0x118 - DITCSRB0"]
    pub ditcsrb0: DITCSRB0,
    #[doc = "0x11c - DITCSRB1"]
    pub ditcsrb1: DITCSRB1,
    #[doc = "0x120 - DITCSRB2"]
    pub ditcsrb2: DITCSRB2,
    #[doc = "0x124 - DITCSRB3"]
    pub ditcsrb3: DITCSRB3,
    #[doc = "0x128 - DITCSRB4"]
    pub ditcsrb4: DITCSRB4,
    #[doc = "0x12c - DITCSRB5"]
    pub ditcsrb5: DITCSRB5,
    #[doc = "0x130 - DITUDRA0"]
    pub ditudra0: DITUDRA0,
    #[doc = "0x134 - DITUDRA1"]
    pub ditudra1: DITUDRA1,
    #[doc = "0x138 - DITUDRA2"]
    pub ditudra2: DITUDRA2,
    #[doc = "0x13c - DITUDRA3"]
    pub ditudra3: DITUDRA3,
    #[doc = "0x140 - DITUDRA4"]
    pub ditudra4: DITUDRA4,
    #[doc = "0x144 - DITUDRA5"]
    pub ditudra5: DITUDRA5,
    #[doc = "0x148 - DITUDRB0"]
    pub ditudrb0: DITUDRB0,
    #[doc = "0x14c - DITUDRB1"]
    pub ditudrb1: DITUDRB1,
    #[doc = "0x150 - DITUDRB2"]
    pub ditudrb2: DITUDRB2,
    #[doc = "0x154 - DITUDRB3"]
    pub ditudrb3: DITUDRB3,
    #[doc = "0x158 - DITUDRB4"]
    pub ditudrb4: DITUDRB4,
    #[doc = "0x15c - DITUDRB5"]
    pub ditudrb5: DITUDRB5,
    _reserved63: [u8; 32usize],
    #[doc = "0x180 - XRSRCTL0"]
    pub xrsrctl0: XRSRCTL0,
    #[doc = "0x184 - XRSRCTL1"]
    pub xrsrctl1: XRSRCTL1,
    #[doc = "0x188 - XRSRCTL2"]
    pub xrsrctl2: XRSRCTL2,
    #[doc = "0x18c - XRSRCTL3"]
    pub xrsrctl3: XRSRCTL3,
    #[doc = "0x190 - XRSRCTL4"]
    pub xrsrctl4: XRSRCTL4,
    #[doc = "0x194 - XRSRCTL5"]
    pub xrsrctl5: XRSRCTL5,
    #[doc = "0x198 - XRSRCTL6"]
    pub xrsrctl6: XRSRCTL6,
    #[doc = "0x19c - XRSRCTL7"]
    pub xrsrctl7: XRSRCTL7,
    #[doc = "0x1a0 - XRSRCTL8"]
    pub xrsrctl8: XRSRCTL8,
    #[doc = "0x1a4 - XRSRCTL9"]
    pub xrsrctl9: XRSRCTL9,
    #[doc = "0x1a8 - XRSRCTL10"]
    pub xrsrctl10: XRSRCTL10,
    #[doc = "0x1ac - XRSRCTL11"]
    pub xrsrctl11: XRSRCTL11,
    #[doc = "0x1b0 - XRSRCTL12"]
    pub xrsrctl12: XRSRCTL12,
    #[doc = "0x1b4 - XRSRCTL13"]
    pub xrsrctl13: XRSRCTL13,
    #[doc = "0x1b8 - XRSRCTL14"]
    pub xrsrctl14: XRSRCTL14,
    #[doc = "0x1bc - XRSRCTL15"]
    pub xrsrctl15: XRSRCTL15,
    _reserved79: [u8; 64usize],
    #[doc = "0x200 - TXBUF0"]
    pub txbuf0: TXBUF0,
    #[doc = "0x204 - TXBUF1"]
    pub txbuf1: TXBUF1,
    #[doc = "0x208 - TXBUF2"]
    pub txbuf2: TXBUF2,
    #[doc = "0x20c - TXBUF3"]
    pub txbuf3: TXBUF3,
    #[doc = "0x210 - TXBUF4"]
    pub txbuf4: TXBUF4,
    #[doc = "0x214 - TXBUF5"]
    pub txbuf5: TXBUF5,
    #[doc = "0x218 - TXBUF6"]
    pub txbuf6: TXBUF6,
    #[doc = "0x21c - TXBUF7"]
    pub txbuf7: TXBUF7,
    #[doc = "0x220 - TXBUF8"]
    pub txbuf8: TXBUF8,
    #[doc = "0x224 - TXBUF9"]
    pub txbuf9: TXBUF9,
    #[doc = "0x228 - TXBUF10"]
    pub txbuf10: TXBUF10,
    #[doc = "0x22c - TXBUF11"]
    pub txbuf11: TXBUF11,
    #[doc = "0x230 - TXBUF12"]
    pub txbuf12: TXBUF12,
    #[doc = "0x234 - TXBUF13"]
    pub txbuf13: TXBUF13,
    #[doc = "0x238 - TXBUF14"]
    pub txbuf14: TXBUF14,
    #[doc = "0x23c - TXBUF15"]
    pub txbuf15: TXBUF15,
    _reserved95: [u8; 64usize],
    #[doc = "0x280 - RXBUF0"]
    pub rxbuf0: RXBUF0,
    #[doc = "0x284 - RXBUF1"]
    pub rxbuf1: RXBUF1,
    #[doc = "0x288 - RXBUF2"]
    pub rxbuf2: RXBUF2,
    #[doc = "0x28c - RXBUF3"]
    pub rxbuf3: RXBUF3,
    #[doc = "0x290 - RXBUF4"]
    pub rxbuf4: RXBUF4,
    #[doc = "0x294 - RXBUF5"]
    pub rxbuf5: RXBUF5,
    #[doc = "0x298 - RXBUF6"]
    pub rxbuf6: RXBUF6,
    #[doc = "0x29c - RXBUF7"]
    pub rxbuf7: RXBUF7,
    #[doc = "0x2a0 - RXBUF8"]
    pub rxbuf8: RXBUF8,
    #[doc = "0x2a4 - RXBUF9"]
    pub rxbuf9: RXBUF9,
    #[doc = "0x2a8 - RXBUF10"]
    pub rxbuf10: RXBUF10,
    #[doc = "0x2ac - RXBUF11"]
    pub rxbuf11: RXBUF11,
    #[doc = "0x2b0 - RXBUF12"]
    pub rxbuf12: RXBUF12,
    #[doc = "0x2b4 - RXBUF13"]
    pub rxbuf13: RXBUF13,
    #[doc = "0x2b8 - RXBUF14"]
    pub rxbuf14: RXBUF14,
    #[doc = "0x2bc - RXBUF15"]
    pub rxbuf15: RXBUF15,
}
#[doc = "PID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "`write(|w| ..)` method takes [pid::W](pid::W) writer structure"]
impl crate::Writable for PID {}
#[doc = "PID"]
pub mod pid;
#[doc = "Power Idle SYSCONFIG register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esysconfig](esysconfig) module"]
pub type ESYSCONFIG = crate::Reg<u32, _ESYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESYSCONFIG;
#[doc = "`read()` method returns [esysconfig::R](esysconfig::R) reader structure"]
impl crate::Readable for ESYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [esysconfig::W](esysconfig::W) writer structure"]
impl crate::Writable for ESYSCONFIG {}
#[doc = "Power Idle SYSCONFIG register."]
pub mod esysconfig;
#[doc = "PFUNC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfunc](pfunc) module"]
pub type PFUNC = crate::Reg<u32, _PFUNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFUNC;
#[doc = "`read()` method returns [pfunc::R](pfunc::R) reader structure"]
impl crate::Readable for PFUNC {}
#[doc = "`write(|w| ..)` method takes [pfunc::W](pfunc::W) writer structure"]
impl crate::Writable for PFUNC {}
#[doc = "PFUNC"]
pub mod pfunc;
#[doc = "PDIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdir](pdir) module"]
pub type PDIR = crate::Reg<u32, _PDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIR;
#[doc = "`read()` method returns [pdir::R](pdir::R) reader structure"]
impl crate::Readable for PDIR {}
#[doc = "`write(|w| ..)` method takes [pdir::W](pdir::W) writer structure"]
impl crate::Writable for PDIR {}
#[doc = "PDIR"]
pub mod pdir;
#[doc = "PDOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdout](pdout) module"]
pub type PDOUT = crate::Reg<u32, _PDOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDOUT;
#[doc = "`read()` method returns [pdout::R](pdout::R) reader structure"]
impl crate::Readable for PDOUT {}
#[doc = "`write(|w| ..)` method takes [pdout::W](pdout::W) writer structure"]
impl crate::Writable for PDOUT {}
#[doc = "PDOUT"]
pub mod pdout;
#[doc = "The pin data input register (PDIN) holds the I/O pin state of each of the McASP pins. PDIN allows the actual value of the pin to be read regardless of the state of PFUNC and PDIR. The pin data set register (PDSET) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDSET bit sets the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic high on the pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdin_pdset](pdin_pdset) module"]
pub type PDIN_PDSET = crate::Reg<u32, _PDIN_PDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIN_PDSET;
#[doc = "`read()` method returns [pdin_pdset::R](pdin_pdset::R) reader structure"]
impl crate::Readable for PDIN_PDSET {}
#[doc = "`write(|w| ..)` method takes [pdin_pdset::W](pdin_pdset::W) writer structure"]
impl crate::Writable for PDIN_PDSET {}
#[doc = "The pin data input register (PDIN) holds the I/O pin state of each of the McASP pins. PDIN allows the actual value of the pin to be read regardless of the state of PFUNC and PDIR. The pin data set register (PDSET) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDSET bit sets the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic high on the pin."]
pub mod pdin_pdset;
#[doc = "The pin data clear register (PDCLR) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDCLR bit clears the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic low on the pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdclr](pdclr) module"]
pub type PDCLR = crate::Reg<u32, _PDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCLR;
#[doc = "`read()` method returns [pdclr::R](pdclr::R) reader structure"]
impl crate::Readable for PDCLR {}
#[doc = "`write(|w| ..)` method takes [pdclr::W](pdclr::W) writer structure"]
impl crate::Writable for PDCLR {}
#[doc = "The pin data clear register (PDCLR) is an alias of the pin data output register (PDOUT) for writes only. Writing a 1 to the PDCLR bit clears the corresponding bit in PDOUT and if PFUNC = 1 (GPIO function) and PDIR = 1 (output) drives a logic low on the pin."]
pub mod pdclr;
#[doc = "for IODFT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlgc](tlgc) module"]
pub type TLGC = crate::Reg<u32, _TLGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLGC;
#[doc = "`read()` method returns [tlgc::R](tlgc::R) reader structure"]
impl crate::Readable for TLGC {}
#[doc = "`write(|w| ..)` method takes [tlgc::W](tlgc::W) writer structure"]
impl crate::Writable for TLGC {}
#[doc = "for IODFT"]
pub mod tlgc;
#[doc = "for IODFT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlmr](tlmr) module"]
pub type TLMR = crate::Reg<u32, _TLMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLMR;
#[doc = "`read()` method returns [tlmr::R](tlmr::R) reader structure"]
impl crate::Readable for TLMR {}
#[doc = "`write(|w| ..)` method takes [tlmr::W](tlmr::W) writer structure"]
impl crate::Writable for TLMR {}
#[doc = "for IODFT"]
pub mod tlmr;
#[doc = "for IODFT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlec](tlec) module"]
pub type TLEC = crate::Reg<u32, _TLEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLEC;
#[doc = "`read()` method returns [tlec::R](tlec::R) reader structure"]
impl crate::Readable for TLEC {}
#[doc = "`write(|w| ..)` method takes [tlec::W](tlec::W) writer structure"]
impl crate::Writable for TLEC {}
#[doc = "for IODFT"]
pub mod tlec;
#[doc = "GBLCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblctl](gblctl) module"]
pub type GBLCTL = crate::Reg<u32, _GBLCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GBLCTL;
#[doc = "`read()` method returns [gblctl::R](gblctl::R) reader structure"]
impl crate::Readable for GBLCTL {}
#[doc = "`write(|w| ..)` method takes [gblctl::W](gblctl::W) writer structure"]
impl crate::Writable for GBLCTL {}
#[doc = "GBLCTL"]
pub mod gblctl;
#[doc = "AMUTE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amute](amute) module"]
pub type AMUTE = crate::Reg<u32, _AMUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMUTE;
#[doc = "`read()` method returns [amute::R](amute::R) reader structure"]
impl crate::Readable for AMUTE {}
#[doc = "`write(|w| ..)` method takes [amute::W](amute::W) writer structure"]
impl crate::Writable for AMUTE {}
#[doc = "AMUTE"]
pub mod amute;
#[doc = "LBCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbctl](lbctl) module"]
pub type LBCTL = crate::Reg<u32, _LBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LBCTL;
#[doc = "`read()` method returns [lbctl::R](lbctl::R) reader structure"]
impl crate::Readable for LBCTL {}
#[doc = "`write(|w| ..)` method takes [lbctl::W](lbctl::W) writer structure"]
impl crate::Writable for LBCTL {}
#[doc = "LBCTL"]
pub mod lbctl;
#[doc = "TXDITCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txditctl](txditctl) module"]
pub type TXDITCTL = crate::Reg<u32, _TXDITCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDITCTL;
#[doc = "`read()` method returns [txditctl::R](txditctl::R) reader structure"]
impl crate::Readable for TXDITCTL {}
#[doc = "`write(|w| ..)` method takes [txditctl::W](txditctl::W) writer structure"]
impl crate::Writable for TXDITCTL {}
#[doc = "TXDITCTL"]
pub mod txditctl;
#[doc = "GBLCTLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblctlr](gblctlr) module"]
pub type GBLCTLR = crate::Reg<u32, _GBLCTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GBLCTLR;
#[doc = "`read()` method returns [gblctlr::R](gblctlr::R) reader structure"]
impl crate::Readable for GBLCTLR {}
#[doc = "`write(|w| ..)` method takes [gblctlr::W](gblctlr::W) writer structure"]
impl crate::Writable for GBLCTLR {}
#[doc = "GBLCTLR"]
pub mod gblctlr;
#[doc = "RXMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmask](rxmask) module"]
pub type RXMASK = crate::Reg<u32, _RXMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMASK;
#[doc = "`read()` method returns [rxmask::R](rxmask::R) reader structure"]
impl crate::Readable for RXMASK {}
#[doc = "`write(|w| ..)` method takes [rxmask::W](rxmask::W) writer structure"]
impl crate::Writable for RXMASK {}
#[doc = "RXMASK"]
pub mod rxmask;
#[doc = "RXFMT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfmt](rxfmt) module"]
pub type RXFMT = crate::Reg<u32, _RXFMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFMT;
#[doc = "`read()` method returns [rxfmt::R](rxfmt::R) reader structure"]
impl crate::Readable for RXFMT {}
#[doc = "`write(|w| ..)` method takes [rxfmt::W](rxfmt::W) writer structure"]
impl crate::Writable for RXFMT {}
#[doc = "RXFMT"]
pub mod rxfmt;
#[doc = "RXFMCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfmctl](rxfmctl) module"]
pub type RXFMCTL = crate::Reg<u32, _RXFMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFMCTL;
#[doc = "`read()` method returns [rxfmctl::R](rxfmctl::R) reader structure"]
impl crate::Readable for RXFMCTL {}
#[doc = "`write(|w| ..)` method takes [rxfmctl::W](rxfmctl::W) writer structure"]
impl crate::Writable for RXFMCTL {}
#[doc = "RXFMCTL"]
pub mod rxfmctl;
#[doc = "ACLKRCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclkrctl](aclkrctl) module"]
pub type ACLKRCTL = crate::Reg<u32, _ACLKRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLKRCTL;
#[doc = "`read()` method returns [aclkrctl::R](aclkrctl::R) reader structure"]
impl crate::Readable for ACLKRCTL {}
#[doc = "`write(|w| ..)` method takes [aclkrctl::W](aclkrctl::W) writer structure"]
impl crate::Writable for ACLKRCTL {}
#[doc = "ACLKRCTL"]
pub mod aclkrctl;
#[doc = "AHCLKRCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahclkrctl](ahclkrctl) module"]
pub type AHCLKRCTL = crate::Reg<u32, _AHCLKRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHCLKRCTL;
#[doc = "`read()` method returns [ahclkrctl::R](ahclkrctl::R) reader structure"]
impl crate::Readable for AHCLKRCTL {}
#[doc = "`write(|w| ..)` method takes [ahclkrctl::W](ahclkrctl::W) writer structure"]
impl crate::Writable for AHCLKRCTL {}
#[doc = "AHCLKRCTL"]
pub mod ahclkrctl;
#[doc = "RXTDM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtdm](rxtdm) module"]
pub type RXTDM = crate::Reg<u32, _RXTDM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTDM;
#[doc = "`read()` method returns [rxtdm::R](rxtdm::R) reader structure"]
impl crate::Readable for RXTDM {}
#[doc = "`write(|w| ..)` method takes [rxtdm::W](rxtdm::W) writer structure"]
impl crate::Writable for RXTDM {}
#[doc = "RXTDM"]
pub mod rxtdm;
#[doc = "EVTCTLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtctlr](evtctlr) module"]
pub type EVTCTLR = crate::Reg<u32, _EVTCTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTCTLR;
#[doc = "`read()` method returns [evtctlr::R](evtctlr::R) reader structure"]
impl crate::Readable for EVTCTLR {}
#[doc = "`write(|w| ..)` method takes [evtctlr::W](evtctlr::W) writer structure"]
impl crate::Writable for EVTCTLR {}
#[doc = "EVTCTLR"]
pub mod evtctlr;
#[doc = "RXSTAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstat](rxstat) module"]
pub type RXSTAT = crate::Reg<u32, _RXSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSTAT;
#[doc = "`read()` method returns [rxstat::R](rxstat::R) reader structure"]
impl crate::Readable for RXSTAT {}
#[doc = "`write(|w| ..)` method takes [rxstat::W](rxstat::W) writer structure"]
impl crate::Writable for RXSTAT {}
#[doc = "RXSTAT"]
pub mod rxstat;
#[doc = "RXTDMSLOT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtdmslot](rxtdmslot) module"]
pub type RXTDMSLOT = crate::Reg<u32, _RXTDMSLOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTDMSLOT;
#[doc = "`read()` method returns [rxtdmslot::R](rxtdmslot::R) reader structure"]
impl crate::Readable for RXTDMSLOT {}
#[doc = "`write(|w| ..)` method takes [rxtdmslot::W](rxtdmslot::W) writer structure"]
impl crate::Writable for RXTDMSLOT {}
#[doc = "RXTDMSLOT"]
pub mod rxtdmslot;
#[doc = "RXCLKCHK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxclkchk](rxclkchk) module"]
pub type RXCLKCHK = crate::Reg<u32, _RXCLKCHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCLKCHK;
#[doc = "`read()` method returns [rxclkchk::R](rxclkchk::R) reader structure"]
impl crate::Readable for RXCLKCHK {}
#[doc = "`write(|w| ..)` method takes [rxclkchk::W](rxclkchk::W) writer structure"]
impl crate::Writable for RXCLKCHK {}
#[doc = "RXCLKCHK"]
pub mod rxclkchk;
#[doc = "REVTCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revtctl](revtctl) module"]
pub type REVTCTL = crate::Reg<u32, _REVTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVTCTL;
#[doc = "`read()` method returns [revtctl::R](revtctl::R) reader structure"]
impl crate::Readable for REVTCTL {}
#[doc = "`write(|w| ..)` method takes [revtctl::W](revtctl::W) writer structure"]
impl crate::Writable for REVTCTL {}
#[doc = "REVTCTL"]
pub mod revtctl;
#[doc = "GBLCTLX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblctlx](gblctlx) module"]
pub type GBLCTLX = crate::Reg<u32, _GBLCTLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GBLCTLX;
#[doc = "`read()` method returns [gblctlx::R](gblctlx::R) reader structure"]
impl crate::Readable for GBLCTLX {}
#[doc = "`write(|w| ..)` method takes [gblctlx::W](gblctlx::W) writer structure"]
impl crate::Writable for GBLCTLX {}
#[doc = "GBLCTLX"]
pub mod gblctlx;
#[doc = "TXMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmask](txmask) module"]
pub type TXMASK = crate::Reg<u32, _TXMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMASK;
#[doc = "`read()` method returns [txmask::R](txmask::R) reader structure"]
impl crate::Readable for TXMASK {}
#[doc = "`write(|w| ..)` method takes [txmask::W](txmask::W) writer structure"]
impl crate::Writable for TXMASK {}
#[doc = "TXMASK"]
pub mod txmask;
#[doc = "TXFMT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfmt](txfmt) module"]
pub type TXFMT = crate::Reg<u32, _TXFMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFMT;
#[doc = "`read()` method returns [txfmt::R](txfmt::R) reader structure"]
impl crate::Readable for TXFMT {}
#[doc = "`write(|w| ..)` method takes [txfmt::W](txfmt::W) writer structure"]
impl crate::Writable for TXFMT {}
#[doc = "TXFMT"]
pub mod txfmt;
#[doc = "TXFMCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfmctl](txfmctl) module"]
pub type TXFMCTL = crate::Reg<u32, _TXFMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFMCTL;
#[doc = "`read()` method returns [txfmctl::R](txfmctl::R) reader structure"]
impl crate::Readable for TXFMCTL {}
#[doc = "`write(|w| ..)` method takes [txfmctl::W](txfmctl::W) writer structure"]
impl crate::Writable for TXFMCTL {}
#[doc = "TXFMCTL"]
pub mod txfmctl;
#[doc = "ACLKXCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclkxctl](aclkxctl) module"]
pub type ACLKXCTL = crate::Reg<u32, _ACLKXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLKXCTL;
#[doc = "`read()` method returns [aclkxctl::R](aclkxctl::R) reader structure"]
impl crate::Readable for ACLKXCTL {}
#[doc = "`write(|w| ..)` method takes [aclkxctl::W](aclkxctl::W) writer structure"]
impl crate::Writable for ACLKXCTL {}
#[doc = "ACLKXCTL"]
pub mod aclkxctl;
#[doc = "AHCLKXCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahclkxctl](ahclkxctl) module"]
pub type AHCLKXCTL = crate::Reg<u32, _AHCLKXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHCLKXCTL;
#[doc = "`read()` method returns [ahclkxctl::R](ahclkxctl::R) reader structure"]
impl crate::Readable for AHCLKXCTL {}
#[doc = "`write(|w| ..)` method takes [ahclkxctl::W](ahclkxctl::W) writer structure"]
impl crate::Writable for AHCLKXCTL {}
#[doc = "AHCLKXCTL"]
pub mod ahclkxctl;
#[doc = "TXTDM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtdm](txtdm) module"]
pub type TXTDM = crate::Reg<u32, _TXTDM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTDM;
#[doc = "`read()` method returns [txtdm::R](txtdm::R) reader structure"]
impl crate::Readable for TXTDM {}
#[doc = "`write(|w| ..)` method takes [txtdm::W](txtdm::W) writer structure"]
impl crate::Writable for TXTDM {}
#[doc = "TXTDM"]
pub mod txtdm;
#[doc = "EVTCTLX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtctlx](evtctlx) module"]
pub type EVTCTLX = crate::Reg<u32, _EVTCTLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTCTLX;
#[doc = "`read()` method returns [evtctlx::R](evtctlx::R) reader structure"]
impl crate::Readable for EVTCTLX {}
#[doc = "`write(|w| ..)` method takes [evtctlx::W](evtctlx::W) writer structure"]
impl crate::Writable for EVTCTLX {}
#[doc = "EVTCTLX"]
pub mod evtctlx;
#[doc = "TXSTAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstat](txstat) module"]
pub type TXSTAT = crate::Reg<u32, _TXSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXSTAT;
#[doc = "`read()` method returns [txstat::R](txstat::R) reader structure"]
impl crate::Readable for TXSTAT {}
#[doc = "`write(|w| ..)` method takes [txstat::W](txstat::W) writer structure"]
impl crate::Writable for TXSTAT {}
#[doc = "TXSTAT"]
pub mod txstat;
#[doc = "TXTDMSLOT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtdmslot](txtdmslot) module"]
pub type TXTDMSLOT = crate::Reg<u32, _TXTDMSLOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTDMSLOT;
#[doc = "`read()` method returns [txtdmslot::R](txtdmslot::R) reader structure"]
impl crate::Readable for TXTDMSLOT {}
#[doc = "`write(|w| ..)` method takes [txtdmslot::W](txtdmslot::W) writer structure"]
impl crate::Writable for TXTDMSLOT {}
#[doc = "TXTDMSLOT"]
pub mod txtdmslot;
#[doc = "TXCLKCHK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txclkchk](txclkchk) module"]
pub type TXCLKCHK = crate::Reg<u32, _TXCLKCHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCLKCHK;
#[doc = "`read()` method returns [txclkchk::R](txclkchk::R) reader structure"]
impl crate::Readable for TXCLKCHK {}
#[doc = "`write(|w| ..)` method takes [txclkchk::W](txclkchk::W) writer structure"]
impl crate::Writable for TXCLKCHK {}
#[doc = "TXCLKCHK"]
pub mod txclkchk;
#[doc = "XEVTCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xevtctl](xevtctl) module"]
pub type XEVTCTL = crate::Reg<u32, _XEVTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XEVTCTL;
#[doc = "`read()` method returns [xevtctl::R](xevtctl::R) reader structure"]
impl crate::Readable for XEVTCTL {}
#[doc = "`write(|w| ..)` method takes [xevtctl::W](xevtctl::W) writer structure"]
impl crate::Writable for XEVTCTL {}
#[doc = "XEVTCTL"]
pub mod xevtctl;
#[doc = "CLKADJEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkadjen](clkadjen) module"]
pub type CLKADJEN = crate::Reg<u32, _CLKADJEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKADJEN;
#[doc = "`read()` method returns [clkadjen::R](clkadjen::R) reader structure"]
impl crate::Readable for CLKADJEN {}
#[doc = "`write(|w| ..)` method takes [clkadjen::W](clkadjen::W) writer structure"]
impl crate::Writable for CLKADJEN {}
#[doc = "CLKADJEN"]
pub mod clkadjen;
#[doc = "DITCSRA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra0](ditcsra0) module"]
pub type DITCSRA0 = crate::Reg<u32, _DITCSRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA0;
#[doc = "`read()` method returns [ditcsra0::R](ditcsra0::R) reader structure"]
impl crate::Readable for DITCSRA0 {}
#[doc = "`write(|w| ..)` method takes [ditcsra0::W](ditcsra0::W) writer structure"]
impl crate::Writable for DITCSRA0 {}
#[doc = "DITCSRA0"]
pub mod ditcsra0;
#[doc = "DITCSRA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra1](ditcsra1) module"]
pub type DITCSRA1 = crate::Reg<u32, _DITCSRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA1;
#[doc = "`read()` method returns [ditcsra1::R](ditcsra1::R) reader structure"]
impl crate::Readable for DITCSRA1 {}
#[doc = "`write(|w| ..)` method takes [ditcsra1::W](ditcsra1::W) writer structure"]
impl crate::Writable for DITCSRA1 {}
#[doc = "DITCSRA1"]
pub mod ditcsra1;
#[doc = "DITCSRA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra2](ditcsra2) module"]
pub type DITCSRA2 = crate::Reg<u32, _DITCSRA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA2;
#[doc = "`read()` method returns [ditcsra2::R](ditcsra2::R) reader structure"]
impl crate::Readable for DITCSRA2 {}
#[doc = "`write(|w| ..)` method takes [ditcsra2::W](ditcsra2::W) writer structure"]
impl crate::Writable for DITCSRA2 {}
#[doc = "DITCSRA2"]
pub mod ditcsra2;
#[doc = "DITCSRA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra3](ditcsra3) module"]
pub type DITCSRA3 = crate::Reg<u32, _DITCSRA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA3;
#[doc = "`read()` method returns [ditcsra3::R](ditcsra3::R) reader structure"]
impl crate::Readable for DITCSRA3 {}
#[doc = "`write(|w| ..)` method takes [ditcsra3::W](ditcsra3::W) writer structure"]
impl crate::Writable for DITCSRA3 {}
#[doc = "DITCSRA3"]
pub mod ditcsra3;
#[doc = "DITCSRA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra4](ditcsra4) module"]
pub type DITCSRA4 = crate::Reg<u32, _DITCSRA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA4;
#[doc = "`read()` method returns [ditcsra4::R](ditcsra4::R) reader structure"]
impl crate::Readable for DITCSRA4 {}
#[doc = "`write(|w| ..)` method takes [ditcsra4::W](ditcsra4::W) writer structure"]
impl crate::Writable for DITCSRA4 {}
#[doc = "DITCSRA4"]
pub mod ditcsra4;
#[doc = "DITCSRA5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsra5](ditcsra5) module"]
pub type DITCSRA5 = crate::Reg<u32, _DITCSRA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRA5;
#[doc = "`read()` method returns [ditcsra5::R](ditcsra5::R) reader structure"]
impl crate::Readable for DITCSRA5 {}
#[doc = "`write(|w| ..)` method takes [ditcsra5::W](ditcsra5::W) writer structure"]
impl crate::Writable for DITCSRA5 {}
#[doc = "DITCSRA5"]
pub mod ditcsra5;
#[doc = "DITCSRB0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb0](ditcsrb0) module"]
pub type DITCSRB0 = crate::Reg<u32, _DITCSRB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB0;
#[doc = "`read()` method returns [ditcsrb0::R](ditcsrb0::R) reader structure"]
impl crate::Readable for DITCSRB0 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb0::W](ditcsrb0::W) writer structure"]
impl crate::Writable for DITCSRB0 {}
#[doc = "DITCSRB0"]
pub mod ditcsrb0;
#[doc = "DITCSRB1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb1](ditcsrb1) module"]
pub type DITCSRB1 = crate::Reg<u32, _DITCSRB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB1;
#[doc = "`read()` method returns [ditcsrb1::R](ditcsrb1::R) reader structure"]
impl crate::Readable for DITCSRB1 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb1::W](ditcsrb1::W) writer structure"]
impl crate::Writable for DITCSRB1 {}
#[doc = "DITCSRB1"]
pub mod ditcsrb1;
#[doc = "DITCSRB2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb2](ditcsrb2) module"]
pub type DITCSRB2 = crate::Reg<u32, _DITCSRB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB2;
#[doc = "`read()` method returns [ditcsrb2::R](ditcsrb2::R) reader structure"]
impl crate::Readable for DITCSRB2 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb2::W](ditcsrb2::W) writer structure"]
impl crate::Writable for DITCSRB2 {}
#[doc = "DITCSRB2"]
pub mod ditcsrb2;
#[doc = "DITCSRB3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb3](ditcsrb3) module"]
pub type DITCSRB3 = crate::Reg<u32, _DITCSRB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB3;
#[doc = "`read()` method returns [ditcsrb3::R](ditcsrb3::R) reader structure"]
impl crate::Readable for DITCSRB3 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb3::W](ditcsrb3::W) writer structure"]
impl crate::Writable for DITCSRB3 {}
#[doc = "DITCSRB3"]
pub mod ditcsrb3;
#[doc = "DITCSRB4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb4](ditcsrb4) module"]
pub type DITCSRB4 = crate::Reg<u32, _DITCSRB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB4;
#[doc = "`read()` method returns [ditcsrb4::R](ditcsrb4::R) reader structure"]
impl crate::Readable for DITCSRB4 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb4::W](ditcsrb4::W) writer structure"]
impl crate::Writable for DITCSRB4 {}
#[doc = "DITCSRB4"]
pub mod ditcsrb4;
#[doc = "DITCSRB5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditcsrb5](ditcsrb5) module"]
pub type DITCSRB5 = crate::Reg<u32, _DITCSRB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITCSRB5;
#[doc = "`read()` method returns [ditcsrb5::R](ditcsrb5::R) reader structure"]
impl crate::Readable for DITCSRB5 {}
#[doc = "`write(|w| ..)` method takes [ditcsrb5::W](ditcsrb5::W) writer structure"]
impl crate::Writable for DITCSRB5 {}
#[doc = "DITCSRB5"]
pub mod ditcsrb5;
#[doc = "DITUDRA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra0](ditudra0) module"]
pub type DITUDRA0 = crate::Reg<u32, _DITUDRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA0;
#[doc = "`read()` method returns [ditudra0::R](ditudra0::R) reader structure"]
impl crate::Readable for DITUDRA0 {}
#[doc = "`write(|w| ..)` method takes [ditudra0::W](ditudra0::W) writer structure"]
impl crate::Writable for DITUDRA0 {}
#[doc = "DITUDRA0"]
pub mod ditudra0;
#[doc = "DITUDRA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra1](ditudra1) module"]
pub type DITUDRA1 = crate::Reg<u32, _DITUDRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA1;
#[doc = "`read()` method returns [ditudra1::R](ditudra1::R) reader structure"]
impl crate::Readable for DITUDRA1 {}
#[doc = "`write(|w| ..)` method takes [ditudra1::W](ditudra1::W) writer structure"]
impl crate::Writable for DITUDRA1 {}
#[doc = "DITUDRA1"]
pub mod ditudra1;
#[doc = "DITUDRA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra2](ditudra2) module"]
pub type DITUDRA2 = crate::Reg<u32, _DITUDRA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA2;
#[doc = "`read()` method returns [ditudra2::R](ditudra2::R) reader structure"]
impl crate::Readable for DITUDRA2 {}
#[doc = "`write(|w| ..)` method takes [ditudra2::W](ditudra2::W) writer structure"]
impl crate::Writable for DITUDRA2 {}
#[doc = "DITUDRA2"]
pub mod ditudra2;
#[doc = "DITUDRA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra3](ditudra3) module"]
pub type DITUDRA3 = crate::Reg<u32, _DITUDRA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA3;
#[doc = "`read()` method returns [ditudra3::R](ditudra3::R) reader structure"]
impl crate::Readable for DITUDRA3 {}
#[doc = "`write(|w| ..)` method takes [ditudra3::W](ditudra3::W) writer structure"]
impl crate::Writable for DITUDRA3 {}
#[doc = "DITUDRA3"]
pub mod ditudra3;
#[doc = "DITUDRA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra4](ditudra4) module"]
pub type DITUDRA4 = crate::Reg<u32, _DITUDRA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA4;
#[doc = "`read()` method returns [ditudra4::R](ditudra4::R) reader structure"]
impl crate::Readable for DITUDRA4 {}
#[doc = "`write(|w| ..)` method takes [ditudra4::W](ditudra4::W) writer structure"]
impl crate::Writable for DITUDRA4 {}
#[doc = "DITUDRA4"]
pub mod ditudra4;
#[doc = "DITUDRA5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudra5](ditudra5) module"]
pub type DITUDRA5 = crate::Reg<u32, _DITUDRA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRA5;
#[doc = "`read()` method returns [ditudra5::R](ditudra5::R) reader structure"]
impl crate::Readable for DITUDRA5 {}
#[doc = "`write(|w| ..)` method takes [ditudra5::W](ditudra5::W) writer structure"]
impl crate::Writable for DITUDRA5 {}
#[doc = "DITUDRA5"]
pub mod ditudra5;
#[doc = "DITUDRB0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb0](ditudrb0) module"]
pub type DITUDRB0 = crate::Reg<u32, _DITUDRB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB0;
#[doc = "`read()` method returns [ditudrb0::R](ditudrb0::R) reader structure"]
impl crate::Readable for DITUDRB0 {}
#[doc = "`write(|w| ..)` method takes [ditudrb0::W](ditudrb0::W) writer structure"]
impl crate::Writable for DITUDRB0 {}
#[doc = "DITUDRB0"]
pub mod ditudrb0;
#[doc = "DITUDRB1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb1](ditudrb1) module"]
pub type DITUDRB1 = crate::Reg<u32, _DITUDRB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB1;
#[doc = "`read()` method returns [ditudrb1::R](ditudrb1::R) reader structure"]
impl crate::Readable for DITUDRB1 {}
#[doc = "`write(|w| ..)` method takes [ditudrb1::W](ditudrb1::W) writer structure"]
impl crate::Writable for DITUDRB1 {}
#[doc = "DITUDRB1"]
pub mod ditudrb1;
#[doc = "DITUDRB2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb2](ditudrb2) module"]
pub type DITUDRB2 = crate::Reg<u32, _DITUDRB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB2;
#[doc = "`read()` method returns [ditudrb2::R](ditudrb2::R) reader structure"]
impl crate::Readable for DITUDRB2 {}
#[doc = "`write(|w| ..)` method takes [ditudrb2::W](ditudrb2::W) writer structure"]
impl crate::Writable for DITUDRB2 {}
#[doc = "DITUDRB2"]
pub mod ditudrb2;
#[doc = "DITUDRB3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb3](ditudrb3) module"]
pub type DITUDRB3 = crate::Reg<u32, _DITUDRB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB3;
#[doc = "`read()` method returns [ditudrb3::R](ditudrb3::R) reader structure"]
impl crate::Readable for DITUDRB3 {}
#[doc = "`write(|w| ..)` method takes [ditudrb3::W](ditudrb3::W) writer structure"]
impl crate::Writable for DITUDRB3 {}
#[doc = "DITUDRB3"]
pub mod ditudrb3;
#[doc = "DITUDRB4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb4](ditudrb4) module"]
pub type DITUDRB4 = crate::Reg<u32, _DITUDRB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB4;
#[doc = "`read()` method returns [ditudrb4::R](ditudrb4::R) reader structure"]
impl crate::Readable for DITUDRB4 {}
#[doc = "`write(|w| ..)` method takes [ditudrb4::W](ditudrb4::W) writer structure"]
impl crate::Writable for DITUDRB4 {}
#[doc = "DITUDRB4"]
pub mod ditudrb4;
#[doc = "DITUDRB5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditudrb5](ditudrb5) module"]
pub type DITUDRB5 = crate::Reg<u32, _DITUDRB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DITUDRB5;
#[doc = "`read()` method returns [ditudrb5::R](ditudrb5::R) reader structure"]
impl crate::Readable for DITUDRB5 {}
#[doc = "`write(|w| ..)` method takes [ditudrb5::W](ditudrb5::W) writer structure"]
impl crate::Writable for DITUDRB5 {}
#[doc = "DITUDRB5"]
pub mod ditudrb5;
#[doc = "XRSRCTL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl0](xrsrctl0) module"]
pub type XRSRCTL0 = crate::Reg<u32, _XRSRCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL0;
#[doc = "`read()` method returns [xrsrctl0::R](xrsrctl0::R) reader structure"]
impl crate::Readable for XRSRCTL0 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl0::W](xrsrctl0::W) writer structure"]
impl crate::Writable for XRSRCTL0 {}
#[doc = "XRSRCTL0"]
pub mod xrsrctl0;
#[doc = "XRSRCTL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl1](xrsrctl1) module"]
pub type XRSRCTL1 = crate::Reg<u32, _XRSRCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL1;
#[doc = "`read()` method returns [xrsrctl1::R](xrsrctl1::R) reader structure"]
impl crate::Readable for XRSRCTL1 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl1::W](xrsrctl1::W) writer structure"]
impl crate::Writable for XRSRCTL1 {}
#[doc = "XRSRCTL1"]
pub mod xrsrctl1;
#[doc = "XRSRCTL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl2](xrsrctl2) module"]
pub type XRSRCTL2 = crate::Reg<u32, _XRSRCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL2;
#[doc = "`read()` method returns [xrsrctl2::R](xrsrctl2::R) reader structure"]
impl crate::Readable for XRSRCTL2 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl2::W](xrsrctl2::W) writer structure"]
impl crate::Writable for XRSRCTL2 {}
#[doc = "XRSRCTL2"]
pub mod xrsrctl2;
#[doc = "XRSRCTL3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl3](xrsrctl3) module"]
pub type XRSRCTL3 = crate::Reg<u32, _XRSRCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL3;
#[doc = "`read()` method returns [xrsrctl3::R](xrsrctl3::R) reader structure"]
impl crate::Readable for XRSRCTL3 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl3::W](xrsrctl3::W) writer structure"]
impl crate::Writable for XRSRCTL3 {}
#[doc = "XRSRCTL3"]
pub mod xrsrctl3;
#[doc = "XRSRCTL4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl4](xrsrctl4) module"]
pub type XRSRCTL4 = crate::Reg<u32, _XRSRCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL4;
#[doc = "`read()` method returns [xrsrctl4::R](xrsrctl4::R) reader structure"]
impl crate::Readable for XRSRCTL4 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl4::W](xrsrctl4::W) writer structure"]
impl crate::Writable for XRSRCTL4 {}
#[doc = "XRSRCTL4"]
pub mod xrsrctl4;
#[doc = "XRSRCTL5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl5](xrsrctl5) module"]
pub type XRSRCTL5 = crate::Reg<u32, _XRSRCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL5;
#[doc = "`read()` method returns [xrsrctl5::R](xrsrctl5::R) reader structure"]
impl crate::Readable for XRSRCTL5 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl5::W](xrsrctl5::W) writer structure"]
impl crate::Writable for XRSRCTL5 {}
#[doc = "XRSRCTL5"]
pub mod xrsrctl5;
#[doc = "XRSRCTL6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl6](xrsrctl6) module"]
pub type XRSRCTL6 = crate::Reg<u32, _XRSRCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL6;
#[doc = "`read()` method returns [xrsrctl6::R](xrsrctl6::R) reader structure"]
impl crate::Readable for XRSRCTL6 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl6::W](xrsrctl6::W) writer structure"]
impl crate::Writable for XRSRCTL6 {}
#[doc = "XRSRCTL6"]
pub mod xrsrctl6;
#[doc = "XRSRCTL7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl7](xrsrctl7) module"]
pub type XRSRCTL7 = crate::Reg<u32, _XRSRCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL7;
#[doc = "`read()` method returns [xrsrctl7::R](xrsrctl7::R) reader structure"]
impl crate::Readable for XRSRCTL7 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl7::W](xrsrctl7::W) writer structure"]
impl crate::Writable for XRSRCTL7 {}
#[doc = "XRSRCTL7"]
pub mod xrsrctl7;
#[doc = "XRSRCTL8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl8](xrsrctl8) module"]
pub type XRSRCTL8 = crate::Reg<u32, _XRSRCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL8;
#[doc = "`read()` method returns [xrsrctl8::R](xrsrctl8::R) reader structure"]
impl crate::Readable for XRSRCTL8 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl8::W](xrsrctl8::W) writer structure"]
impl crate::Writable for XRSRCTL8 {}
#[doc = "XRSRCTL8"]
pub mod xrsrctl8;
#[doc = "XRSRCTL9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl9](xrsrctl9) module"]
pub type XRSRCTL9 = crate::Reg<u32, _XRSRCTL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL9;
#[doc = "`read()` method returns [xrsrctl9::R](xrsrctl9::R) reader structure"]
impl crate::Readable for XRSRCTL9 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl9::W](xrsrctl9::W) writer structure"]
impl crate::Writable for XRSRCTL9 {}
#[doc = "XRSRCTL9"]
pub mod xrsrctl9;
#[doc = "XRSRCTL10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl10](xrsrctl10) module"]
pub type XRSRCTL10 = crate::Reg<u32, _XRSRCTL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL10;
#[doc = "`read()` method returns [xrsrctl10::R](xrsrctl10::R) reader structure"]
impl crate::Readable for XRSRCTL10 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl10::W](xrsrctl10::W) writer structure"]
impl crate::Writable for XRSRCTL10 {}
#[doc = "XRSRCTL10"]
pub mod xrsrctl10;
#[doc = "XRSRCTL11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl11](xrsrctl11) module"]
pub type XRSRCTL11 = crate::Reg<u32, _XRSRCTL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL11;
#[doc = "`read()` method returns [xrsrctl11::R](xrsrctl11::R) reader structure"]
impl crate::Readable for XRSRCTL11 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl11::W](xrsrctl11::W) writer structure"]
impl crate::Writable for XRSRCTL11 {}
#[doc = "XRSRCTL11"]
pub mod xrsrctl11;
#[doc = "XRSRCTL12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl12](xrsrctl12) module"]
pub type XRSRCTL12 = crate::Reg<u32, _XRSRCTL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL12;
#[doc = "`read()` method returns [xrsrctl12::R](xrsrctl12::R) reader structure"]
impl crate::Readable for XRSRCTL12 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl12::W](xrsrctl12::W) writer structure"]
impl crate::Writable for XRSRCTL12 {}
#[doc = "XRSRCTL12"]
pub mod xrsrctl12;
#[doc = "XRSRCTL13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl13](xrsrctl13) module"]
pub type XRSRCTL13 = crate::Reg<u32, _XRSRCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL13;
#[doc = "`read()` method returns [xrsrctl13::R](xrsrctl13::R) reader structure"]
impl crate::Readable for XRSRCTL13 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl13::W](xrsrctl13::W) writer structure"]
impl crate::Writable for XRSRCTL13 {}
#[doc = "XRSRCTL13"]
pub mod xrsrctl13;
#[doc = "XRSRCTL14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl14](xrsrctl14) module"]
pub type XRSRCTL14 = crate::Reg<u32, _XRSRCTL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL14;
#[doc = "`read()` method returns [xrsrctl14::R](xrsrctl14::R) reader structure"]
impl crate::Readable for XRSRCTL14 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl14::W](xrsrctl14::W) writer structure"]
impl crate::Writable for XRSRCTL14 {}
#[doc = "XRSRCTL14"]
pub mod xrsrctl14;
#[doc = "XRSRCTL15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xrsrctl15](xrsrctl15) module"]
pub type XRSRCTL15 = crate::Reg<u32, _XRSRCTL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRSRCTL15;
#[doc = "`read()` method returns [xrsrctl15::R](xrsrctl15::R) reader structure"]
impl crate::Readable for XRSRCTL15 {}
#[doc = "`write(|w| ..)` method takes [xrsrctl15::W](xrsrctl15::W) writer structure"]
impl crate::Writable for XRSRCTL15 {}
#[doc = "XRSRCTL15"]
pub mod xrsrctl15;
#[doc = "TXBUF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf0](txbuf0) module"]
pub type TXBUF0 = crate::Reg<u32, _TXBUF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF0;
#[doc = "`read()` method returns [txbuf0::R](txbuf0::R) reader structure"]
impl crate::Readable for TXBUF0 {}
#[doc = "`write(|w| ..)` method takes [txbuf0::W](txbuf0::W) writer structure"]
impl crate::Writable for TXBUF0 {}
#[doc = "TXBUF0"]
pub mod txbuf0;
#[doc = "TXBUF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf1](txbuf1) module"]
pub type TXBUF1 = crate::Reg<u32, _TXBUF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF1;
#[doc = "`read()` method returns [txbuf1::R](txbuf1::R) reader structure"]
impl crate::Readable for TXBUF1 {}
#[doc = "`write(|w| ..)` method takes [txbuf1::W](txbuf1::W) writer structure"]
impl crate::Writable for TXBUF1 {}
#[doc = "TXBUF1"]
pub mod txbuf1;
#[doc = "TXBUF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf2](txbuf2) module"]
pub type TXBUF2 = crate::Reg<u32, _TXBUF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF2;
#[doc = "`read()` method returns [txbuf2::R](txbuf2::R) reader structure"]
impl crate::Readable for TXBUF2 {}
#[doc = "`write(|w| ..)` method takes [txbuf2::W](txbuf2::W) writer structure"]
impl crate::Writable for TXBUF2 {}
#[doc = "TXBUF2"]
pub mod txbuf2;
#[doc = "TXBUF3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf3](txbuf3) module"]
pub type TXBUF3 = crate::Reg<u32, _TXBUF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF3;
#[doc = "`read()` method returns [txbuf3::R](txbuf3::R) reader structure"]
impl crate::Readable for TXBUF3 {}
#[doc = "`write(|w| ..)` method takes [txbuf3::W](txbuf3::W) writer structure"]
impl crate::Writable for TXBUF3 {}
#[doc = "TXBUF3"]
pub mod txbuf3;
#[doc = "TXBUF4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf4](txbuf4) module"]
pub type TXBUF4 = crate::Reg<u32, _TXBUF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF4;
#[doc = "`read()` method returns [txbuf4::R](txbuf4::R) reader structure"]
impl crate::Readable for TXBUF4 {}
#[doc = "`write(|w| ..)` method takes [txbuf4::W](txbuf4::W) writer structure"]
impl crate::Writable for TXBUF4 {}
#[doc = "TXBUF4"]
pub mod txbuf4;
#[doc = "TXBUF5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf5](txbuf5) module"]
pub type TXBUF5 = crate::Reg<u32, _TXBUF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF5;
#[doc = "`read()` method returns [txbuf5::R](txbuf5::R) reader structure"]
impl crate::Readable for TXBUF5 {}
#[doc = "`write(|w| ..)` method takes [txbuf5::W](txbuf5::W) writer structure"]
impl crate::Writable for TXBUF5 {}
#[doc = "TXBUF5"]
pub mod txbuf5;
#[doc = "TXBUF6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf6](txbuf6) module"]
pub type TXBUF6 = crate::Reg<u32, _TXBUF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF6;
#[doc = "`read()` method returns [txbuf6::R](txbuf6::R) reader structure"]
impl crate::Readable for TXBUF6 {}
#[doc = "`write(|w| ..)` method takes [txbuf6::W](txbuf6::W) writer structure"]
impl crate::Writable for TXBUF6 {}
#[doc = "TXBUF6"]
pub mod txbuf6;
#[doc = "TXBUF7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf7](txbuf7) module"]
pub type TXBUF7 = crate::Reg<u32, _TXBUF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF7;
#[doc = "`read()` method returns [txbuf7::R](txbuf7::R) reader structure"]
impl crate::Readable for TXBUF7 {}
#[doc = "`write(|w| ..)` method takes [txbuf7::W](txbuf7::W) writer structure"]
impl crate::Writable for TXBUF7 {}
#[doc = "TXBUF7"]
pub mod txbuf7;
#[doc = "TXBUF8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf8](txbuf8) module"]
pub type TXBUF8 = crate::Reg<u32, _TXBUF8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF8;
#[doc = "`read()` method returns [txbuf8::R](txbuf8::R) reader structure"]
impl crate::Readable for TXBUF8 {}
#[doc = "`write(|w| ..)` method takes [txbuf8::W](txbuf8::W) writer structure"]
impl crate::Writable for TXBUF8 {}
#[doc = "TXBUF8"]
pub mod txbuf8;
#[doc = "TXBUF9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf9](txbuf9) module"]
pub type TXBUF9 = crate::Reg<u32, _TXBUF9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF9;
#[doc = "`read()` method returns [txbuf9::R](txbuf9::R) reader structure"]
impl crate::Readable for TXBUF9 {}
#[doc = "`write(|w| ..)` method takes [txbuf9::W](txbuf9::W) writer structure"]
impl crate::Writable for TXBUF9 {}
#[doc = "TXBUF9"]
pub mod txbuf9;
#[doc = "TXBUF10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf10](txbuf10) module"]
pub type TXBUF10 = crate::Reg<u32, _TXBUF10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF10;
#[doc = "`read()` method returns [txbuf10::R](txbuf10::R) reader structure"]
impl crate::Readable for TXBUF10 {}
#[doc = "`write(|w| ..)` method takes [txbuf10::W](txbuf10::W) writer structure"]
impl crate::Writable for TXBUF10 {}
#[doc = "TXBUF10"]
pub mod txbuf10;
#[doc = "TXBUF11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf11](txbuf11) module"]
pub type TXBUF11 = crate::Reg<u32, _TXBUF11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF11;
#[doc = "`read()` method returns [txbuf11::R](txbuf11::R) reader structure"]
impl crate::Readable for TXBUF11 {}
#[doc = "`write(|w| ..)` method takes [txbuf11::W](txbuf11::W) writer structure"]
impl crate::Writable for TXBUF11 {}
#[doc = "TXBUF11"]
pub mod txbuf11;
#[doc = "TXBUF12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf12](txbuf12) module"]
pub type TXBUF12 = crate::Reg<u32, _TXBUF12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF12;
#[doc = "`read()` method returns [txbuf12::R](txbuf12::R) reader structure"]
impl crate::Readable for TXBUF12 {}
#[doc = "`write(|w| ..)` method takes [txbuf12::W](txbuf12::W) writer structure"]
impl crate::Writable for TXBUF12 {}
#[doc = "TXBUF12"]
pub mod txbuf12;
#[doc = "TXBUF13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf13](txbuf13) module"]
pub type TXBUF13 = crate::Reg<u32, _TXBUF13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF13;
#[doc = "`read()` method returns [txbuf13::R](txbuf13::R) reader structure"]
impl crate::Readable for TXBUF13 {}
#[doc = "`write(|w| ..)` method takes [txbuf13::W](txbuf13::W) writer structure"]
impl crate::Writable for TXBUF13 {}
#[doc = "TXBUF13"]
pub mod txbuf13;
#[doc = "TXBUF14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf14](txbuf14) module"]
pub type TXBUF14 = crate::Reg<u32, _TXBUF14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF14;
#[doc = "`read()` method returns [txbuf14::R](txbuf14::R) reader structure"]
impl crate::Readable for TXBUF14 {}
#[doc = "`write(|w| ..)` method takes [txbuf14::W](txbuf14::W) writer structure"]
impl crate::Writable for TXBUF14 {}
#[doc = "TXBUF14"]
pub mod txbuf14;
#[doc = "TXBUF15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbuf15](txbuf15) module"]
pub type TXBUF15 = crate::Reg<u32, _TXBUF15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBUF15;
#[doc = "`read()` method returns [txbuf15::R](txbuf15::R) reader structure"]
impl crate::Readable for TXBUF15 {}
#[doc = "`write(|w| ..)` method takes [txbuf15::W](txbuf15::W) writer structure"]
impl crate::Writable for TXBUF15 {}
#[doc = "TXBUF15"]
pub mod txbuf15;
#[doc = "RXBUF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf0](rxbuf0) module"]
pub type RXBUF0 = crate::Reg<u32, _RXBUF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF0;
#[doc = "`read()` method returns [rxbuf0::R](rxbuf0::R) reader structure"]
impl crate::Readable for RXBUF0 {}
#[doc = "`write(|w| ..)` method takes [rxbuf0::W](rxbuf0::W) writer structure"]
impl crate::Writable for RXBUF0 {}
#[doc = "RXBUF0"]
pub mod rxbuf0;
#[doc = "RXBUF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf1](rxbuf1) module"]
pub type RXBUF1 = crate::Reg<u32, _RXBUF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF1;
#[doc = "`read()` method returns [rxbuf1::R](rxbuf1::R) reader structure"]
impl crate::Readable for RXBUF1 {}
#[doc = "`write(|w| ..)` method takes [rxbuf1::W](rxbuf1::W) writer structure"]
impl crate::Writable for RXBUF1 {}
#[doc = "RXBUF1"]
pub mod rxbuf1;
#[doc = "RXBUF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf2](rxbuf2) module"]
pub type RXBUF2 = crate::Reg<u32, _RXBUF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF2;
#[doc = "`read()` method returns [rxbuf2::R](rxbuf2::R) reader structure"]
impl crate::Readable for RXBUF2 {}
#[doc = "`write(|w| ..)` method takes [rxbuf2::W](rxbuf2::W) writer structure"]
impl crate::Writable for RXBUF2 {}
#[doc = "RXBUF2"]
pub mod rxbuf2;
#[doc = "RXBUF3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf3](rxbuf3) module"]
pub type RXBUF3 = crate::Reg<u32, _RXBUF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF3;
#[doc = "`read()` method returns [rxbuf3::R](rxbuf3::R) reader structure"]
impl crate::Readable for RXBUF3 {}
#[doc = "`write(|w| ..)` method takes [rxbuf3::W](rxbuf3::W) writer structure"]
impl crate::Writable for RXBUF3 {}
#[doc = "RXBUF3"]
pub mod rxbuf3;
#[doc = "RXBUF4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf4](rxbuf4) module"]
pub type RXBUF4 = crate::Reg<u32, _RXBUF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF4;
#[doc = "`read()` method returns [rxbuf4::R](rxbuf4::R) reader structure"]
impl crate::Readable for RXBUF4 {}
#[doc = "`write(|w| ..)` method takes [rxbuf4::W](rxbuf4::W) writer structure"]
impl crate::Writable for RXBUF4 {}
#[doc = "RXBUF4"]
pub mod rxbuf4;
#[doc = "RXBUF5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf5](rxbuf5) module"]
pub type RXBUF5 = crate::Reg<u32, _RXBUF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF5;
#[doc = "`read()` method returns [rxbuf5::R](rxbuf5::R) reader structure"]
impl crate::Readable for RXBUF5 {}
#[doc = "`write(|w| ..)` method takes [rxbuf5::W](rxbuf5::W) writer structure"]
impl crate::Writable for RXBUF5 {}
#[doc = "RXBUF5"]
pub mod rxbuf5;
#[doc = "RXBUF6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf6](rxbuf6) module"]
pub type RXBUF6 = crate::Reg<u32, _RXBUF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF6;
#[doc = "`read()` method returns [rxbuf6::R](rxbuf6::R) reader structure"]
impl crate::Readable for RXBUF6 {}
#[doc = "`write(|w| ..)` method takes [rxbuf6::W](rxbuf6::W) writer structure"]
impl crate::Writable for RXBUF6 {}
#[doc = "RXBUF6"]
pub mod rxbuf6;
#[doc = "RXBUF7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf7](rxbuf7) module"]
pub type RXBUF7 = crate::Reg<u32, _RXBUF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF7;
#[doc = "`read()` method returns [rxbuf7::R](rxbuf7::R) reader structure"]
impl crate::Readable for RXBUF7 {}
#[doc = "`write(|w| ..)` method takes [rxbuf7::W](rxbuf7::W) writer structure"]
impl crate::Writable for RXBUF7 {}
#[doc = "RXBUF7"]
pub mod rxbuf7;
#[doc = "RXBUF8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf8](rxbuf8) module"]
pub type RXBUF8 = crate::Reg<u32, _RXBUF8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF8;
#[doc = "`read()` method returns [rxbuf8::R](rxbuf8::R) reader structure"]
impl crate::Readable for RXBUF8 {}
#[doc = "`write(|w| ..)` method takes [rxbuf8::W](rxbuf8::W) writer structure"]
impl crate::Writable for RXBUF8 {}
#[doc = "RXBUF8"]
pub mod rxbuf8;
#[doc = "RXBUF9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf9](rxbuf9) module"]
pub type RXBUF9 = crate::Reg<u32, _RXBUF9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF9;
#[doc = "`read()` method returns [rxbuf9::R](rxbuf9::R) reader structure"]
impl crate::Readable for RXBUF9 {}
#[doc = "`write(|w| ..)` method takes [rxbuf9::W](rxbuf9::W) writer structure"]
impl crate::Writable for RXBUF9 {}
#[doc = "RXBUF9"]
pub mod rxbuf9;
#[doc = "RXBUF10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf10](rxbuf10) module"]
pub type RXBUF10 = crate::Reg<u32, _RXBUF10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF10;
#[doc = "`read()` method returns [rxbuf10::R](rxbuf10::R) reader structure"]
impl crate::Readable for RXBUF10 {}
#[doc = "`write(|w| ..)` method takes [rxbuf10::W](rxbuf10::W) writer structure"]
impl crate::Writable for RXBUF10 {}
#[doc = "RXBUF10"]
pub mod rxbuf10;
#[doc = "RXBUF11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf11](rxbuf11) module"]
pub type RXBUF11 = crate::Reg<u32, _RXBUF11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF11;
#[doc = "`read()` method returns [rxbuf11::R](rxbuf11::R) reader structure"]
impl crate::Readable for RXBUF11 {}
#[doc = "`write(|w| ..)` method takes [rxbuf11::W](rxbuf11::W) writer structure"]
impl crate::Writable for RXBUF11 {}
#[doc = "RXBUF11"]
pub mod rxbuf11;
#[doc = "RXBUF12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf12](rxbuf12) module"]
pub type RXBUF12 = crate::Reg<u32, _RXBUF12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF12;
#[doc = "`read()` method returns [rxbuf12::R](rxbuf12::R) reader structure"]
impl crate::Readable for RXBUF12 {}
#[doc = "`write(|w| ..)` method takes [rxbuf12::W](rxbuf12::W) writer structure"]
impl crate::Writable for RXBUF12 {}
#[doc = "RXBUF12"]
pub mod rxbuf12;
#[doc = "RXBUF13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf13](rxbuf13) module"]
pub type RXBUF13 = crate::Reg<u32, _RXBUF13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF13;
#[doc = "`read()` method returns [rxbuf13::R](rxbuf13::R) reader structure"]
impl crate::Readable for RXBUF13 {}
#[doc = "`write(|w| ..)` method takes [rxbuf13::W](rxbuf13::W) writer structure"]
impl crate::Writable for RXBUF13 {}
#[doc = "RXBUF13"]
pub mod rxbuf13;
#[doc = "RXBUF14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf14](rxbuf14) module"]
pub type RXBUF14 = crate::Reg<u32, _RXBUF14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF14;
#[doc = "`read()` method returns [rxbuf14::R](rxbuf14::R) reader structure"]
impl crate::Readable for RXBUF14 {}
#[doc = "`write(|w| ..)` method takes [rxbuf14::W](rxbuf14::W) writer structure"]
impl crate::Writable for RXBUF14 {}
#[doc = "RXBUF14"]
pub mod rxbuf14;
#[doc = "RXBUF15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbuf15](rxbuf15) module"]
pub type RXBUF15 = crate::Reg<u32, _RXBUF15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBUF15;
#[doc = "`read()` method returns [rxbuf15::R](rxbuf15::R) reader structure"]
impl crate::Readable for RXBUF15 {}
#[doc = "`write(|w| ..)` method takes [rxbuf15::W](rxbuf15::W) writer structure"]
impl crate::Writable for RXBUF15 {}
#[doc = "RXBUF15"]
pub mod rxbuf15;
