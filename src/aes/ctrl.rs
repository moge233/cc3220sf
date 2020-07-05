#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCM`"]
pub type CCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCM`"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `CCM_L`"]
pub type CCM_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCM_L`"]
pub struct CCM_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `GCM`"]
pub type GCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GCM`"]
pub struct GCM_W<'a> {
    w: &'a mut W,
}
impl<'a> GCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `XTS`"]
pub type XTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTS`"]
pub struct XTS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `CTR_WIDTH`"]
pub type CTR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTR_WIDTH`"]
pub struct CTR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `KEY_SIZE`"]
pub type KEY_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_SIZE`"]
pub struct KEY_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:24 - Defines M? that indicated the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note that the AES Engine always returns a 128-bit authentication field@@ of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Defines L? that indicated the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. Supported values for L are (programmed value): 2 (1)@@ 4 (3) and 8 (7)."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - AES-GCM mode is selected.this is a combined mode@@ using the Galois field multiplier GF(2^128) for authentication and AES-CTR mode for encryption@@ the bits specify the GCM mode. 0x0 No operation 0x1 GHASH with H loaded and Y0-encrypted forced to zero 0x2 GHASH with H loaded and Y0-encrypted calculated internally 0x3 Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn gcm(&self) -> GCM_R {
        GCM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - AES-XTS operation is selected; the bits specify the XTS mode.01 = Previous/intermediate tweak value and j loaded (value is loaded via IV@@ j is loaded via the AAD length register) 0x0 No operation 0x1 Previous/intermediate tweak value and j loaded (value is loaded via IV@@ j is loaded via the AAD length register) 0x2 Key2@@ i and j loaded (i is loaded via IV@@ j is loaded via the AAD length register) 0x3 Key2 and i loaded@@ j=0 (i is loaded via IV)"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 0x0 Counter is 32 bits 0x1 Counter is 64 bits 0x2 Counter is 128 bits 0x3 Counter is 192 bits"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CTR_WIDTH_R {
        CTR_WIDTH_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - key size 0x0 reserved 0x1 Key is 128 bits. 0x2 Key is 192 bits 0x3 Key is 256"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:24 - Defines M? that indicated the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note that the AES Engine always returns a 128-bit authentication field@@ of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bits 19:21 - Defines L? that indicated the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. Supported values for L are (programmed value): 2 (1)@@ 4 (3) and 8 (7)."]
    #[inline(always)]
    pub fn ccm_l(&mut self) -> CCM_L_W {
        CCM_L_W { w: self }
    }
    #[doc = "Bits 16:17 - AES-GCM mode is selected.this is a combined mode@@ using the Galois field multiplier GF(2^128) for authentication and AES-CTR mode for encryption@@ the bits specify the GCM mode. 0x0 No operation 0x1 GHASH with H loaded and Y0-encrypted forced to zero 0x2 GHASH with H loaded and Y0-encrypted calculated internally 0x3 Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn gcm(&mut self) -> GCM_W {
        GCM_W { w: self }
    }
    #[doc = "Bits 11:12 - AES-XTS operation is selected; the bits specify the XTS mode.01 = Previous/intermediate tweak value and j loaded (value is loaded via IV@@ j is loaded via the AAD length register) 0x0 No operation 0x1 Previous/intermediate tweak value and j loaded (value is loaded via IV@@ j is loaded via the AAD length register) 0x2 Key2@@ i and j loaded (i is loaded via IV@@ j is loaded via the AAD length register) 0x3 Key2 and i loaded@@ j=0 (i is loaded via IV)"]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W {
        XTS_W { w: self }
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 0x0 Counter is 32 bits 0x1 Counter is 64 bits 0x2 Counter is 128 bits 0x3 Counter is 192 bits"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CTR_WIDTH_W {
        CTR_WIDTH_W { w: self }
    }
    #[doc = "Bits 3:4 - key size 0x0 reserved 0x1 Key is 128 bits. 0x2 Key is 192 bits 0x3 Key is 256"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W {
        KEY_SIZE_W { w: self }
    }
}
