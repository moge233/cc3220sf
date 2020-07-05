#[doc = "Reader of register PSTATE"]
pub type R = crate::R<u32, super::PSTATE>;
#[doc = "Writer for register PSTATE"]
pub type W = crate::W<u32, super::PSTATE>;
#[doc = "Register PSTATE `reset()`'s with value 0"]
impl crate::ResetValue for super::PSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLEV`"]
pub type DLEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLEV`"]
pub struct DLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> DLEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
line signal level DAT\\[3\\]
=> bit 23 DAT\\[2\\]
=> bit 22 DAT\\[1\\]
=> bit 21 DAT\\[0\\]
=> bit 20 This status is used to check DAT line level to recover from errors and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]. The value of these registers after reset depends on the DAT lines level at that time."]
    #[inline(always)]
    pub fn dlev(&self) -> DLEV_R {
        DLEV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
line signal level DAT\\[3\\]
=> bit 23 DAT\\[2\\]
=> bit 22 DAT\\[1\\]
=> bit 21 DAT\\[0\\]
=> bit 20 This status is used to check DAT line level to recover from errors and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]. The value of these registers after reset depends on the DAT lines level at that time."]
    #[inline(always)]
    pub fn dlev(&mut self) -> DLEV_W {
        DLEV_W { w: self }
    }
}
