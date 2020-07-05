#[doc = "Reader of register CC_CTRL_XCLK"]
pub type R = crate::R<u32, super::CC_CTRL_XCLK>;
#[doc = "Writer for register CC_CTRL_XCLK"]
pub type W = crate::W<u32, super::CC_CTRL_XCLK>;
#[doc = "Register CC_CTRL_XCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_CTRL_XCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XCLK_DIV`"]
pub type XCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XCLK_DIV`"]
pub struct XCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> XCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Sets the clock divisor value for CAM_XCLK generation. based on CAM_MCK (value of CAM_MCLK is 96MHz) &quot;&quot;&quot;00000&quot;&quot; CAM_XCLK Stable Low Level&quot; Divider not enabled &quot;&quot;&quot;00001&quot;&quot; CAM_XCLK Stable High Level&quot; Divider not enabled from 2 to 30 CAM_XCLK = CAM_MCLK / XCLK_DIV &quot;&quot;&quot;11111&quot;&quot; Bypass - CAM_XCLK = CAM_MCLK&quot;"]
    #[inline(always)]
    pub fn xclk_div(&self) -> XCLK_DIV_R {
        XCLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the clock divisor value for CAM_XCLK generation. based on CAM_MCK (value of CAM_MCLK is 96MHz) &quot;&quot;&quot;00000&quot;&quot; CAM_XCLK Stable Low Level&quot; Divider not enabled &quot;&quot;&quot;00001&quot;&quot; CAM_XCLK Stable High Level&quot; Divider not enabled from 2 to 30 CAM_XCLK = CAM_MCLK / XCLK_DIV &quot;&quot;&quot;11111&quot;&quot; Bypass - CAM_XCLK = CAM_MCLK&quot;"]
    #[inline(always)]
    pub fn xclk_div(&mut self) -> XCLK_DIV_W {
        XCLK_DIV_W { w: self }
    }
}
