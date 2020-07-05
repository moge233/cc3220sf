#[doc = "Reader of register CC3XX_SHARED_MEM_SEL_MSB"]
pub type R = crate::R<u32, super::CC3XX_SHARED_MEM_SEL_MSB>;
#[doc = "Writer for register CC3XX_SHARED_MEM_SEL_MSB"]
pub type W = crate::W<u32, super::CC3XX_SHARED_MEM_SEL_MSB>;
#[doc = "Register CC3XX_SHARED_MEM_SEL_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3XX_SHARED_MEM_SEL_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SHARED_MEM_SEL_MSB`"]
pub type MEM_SHARED_MEM_SEL_MSB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_SHARED_MEM_SEL_MSB`"]
pub struct MEM_SHARED_MEM_SEL_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SHARED_MEM_SEL_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This register provides memss RAM column configuration for column 10 to 15. 3 bits are allocated per column. This register is required to be configured before starting RAM access. Changing register setting while code is running will result into unpredictable memory behaviour. Register is supported to configured ones after core is booted up. 3 bit encoding per column is as follows: when 000 : WLAN, 001: NWP, 010: APPS, 011: PHY, 100: OCLA column 11 select : bit \\[2:0\\]
column 12 select : bit \\[5:3\\]
column 13 select : bit \\[8 : 6\\]
column 14 select :"]
    #[inline(always)]
    pub fn mem_shared_mem_sel_msb(&self) -> MEM_SHARED_MEM_SEL_MSB_R {
        MEM_SHARED_MEM_SEL_MSB_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register provides memss RAM column configuration for column 10 to 15. 3 bits are allocated per column. This register is required to be configured before starting RAM access. Changing register setting while code is running will result into unpredictable memory behaviour. Register is supported to configured ones after core is booted up. 3 bit encoding per column is as follows: when 000 : WLAN, 001: NWP, 010: APPS, 011: PHY, 100: OCLA column 11 select : bit \\[2:0\\]
column 12 select : bit \\[5:3\\]
column 13 select : bit \\[8 : 6\\]
column 14 select :"]
    #[inline(always)]
    pub fn mem_shared_mem_sel_msb(&mut self) -> MEM_SHARED_MEM_SEL_MSB_W {
        MEM_SHARED_MEM_SEL_MSB_W { w: self }
    }
}
