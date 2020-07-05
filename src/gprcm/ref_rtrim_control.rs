#[doc = "Reader of register REF_RTRIM_CONTROL"]
pub type R = crate::R<u32, super::REF_RTRIM_CONTROL>;
#[doc = "Writer for register REF_RTRIM_CONTROL"]
pub type W = crate::W<u32, super::REF_RTRIM_CONTROL>;
#[doc = "Register REF_RTRIM_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_RTRIM_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP_PM_REG0_5_4`"]
pub type TOP_PM_REG0_5_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_PM_REG0_5_4`"]
pub struct TOP_PM_REG0_5_4_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PM_REG0_5_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `TOP_CLKM_REG0_15_5`"]
pub type TOP_CLKM_REG0_15_5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP_CLKM_REG0_15_5`"]
pub struct TOP_CLKM_REG0_15_5_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_CLKM_REG0_15_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `REF_CLKM_RTRIM`"]
pub type REF_CLKM_RTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_CLKM_RTRIM`"]
pub struct REF_CLKM_RTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CLKM_RTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - This is \\[5:4\\]
bits of TOP_PM_REG0"]
    #[inline(always)]
    pub fn top_pm_reg0_5_4(&self) -> TOP_PM_REG0_5_4_R {
        TOP_PM_REG0_5_4_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 16:26 - This is \\[15:5\\]
bits of TOP_CLKM_REG0"]
    #[inline(always)]
    pub fn top_clkm_reg0_15_5(&self) -> TOP_CLKM_REG0_15_5_R {
        TOP_CLKM_REG0_15_5_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:4 - CLKM_TRIM Override. Applicable when efuse_done = 0 or bit\\[8\\]
is set to 1."]
    #[inline(always)]
    pub fn ref_clkm_rtrim(&self) -> REF_CLKM_RTRIM_R {
        REF_CLKM_RTRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - This is \\[5:4\\]
bits of TOP_PM_REG0"]
    #[inline(always)]
    pub fn top_pm_reg0_5_4(&mut self) -> TOP_PM_REG0_5_4_W {
        TOP_PM_REG0_5_4_W { w: self }
    }
    #[doc = "Bits 16:26 - This is \\[15:5\\]
bits of TOP_CLKM_REG0"]
    #[inline(always)]
    pub fn top_clkm_reg0_15_5(&mut self) -> TOP_CLKM_REG0_15_5_W {
        TOP_CLKM_REG0_15_5_W { w: self }
    }
    #[doc = "Bits 0:4 - CLKM_TRIM Override. Applicable when efuse_done = 0 or bit\\[8\\]
is set to 1."]
    #[inline(always)]
    pub fn ref_clkm_rtrim(&mut self) -> REF_CLKM_RTRIM_W {
        REF_CLKM_RTRIM_W { w: self }
    }
}
