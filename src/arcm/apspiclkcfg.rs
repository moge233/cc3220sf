#[doc = "Reader of register APSPICLKCFG"]
pub type R = crate::R<u32, super::APSPICLKCFG>;
#[doc = "Writer for register APSPICLKCFG"]
pub type W = crate::W<u32, super::APSPICLKCFG>;
#[doc = "Register APSPICLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::APSPICLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAUDSEL`"]
pub type BAUDSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAUDSEL`"]
pub struct BAUDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIVOFFTIM`"]
pub type DIVOFFTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVOFFTIM`"]
pub struct DIVOFFTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVOFFTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `NU2`"]
pub type NU2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIVONTIM`"]
pub type DIVONTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVONTIM`"]
pub struct DIVONTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVONTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - MCSPI_A1_BAUD_CLK_SEL"]
    #[inline(always)]
    pub fn baudsel(&self) -> BAUDSEL_R {
        BAUDSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Configuration of OFF-TIME for dividing PLL clk (240 MHz) in generation of MCSPI_A1 func-clk : &quot;000&quot; - 1 &quot;001&quot; - 2 &quot;010&quot; - 3 &quot;011&quot; - 4 &quot;100&quot; - 5 &quot;101&quot; - 6 &quot;110&quot; - 7 &quot;111&quot; - 8"]
    #[inline(always)]
    pub fn divofftim(&self) -> DIVOFFTIM_R {
        DIVOFFTIM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - NU2"]
    #[inline(always)]
    pub fn nu2(&self) -> NU2_R {
        NU2_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - Configuration of ON-TIME for dividing PLL clk (240 MHz) in generation of MCSPI_A1 func-clk : &quot;000&quot; - 1 &quot;001&quot; - 2 &quot;010&quot; - 3 &quot;011&quot; - 4 &quot;100&quot; - 5 &quot;101&quot; - 6 &quot;110&quot; - 7 &quot;111&quot; - 8"]
    #[inline(always)]
    pub fn divontim(&self) -> DIVONTIM_R {
        DIVONTIM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - MCSPI_A1_BAUD_CLK_SEL"]
    #[inline(always)]
    pub fn baudsel(&mut self) -> BAUDSEL_W {
        BAUDSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Configuration of OFF-TIME for dividing PLL clk (240 MHz) in generation of MCSPI_A1 func-clk : &quot;000&quot; - 1 &quot;001&quot; - 2 &quot;010&quot; - 3 &quot;011&quot; - 4 &quot;100&quot; - 5 &quot;101&quot; - 6 &quot;110&quot; - 7 &quot;111&quot; - 8"]
    #[inline(always)]
    pub fn divofftim(&mut self) -> DIVOFFTIM_W {
        DIVOFFTIM_W { w: self }
    }
    #[doc = "Bits 0:2 - Configuration of ON-TIME for dividing PLL clk (240 MHz) in generation of MCSPI_A1 func-clk : &quot;000&quot; - 1 &quot;001&quot; - 2 &quot;010&quot; - 3 &quot;011&quot; - 4 &quot;100&quot; - 5 &quot;101&quot; - 6 &quot;110&quot; - 7 &quot;111&quot; - 8"]
    #[inline(always)]
    pub fn divontim(&mut self) -> DIVONTIM_W {
        DIVONTIM_W { w: self }
    }
}
