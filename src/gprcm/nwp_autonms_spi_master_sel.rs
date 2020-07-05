#[doc = "Reader of register NWP_AUTONMS_SPI_MASTER_SEL"]
pub type R = crate::R<u32, super::NWP_AUTONMS_SPI_MASTER_SEL>;
#[doc = "Writer for register NWP_AUTONMS_SPI_MASTER_SEL"]
pub type W = crate::W<u32, super::NWP_AUTONMS_SPI_MASTER_SEL>;
#[doc = "Register NWP_AUTONMS_SPI_MASTER_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_AUTONMS_SPI_MASTER_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F`"]
pub type F_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `F`"]
pub struct F_W<'a> {
    w: &'a mut W,
}
impl<'a> F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - F"]
    #[inline(always)]
    pub fn f(&self) -> F_R {
        F_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 17:31 - F"]
    #[inline(always)]
    pub fn f(&mut self) -> F_W {
        F_W { w: self }
    }
}
