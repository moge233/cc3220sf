#[doc = "Reader of register HL_HWINFO"]
pub type R = crate::R<u32, super::HL_HWINFO>;
#[doc = "Writer for register HL_HWINFO"]
pub type W = crate::W<u32, super::HL_HWINFO>;
#[doc = "Register HL_HWINFO `reset()`'s with value 0"]
impl crate::ResetValue for super::HL_HWINFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SIZE`"]
pub type MEM_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SIZE`"]
pub struct MEM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:5 - MEM_SIZE"]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5 - MEM_SIZE"]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W {
        MEM_SIZE_W { w: self }
    }
}
