#[doc = "Reader of register CC_REVISION"]
pub type R = crate::R<u32, super::CC_REVISION>;
#[doc = "Writer for register CC_REVISION"]
pub type W = crate::W<u32, super::CC_REVISION>;
#[doc = "Register CC_REVISION `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_REVISION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REV`"]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0 0x21 for 2.1"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0 0x21 for 2.1"]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
}
