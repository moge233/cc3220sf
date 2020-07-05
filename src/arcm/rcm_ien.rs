#[doc = "Reader of register RCM_IEN"]
pub type R = crate::R<u32, super::RCM_IEN>;
#[doc = "Writer for register RCM_IEN"]
pub type W = crate::W<u32, super::RCM_IEN>;
#[doc = "Register RCM_IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::RCM_IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKETIMERIRQ`"]
pub type WAKETIMERIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKETIMERIRQ`"]
pub struct WAKETIMERIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKETIMERIRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PLLLOCKIRQ`"]
pub type PLLLOCKIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLLOCKIRQ`"]
pub struct PLLLOCKIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLOCKIRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - WAKETIMERIRQ"]
    #[inline(always)]
    pub fn waketimerirq(&self) -> WAKETIMERIRQ_R {
        WAKETIMERIRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PLLLOCKIRQ"]
    #[inline(always)]
    pub fn plllockirq(&self) -> PLLLOCKIRQ_R {
        PLLLOCKIRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - WAKETIMERIRQ"]
    #[inline(always)]
    pub fn waketimerirq(&mut self) -> WAKETIMERIRQ_W {
        WAKETIMERIRQ_W { w: self }
    }
    #[doc = "Bit 0 - PLLLOCKIRQ"]
    #[inline(always)]
    pub fn plllockirq(&mut self) -> PLLLOCKIRQ_W {
        PLLLOCKIRQ_W { w: self }
    }
}
