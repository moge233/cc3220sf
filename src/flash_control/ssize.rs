#[doc = "Reader of register SSIZE"]
pub type R = crate::R<u32, super::SSIZE>;
#[doc = "Writer for register SSIZE"]
pub type W = crate::W<u32, super::SSIZE>;
#[doc = "Register SSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::SSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM_SIZE`"]
pub type SRAM_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SRAM_SIZE`"]
pub struct SRAM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SRAM Size Indicates the size of the on-chip SRAM. Value Description 0x0007 2 KB of SRAM 0x000F 4 KB of SRAM 0x0017 6 KB of SRAM 0x001F 8 KB of SRAM 0x002F 12 KB of SRAM 0x003F 16 KB of SRAM 0x004F 20 KB of SRAM 0x005F 24 KB of SRAM 0x007F 32 KB of SRAM"]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRAM Size Indicates the size of the on-chip SRAM. Value Description 0x0007 2 KB of SRAM 0x000F 4 KB of SRAM 0x0017 6 KB of SRAM 0x001F 8 KB of SRAM 0x002F 12 KB of SRAM 0x003F 16 KB of SRAM 0x004F 20 KB of SRAM 0x005F 24 KB of SRAM 0x007F 32 KB of SRAM"]
    #[inline(always)]
    pub fn sram_size(&mut self) -> SRAM_SIZE_W {
        SRAM_SIZE_W { w: self }
    }
}
