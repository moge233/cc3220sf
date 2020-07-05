#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Patch trap address Register array"]
    pub patch_trap_addr_reg: PATCH_TRAP_ADDR_REG,
    _reserved1: [u8; 116usize],
    #[doc = "0x78 - PATCH_TRAP_EN_REG"]
    pub patch_trap_en_reg: PATCH_TRAP_EN_REG,
    #[doc = "0x7c - FAULT_STATUS_REG"]
    pub fault_status_reg: FAULT_STATUS_REG,
    #[doc = "0x80 - MEMSS_WR_ERR_CLR_REG"]
    pub memss_wr_err_clr_reg: MEMSS_WR_ERR_CLR_REG,
    #[doc = "0x84 - MEMSS_WR_ERR_ADDR_REG"]
    pub memss_wr_err_addr_reg: MEMSS_WR_ERR_ADDR_REG,
    _reserved5: [u8; 4usize],
    #[doc = "0x8c - DMA_DONE_INT_MASK"]
    pub dma_done_int_mask: DMA_DONE_INT_MASK,
    #[doc = "0x90 - DMA_DONE_INT_MASK_SET"]
    pub dma_done_int_mask_set: DMA_DONE_INT_MASK_SET,
    #[doc = "0x94 - DMA_DONE_INT_MASK_CLR"]
    pub dma_done_int_mask_clr: DMA_DONE_INT_MASK_CLR,
    #[doc = "0x98 - DMA_DONE_INT_STS_CLR"]
    pub dma_done_int_sts_clr: DMA_DONE_INT_STS_CLR,
    #[doc = "0x9c - DMA_DONE_INT_ACK"]
    pub dma_done_int_ack: DMA_DONE_INT_ACK,
    #[doc = "0xa0 - DMA_DONE_INT_STS_MASKED"]
    pub dma_done_int_sts_masked: DMA_DONE_INT_STS_MASKED,
    #[doc = "0xa4 - DMA_DONE_INT_STS_RAW"]
    pub dma_done_int_sts_raw: DMA_DONE_INT_STS_RAW,
    #[doc = "0xa8 - FAULT_STATUS_CLR_REG"]
    pub fault_status_clr_reg: FAULT_STATUS_CLR_REG,
    #[doc = "0xac - RESERVD_REG_0"]
    pub reservd_reg_0: RESERVD_REG_0,
    #[doc = "0xb0 - GPT_TRIG_SEL"]
    pub gpt_trig_sel: GPT_TRIG_SEL,
    #[doc = "0xb4 - TOP_DIE_SPARE_DIN_REG"]
    pub top_die_spare_din_reg: TOP_DIE_SPARE_DIN_REG,
    #[doc = "0xb8 - TOP_DIE_SPARE_DOUT_REG"]
    pub top_die_spare_dout_reg: TOP_DIE_SPARE_DOUT_REG,
}
#[doc = "Patch trap address Register array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patch_trap_addr_reg](patch_trap_addr_reg) module"]
pub type PATCH_TRAP_ADDR_REG = crate::Reg<u32, _PATCH_TRAP_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATCH_TRAP_ADDR_REG;
#[doc = "`read()` method returns [patch_trap_addr_reg::R](patch_trap_addr_reg::R) reader structure"]
impl crate::Readable for PATCH_TRAP_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [patch_trap_addr_reg::W](patch_trap_addr_reg::W) writer structure"]
impl crate::Writable for PATCH_TRAP_ADDR_REG {}
#[doc = "Patch trap address Register array"]
pub mod patch_trap_addr_reg;
#[doc = "PATCH_TRAP_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patch_trap_en_reg](patch_trap_en_reg) module"]
pub type PATCH_TRAP_EN_REG = crate::Reg<u32, _PATCH_TRAP_EN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATCH_TRAP_EN_REG;
#[doc = "`read()` method returns [patch_trap_en_reg::R](patch_trap_en_reg::R) reader structure"]
impl crate::Readable for PATCH_TRAP_EN_REG {}
#[doc = "`write(|w| ..)` method takes [patch_trap_en_reg::W](patch_trap_en_reg::W) writer structure"]
impl crate::Writable for PATCH_TRAP_EN_REG {}
#[doc = "PATCH_TRAP_EN_REG"]
pub mod patch_trap_en_reg;
#[doc = "FAULT_STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault_status_reg](fault_status_reg) module"]
pub type FAULT_STATUS_REG = crate::Reg<u32, _FAULT_STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULT_STATUS_REG;
#[doc = "`read()` method returns [fault_status_reg::R](fault_status_reg::R) reader structure"]
impl crate::Readable for FAULT_STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [fault_status_reg::W](fault_status_reg::W) writer structure"]
impl crate::Writable for FAULT_STATUS_REG {}
#[doc = "FAULT_STATUS_REG"]
pub mod fault_status_reg;
#[doc = "MEMSS_WR_ERR_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memss_wr_err_clr_reg](memss_wr_err_clr_reg) module"]
pub type MEMSS_WR_ERR_CLR_REG = crate::Reg<u32, _MEMSS_WR_ERR_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSS_WR_ERR_CLR_REG;
#[doc = "`read()` method returns [memss_wr_err_clr_reg::R](memss_wr_err_clr_reg::R) reader structure"]
impl crate::Readable for MEMSS_WR_ERR_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [memss_wr_err_clr_reg::W](memss_wr_err_clr_reg::W) writer structure"]
impl crate::Writable for MEMSS_WR_ERR_CLR_REG {}
#[doc = "MEMSS_WR_ERR_CLR_REG"]
pub mod memss_wr_err_clr_reg;
#[doc = "MEMSS_WR_ERR_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memss_wr_err_addr_reg](memss_wr_err_addr_reg) module"]
pub type MEMSS_WR_ERR_ADDR_REG = crate::Reg<u32, _MEMSS_WR_ERR_ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSS_WR_ERR_ADDR_REG;
#[doc = "`read()` method returns [memss_wr_err_addr_reg::R](memss_wr_err_addr_reg::R) reader structure"]
impl crate::Readable for MEMSS_WR_ERR_ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [memss_wr_err_addr_reg::W](memss_wr_err_addr_reg::W) writer structure"]
impl crate::Writable for MEMSS_WR_ERR_ADDR_REG {}
#[doc = "MEMSS_WR_ERR_ADDR_REG"]
pub mod memss_wr_err_addr_reg;
#[doc = "DMA_DONE_INT_MASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_mask](dma_done_int_mask) module"]
pub type DMA_DONE_INT_MASK = crate::Reg<u32, _DMA_DONE_INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_MASK;
#[doc = "`read()` method returns [dma_done_int_mask::R](dma_done_int_mask::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_MASK {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_mask::W](dma_done_int_mask::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_MASK {}
#[doc = "DMA_DONE_INT_MASK"]
pub mod dma_done_int_mask;
#[doc = "DMA_DONE_INT_MASK_SET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_mask_set](dma_done_int_mask_set) module"]
pub type DMA_DONE_INT_MASK_SET = crate::Reg<u32, _DMA_DONE_INT_MASK_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_MASK_SET;
#[doc = "`read()` method returns [dma_done_int_mask_set::R](dma_done_int_mask_set::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_MASK_SET {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_mask_set::W](dma_done_int_mask_set::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_MASK_SET {}
#[doc = "DMA_DONE_INT_MASK_SET"]
pub mod dma_done_int_mask_set;
#[doc = "DMA_DONE_INT_MASK_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_mask_clr](dma_done_int_mask_clr) module"]
pub type DMA_DONE_INT_MASK_CLR = crate::Reg<u32, _DMA_DONE_INT_MASK_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_MASK_CLR;
#[doc = "`read()` method returns [dma_done_int_mask_clr::R](dma_done_int_mask_clr::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_MASK_CLR {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_mask_clr::W](dma_done_int_mask_clr::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_MASK_CLR {}
#[doc = "DMA_DONE_INT_MASK_CLR"]
pub mod dma_done_int_mask_clr;
#[doc = "DMA_DONE_INT_STS_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_sts_clr](dma_done_int_sts_clr) module"]
pub type DMA_DONE_INT_STS_CLR = crate::Reg<u32, _DMA_DONE_INT_STS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_STS_CLR;
#[doc = "`read()` method returns [dma_done_int_sts_clr::R](dma_done_int_sts_clr::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_STS_CLR {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_sts_clr::W](dma_done_int_sts_clr::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_STS_CLR {}
#[doc = "DMA_DONE_INT_STS_CLR"]
pub mod dma_done_int_sts_clr;
#[doc = "DMA_DONE_INT_ACK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_ack](dma_done_int_ack) module"]
pub type DMA_DONE_INT_ACK = crate::Reg<u32, _DMA_DONE_INT_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_ACK;
#[doc = "`read()` method returns [dma_done_int_ack::R](dma_done_int_ack::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_ACK {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_ack::W](dma_done_int_ack::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_ACK {}
#[doc = "DMA_DONE_INT_ACK"]
pub mod dma_done_int_ack;
#[doc = "DMA_DONE_INT_STS_MASKED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_sts_masked](dma_done_int_sts_masked) module"]
pub type DMA_DONE_INT_STS_MASKED = crate::Reg<u32, _DMA_DONE_INT_STS_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_STS_MASKED;
#[doc = "`read()` method returns [dma_done_int_sts_masked::R](dma_done_int_sts_masked::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_STS_MASKED {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_sts_masked::W](dma_done_int_sts_masked::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_STS_MASKED {}
#[doc = "DMA_DONE_INT_STS_MASKED"]
pub mod dma_done_int_sts_masked;
#[doc = "DMA_DONE_INT_STS_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_done_int_sts_raw](dma_done_int_sts_raw) module"]
pub type DMA_DONE_INT_STS_RAW = crate::Reg<u32, _DMA_DONE_INT_STS_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DONE_INT_STS_RAW;
#[doc = "`read()` method returns [dma_done_int_sts_raw::R](dma_done_int_sts_raw::R) reader structure"]
impl crate::Readable for DMA_DONE_INT_STS_RAW {}
#[doc = "`write(|w| ..)` method takes [dma_done_int_sts_raw::W](dma_done_int_sts_raw::W) writer structure"]
impl crate::Writable for DMA_DONE_INT_STS_RAW {}
#[doc = "DMA_DONE_INT_STS_RAW"]
pub mod dma_done_int_sts_raw;
#[doc = "FAULT_STATUS_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault_status_clr_reg](fault_status_clr_reg) module"]
pub type FAULT_STATUS_CLR_REG = crate::Reg<u32, _FAULT_STATUS_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULT_STATUS_CLR_REG;
#[doc = "`read()` method returns [fault_status_clr_reg::R](fault_status_clr_reg::R) reader structure"]
impl crate::Readable for FAULT_STATUS_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [fault_status_clr_reg::W](fault_status_clr_reg::W) writer structure"]
impl crate::Writable for FAULT_STATUS_CLR_REG {}
#[doc = "FAULT_STATUS_CLR_REG"]
pub mod fault_status_clr_reg;
#[doc = "RESERVD_REG_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reservd_reg_0](reservd_reg_0) module"]
pub type RESERVD_REG_0 = crate::Reg<u32, _RESERVD_REG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVD_REG_0;
#[doc = "`read()` method returns [reservd_reg_0::R](reservd_reg_0::R) reader structure"]
impl crate::Readable for RESERVD_REG_0 {}
#[doc = "`write(|w| ..)` method takes [reservd_reg_0::W](reservd_reg_0::W) writer structure"]
impl crate::Writable for RESERVD_REG_0 {}
#[doc = "RESERVD_REG_0"]
pub mod reservd_reg_0;
#[doc = "GPT_TRIG_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt_trig_sel](gpt_trig_sel) module"]
pub type GPT_TRIG_SEL = crate::Reg<u32, _GPT_TRIG_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT_TRIG_SEL;
#[doc = "`read()` method returns [gpt_trig_sel::R](gpt_trig_sel::R) reader structure"]
impl crate::Readable for GPT_TRIG_SEL {}
#[doc = "`write(|w| ..)` method takes [gpt_trig_sel::W](gpt_trig_sel::W) writer structure"]
impl crate::Writable for GPT_TRIG_SEL {}
#[doc = "GPT_TRIG_SEL"]
pub mod gpt_trig_sel;
#[doc = "TOP_DIE_SPARE_DIN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_die_spare_din_reg](top_die_spare_din_reg) module"]
pub type TOP_DIE_SPARE_DIN_REG = crate::Reg<u32, _TOP_DIE_SPARE_DIN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP_DIE_SPARE_DIN_REG;
#[doc = "`read()` method returns [top_die_spare_din_reg::R](top_die_spare_din_reg::R) reader structure"]
impl crate::Readable for TOP_DIE_SPARE_DIN_REG {}
#[doc = "`write(|w| ..)` method takes [top_die_spare_din_reg::W](top_die_spare_din_reg::W) writer structure"]
impl crate::Writable for TOP_DIE_SPARE_DIN_REG {}
#[doc = "TOP_DIE_SPARE_DIN_REG"]
pub mod top_die_spare_din_reg;
#[doc = "TOP_DIE_SPARE_DOUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_die_spare_dout_reg](top_die_spare_dout_reg) module"]
pub type TOP_DIE_SPARE_DOUT_REG = crate::Reg<u32, _TOP_DIE_SPARE_DOUT_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOP_DIE_SPARE_DOUT_REG;
#[doc = "`read()` method returns [top_die_spare_dout_reg::R](top_die_spare_dout_reg::R) reader structure"]
impl crate::Readable for TOP_DIE_SPARE_DOUT_REG {}
#[doc = "`write(|w| ..)` method takes [top_die_spare_dout_reg::W](top_die_spare_dout_reg::W) writer structure"]
impl crate::Writable for TOP_DIE_SPARE_DOUT_REG {}
#[doc = "TOP_DIE_SPARE_DOUT_REG"]
pub mod top_die_spare_dout_reg;
