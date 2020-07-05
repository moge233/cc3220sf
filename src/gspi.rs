#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
    pub hl_rev: HL_REV,
    #[doc = "0x04 - Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
    pub hl_hwinfo: HL_HWINFO,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - 0x4402 1010 0x4402 2010 Clock management configuration"]
    pub hl_sysconfig: HL_SYSCONFIG,
    _reserved3: [u8; 236usize],
    #[doc = "0x100 - 0x4402 1100 0x4402 2100 This register contains the hard coded RTL revision number."]
    pub revision: REVISION,
    _reserved4: [u8; 12usize],
    #[doc = "0x110 - 0x4402 1110 0x4402 2110 This register allows controlling various parameters of the OCP interface."]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x114 - 0x4402 1114 0x4402 2114 This register provides status information about the module excluding the interrupt status information"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x118 - 0x4402 1118 0x4402 2118 The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x11c - 0x4402 111C 0x4402 211C This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis."]
    pub irqenable: IRQENABLE,
    #[doc = "0x120 - 0x4402 1120 0x4402 2120 The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
    pub wakeupenable: WAKEUPENABLE,
    #[doc = "0x124 - 0x4402 1124 0x4402 2124 This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode."]
    pub syst: SYST,
    #[doc = "0x128 - 0x4402 1128 0x4402 2128 This register is dedicated to the configuration of the serial port interface."]
    pub modulctrl: MODULCTRL,
    #[doc = "0x12c - 0x4402 112C 0x4402 212C This register is dedicated to the configuration of the channel 0"]
    pub ch0conf: CH0CONF,
    #[doc = "0x130 - 0x4402 1130 0x4402 2130 This register provides status information about transmitter and receiver registers of channel 0"]
    pub ch0stat: CH0STAT,
    #[doc = "0x134 - 0x4402 1134 0x4402 2134 This register is dedicated to enable the channel 0"]
    pub ch0ctrl: CH0CTRL,
    #[doc = "0x138 - 0x4402 1138 0x4402 2138 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    pub tx0: TX0,
    #[doc = "0x13c - 0x4402 113C 0x4402 213C This register contains a single SPI word received through the serial link what ever SPI word length is."]
    pub rx0: RX0,
    #[doc = "0x140 - 0x4402 1140 0x4402 2140 This register is dedicated to the configuration of the channel."]
    pub ch1conf: CH1CONF,
    #[doc = "0x144 - 0x4402 1144 0x4402 2144 This register provides status information about transmitter and receiver registers of channel 1"]
    pub ch1stat: CH1STAT,
    #[doc = "0x148 - 0x4402 1148 0x4402 2148 This register is dedicated to enable the channel 1"]
    pub ch1ctrl: CH1CTRL,
    #[doc = "0x14c - 0x4402 114C 0x4402 214C This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    pub tx1: TX1,
    #[doc = "0x150 - 0x4402 1150 0x4402 2150 This register contains a single SPI word received through the serial link what ever SPI word length is."]
    pub rx1: RX1,
    #[doc = "0x154 - 0x4402 1154 0x4402 2154 This register is dedicated to the configuration of the channel 2"]
    pub ch2conf: CH2CONF,
    #[doc = "0x158 - 0x4402 1158 0x4402 2158 This register provides status information about transmitter and receiver registers of channel 2"]
    pub ch2stat: CH2STAT,
    #[doc = "0x15c - 0x4402 115C 0x4402 215C This register is dedicated to enable the channel 2"]
    pub ch2ctrl: CH2CTRL,
    #[doc = "0x160 - 0x4402 1160 0x4402 2160 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    pub tx2: TX2,
    #[doc = "0x164 - 0x4402 1164 0x4402 2164 This register contains a single SPI word received through the serial link what ever SPI word length is."]
    pub rx2: RX2,
    #[doc = "0x168 - 0x4402 1168 0x4402 2168 This register is dedicated to the configuration of the channel 3"]
    pub ch3conf: CH3CONF,
    #[doc = "0x16c - 0x4402 116C 0x4402 216C This register provides status information about transmitter and receiver registers of channel 3"]
    pub ch3stat: CH3STAT,
    #[doc = "0x170 - 0x4402 1170 0x4402 2170 This register is dedicated to enable the channel 3"]
    pub ch3ctrl: CH3CTRL,
    #[doc = "0x174 - 0x4402 1174 0x4402 2174 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
    pub tx3: TX3,
    #[doc = "0x178 - 0x4402 1178 0x4402 2178 This register contains a single SPI word received through the serial link what ever SPI word length is."]
    pub rx3: RX3,
    #[doc = "0x17c - 0x4402 117C 0x4402 217C This register provides transfer levels needed while using FIFO buffer during transfer."]
    pub xferlevel: XFERLEVEL,
    #[doc = "0x180 - 0x4402 1180 0x4402 2180 This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
    pub daftx: DAFTX,
    _reserved33: [u8; 28usize],
    #[doc = "0x1a0 - 0x4402 11A0 0x4402 21A0 This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled. ****************************************************************************"]
    pub dafrx: DAFRX,
}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_rev](hl_rev) module"]
pub type HL_REV = crate::Reg<u32, _HL_REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_REV;
#[doc = "`read()` method returns [hl_rev::R](hl_rev::R) reader structure"]
impl crate::Readable for HL_REV {}
#[doc = "`write(|w| ..)` method takes [hl_rev::W](hl_rev::W) writer structure"]
impl crate::Writable for HL_REV {}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
pub mod hl_rev;
#[doc = "Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_hwinfo](hl_hwinfo) module"]
pub type HL_HWINFO = crate::Reg<u32, _HL_HWINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_HWINFO;
#[doc = "`read()` method returns [hl_hwinfo::R](hl_hwinfo::R) reader structure"]
impl crate::Readable for HL_HWINFO {}
#[doc = "`write(|w| ..)` method takes [hl_hwinfo::W](hl_hwinfo::W) writer structure"]
impl crate::Writable for HL_HWINFO {}
#[doc = "Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
pub mod hl_hwinfo;
#[doc = "0x4402 1010 0x4402 2010 Clock management configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_sysconfig](hl_sysconfig) module"]
pub type HL_SYSCONFIG = crate::Reg<u32, _HL_SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_SYSCONFIG;
#[doc = "`read()` method returns [hl_sysconfig::R](hl_sysconfig::R) reader structure"]
impl crate::Readable for HL_SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [hl_sysconfig::W](hl_sysconfig::W) writer structure"]
impl crate::Writable for HL_SYSCONFIG {}
#[doc = "0x4402 1010 0x4402 2010 Clock management configuration"]
pub mod hl_sysconfig;
#[doc = "0x4402 1100 0x4402 2100 This register contains the hard coded RTL revision number.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "`write(|w| ..)` method takes [revision::W](revision::W) writer structure"]
impl crate::Writable for REVISION {}
#[doc = "0x4402 1100 0x4402 2100 This register contains the hard coded RTL revision number."]
pub mod revision;
#[doc = "0x4402 1110 0x4402 2110 This register allows controlling various parameters of the OCP interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](sysconfig) module"]
pub type SYSCONFIG = crate::Reg<u32, _SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCONFIG;
#[doc = "`read()` method returns [sysconfig::R](sysconfig::R) reader structure"]
impl crate::Readable for SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](sysconfig::W) writer structure"]
impl crate::Writable for SYSCONFIG {}
#[doc = "0x4402 1110 0x4402 2110 This register allows controlling various parameters of the OCP interface."]
pub mod sysconfig;
#[doc = "0x4402 1114 0x4402 2114 This register provides status information about the module excluding the interrupt status information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysstatus](sysstatus) module"]
pub type SYSSTATUS = crate::Reg<u32, _SYSSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSTATUS;
#[doc = "`read()` method returns [sysstatus::R](sysstatus::R) reader structure"]
impl crate::Readable for SYSSTATUS {}
#[doc = "`write(|w| ..)` method takes [sysstatus::W](sysstatus::W) writer structure"]
impl crate::Writable for SYSSTATUS {}
#[doc = "0x4402 1114 0x4402 2114 This register provides status information about the module excluding the interrupt status information"]
pub mod sysstatus;
#[doc = "0x4402 1118 0x4402 2118 The interrupt status regroups all the status of the module internal events that can generate an interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](irqstatus) module"]
pub type IRQSTATUS = crate::Reg<u32, _IRQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATUS;
#[doc = "`read()` method returns [irqstatus::R](irqstatus::R) reader structure"]
impl crate::Readable for IRQSTATUS {}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](irqstatus::W) writer structure"]
impl crate::Writable for IRQSTATUS {}
#[doc = "0x4402 1118 0x4402 2118 The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
pub mod irqstatus;
#[doc = "0x4402 111C 0x4402 211C This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](irqenable) module"]
pub type IRQENABLE = crate::Reg<u32, _IRQENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQENABLE;
#[doc = "`read()` method returns [irqenable::R](irqenable::R) reader structure"]
impl crate::Readable for IRQENABLE {}
#[doc = "`write(|w| ..)` method takes [irqenable::W](irqenable::W) writer structure"]
impl crate::Writable for IRQENABLE {}
#[doc = "0x4402 111C 0x4402 211C This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis."]
pub mod irqenable;
#[doc = "0x4402 1120 0x4402 2120 The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeupenable](wakeupenable) module"]
pub type WAKEUPENABLE = crate::Reg<u32, _WAKEUPENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUPENABLE;
#[doc = "`read()` method returns [wakeupenable::R](wakeupenable::R) reader structure"]
impl crate::Readable for WAKEUPENABLE {}
#[doc = "`write(|w| ..)` method takes [wakeupenable::W](wakeupenable::W) writer structure"]
impl crate::Writable for WAKEUPENABLE {}
#[doc = "0x4402 1120 0x4402 2120 The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
pub mod wakeupenable;
#[doc = "0x4402 1124 0x4402 2124 This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst](syst) module"]
pub type SYST = crate::Reg<u32, _SYST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST;
#[doc = "`read()` method returns [syst::R](syst::R) reader structure"]
impl crate::Readable for SYST {}
#[doc = "`write(|w| ..)` method takes [syst::W](syst::W) writer structure"]
impl crate::Writable for SYST {}
#[doc = "0x4402 1124 0x4402 2124 This register is used to check the correctness of the system interconnect either internally to peripheral bus or externally to device IO pads when the module is configured in system test (SYSTEST) mode."]
pub mod syst;
#[doc = "0x4402 1128 0x4402 2128 This register is dedicated to the configuration of the serial port interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modulctrl](modulctrl) module"]
pub type MODULCTRL = crate::Reg<u32, _MODULCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULCTRL;
#[doc = "`read()` method returns [modulctrl::R](modulctrl::R) reader structure"]
impl crate::Readable for MODULCTRL {}
#[doc = "`write(|w| ..)` method takes [modulctrl::W](modulctrl::W) writer structure"]
impl crate::Writable for MODULCTRL {}
#[doc = "0x4402 1128 0x4402 2128 This register is dedicated to the configuration of the serial port interface."]
pub mod modulctrl;
#[doc = "0x4402 112C 0x4402 212C This register is dedicated to the configuration of the channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0conf](ch0conf) module"]
pub type CH0CONF = crate::Reg<u32, _CH0CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CONF;
#[doc = "`read()` method returns [ch0conf::R](ch0conf::R) reader structure"]
impl crate::Readable for CH0CONF {}
#[doc = "`write(|w| ..)` method takes [ch0conf::W](ch0conf::W) writer structure"]
impl crate::Writable for CH0CONF {}
#[doc = "0x4402 112C 0x4402 212C This register is dedicated to the configuration of the channel 0"]
pub mod ch0conf;
#[doc = "0x4402 1130 0x4402 2130 This register provides status information about transmitter and receiver registers of channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0stat](ch0stat) module"]
pub type CH0STAT = crate::Reg<u32, _CH0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STAT;
#[doc = "`read()` method returns [ch0stat::R](ch0stat::R) reader structure"]
impl crate::Readable for CH0STAT {}
#[doc = "`write(|w| ..)` method takes [ch0stat::W](ch0stat::W) writer structure"]
impl crate::Writable for CH0STAT {}
#[doc = "0x4402 1130 0x4402 2130 This register provides status information about transmitter and receiver registers of channel 0"]
pub mod ch0stat;
#[doc = "0x4402 1134 0x4402 2134 This register is dedicated to enable the channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ctrl](ch0ctrl) module"]
pub type CH0CTRL = crate::Reg<u32, _CH0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CTRL;
#[doc = "`read()` method returns [ch0ctrl::R](ch0ctrl::R) reader structure"]
impl crate::Readable for CH0CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0ctrl::W](ch0ctrl::W) writer structure"]
impl crate::Writable for CH0CTRL {}
#[doc = "0x4402 1134 0x4402 2134 This register is dedicated to enable the channel 0"]
pub mod ch0ctrl;
#[doc = "0x4402 1138 0x4402 2138 This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx0](tx0) module"]
pub type TX0 = crate::Reg<u32, _TX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX0;
#[doc = "`read()` method returns [tx0::R](tx0::R) reader structure"]
impl crate::Readable for TX0 {}
#[doc = "`write(|w| ..)` method takes [tx0::W](tx0::W) writer structure"]
impl crate::Writable for TX0 {}
#[doc = "0x4402 1138 0x4402 2138 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx0;
#[doc = "0x4402 113C 0x4402 213C This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx0](rx0) module"]
pub type RX0 = crate::Reg<u32, _RX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX0;
#[doc = "`read()` method returns [rx0::R](rx0::R) reader structure"]
impl crate::Readable for RX0 {}
#[doc = "`write(|w| ..)` method takes [rx0::W](rx0::W) writer structure"]
impl crate::Writable for RX0 {}
#[doc = "0x4402 113C 0x4402 213C This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx0;
#[doc = "0x4402 1140 0x4402 2140 This register is dedicated to the configuration of the channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1conf](ch1conf) module"]
pub type CH1CONF = crate::Reg<u32, _CH1CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CONF;
#[doc = "`read()` method returns [ch1conf::R](ch1conf::R) reader structure"]
impl crate::Readable for CH1CONF {}
#[doc = "`write(|w| ..)` method takes [ch1conf::W](ch1conf::W) writer structure"]
impl crate::Writable for CH1CONF {}
#[doc = "0x4402 1140 0x4402 2140 This register is dedicated to the configuration of the channel."]
pub mod ch1conf;
#[doc = "0x4402 1144 0x4402 2144 This register provides status information about transmitter and receiver registers of channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1stat](ch1stat) module"]
pub type CH1STAT = crate::Reg<u32, _CH1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STAT;
#[doc = "`read()` method returns [ch1stat::R](ch1stat::R) reader structure"]
impl crate::Readable for CH1STAT {}
#[doc = "`write(|w| ..)` method takes [ch1stat::W](ch1stat::W) writer structure"]
impl crate::Writable for CH1STAT {}
#[doc = "0x4402 1144 0x4402 2144 This register provides status information about transmitter and receiver registers of channel 1"]
pub mod ch1stat;
#[doc = "0x4402 1148 0x4402 2148 This register is dedicated to enable the channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ctrl](ch1ctrl) module"]
pub type CH1CTRL = crate::Reg<u32, _CH1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CTRL;
#[doc = "`read()` method returns [ch1ctrl::R](ch1ctrl::R) reader structure"]
impl crate::Readable for CH1CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1ctrl::W](ch1ctrl::W) writer structure"]
impl crate::Writable for CH1CTRL {}
#[doc = "0x4402 1148 0x4402 2148 This register is dedicated to enable the channel 1"]
pub mod ch1ctrl;
#[doc = "0x4402 114C 0x4402 214C This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx1](tx1) module"]
pub type TX1 = crate::Reg<u32, _TX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX1;
#[doc = "`read()` method returns [tx1::R](tx1::R) reader structure"]
impl crate::Readable for TX1 {}
#[doc = "`write(|w| ..)` method takes [tx1::W](tx1::W) writer structure"]
impl crate::Writable for TX1 {}
#[doc = "0x4402 114C 0x4402 214C This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx1;
#[doc = "0x4402 1150 0x4402 2150 This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx1](rx1) module"]
pub type RX1 = crate::Reg<u32, _RX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX1;
#[doc = "`read()` method returns [rx1::R](rx1::R) reader structure"]
impl crate::Readable for RX1 {}
#[doc = "`write(|w| ..)` method takes [rx1::W](rx1::W) writer structure"]
impl crate::Writable for RX1 {}
#[doc = "0x4402 1150 0x4402 2150 This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx1;
#[doc = "0x4402 1154 0x4402 2154 This register is dedicated to the configuration of the channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2conf](ch2conf) module"]
pub type CH2CONF = crate::Reg<u32, _CH2CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CONF;
#[doc = "`read()` method returns [ch2conf::R](ch2conf::R) reader structure"]
impl crate::Readable for CH2CONF {}
#[doc = "`write(|w| ..)` method takes [ch2conf::W](ch2conf::W) writer structure"]
impl crate::Writable for CH2CONF {}
#[doc = "0x4402 1154 0x4402 2154 This register is dedicated to the configuration of the channel 2"]
pub mod ch2conf;
#[doc = "0x4402 1158 0x4402 2158 This register provides status information about transmitter and receiver registers of channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2stat](ch2stat) module"]
pub type CH2STAT = crate::Reg<u32, _CH2STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STAT;
#[doc = "`read()` method returns [ch2stat::R](ch2stat::R) reader structure"]
impl crate::Readable for CH2STAT {}
#[doc = "`write(|w| ..)` method takes [ch2stat::W](ch2stat::W) writer structure"]
impl crate::Writable for CH2STAT {}
#[doc = "0x4402 1158 0x4402 2158 This register provides status information about transmitter and receiver registers of channel 2"]
pub mod ch2stat;
#[doc = "0x4402 115C 0x4402 215C This register is dedicated to enable the channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ctrl](ch2ctrl) module"]
pub type CH2CTRL = crate::Reg<u32, _CH2CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CTRL;
#[doc = "`read()` method returns [ch2ctrl::R](ch2ctrl::R) reader structure"]
impl crate::Readable for CH2CTRL {}
#[doc = "`write(|w| ..)` method takes [ch2ctrl::W](ch2ctrl::W) writer structure"]
impl crate::Writable for CH2CTRL {}
#[doc = "0x4402 115C 0x4402 215C This register is dedicated to enable the channel 2"]
pub mod ch2ctrl;
#[doc = "0x4402 1160 0x4402 2160 This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx2](tx2) module"]
pub type TX2 = crate::Reg<u32, _TX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX2;
#[doc = "`read()` method returns [tx2::R](tx2::R) reader structure"]
impl crate::Readable for TX2 {}
#[doc = "`write(|w| ..)` method takes [tx2::W](tx2::W) writer structure"]
impl crate::Writable for TX2 {}
#[doc = "0x4402 1160 0x4402 2160 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx2;
#[doc = "0x4402 1164 0x4402 2164 This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx2](rx2) module"]
pub type RX2 = crate::Reg<u32, _RX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX2;
#[doc = "`read()` method returns [rx2::R](rx2::R) reader structure"]
impl crate::Readable for RX2 {}
#[doc = "`write(|w| ..)` method takes [rx2::W](rx2::W) writer structure"]
impl crate::Writable for RX2 {}
#[doc = "0x4402 1164 0x4402 2164 This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx2;
#[doc = "0x4402 1168 0x4402 2168 This register is dedicated to the configuration of the channel 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf](ch3conf) module"]
pub type CH3CONF = crate::Reg<u32, _CH3CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CONF;
#[doc = "`read()` method returns [ch3conf::R](ch3conf::R) reader structure"]
impl crate::Readable for CH3CONF {}
#[doc = "`write(|w| ..)` method takes [ch3conf::W](ch3conf::W) writer structure"]
impl crate::Writable for CH3CONF {}
#[doc = "0x4402 1168 0x4402 2168 This register is dedicated to the configuration of the channel 3"]
pub mod ch3conf;
#[doc = "0x4402 116C 0x4402 216C This register provides status information about transmitter and receiver registers of channel 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat](ch3stat) module"]
pub type CH3STAT = crate::Reg<u32, _CH3STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STAT;
#[doc = "`read()` method returns [ch3stat::R](ch3stat::R) reader structure"]
impl crate::Readable for CH3STAT {}
#[doc = "`write(|w| ..)` method takes [ch3stat::W](ch3stat::W) writer structure"]
impl crate::Writable for CH3STAT {}
#[doc = "0x4402 116C 0x4402 216C This register provides status information about transmitter and receiver registers of channel 3"]
pub mod ch3stat;
#[doc = "0x4402 1170 0x4402 2170 This register is dedicated to enable the channel 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ctrl](ch3ctrl) module"]
pub type CH3CTRL = crate::Reg<u32, _CH3CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CTRL;
#[doc = "`read()` method returns [ch3ctrl::R](ch3ctrl::R) reader structure"]
impl crate::Readable for CH3CTRL {}
#[doc = "`write(|w| ..)` method takes [ch3ctrl::W](ch3ctrl::W) writer structure"]
impl crate::Writable for CH3CTRL {}
#[doc = "0x4402 1170 0x4402 2170 This register is dedicated to enable the channel 3"]
pub mod ch3ctrl;
#[doc = "0x4402 1174 0x4402 2174 This register contains a single SPI word to transmit on the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx3](tx3) module"]
pub type TX3 = crate::Reg<u32, _TX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX3;
#[doc = "`read()` method returns [tx3::R](tx3::R) reader structure"]
impl crate::Readable for TX3 {}
#[doc = "`write(|w| ..)` method takes [tx3::W](tx3::W) writer structure"]
impl crate::Writable for TX3 {}
#[doc = "0x4402 1174 0x4402 2174 This register contains a single SPI word to transmit on the serial link what ever SPI word length is."]
pub mod tx3;
#[doc = "0x4402 1178 0x4402 2178 This register contains a single SPI word received through the serial link what ever SPI word length is.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx3](rx3) module"]
pub type RX3 = crate::Reg<u32, _RX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX3;
#[doc = "`read()` method returns [rx3::R](rx3::R) reader structure"]
impl crate::Readable for RX3 {}
#[doc = "`write(|w| ..)` method takes [rx3::W](rx3::W) writer structure"]
impl crate::Writable for RX3 {}
#[doc = "0x4402 1178 0x4402 2178 This register contains a single SPI word received through the serial link what ever SPI word length is."]
pub mod rx3;
#[doc = "0x4402 117C 0x4402 217C This register provides transfer levels needed while using FIFO buffer during transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xferlevel](xferlevel) module"]
pub type XFERLEVEL = crate::Reg<u32, _XFERLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XFERLEVEL;
#[doc = "`read()` method returns [xferlevel::R](xferlevel::R) reader structure"]
impl crate::Readable for XFERLEVEL {}
#[doc = "`write(|w| ..)` method takes [xferlevel::W](xferlevel::W) writer structure"]
impl crate::Writable for XFERLEVEL {}
#[doc = "0x4402 117C 0x4402 217C This register provides transfer levels needed while using FIFO buffer during transfer."]
pub mod xferlevel;
#[doc = "0x4402 1180 0x4402 2180 This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daftx](daftx) module"]
pub type DAFTX = crate::Reg<u32, _DAFTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAFTX;
#[doc = "`read()` method returns [daftx::R](daftx::R) reader structure"]
impl crate::Readable for DAFTX {}
#[doc = "`write(|w| ..)` method takes [daftx::W](daftx::W) writer structure"]
impl crate::Writable for DAFTX {}
#[doc = "0x4402 1180 0x4402 2180 This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
pub mod daftx;
#[doc = "0x4402 11A0 0x4402 21A0 This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled. ****************************************************************************\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dafrx](dafrx) module"]
pub type DAFRX = crate::Reg<u32, _DAFRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAFRX;
#[doc = "`read()` method returns [dafrx::R](dafrx::R) reader structure"]
impl crate::Readable for DAFRX {}
#[doc = "`write(|w| ..)` method takes [dafrx::W](dafrx::W) writer structure"]
impl crate::Writable for DAFRX {}
#[doc = "0x4402 11A0 0x4402 21A0 This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled. ****************************************************************************"]
pub mod dafrx;
