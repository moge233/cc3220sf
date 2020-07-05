#[doc = "Reader of register HL_REV"]
pub type R = crate::R<u32, super::HL_REV>;
#[doc = "Writer for register HL_REV"]
pub type W = crate::W<u32, super::HL_REV>;
#[doc = "Register HL_REV `reset()`'s with value 0"]
impl crate::ResetValue for super::HL_REV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCHEME`"]
pub type SCHEME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCHEME`"]
pub struct SCHEME_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHEME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `R_RTL`"]
pub type R_RTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R_RTL`"]
pub struct R_RTL_W<'a> {
    w: &'a mut W,
}
impl<'a> R_RTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `X_MAJOR`"]
pub type X_MAJOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X_MAJOR`"]
pub struct X_MAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> X_MAJOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CUSTOM`"]
pub type CUSTOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUSTOM`"]
pub struct CUSTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CUSTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `Y_MINOR`"]
pub type Y_MINOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_MINOR`"]
pub struct Y_MINOR_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_MINOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - SCHEME"]
    #[inline(always)]
    pub fn scheme(&self) -> SCHEME_R {
        SCHEME_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 16:27 - Function indicates a software compatible module family. If there is no level of software compatibility a new Func number (and hence REVISION) should be assigned."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 11:15 - RTL Version (R) maintained by IP design owner. RTL follows a numbering such as X.Y.R.Z which are explained in this table. R changes ONLY when: (1) PDS uploads occur which may have been due to spec changes (2) Bug fixes occur (3) Resets to '0' when X or Y changes. Design team has an internal 'Z' (customer invisible) number which increments on every drop that happens due to DV and RTL updates. Z resets to 0 when R increments."]
    #[inline(always)]
    pub fn r_rtl(&self) -> R_RTL_R {
        R_RTL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision (X) maintained by IP specification owner. X changes ONLY when: (1) There is a major feature addition. An example would be adding Master Mode to Utopia Level2. The Func field (or Class/Type in old PID format) will remain the same. X does NOT change due to: (1) Bug fixes (2) Change in feature parameters."]
    #[inline(always)]
    pub fn x_major(&self) -> X_MAJOR_R {
        X_MAJOR_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - CUSTOM"]
    #[inline(always)]
    pub fn custom(&self) -> CUSTOM_R {
        CUSTOM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - Minor Revision (Y) maintained by IP specification owner. Y changes ONLY when: (1) Features are scaled (up or down). Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available. (2) When feature creeps from Is-Not list to Is list. But this may not be the case once it sees silicon; in which case X will change. Y does NOT change due to: (1) Bug fixes (2) Typos or clarifications (3) major functional/feature change/addition/deletion. Instead these changes may be reflected via R S X as applicable. Spec owner maintains a customer-invisible number 'S' which changes due to: (1) Typos/clarifications (2) Bug documentation. Note that this bug is not due to a spec change but due to implementation. Nevertheless the spec tracks the IP bugs. An RTL release (say for silicon PG1.1) that occurs due to bug fix should document the corresponding spec number (X.Y.S) in its release notes."]
    #[inline(always)]
    pub fn y_minor(&self) -> Y_MINOR_R {
        Y_MINOR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - SCHEME"]
    #[inline(always)]
    pub fn scheme(&mut self) -> SCHEME_W {
        SCHEME_W { w: self }
    }
    #[doc = "Bits 16:27 - Function indicates a software compatible module family. If there is no level of software compatibility a new Func number (and hence REVISION) should be assigned."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 11:15 - RTL Version (R) maintained by IP design owner. RTL follows a numbering such as X.Y.R.Z which are explained in this table. R changes ONLY when: (1) PDS uploads occur which may have been due to spec changes (2) Bug fixes occur (3) Resets to '0' when X or Y changes. Design team has an internal 'Z' (customer invisible) number which increments on every drop that happens due to DV and RTL updates. Z resets to 0 when R increments."]
    #[inline(always)]
    pub fn r_rtl(&mut self) -> R_RTL_W {
        R_RTL_W { w: self }
    }
    #[doc = "Bits 8:10 - Major Revision (X) maintained by IP specification owner. X changes ONLY when: (1) There is a major feature addition. An example would be adding Master Mode to Utopia Level2. The Func field (or Class/Type in old PID format) will remain the same. X does NOT change due to: (1) Bug fixes (2) Change in feature parameters."]
    #[inline(always)]
    pub fn x_major(&mut self) -> X_MAJOR_W {
        X_MAJOR_W { w: self }
    }
    #[doc = "Bits 6:7 - CUSTOM"]
    #[inline(always)]
    pub fn custom(&mut self) -> CUSTOM_W {
        CUSTOM_W { w: self }
    }
    #[doc = "Bits 0:5 - Minor Revision (Y) maintained by IP specification owner. Y changes ONLY when: (1) Features are scaled (up or down). Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available. (2) When feature creeps from Is-Not list to Is list. But this may not be the case once it sees silicon; in which case X will change. Y does NOT change due to: (1) Bug fixes (2) Typos or clarifications (3) major functional/feature change/addition/deletion. Instead these changes may be reflected via R S X as applicable. Spec owner maintains a customer-invisible number 'S' which changes due to: (1) Typos/clarifications (2) Bug documentation. Note that this bug is not due to a spec change but due to implementation. Nevertheless the spec tracks the IP bugs. An RTL release (say for silicon PG1.1) that occurs due to bug fix should document the corresponding spec number (X.Y.S) in its release notes."]
    #[inline(always)]
    pub fn y_minor(&mut self) -> Y_MINOR_W {
        Y_MINOR_W { w: self }
    }
}
