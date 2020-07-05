#[doc = "Reader of register HIB_TIMER_SYNC_CFG2"]
pub type R = crate::R<u32, super::HIB_TIMER_SYNC_CFG2>;
#[doc = "Writer for register HIB_TIMER_SYNC_CFG2"]
pub type W = crate::W<u32, super::HIB_TIMER_SYNC_CFG2>;
#[doc = "Register HIB_TIMER_SYNC_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TIMER_SYNC_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU1`"]
pub struct NU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
}
