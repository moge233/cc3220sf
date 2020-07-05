#[doc = "Reader of register MCU_SEMAPHORE_PEND"]
pub type R = crate::R<u32, super::MCU_SEMAPHORE_PEND>;
#[doc = "Writer for register MCU_SEMAPHORE_PEND"]
pub type W = crate::W<u32, super::MCU_SEMAPHORE_PEND>;
#[doc = "Register MCU_SEMAPHORE_PEND `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_SEMAPHORE_PEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_MCU_SEMAPHORE_PEND`"]
pub type MEM_MCU_SEMAPHORE_PEND_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_MCU_SEMAPHORE_PEND`"]
pub struct MEM_MCU_SEMAPHORE_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MCU_SEMAPHORE_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register specifies the semaphore for which the NWP mcu is waiting to be released. It is set to the serial number of a given locked semaphore after it was read by the NWP mcu. Only \\[11:0\\]
is used."]
    #[inline(always)]
    pub fn mem_mcu_semaphore_pend(&self) -> MEM_MCU_SEMAPHORE_PEND_R {
        MEM_MCU_SEMAPHORE_PEND_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register specifies the semaphore for which the NWP mcu is waiting to be released. It is set to the serial number of a given locked semaphore after it was read by the NWP mcu. Only \\[11:0\\]
is used."]
    #[inline(always)]
    pub fn mem_mcu_semaphore_pend(&mut self) -> MEM_MCU_SEMAPHORE_PEND_W {
        MEM_MCU_SEMAPHORE_PEND_W { w: self }
    }
}
