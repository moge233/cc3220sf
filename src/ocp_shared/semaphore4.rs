#[doc = "Reader of register SEMAPHORE4"]
pub type R = crate::R<u32, super::SEMAPHORE4>;
#[doc = "Writer for register SEMAPHORE4"]
pub type W = crate::W<u32, super::SEMAPHORE4>;
#[doc = "Register SEMAPHORE4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORE4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SEMAPHORE4`"]
pub type MEM_SEMAPHORE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SEMAPHORE4`"]
pub struct MEM_SEMAPHORE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SEMAPHORE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - General Purpose Semaphore for SW Usage. If any of the 2 bits of a given register is set to 1, it means that the semaphore is locked by one of the masters. Each bit represents a master IP as follows: {WLAN,NWP}. The JTAG cannot capture the semaphore but it can release it. As a master IP reads the semaphore, it will be caputed and the masters correlating bit will be set to 1 (set upon read). As any IP writes to this address (independent of the written data) the semaphore will be set to 2'b00."]
    #[inline(always)]
    pub fn mem_semaphore4(&self) -> MEM_SEMAPHORE4_R {
        MEM_SEMAPHORE4_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - General Purpose Semaphore for SW Usage. If any of the 2 bits of a given register is set to 1, it means that the semaphore is locked by one of the masters. Each bit represents a master IP as follows: {WLAN,NWP}. The JTAG cannot capture the semaphore but it can release it. As a master IP reads the semaphore, it will be caputed and the masters correlating bit will be set to 1 (set upon read). As any IP writes to this address (independent of the written data) the semaphore will be set to 2'b00."]
    #[inline(always)]
    pub fn mem_semaphore4(&mut self) -> MEM_SEMAPHORE4_W {
        MEM_SEMAPHORE4_W { w: self }
    }
}
