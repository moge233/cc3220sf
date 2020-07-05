#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC control register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Channel 0 gain setting"]
    pub ch0_gain: CH0_GAIN,
    #[doc = "0x08 - Channel 1 gain setting"]
    pub ch1_gain: CH1_GAIN,
    #[doc = "0x0c - Channel 2 gain setting"]
    pub ch2_gain: CH2_GAIN,
    #[doc = "0x10 - Channel 3 gain setting"]
    pub ch3_gain: CH3_GAIN,
    #[doc = "0x14 - Channel 4 gain setting"]
    pub ch4_gain: CH4_GAIN,
    #[doc = "0x18 - Channel 5 gain setting"]
    pub ch5_gain: CH5_GAIN,
    #[doc = "0x1c - Channel 6 gain setting"]
    pub ch6_gain: CH6_GAIN,
    #[doc = "0x20 - Channel 7 gain setting"]
    pub ch7_gain: CH7_GAIN,
    #[doc = "0x24 - Channel 0 interrupt enable register"]
    pub ch0_irq_en: CH0_IRQ_EN,
    #[doc = "0x28 - Channel 1 interrupt enable register"]
    pub ch1_irq_en: CH1_IRQ_EN,
    #[doc = "0x2c - Channel 2 interrupt enable register"]
    pub ch2_irq_en: CH2_IRQ_EN,
    #[doc = "0x30 - Channel 3 interrupt enable register"]
    pub ch3_irq_en: CH3_IRQ_EN,
    #[doc = "0x34 - Channel 4 interrupt enable register"]
    pub ch4_irq_en: CH4_IRQ_EN,
    #[doc = "0x38 - Channel 5 interrupt enable register"]
    pub ch5_irq_en: CH5_IRQ_EN,
    #[doc = "0x3c - Channel 6 interrupt enable register"]
    pub ch6_irq_en: CH6_IRQ_EN,
    #[doc = "0x40 - Channel 7 interrupt enable register"]
    pub ch7_irq_en: CH7_IRQ_EN,
    #[doc = "0x44 - Channel 0 interrupt status register"]
    pub ch0_irq_status: CH0_IRQ_STATUS,
    #[doc = "0x48 - Channel 1 interrupt status register"]
    pub ch1_irq_status: CH1_IRQ_STATUS,
    #[doc = "0x4c - CH2_IRQ_STATUS"]
    pub ch2_irq_status: CH2_IRQ_STATUS,
    #[doc = "0x50 - Channel 3 interrupt status register"]
    pub ch3_irq_status: CH3_IRQ_STATUS,
    #[doc = "0x54 - Channel 4 interrupt status register"]
    pub ch4_irq_status: CH4_IRQ_STATUS,
    #[doc = "0x58 - CH5_IRQ_STATUS"]
    pub ch5_irq_status: CH5_IRQ_STATUS,
    #[doc = "0x5c - Channel 6 interrupt status register"]
    pub ch6_irq_status: CH6_IRQ_STATUS,
    #[doc = "0x60 - Channel 7 interrupt status register"]
    pub ch7_irq_status: CH7_IRQ_STATUS,
    #[doc = "0x64 - DMA mode enable register"]
    pub dma_mode_en: DMA_MODE_EN,
    #[doc = "0x68 - ADC timer configuration register"]
    pub timer_configuration: TIMER_CONFIGURATION,
    _reserved27: [u8; 4usize],
    #[doc = "0x70 - ADC timer current count register"]
    pub timer_current_count: TIMER_CURRENT_COUNT,
    #[doc = "0x74 - CH0 FIFO DATA register"]
    pub channel0fifodata: CHANNEL0FIFODATA,
    #[doc = "0x78 - CH1 FIFO DATA register"]
    pub channel1fifodata: CHANNEL1FIFODATA,
    #[doc = "0x7c - CH2 FIFO DATA register"]
    pub channel2fifodata: CHANNEL2FIFODATA,
    #[doc = "0x80 - CH3 FIFO DATA register"]
    pub channel3fifodata: CHANNEL3FIFODATA,
    #[doc = "0x84 - CH4 FIFO DATA register"]
    pub channel4fifodata: CHANNEL4FIFODATA,
    #[doc = "0x88 - CH5 FIFO DATA register"]
    pub channel5fifodata: CHANNEL5FIFODATA,
    #[doc = "0x8c - CH6 FIFO DATA register"]
    pub channel6fifodata: CHANNEL6FIFODATA,
    #[doc = "0x90 - CH7 FIFO DATA register"]
    pub channel7fifodata: CHANNEL7FIFODATA,
    #[doc = "0x94 - channel 0 FIFO Level register"]
    pub ch0_fifo_lvl: CH0_FIFO_LVL,
    #[doc = "0x98 - Channel 1 interrupt status register"]
    pub ch1_fifo_lvl: CH1_FIFO_LVL,
    #[doc = "0x9c - CH2_FIFO_LVL"]
    pub ch2_fifo_lvl: CH2_FIFO_LVL,
    #[doc = "0xa0 - Channel 3 interrupt status register"]
    pub ch3_fifo_lvl: CH3_FIFO_LVL,
    #[doc = "0xa4 - Channel 4 interrupt status register"]
    pub ch4_fifo_lvl: CH4_FIFO_LVL,
    #[doc = "0xa8 - CH5_FIFO_LVL"]
    pub ch5_fifo_lvl: CH5_FIFO_LVL,
    #[doc = "0xac - Channel 6 interrupt status register"]
    pub ch6_fifo_lvl: CH6_FIFO_LVL,
    #[doc = "0xb0 - Channel 7 interrupt status register"]
    pub ch7_fifo_lvl: CH7_FIFO_LVL,
    _reserved44: [u8; 4usize],
    #[doc = "0xb8 - CH_ENABLE"]
    pub ch_enable: CH_ENABLE,
}
#[doc = "ADC control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "ADC control register."]
pub mod ctrl;
#[doc = "Channel 0 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_gain](ch0_gain) module"]
pub type CH0_GAIN = crate::Reg<u32, _CH0_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_GAIN;
#[doc = "`read()` method returns [ch0_gain::R](ch0_gain::R) reader structure"]
impl crate::Readable for CH0_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch0_gain::W](ch0_gain::W) writer structure"]
impl crate::Writable for CH0_GAIN {}
#[doc = "Channel 0 gain setting"]
pub mod ch0_gain;
#[doc = "Channel 1 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_gain](ch1_gain) module"]
pub type CH1_GAIN = crate::Reg<u32, _CH1_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_GAIN;
#[doc = "`read()` method returns [ch1_gain::R](ch1_gain::R) reader structure"]
impl crate::Readable for CH1_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch1_gain::W](ch1_gain::W) writer structure"]
impl crate::Writable for CH1_GAIN {}
#[doc = "Channel 1 gain setting"]
pub mod ch1_gain;
#[doc = "Channel 2 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_gain](ch2_gain) module"]
pub type CH2_GAIN = crate::Reg<u32, _CH2_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_GAIN;
#[doc = "`read()` method returns [ch2_gain::R](ch2_gain::R) reader structure"]
impl crate::Readable for CH2_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch2_gain::W](ch2_gain::W) writer structure"]
impl crate::Writable for CH2_GAIN {}
#[doc = "Channel 2 gain setting"]
pub mod ch2_gain;
#[doc = "Channel 3 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_gain](ch3_gain) module"]
pub type CH3_GAIN = crate::Reg<u32, _CH3_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_GAIN;
#[doc = "`read()` method returns [ch3_gain::R](ch3_gain::R) reader structure"]
impl crate::Readable for CH3_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch3_gain::W](ch3_gain::W) writer structure"]
impl crate::Writable for CH3_GAIN {}
#[doc = "Channel 3 gain setting"]
pub mod ch3_gain;
#[doc = "Channel 4 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_gain](ch4_gain) module"]
pub type CH4_GAIN = crate::Reg<u32, _CH4_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_GAIN;
#[doc = "`read()` method returns [ch4_gain::R](ch4_gain::R) reader structure"]
impl crate::Readable for CH4_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch4_gain::W](ch4_gain::W) writer structure"]
impl crate::Writable for CH4_GAIN {}
#[doc = "Channel 4 gain setting"]
pub mod ch4_gain;
#[doc = "Channel 5 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_gain](ch5_gain) module"]
pub type CH5_GAIN = crate::Reg<u32, _CH5_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_GAIN;
#[doc = "`read()` method returns [ch5_gain::R](ch5_gain::R) reader structure"]
impl crate::Readable for CH5_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch5_gain::W](ch5_gain::W) writer structure"]
impl crate::Writable for CH5_GAIN {}
#[doc = "Channel 5 gain setting"]
pub mod ch5_gain;
#[doc = "Channel 6 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_gain](ch6_gain) module"]
pub type CH6_GAIN = crate::Reg<u32, _CH6_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_GAIN;
#[doc = "`read()` method returns [ch6_gain::R](ch6_gain::R) reader structure"]
impl crate::Readable for CH6_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch6_gain::W](ch6_gain::W) writer structure"]
impl crate::Writable for CH6_GAIN {}
#[doc = "Channel 6 gain setting"]
pub mod ch6_gain;
#[doc = "Channel 7 gain setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_gain](ch7_gain) module"]
pub type CH7_GAIN = crate::Reg<u32, _CH7_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_GAIN;
#[doc = "`read()` method returns [ch7_gain::R](ch7_gain::R) reader structure"]
impl crate::Readable for CH7_GAIN {}
#[doc = "`write(|w| ..)` method takes [ch7_gain::W](ch7_gain::W) writer structure"]
impl crate::Writable for CH7_GAIN {}
#[doc = "Channel 7 gain setting"]
pub mod ch7_gain;
#[doc = "Channel 0 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_irq_en](ch0_irq_en) module"]
pub type CH0_IRQ_EN = crate::Reg<u32, _CH0_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_IRQ_EN;
#[doc = "`read()` method returns [ch0_irq_en::R](ch0_irq_en::R) reader structure"]
impl crate::Readable for CH0_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch0_irq_en::W](ch0_irq_en::W) writer structure"]
impl crate::Writable for CH0_IRQ_EN {}
#[doc = "Channel 0 interrupt enable register"]
pub mod ch0_irq_en;
#[doc = "Channel 1 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_irq_en](ch1_irq_en) module"]
pub type CH1_IRQ_EN = crate::Reg<u32, _CH1_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_IRQ_EN;
#[doc = "`read()` method returns [ch1_irq_en::R](ch1_irq_en::R) reader structure"]
impl crate::Readable for CH1_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch1_irq_en::W](ch1_irq_en::W) writer structure"]
impl crate::Writable for CH1_IRQ_EN {}
#[doc = "Channel 1 interrupt enable register"]
pub mod ch1_irq_en;
#[doc = "Channel 2 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_irq_en](ch2_irq_en) module"]
pub type CH2_IRQ_EN = crate::Reg<u32, _CH2_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_IRQ_EN;
#[doc = "`read()` method returns [ch2_irq_en::R](ch2_irq_en::R) reader structure"]
impl crate::Readable for CH2_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch2_irq_en::W](ch2_irq_en::W) writer structure"]
impl crate::Writable for CH2_IRQ_EN {}
#[doc = "Channel 2 interrupt enable register"]
pub mod ch2_irq_en;
#[doc = "Channel 3 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_irq_en](ch3_irq_en) module"]
pub type CH3_IRQ_EN = crate::Reg<u32, _CH3_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_IRQ_EN;
#[doc = "`read()` method returns [ch3_irq_en::R](ch3_irq_en::R) reader structure"]
impl crate::Readable for CH3_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch3_irq_en::W](ch3_irq_en::W) writer structure"]
impl crate::Writable for CH3_IRQ_EN {}
#[doc = "Channel 3 interrupt enable register"]
pub mod ch3_irq_en;
#[doc = "Channel 4 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_irq_en](ch4_irq_en) module"]
pub type CH4_IRQ_EN = crate::Reg<u32, _CH4_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_IRQ_EN;
#[doc = "`read()` method returns [ch4_irq_en::R](ch4_irq_en::R) reader structure"]
impl crate::Readable for CH4_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch4_irq_en::W](ch4_irq_en::W) writer structure"]
impl crate::Writable for CH4_IRQ_EN {}
#[doc = "Channel 4 interrupt enable register"]
pub mod ch4_irq_en;
#[doc = "Channel 5 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_irq_en](ch5_irq_en) module"]
pub type CH5_IRQ_EN = crate::Reg<u32, _CH5_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_IRQ_EN;
#[doc = "`read()` method returns [ch5_irq_en::R](ch5_irq_en::R) reader structure"]
impl crate::Readable for CH5_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch5_irq_en::W](ch5_irq_en::W) writer structure"]
impl crate::Writable for CH5_IRQ_EN {}
#[doc = "Channel 5 interrupt enable register"]
pub mod ch5_irq_en;
#[doc = "Channel 6 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_irq_en](ch6_irq_en) module"]
pub type CH6_IRQ_EN = crate::Reg<u32, _CH6_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_IRQ_EN;
#[doc = "`read()` method returns [ch6_irq_en::R](ch6_irq_en::R) reader structure"]
impl crate::Readable for CH6_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch6_irq_en::W](ch6_irq_en::W) writer structure"]
impl crate::Writable for CH6_IRQ_EN {}
#[doc = "Channel 6 interrupt enable register"]
pub mod ch6_irq_en;
#[doc = "Channel 7 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_irq_en](ch7_irq_en) module"]
pub type CH7_IRQ_EN = crate::Reg<u32, _CH7_IRQ_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_IRQ_EN;
#[doc = "`read()` method returns [ch7_irq_en::R](ch7_irq_en::R) reader structure"]
impl crate::Readable for CH7_IRQ_EN {}
#[doc = "`write(|w| ..)` method takes [ch7_irq_en::W](ch7_irq_en::W) writer structure"]
impl crate::Writable for CH7_IRQ_EN {}
#[doc = "Channel 7 interrupt enable register"]
pub mod ch7_irq_en;
#[doc = "Channel 0 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_irq_status](ch0_irq_status) module"]
pub type CH0_IRQ_STATUS = crate::Reg<u32, _CH0_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_IRQ_STATUS;
#[doc = "`read()` method returns [ch0_irq_status::R](ch0_irq_status::R) reader structure"]
impl crate::Readable for CH0_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch0_irq_status::W](ch0_irq_status::W) writer structure"]
impl crate::Writable for CH0_IRQ_STATUS {}
#[doc = "Channel 0 interrupt status register"]
pub mod ch0_irq_status;
#[doc = "Channel 1 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_irq_status](ch1_irq_status) module"]
pub type CH1_IRQ_STATUS = crate::Reg<u32, _CH1_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_IRQ_STATUS;
#[doc = "`read()` method returns [ch1_irq_status::R](ch1_irq_status::R) reader structure"]
impl crate::Readable for CH1_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch1_irq_status::W](ch1_irq_status::W) writer structure"]
impl crate::Writable for CH1_IRQ_STATUS {}
#[doc = "Channel 1 interrupt status register"]
pub mod ch1_irq_status;
#[doc = "CH2_IRQ_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_irq_status](ch2_irq_status) module"]
pub type CH2_IRQ_STATUS = crate::Reg<u32, _CH2_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_IRQ_STATUS;
#[doc = "`read()` method returns [ch2_irq_status::R](ch2_irq_status::R) reader structure"]
impl crate::Readable for CH2_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch2_irq_status::W](ch2_irq_status::W) writer structure"]
impl crate::Writable for CH2_IRQ_STATUS {}
#[doc = "CH2_IRQ_STATUS"]
pub mod ch2_irq_status;
#[doc = "Channel 3 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_irq_status](ch3_irq_status) module"]
pub type CH3_IRQ_STATUS = crate::Reg<u32, _CH3_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_IRQ_STATUS;
#[doc = "`read()` method returns [ch3_irq_status::R](ch3_irq_status::R) reader structure"]
impl crate::Readable for CH3_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch3_irq_status::W](ch3_irq_status::W) writer structure"]
impl crate::Writable for CH3_IRQ_STATUS {}
#[doc = "Channel 3 interrupt status register"]
pub mod ch3_irq_status;
#[doc = "Channel 4 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_irq_status](ch4_irq_status) module"]
pub type CH4_IRQ_STATUS = crate::Reg<u32, _CH4_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_IRQ_STATUS;
#[doc = "`read()` method returns [ch4_irq_status::R](ch4_irq_status::R) reader structure"]
impl crate::Readable for CH4_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch4_irq_status::W](ch4_irq_status::W) writer structure"]
impl crate::Writable for CH4_IRQ_STATUS {}
#[doc = "Channel 4 interrupt status register"]
pub mod ch4_irq_status;
#[doc = "CH5_IRQ_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_irq_status](ch5_irq_status) module"]
pub type CH5_IRQ_STATUS = crate::Reg<u32, _CH5_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_IRQ_STATUS;
#[doc = "`read()` method returns [ch5_irq_status::R](ch5_irq_status::R) reader structure"]
impl crate::Readable for CH5_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch5_irq_status::W](ch5_irq_status::W) writer structure"]
impl crate::Writable for CH5_IRQ_STATUS {}
#[doc = "CH5_IRQ_STATUS"]
pub mod ch5_irq_status;
#[doc = "Channel 6 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_irq_status](ch6_irq_status) module"]
pub type CH6_IRQ_STATUS = crate::Reg<u32, _CH6_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_IRQ_STATUS;
#[doc = "`read()` method returns [ch6_irq_status::R](ch6_irq_status::R) reader structure"]
impl crate::Readable for CH6_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch6_irq_status::W](ch6_irq_status::W) writer structure"]
impl crate::Writable for CH6_IRQ_STATUS {}
#[doc = "Channel 6 interrupt status register"]
pub mod ch6_irq_status;
#[doc = "Channel 7 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_irq_status](ch7_irq_status) module"]
pub type CH7_IRQ_STATUS = crate::Reg<u32, _CH7_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_IRQ_STATUS;
#[doc = "`read()` method returns [ch7_irq_status::R](ch7_irq_status::R) reader structure"]
impl crate::Readable for CH7_IRQ_STATUS {}
#[doc = "`write(|w| ..)` method takes [ch7_irq_status::W](ch7_irq_status::W) writer structure"]
impl crate::Writable for CH7_IRQ_STATUS {}
#[doc = "Channel 7 interrupt status register"]
pub mod ch7_irq_status;
#[doc = "DMA mode enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_mode_en](dma_mode_en) module"]
pub type DMA_MODE_EN = crate::Reg<u32, _DMA_MODE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_MODE_EN;
#[doc = "`read()` method returns [dma_mode_en::R](dma_mode_en::R) reader structure"]
impl crate::Readable for DMA_MODE_EN {}
#[doc = "`write(|w| ..)` method takes [dma_mode_en::W](dma_mode_en::W) writer structure"]
impl crate::Writable for DMA_MODE_EN {}
#[doc = "DMA mode enable register"]
pub mod dma_mode_en;
#[doc = "ADC timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_configuration](timer_configuration) module"]
pub type TIMER_CONFIGURATION = crate::Reg<u32, _TIMER_CONFIGURATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CONFIGURATION;
#[doc = "`read()` method returns [timer_configuration::R](timer_configuration::R) reader structure"]
impl crate::Readable for TIMER_CONFIGURATION {}
#[doc = "`write(|w| ..)` method takes [timer_configuration::W](timer_configuration::W) writer structure"]
impl crate::Writable for TIMER_CONFIGURATION {}
#[doc = "ADC timer configuration register"]
pub mod timer_configuration;
#[doc = "ADC timer current count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_current_count](timer_current_count) module"]
pub type TIMER_CURRENT_COUNT = crate::Reg<u32, _TIMER_CURRENT_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CURRENT_COUNT;
#[doc = "`read()` method returns [timer_current_count::R](timer_current_count::R) reader structure"]
impl crate::Readable for TIMER_CURRENT_COUNT {}
#[doc = "`write(|w| ..)` method takes [timer_current_count::W](timer_current_count::W) writer structure"]
impl crate::Writable for TIMER_CURRENT_COUNT {}
#[doc = "ADC timer current count register"]
pub mod timer_current_count;
#[doc = "CH0 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel0fifodata](channel0fifodata) module"]
pub type CHANNEL0FIFODATA = crate::Reg<u32, _CHANNEL0FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL0FIFODATA;
#[doc = "`read()` method returns [channel0fifodata::R](channel0fifodata::R) reader structure"]
impl crate::Readable for CHANNEL0FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel0fifodata::W](channel0fifodata::W) writer structure"]
impl crate::Writable for CHANNEL0FIFODATA {}
#[doc = "CH0 FIFO DATA register"]
pub mod channel0fifodata;
#[doc = "CH1 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel1fifodata](channel1fifodata) module"]
pub type CHANNEL1FIFODATA = crate::Reg<u32, _CHANNEL1FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL1FIFODATA;
#[doc = "`read()` method returns [channel1fifodata::R](channel1fifodata::R) reader structure"]
impl crate::Readable for CHANNEL1FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel1fifodata::W](channel1fifodata::W) writer structure"]
impl crate::Writable for CHANNEL1FIFODATA {}
#[doc = "CH1 FIFO DATA register"]
pub mod channel1fifodata;
#[doc = "CH2 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel2fifodata](channel2fifodata) module"]
pub type CHANNEL2FIFODATA = crate::Reg<u32, _CHANNEL2FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL2FIFODATA;
#[doc = "`read()` method returns [channel2fifodata::R](channel2fifodata::R) reader structure"]
impl crate::Readable for CHANNEL2FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel2fifodata::W](channel2fifodata::W) writer structure"]
impl crate::Writable for CHANNEL2FIFODATA {}
#[doc = "CH2 FIFO DATA register"]
pub mod channel2fifodata;
#[doc = "CH3 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel3fifodata](channel3fifodata) module"]
pub type CHANNEL3FIFODATA = crate::Reg<u32, _CHANNEL3FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL3FIFODATA;
#[doc = "`read()` method returns [channel3fifodata::R](channel3fifodata::R) reader structure"]
impl crate::Readable for CHANNEL3FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel3fifodata::W](channel3fifodata::W) writer structure"]
impl crate::Writable for CHANNEL3FIFODATA {}
#[doc = "CH3 FIFO DATA register"]
pub mod channel3fifodata;
#[doc = "CH4 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel4fifodata](channel4fifodata) module"]
pub type CHANNEL4FIFODATA = crate::Reg<u32, _CHANNEL4FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL4FIFODATA;
#[doc = "`read()` method returns [channel4fifodata::R](channel4fifodata::R) reader structure"]
impl crate::Readable for CHANNEL4FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel4fifodata::W](channel4fifodata::W) writer structure"]
impl crate::Writable for CHANNEL4FIFODATA {}
#[doc = "CH4 FIFO DATA register"]
pub mod channel4fifodata;
#[doc = "CH5 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel5fifodata](channel5fifodata) module"]
pub type CHANNEL5FIFODATA = crate::Reg<u32, _CHANNEL5FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL5FIFODATA;
#[doc = "`read()` method returns [channel5fifodata::R](channel5fifodata::R) reader structure"]
impl crate::Readable for CHANNEL5FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel5fifodata::W](channel5fifodata::W) writer structure"]
impl crate::Writable for CHANNEL5FIFODATA {}
#[doc = "CH5 FIFO DATA register"]
pub mod channel5fifodata;
#[doc = "CH6 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel6fifodata](channel6fifodata) module"]
pub type CHANNEL6FIFODATA = crate::Reg<u32, _CHANNEL6FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL6FIFODATA;
#[doc = "`read()` method returns [channel6fifodata::R](channel6fifodata::R) reader structure"]
impl crate::Readable for CHANNEL6FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel6fifodata::W](channel6fifodata::W) writer structure"]
impl crate::Writable for CHANNEL6FIFODATA {}
#[doc = "CH6 FIFO DATA register"]
pub mod channel6fifodata;
#[doc = "CH7 FIFO DATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel7fifodata](channel7fifodata) module"]
pub type CHANNEL7FIFODATA = crate::Reg<u32, _CHANNEL7FIFODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNEL7FIFODATA;
#[doc = "`read()` method returns [channel7fifodata::R](channel7fifodata::R) reader structure"]
impl crate::Readable for CHANNEL7FIFODATA {}
#[doc = "`write(|w| ..)` method takes [channel7fifodata::W](channel7fifodata::W) writer structure"]
impl crate::Writable for CHANNEL7FIFODATA {}
#[doc = "CH7 FIFO DATA register"]
pub mod channel7fifodata;
#[doc = "channel 0 FIFO Level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_fifo_lvl](ch0_fifo_lvl) module"]
pub type CH0_FIFO_LVL = crate::Reg<u32, _CH0_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_FIFO_LVL;
#[doc = "`read()` method returns [ch0_fifo_lvl::R](ch0_fifo_lvl::R) reader structure"]
impl crate::Readable for CH0_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch0_fifo_lvl::W](ch0_fifo_lvl::W) writer structure"]
impl crate::Writable for CH0_FIFO_LVL {}
#[doc = "channel 0 FIFO Level register"]
pub mod ch0_fifo_lvl;
#[doc = "Channel 1 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_fifo_lvl](ch1_fifo_lvl) module"]
pub type CH1_FIFO_LVL = crate::Reg<u32, _CH1_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_FIFO_LVL;
#[doc = "`read()` method returns [ch1_fifo_lvl::R](ch1_fifo_lvl::R) reader structure"]
impl crate::Readable for CH1_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch1_fifo_lvl::W](ch1_fifo_lvl::W) writer structure"]
impl crate::Writable for CH1_FIFO_LVL {}
#[doc = "Channel 1 interrupt status register"]
pub mod ch1_fifo_lvl;
#[doc = "CH2_FIFO_LVL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_fifo_lvl](ch2_fifo_lvl) module"]
pub type CH2_FIFO_LVL = crate::Reg<u32, _CH2_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_FIFO_LVL;
#[doc = "`read()` method returns [ch2_fifo_lvl::R](ch2_fifo_lvl::R) reader structure"]
impl crate::Readable for CH2_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch2_fifo_lvl::W](ch2_fifo_lvl::W) writer structure"]
impl crate::Writable for CH2_FIFO_LVL {}
#[doc = "CH2_FIFO_LVL"]
pub mod ch2_fifo_lvl;
#[doc = "Channel 3 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_fifo_lvl](ch3_fifo_lvl) module"]
pub type CH3_FIFO_LVL = crate::Reg<u32, _CH3_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_FIFO_LVL;
#[doc = "`read()` method returns [ch3_fifo_lvl::R](ch3_fifo_lvl::R) reader structure"]
impl crate::Readable for CH3_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch3_fifo_lvl::W](ch3_fifo_lvl::W) writer structure"]
impl crate::Writable for CH3_FIFO_LVL {}
#[doc = "Channel 3 interrupt status register"]
pub mod ch3_fifo_lvl;
#[doc = "Channel 4 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_fifo_lvl](ch4_fifo_lvl) module"]
pub type CH4_FIFO_LVL = crate::Reg<u32, _CH4_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_FIFO_LVL;
#[doc = "`read()` method returns [ch4_fifo_lvl::R](ch4_fifo_lvl::R) reader structure"]
impl crate::Readable for CH4_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch4_fifo_lvl::W](ch4_fifo_lvl::W) writer structure"]
impl crate::Writable for CH4_FIFO_LVL {}
#[doc = "Channel 4 interrupt status register"]
pub mod ch4_fifo_lvl;
#[doc = "CH5_FIFO_LVL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_fifo_lvl](ch5_fifo_lvl) module"]
pub type CH5_FIFO_LVL = crate::Reg<u32, _CH5_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_FIFO_LVL;
#[doc = "`read()` method returns [ch5_fifo_lvl::R](ch5_fifo_lvl::R) reader structure"]
impl crate::Readable for CH5_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch5_fifo_lvl::W](ch5_fifo_lvl::W) writer structure"]
impl crate::Writable for CH5_FIFO_LVL {}
#[doc = "CH5_FIFO_LVL"]
pub mod ch5_fifo_lvl;
#[doc = "Channel 6 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_fifo_lvl](ch6_fifo_lvl) module"]
pub type CH6_FIFO_LVL = crate::Reg<u32, _CH6_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_FIFO_LVL;
#[doc = "`read()` method returns [ch6_fifo_lvl::R](ch6_fifo_lvl::R) reader structure"]
impl crate::Readable for CH6_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch6_fifo_lvl::W](ch6_fifo_lvl::W) writer structure"]
impl crate::Writable for CH6_FIFO_LVL {}
#[doc = "Channel 6 interrupt status register"]
pub mod ch6_fifo_lvl;
#[doc = "Channel 7 interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_fifo_lvl](ch7_fifo_lvl) module"]
pub type CH7_FIFO_LVL = crate::Reg<u32, _CH7_FIFO_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_FIFO_LVL;
#[doc = "`read()` method returns [ch7_fifo_lvl::R](ch7_fifo_lvl::R) reader structure"]
impl crate::Readable for CH7_FIFO_LVL {}
#[doc = "`write(|w| ..)` method takes [ch7_fifo_lvl::W](ch7_fifo_lvl::W) writer structure"]
impl crate::Writable for CH7_FIFO_LVL {}
#[doc = "Channel 7 interrupt status register"]
pub mod ch7_fifo_lvl;
#[doc = "CH_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_enable](ch_enable) module"]
pub type CH_ENABLE = crate::Reg<u32, _CH_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_ENABLE;
#[doc = "`read()` method returns [ch_enable::R](ch_enable::R) reader structure"]
impl crate::Readable for CH_ENABLE {}
#[doc = "`write(|w| ..)` method takes [ch_enable::W](ch_enable::W) writer structure"]
impl crate::Writable for CH_ENABLE {}
#[doc = "CH_ENABLE"]
pub mod ch_enable;
