#[doc = "Reader of register CC3XX_DEVICE_TYPE"]
pub type R = crate::R<u32, super::CC3XX_DEVICE_TYPE>;
#[doc = "Writer for register CC3XX_DEVICE_TYPE"]
pub type W = crate::W<u32, super::CC3XX_DEVICE_TYPE>;
#[doc = "Register CC3XX_DEVICE_TYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3XX_DEVICE_TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVICE_TYPE`"]
pub type DEVICE_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVICE_TYPE`"]
pub struct DEVICE_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - CC3XX Device type information."]
    #[inline(always)]
    pub fn device_type(&self) -> DEVICE_TYPE_R {
        DEVICE_TYPE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CC3XX Device type information."]
    #[inline(always)]
    pub fn device_type(&mut self) -> DEVICE_TYPE_W {
        DEVICE_TYPE_W { w: self }
    }
}
