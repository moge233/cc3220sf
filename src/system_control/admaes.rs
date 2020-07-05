#[doc = "Reader of register ADMAES"]
pub type R = crate::R<u32, super::ADMAES>;
#[doc = "Writer for register ADMAES"]
pub type W = crate::W<u32, super::ADMAES>;
#[doc = "Register ADMAES `reset()`'s with value 0"]
impl crate::ResetValue for super::ADMAES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State his field indicates the state of ADMA when error is occurred during ADMA data transfer. &quot;This field never indicates &quot;&quot;10&quot;&quot; because ADMA never stops in this state.&quot; 0x0 ST_STOP (Stop DMA)Contents of SYS_SDR register 0x1 ST_STOP (Stop DMA)Points the error descriptor 0x2 Never set this state(Not used) 0x3 ST_TFR (Transfer Data)Points the next of the error descriptor"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMA Error State his field indicates the state of ADMA when error is occurred during ADMA data transfer. &quot;This field never indicates &quot;&quot;10&quot;&quot; because ADMA never stops in this state.&quot; 0x0 ST_STOP (Stop DMA)Contents of SYS_SDR register 0x1 ST_STOP (Stop DMA)Points the error descriptor 0x2 Never set this state(Not used) 0x3 ST_TFR (Transfer Data)Points the next of the error descriptor"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
}
