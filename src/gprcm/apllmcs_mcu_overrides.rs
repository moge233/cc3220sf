#[doc = "Reader of register APLLMCS_MCU_OVERRIDES"]
pub type R = crate::R<u32, super::APLLMCS_MCU_OVERRIDES>;
#[doc = "Writer for register APLLMCS_MCU_OVERRIDES"]
pub type W = crate::W<u32, super::APLLMCS_MCU_OVERRIDES>;
#[doc = "Register APLLMCS_MCU_OVERRIDES `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_MCU_OVERRIDES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCLK_SRC_OVERRIDE`"]
pub type SYSCLK_SRC_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCLK_SRC_OVERRIDE`"]
pub struct SYSCLK_SRC_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_SRC_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - Override for sysclk src (applicable only if bit \\[0\\]
is set to 1. &quot;00&quot;- SLOW_CLK &quot;01&quot;- XTAL_CLK &quot;10&quot;- PLL_CLK"]
    #[inline(always)]
    pub fn sysclk_src_override(&self) -> SYSCLK_SRC_OVERRIDE_R {
        SYSCLK_SRC_OVERRIDE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Override for sysclk src (applicable only if bit \\[0\\]
is set to 1. &quot;00&quot;- SLOW_CLK &quot;01&quot;- XTAL_CLK &quot;10&quot;- PLL_CLK"]
    #[inline(always)]
    pub fn sysclk_src_override(&mut self) -> SYSCLK_SRC_OVERRIDE_W {
        SYSCLK_SRC_OVERRIDE_W { w: self }
    }
}
