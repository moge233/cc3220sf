#[doc = "Reader of register CC_CTRL"]
pub type R = crate::R<u32, super::CC_CTRL>;
#[doc = "Writer for register CC_CTRL"]
pub type W = crate::W<u32, super::CC_CTRL>;
#[doc = "Register CC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAR_MODE`"]
pub type PAR_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAR_MODE`"]
pub struct PAR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - Sets the Protocol Mode of the Camera Core module in parallel mode (when CCP_MODE = 0) &quot;&quot;&quot;000&quot;&quot; Parallel NOBT 8-bit&quot; &quot;&quot;&quot;001&quot;&quot; Parallel NOBT 10-bit&quot; &quot;&quot;&quot;010&quot;&quot; Parallel NOBT 12-bit&quot; &quot;&quot;&quot;011&quot;&quot; reserved&quot; &quot;&quot;&quot;100&quot;&quot; Parallet BT 8-bit&quot; &quot;&quot;&quot;101&quot;&quot; Parallel BT 10-bit&quot; &quot;&quot;&quot;110&quot;&quot; reserved&quot; &quot;&quot;&quot;111&quot;&quot; FIFO test mode. Refer to Table 12 - FIFO Write and Read access&quot;"]
    #[inline(always)]
    pub fn par_mode(&self) -> PAR_MODE_R {
        PAR_MODE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Sets the Protocol Mode of the Camera Core module in parallel mode (when CCP_MODE = 0) &quot;&quot;&quot;000&quot;&quot; Parallel NOBT 8-bit&quot; &quot;&quot;&quot;001&quot;&quot; Parallel NOBT 10-bit&quot; &quot;&quot;&quot;010&quot;&quot; Parallel NOBT 12-bit&quot; &quot;&quot;&quot;011&quot;&quot; reserved&quot; &quot;&quot;&quot;100&quot;&quot; Parallet BT 8-bit&quot; &quot;&quot;&quot;101&quot;&quot; Parallel BT 10-bit&quot; &quot;&quot;&quot;110&quot;&quot; reserved&quot; &quot;&quot;&quot;111&quot;&quot; FIFO test mode. Refer to Table 12 - FIFO Write and Read access&quot;"]
    #[inline(always)]
    pub fn par_mode(&mut self) -> PAR_MODE_W {
        PAR_MODE_W { w: self }
    }
}
