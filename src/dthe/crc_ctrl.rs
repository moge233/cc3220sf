#[doc = "Reader of register CRC_CTRL"]
pub type R = crate::R<u32, super::CRC_CTRL>;
#[doc = "Writer for register CRC_CTRL"]
pub type W = crate::W<u32, super::CRC_CTRL>;
#[doc = "Register CRC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `ENDIAN`"]
pub type ENDIAN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENDIAN`"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:14 - Initialize the CRC 00 use SEED register context as starting value 10 all zero? 11 all one? This is self clearing. With first write to data register this value clears to zero and remain zero for rest of the operation unless written again"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Endian control \\[0\\]
swap byte in half-word \\[1\\]
swap half word"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Type of operation 0000 polynomial 0x8005 0001 polynomial 0x1021 0010 polynomial 0x4C11DB7 0011 polynomial 0x1EDC6F41 1000 TCP checksum TYPE in DTHE_S_CRC_CTRL & DTHE_S_CRC_CTRL should be exclusive"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 13:14 - Initialize the CRC 00 use SEED register context as starting value 10 all zero? 11 all one? This is self clearing. With first write to data register this value clears to zero and remain zero for rest of the operation unless written again"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bits 4:5 - Endian control \\[0\\]
swap byte in half-word \\[1\\]
swap half word"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bits 0:3 - Type of operation 0000 polynomial 0x8005 0001 polynomial 0x1021 0010 polynomial 0x4C11DB7 0011 polynomial 0x1EDC6F41 1000 TCP checksum TYPE in DTHE_S_CRC_CTRL & DTHE_S_CRC_CTRL should be exclusive"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
}
