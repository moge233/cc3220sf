#[doc = "Reader of register MEM_HCLK_DIV_CFG"]
pub type R = crate::R<u32, super::MEM_HCLK_DIV_CFG>;
#[doc = "Writer for register MEM_HCLK_DIV_CFG"]
pub type W = crate::W<u32, super::MEM_HCLK_DIV_CFG>;
#[doc = "Register MEM_HCLK_DIV_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HCLK_DIV_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HCLK_DIV_CFG`"]
pub type MEM_HCLK_DIV_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HCLK_DIV_CFG`"]
pub struct MEM_HCLK_DIV_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HCLK_DIV_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Division configuration for HCLKDIVOUT : &quot;000&quot; - Divide by 1 ; &quot;001&quot; - Divide by 2 ; &quot;010&quot; - Divide by 3 ; &quot;011&quot; - Divide by 4 ; &quot;100&quot; - Divide by 5 ; &quot;101&quot; - Divide by 6 ; &quot;110&quot; - Divide by 7 ; &quot;111&quot; - Divide by 8"]
    #[inline(always)]
    pub fn mem_hclk_div_cfg(&self) -> MEM_HCLK_DIV_CFG_R {
        MEM_HCLK_DIV_CFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Division configuration for HCLKDIVOUT : &quot;000&quot; - Divide by 1 ; &quot;001&quot; - Divide by 2 ; &quot;010&quot; - Divide by 3 ; &quot;011&quot; - Divide by 4 ; &quot;100&quot; - Divide by 5 ; &quot;101&quot; - Divide by 6 ; &quot;110&quot; - Divide by 7 ; &quot;111&quot; - Divide by 8"]
    #[inline(always)]
    pub fn mem_hclk_div_cfg(&mut self) -> MEM_HCLK_DIV_CFG_W {
        MEM_HCLK_DIV_CFG_W { w: self }
    }
}
