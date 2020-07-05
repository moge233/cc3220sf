#[doc = "Reader of register MEM_HIB_WAKE_STATUS"]
pub type R = crate::R<u32, super::MEM_HIB_WAKE_STATUS>;
#[doc = "Writer for register MEM_HIB_WAKE_STATUS"]
pub type W = crate::W<u32, super::MEM_HIB_WAKE_STATUS>;
#[doc = "Register MEM_HIB_WAKE_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_WAKE_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_WAKE_SRC`"]
pub type HIB_WAKE_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIB_WAKE_SRC`"]
pub struct HIB_WAKE_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_WAKE_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - &quot;0100&quot; - GPIO ; &quot;0010&quot; - RTC ; &quot;0001&quot; - UART Others - Reserved"]
    #[inline(always)]
    pub fn hib_wake_src(&self) -> HIB_WAKE_SRC_R {
        HIB_WAKE_SRC_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - &quot;0100&quot; - GPIO ; &quot;0010&quot; - RTC ; &quot;0001&quot; - UART Others - Reserved"]
    #[inline(always)]
    pub fn hib_wake_src(&mut self) -> HIB_WAKE_SRC_W {
        HIB_WAKE_SRC_W { w: self }
    }
}
