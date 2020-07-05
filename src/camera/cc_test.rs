#[doc = "Reader of register CC_TEST"]
pub type R = crate::R<u32, super::CC_TEST>;
#[doc = "Writer for register CC_TEST"]
pub type W = crate::W<u32, super::CC_TEST>;
#[doc = "Register CC_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFO_RD_POINTER`"]
pub type FIFO_RD_POINTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_RD_POINTER`"]
pub struct FIFO_RD_POINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RD_POINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `FIFO_WR_POINTER`"]
pub type FIFO_WR_POINTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_WR_POINTER`"]
pub struct FIFO_WR_POINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_WR_POINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FIFO_LEVEL`"]
pub type FIFO_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_LEVEL`"]
pub struct FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFO_LEVEL_PEAK`"]
pub type FIFO_LEVEL_PEAK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_LEVEL_PEAK`"]
pub struct FIFO_LEVEL_PEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_LEVEL_PEAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - FIFO READ Pointer This field shows the value of the FIFO read pointer Expected value ranges from 0 to 127"]
    #[inline(always)]
    pub fn fifo_rd_pointer(&self) -> FIFO_RD_POINTER_R {
        FIFO_RD_POINTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FIFO WRITE pointer This field shows the value of the FIFO write pointer Expected value ranges from 0 to 127"]
    #[inline(always)]
    pub fn fifo_wr_pointer(&self) -> FIFO_WR_POINTER_R {
        FIFO_WR_POINTER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - FIFO level (how many 32-bit words the FIFO contains) This field shows the value of the FIFO level and can assume values from 0 to 128"]
    #[inline(always)]
    pub fn fifo_level(&self) -> FIFO_LEVEL_R {
        FIFO_LEVEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - FIFO level peak This field shows the max value of the FIFO level and can assume values from 0 to 128"]
    #[inline(always)]
    pub fn fifo_level_peak(&self) -> FIFO_LEVEL_PEAK_R {
        FIFO_LEVEL_PEAK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - FIFO READ Pointer This field shows the value of the FIFO read pointer Expected value ranges from 0 to 127"]
    #[inline(always)]
    pub fn fifo_rd_pointer(&mut self) -> FIFO_RD_POINTER_W {
        FIFO_RD_POINTER_W { w: self }
    }
    #[doc = "Bits 16:23 - FIFO WRITE pointer This field shows the value of the FIFO write pointer Expected value ranges from 0 to 127"]
    #[inline(always)]
    pub fn fifo_wr_pointer(&mut self) -> FIFO_WR_POINTER_W {
        FIFO_WR_POINTER_W { w: self }
    }
    #[doc = "Bits 8:15 - FIFO level (how many 32-bit words the FIFO contains) This field shows the value of the FIFO level and can assume values from 0 to 128"]
    #[inline(always)]
    pub fn fifo_level(&mut self) -> FIFO_LEVEL_W {
        FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:7 - FIFO level peak This field shows the max value of the FIFO level and can assume values from 0 to 128"]
    #[inline(always)]
    pub fn fifo_level_peak(&mut self) -> FIFO_LEVEL_PEAK_W {
        FIFO_LEVEL_PEAK_W { w: self }
    }
}
