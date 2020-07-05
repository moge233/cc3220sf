#[doc = "Reader of register TBPMR"]
pub type R = crate::R<u32, super::TBPMR>;
#[doc = "Writer for register TBPMR"]
pub type W = crate::W<u32, super::TBPMR>;
#[doc = "Register TBPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPSMRH`"]
pub type TBPSMRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSMRH`"]
pub struct TBPSMRH_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSMRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBPSMR`"]
pub type TBPSMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSMR`"]
pub struct TBPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - GPTM Timer B Prescale Match High Byte ##### GARNET END #####"]
    #[inline(always)]
    pub fn tbpsmrh(&self) -> TBPSMRH_R {
        TBPSMRH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match ##### GARNET BEGIN #####"]
    #[inline(always)]
    pub fn tbpsmr(&self) -> TBPSMR_R {
        TBPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - GPTM Timer B Prescale Match High Byte ##### GARNET END #####"]
    #[inline(always)]
    pub fn tbpsmrh(&mut self) -> TBPSMRH_W {
        TBPSMRH_W { w: self }
    }
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match ##### GARNET BEGIN #####"]
    #[inline(always)]
    pub fn tbpsmr(&mut self) -> TBPSMR_W {
        TBPSMR_W { w: self }
    }
}
