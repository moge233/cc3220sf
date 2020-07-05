#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNC11`"]
pub type SYNC11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC11`"]
pub struct SYNC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SYNC10`"]
pub type SYNC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC10`"]
pub struct SYNC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SYNC9`"]
pub type SYNC9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC9`"]
pub struct SYNC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SYNC8`"]
pub type SYNC8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC8`"]
pub struct SYNC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SYNC7`"]
pub type SYNC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC7`"]
pub struct SYNC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SYNC6`"]
pub type SYNC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC6`"]
pub struct SYNC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYNC5`"]
pub type SYNC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC5`"]
pub struct SYNC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SYNC4`"]
pub type SYNC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC4`"]
pub struct SYNC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SYNC3`"]
pub type SYNC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC3`"]
pub struct SYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYNC2`"]
pub type SYNC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC2`"]
pub struct SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYNC1`"]
pub type SYNC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC1`"]
pub struct SYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYNC0`"]
pub type SYNC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC0`"]
pub struct SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - Synchronize GPTM Timer 11"]
    #[inline(always)]
    pub fn sync11(&self) -> SYNC11_R {
        SYNC11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Synchronize GPTM Timer 10"]
    #[inline(always)]
    pub fn sync10(&self) -> SYNC10_R {
        SYNC10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Synchronize GPTM Timer 9"]
    #[inline(always)]
    pub fn sync9(&self) -> SYNC9_R {
        SYNC9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Synchronize GPTM Timer 8"]
    #[inline(always)]
    pub fn sync8(&self) -> SYNC8_R {
        SYNC8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn sync7(&self) -> SYNC7_R {
        SYNC7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn sync6(&self) -> SYNC6_R {
        SYNC6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn sync5(&self) -> SYNC5_R {
        SYNC5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn sync4(&self) -> SYNC4_R {
        SYNC4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - Synchronize GPTM Timer 11"]
    #[inline(always)]
    pub fn sync11(&mut self) -> SYNC11_W {
        SYNC11_W { w: self }
    }
    #[doc = "Bits 20:21 - Synchronize GPTM Timer 10"]
    #[inline(always)]
    pub fn sync10(&mut self) -> SYNC10_W {
        SYNC10_W { w: self }
    }
    #[doc = "Bits 18:19 - Synchronize GPTM Timer 9"]
    #[inline(always)]
    pub fn sync9(&mut self) -> SYNC9_W {
        SYNC9_W { w: self }
    }
    #[doc = "Bits 16:17 - Synchronize GPTM Timer 8"]
    #[inline(always)]
    pub fn sync8(&mut self) -> SYNC8_W {
        SYNC8_W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn sync7(&mut self) -> SYNC7_W {
        SYNC7_W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn sync6(&mut self) -> SYNC6_W {
        SYNC6_W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn sync5(&mut self) -> SYNC5_W {
        SYNC5_W { w: self }
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn sync4(&mut self) -> SYNC4_W {
        SYNC4_W { w: self }
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn sync3(&mut self) -> SYNC3_W {
        SYNC3_W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W {
        SYNC2_W { w: self }
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> SYNC1_W {
        SYNC1_W { w: self }
    }
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W {
        SYNC0_W { w: self }
    }
}
