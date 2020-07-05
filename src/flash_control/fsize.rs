#[doc = "Reader of register FSIZE"]
pub type R = crate::R<u32, super::FSIZE>;
#[doc = "Writer for register FSIZE"]
pub type W = crate::W<u32, super::FSIZE>;
#[doc = "Register FSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::FSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Size Indicates the size of the on-chip Flash memory. Value Description 0x0003 8 KB of Flash 0x0007 16 KB of Flash 0x000F 32 KB of Flash 0x001F 64 KB of Flash 0x002F 96 KB of Flash 0x003F 128 KB of Flash 0x005F 192 KB of Flash 0x007F 256 KB of Flash"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Size Indicates the size of the on-chip Flash memory. Value Description 0x0003 8 KB of Flash 0x0007 16 KB of Flash 0x000F 32 KB of Flash 0x001F 64 KB of Flash 0x002F 96 KB of Flash 0x003F 128 KB of Flash 0x005F 192 KB of Flash 0x007F 256 KB of Flash"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}
