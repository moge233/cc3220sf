#[doc = "Reader of register CC3XX_DEV_PADCONF"]
pub type R = crate::R<u32, super::CC3XX_DEV_PADCONF>;
#[doc = "Writer for register CC3XX_DEV_PADCONF"]
pub type W = crate::W<u32, super::CC3XX_DEV_PADCONF>;
#[doc = "Register CC3XX_DEV_PADCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3XX_DEV_PADCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CC3XX_DEV_PADCONF`"]
pub type MEM_CC3XX_DEV_PADCONF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_CC3XX_DEV_PADCONF`"]
pub struct MEM_CC3XX_DEV_PADCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CC3XX_DEV_PADCONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MEM_CC3XX_DEV_PADCONF"]
    #[inline(always)]
    pub fn mem_cc3xx_dev_padconf(&self) -> MEM_CC3XX_DEV_PADCONF_R {
        MEM_CC3XX_DEV_PADCONF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MEM_CC3XX_DEV_PADCONF"]
    #[inline(always)]
    pub fn mem_cc3xx_dev_padconf(&mut self) -> MEM_CC3XX_DEV_PADCONF_W {
        MEM_CC3XX_DEV_PADCONF_W { w: self }
    }
}
