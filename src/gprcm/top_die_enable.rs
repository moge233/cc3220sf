#[doc = "Reader of register TOP_DIE_ENABLE"]
pub type R = crate::R<u32, super::TOP_DIE_ENABLE>;
#[doc = "Writer for register TOP_DIE_ENABLE"]
pub type W = crate::W<u32, super::TOP_DIE_ENABLE>;
#[doc = "Register TOP_DIE_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::TOP_DIE_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP_DIE_PWR_PS`"]
pub type TOP_DIE_PWR_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_DIE_PWR_PS`"]
pub struct TOP_DIE_PWR_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_DIE_PWR_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - TOP_DIE_PWR_PS"]
    #[inline(always)]
    pub fn top_die_pwr_ps(&self) -> TOP_DIE_PWR_PS_R {
        TOP_DIE_PWR_PS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - TOP_DIE_PWR_PS"]
    #[inline(always)]
    pub fn top_die_pwr_ps(&mut self) -> TOP_DIE_PWR_PS_W {
        TOP_DIE_PWR_PS_W { w: self }
    }
}
