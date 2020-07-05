#[doc = "Reader of register C_LENGTH_1"]
pub type R = crate::R<u32, super::C_LENGTH_1>;
#[doc = "Writer for register C_LENGTH_1"]
pub type W = crate::W<u32, super::C_LENGTH_1>;
#[doc = "Register C_LENGTH_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C_LENGTH_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Data length (MSW) length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Data length (MSW) length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
}
