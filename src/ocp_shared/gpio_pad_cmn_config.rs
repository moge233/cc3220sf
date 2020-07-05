#[doc = "Reader of register GPIO_PAD_CMN_CONFIG"]
pub type R = crate::R<u32, super::GPIO_PAD_CMN_CONFIG>;
#[doc = "Writer for register GPIO_PAD_CMN_CONFIG"]
pub type W = crate::W<u32, super::GPIO_PAD_CMN_CONFIG>;
#[doc = "Register GPIO_PAD_CMN_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_PAD_CMN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_PAD_HYSTVAL`"]
pub type MEM_PAD_HYSTVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_PAD_HYSTVAL`"]
pub struct MEM_PAD_HYSTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PAD_HYSTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - 00: hysteriris = 10% of VDDS (difference between upper and lower threshold of the schmit trigger) 01: hysteriris = 20% of VDDS (difference between upper and lower threshold of the schmit trigger) 10: hysteriris = 30% of VDDS (difference between upper and lower threshold of the schmit trigger) 11: hysteriris = 40% of VDDS (difference between upper and lower threshold of the schmit trigger)&quot; &quot;&quot;&quot;"]
    #[inline(always)]
    pub fn mem_pad_hystval(&self) -> MEM_PAD_HYSTVAL_R {
        MEM_PAD_HYSTVAL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - 00: hysteriris = 10% of VDDS (difference between upper and lower threshold of the schmit trigger) 01: hysteriris = 20% of VDDS (difference between upper and lower threshold of the schmit trigger) 10: hysteriris = 30% of VDDS (difference between upper and lower threshold of the schmit trigger) 11: hysteriris = 40% of VDDS (difference between upper and lower threshold of the schmit trigger)&quot; &quot;&quot;&quot;"]
    #[inline(always)]
    pub fn mem_pad_hystval(&mut self) -> MEM_PAD_HYSTVAL_W {
        MEM_PAD_HYSTVAL_W { w: self }
    }
}
