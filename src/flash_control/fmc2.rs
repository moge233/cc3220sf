#[doc = "Reader of register FMC2"]
pub type R = crate::R<u32, super::FMC2>;
#[doc = "Writer for register FMC2"]
pub type W = crate::W<u32, super::FMC2>;
#[doc = "Register FMC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRKEY`"]
pub type WRKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WRKEY`"]
pub struct WRKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Flash Memory Write Key This field contains a write key which is used to minimize the incidence of accidental Flash memory writes. The value 0xA442 must be written into this field for a write to occur. Writes to the FMC2 register without this WRKEY value are ignored. A read of this field returns the value 0."]
    #[inline(always)]
    pub fn wrkey(&self) -> WRKEY_R {
        WRKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Flash Memory Write Key This field contains a write key which is used to minimize the incidence of accidental Flash memory writes. The value 0xA442 must be written into this field for a write to occur. Writes to the FMC2 register without this WRKEY value are ignored. A read of this field returns the value 0."]
    #[inline(always)]
    pub fn wrkey(&mut self) -> WRKEY_W {
        WRKEY_W { w: self }
    }
}
