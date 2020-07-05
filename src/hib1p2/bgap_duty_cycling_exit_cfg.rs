#[doc = "Reader of register BGAP_DUTY_CYCLING_EXIT_CFG"]
pub type R = crate::R<u32, super::BGAP_DUTY_CYCLING_EXIT_CFG>;
#[doc = "Writer for register BGAP_DUTY_CYCLING_EXIT_CFG"]
pub type W = crate::W<u32, super::BGAP_DUTY_CYCLING_EXIT_CFG>;
#[doc = "Register BGAP_DUTY_CYCLING_EXIT_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BGAP_DUTY_CYCLING_EXIT_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BGAP_DUTY_CYCLING_EXIT_TIME`"]
pub type MEM_BGAP_DUTY_CYCLING_EXIT_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BGAP_DUTY_CYCLING_EXIT_TIME`"]
pub struct MEM_BGAP_DUTY_CYCLING_EXIT_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BGAP_DUTY_CYCLING_EXIT_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MEM_BGAP_DUTY_CYCLING_EXIT_TIME"]
    #[inline(always)]
    pub fn mem_bgap_duty_cycling_exit_time(&self) -> MEM_BGAP_DUTY_CYCLING_EXIT_TIME_R {
        MEM_BGAP_DUTY_CYCLING_EXIT_TIME_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MEM_BGAP_DUTY_CYCLING_EXIT_TIME"]
    #[inline(always)]
    pub fn mem_bgap_duty_cycling_exit_time(&mut self) -> MEM_BGAP_DUTY_CYCLING_EXIT_TIME_W {
        MEM_BGAP_DUTY_CYCLING_EXIT_TIME_W { w: self }
    }
}
