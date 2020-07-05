#[doc = "Reader of register D2D_MISC_PAD_CONF"]
pub type R = crate::R<u32, super::D2D_MISC_PAD_CONF>;
#[doc = "Writer for register D2D_MISC_PAD_CONF"]
pub type W = crate::W<u32, super::D2D_MISC_PAD_CONF>;
#[doc = "Register D2D_MISC_PAD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::D2D_MISC_PAD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_D2D_SPARE`"]
pub type MEM_D2D_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_D2D_SPARE`"]
pub struct MEM_D2D_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_D2D_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - D2D SPARE PAD OEN/OEN2X control. When 0: Act as input buffer else output buffer with drive strength 2."]
    #[inline(always)]
    pub fn mem_d2d_spare(&self) -> MEM_D2D_SPARE_R {
        MEM_D2D_SPARE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - D2D SPARE PAD OEN/OEN2X control. When 0: Act as input buffer else output buffer with drive strength 2."]
    #[inline(always)]
    pub fn mem_d2d_spare(&mut self) -> MEM_D2D_SPARE_W {
        MEM_D2D_SPARE_W { w: self }
    }
}
