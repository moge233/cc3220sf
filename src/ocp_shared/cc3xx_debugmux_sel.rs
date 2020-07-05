#[doc = "Reader of register CC3XX_DEBUGMUX_SEL"]
pub type R = crate::R<u32, super::CC3XX_DEBUGMUX_SEL>;
#[doc = "Writer for register CC3XX_DEBUGMUX_SEL"]
pub type W = crate::W<u32, super::CC3XX_DEBUGMUX_SEL>;
#[doc = "Register CC3XX_DEBUGMUX_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3XX_DEBUGMUX_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CC3XX_DEBUGMUX_SEL`"]
pub type MEM_CC3XX_DEBUGMUX_SEL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_CC3XX_DEBUGMUX_SEL`"]
pub struct MEM_CC3XX_DEBUGMUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CC3XX_DEBUGMUX_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - debug mux select register. Upper 8 bits are used for debug module selection. Lower 8 bit \\[7:0\\]
used inside debug module for selecting module specific signals. Bits\\[15:8: when set x&quot;00&quot; : GPRCM debug bus. When &quot;o1&quot; : SDIO debug debug bus when x&quot;02&quot; : autonoumous SPI when x&quot;03&quot; : TOPIC when x&quot;04&quot;: memss when x&quot;25&quot;: mcu debug bus : APPS debug when x&quot;45&quot;: mcu debug bus : NWP debug when x&quot;65&quot;: mcu debug bus : AHB2VBUS debug when x&quot;85&quot;: mcu debug bus : VBUS2HAB debug when x&quot;95&quot;: mcu debug bus : RCM debug when x&quot;A5&quot;: mcu debug bus : crypto debug when x&quot;06&quot;: WLAN debug bus when x&quot;07&quot;: debugss bus when x&quot;08&quot;: ADC debug when x&quot;09&quot;: SDIO PHY debug bus then &quot;others&quot; : no debug is selected"]
    #[inline(always)]
    pub fn mem_cc3xx_debugmux_sel(&self) -> MEM_CC3XX_DEBUGMUX_SEL_R {
        MEM_CC3XX_DEBUGMUX_SEL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - debug mux select register. Upper 8 bits are used for debug module selection. Lower 8 bit \\[7:0\\]
used inside debug module for selecting module specific signals. Bits\\[15:8: when set x&quot;00&quot; : GPRCM debug bus. When &quot;o1&quot; : SDIO debug debug bus when x&quot;02&quot; : autonoumous SPI when x&quot;03&quot; : TOPIC when x&quot;04&quot;: memss when x&quot;25&quot;: mcu debug bus : APPS debug when x&quot;45&quot;: mcu debug bus : NWP debug when x&quot;65&quot;: mcu debug bus : AHB2VBUS debug when x&quot;85&quot;: mcu debug bus : VBUS2HAB debug when x&quot;95&quot;: mcu debug bus : RCM debug when x&quot;A5&quot;: mcu debug bus : crypto debug when x&quot;06&quot;: WLAN debug bus when x&quot;07&quot;: debugss bus when x&quot;08&quot;: ADC debug when x&quot;09&quot;: SDIO PHY debug bus then &quot;others&quot; : no debug is selected"]
    #[inline(always)]
    pub fn mem_cc3xx_debugmux_sel(&mut self) -> MEM_CC3XX_DEBUGMUX_SEL_W {
        MEM_CC3XX_DEBUGMUX_SEL_W { w: self }
    }
}
