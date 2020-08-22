#[doc = "Reader of register UARTRSR_UARTECR"]
pub type R = crate::R<u32, super::UARTRSR_UARTECR>;
#[doc = "Writer for register UARTRSR_UARTECR"]
pub type W = crate::W<u32, super::UARTRSR_UARTECR>;
#[doc = "Register UARTRSR_UARTECR `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTRSR_UARTECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `OE_OR_DATA`"]
pub type OE_OR_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE_OR_DATA`"]
pub struct OE_OR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_OR_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BE_OR_DATA`"]
pub type BE_OR_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BE_OR_DATA`"]
pub struct BE_OR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BE_OR_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PE_OR_DATA`"]
pub type PE_OR_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE_OR_DATA`"]
pub struct PE_OR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_OR_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FE_OR_DATA`"]
pub type FE_OR_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FE_OR_DATA`"]
pub struct FE_OR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_OR_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Error Clear"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - UART overrun error (R) or error clear (W)"]
    #[inline(always)]
    pub fn oe_or_data(&self) -> OE_OR_DATA_R {
        OE_OR_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART break error (R) or error clear (W)"]
    #[inline(always)]
    pub fn be_or_data(&self) -> BE_OR_DATA_R {
        BE_OR_DATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART parity error (R) or error clear (W)"]
    #[inline(always)]
    pub fn pe_or_data(&self) -> PE_OR_DATA_R {
        PE_OR_DATA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - UART framing error (R) or error clear (W)"]
    #[inline(always)]
    pub fn fe_or_data(&self) -> FE_OR_DATA_R {
        FE_OR_DATA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Error Clear"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bit 3 - UART overrun error (R) or error clear (W)"]
    #[inline(always)]
    pub fn oe_or_data(&mut self) -> OE_OR_DATA_W {
        OE_OR_DATA_W { w: self }
    }
    #[doc = "Bit 2 - UART break error (R) or error clear (W)"]
    #[inline(always)]
    pub fn be_or_data(&mut self) -> BE_OR_DATA_W {
        BE_OR_DATA_W { w: self }
    }
    #[doc = "Bit 1 - UART parity error (R) or error clear (W)"]
    #[inline(always)]
    pub fn pe_or_data(&mut self) -> PE_OR_DATA_W {
        PE_OR_DATA_W { w: self }
    }
    #[doc = "Bit 0 - UART framing error (R) or error clear (W)"]
    #[inline(always)]
    pub fn fe_or_data(&mut self) -> FE_OR_DATA_W {
        FE_OR_DATA_W { w: self }
    }
}
