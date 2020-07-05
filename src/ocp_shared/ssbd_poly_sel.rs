#[doc = "Reader of register SSBD_POLY_SEL"]
pub type R = crate::R<u32, super::SSBD_POLY_SEL>;
#[doc = "Writer for register SSBD_POLY_SEL"]
pub type W = crate::W<u32, super::SSBD_POLY_SEL>;
#[doc = "Register SSBD_POLY_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SSBD_POLY_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SSBD_POLY_SEL`"]
pub type MEM_SSBD_POLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SSBD_POLY_SEL`"]
pub struct MEM_SSBD_POLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SSBD_POLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 2 bit, Writable only during devinit, and whole 2 bit should be output of the config register module."]
    #[inline(always)]
    pub fn mem_ssbd_poly_sel(&self) -> MEM_SSBD_POLY_SEL_R {
        MEM_SSBD_POLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2 bit, Writable only during devinit, and whole 2 bit should be output of the config register module."]
    #[inline(always)]
    pub fn mem_ssbd_poly_sel(&mut self) -> MEM_SSBD_POLY_SEL_W {
        MEM_SSBD_POLY_SEL_W { w: self }
    }
}
