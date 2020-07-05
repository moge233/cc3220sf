#[doc = "Reader of register CC3XX_SHARED_MEM_SEL_LSB"]
pub type R = crate::R<u32, super::CC3XX_SHARED_MEM_SEL_LSB>;
#[doc = "Writer for register CC3XX_SHARED_MEM_SEL_LSB"]
pub type W = crate::W<u32, super::CC3XX_SHARED_MEM_SEL_LSB>;
#[doc = "Register CC3XX_SHARED_MEM_SEL_LSB `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3XX_SHARED_MEM_SEL_LSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SHARED_MEM_SEL_LSB`"]
pub type MEM_SHARED_MEM_SEL_LSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_SHARED_MEM_SEL_LSB`"]
pub struct MEM_SHARED_MEM_SEL_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SHARED_MEM_SEL_LSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - This register provides memss RAM column configuration for column 0 to 9. 3 bits are allocated per column. This register is required to be configured before starting RAM access. Changing register setting while code is running will result into unpredictable memory behaviour. Register is supported to configured ones after core is booted up. 3 bit encoding per column is as follows: when 000 : WLAN, 001: NWP, 010: APPS, 011: PHY, 100: OCLA column 0 select: bit \\[2:0\\]
:when 000 -> WLAN,001 -> NWP,010 -> APPS, 011 -> PHY, 100 -> OCLA column 1 select: bit \\[5:3\\]
:column 2 select: bit \\[8 : 6\\]: column 3 select : bit \\[11: 9\\]
column 4 select : bit \\[14:12\\]
column 5 select : bit \\[17:15\\]
column 6 select : bit \\[20:18\\]
column 7 select : bit \\[23:21\\]
column 8 select : bit \\[26:24\\]
column 9 select : bit \\[29:27\\]
column 10 select"]
    #[inline(always)]
    pub fn mem_shared_mem_sel_lsb(&self) -> MEM_SHARED_MEM_SEL_LSB_R {
        MEM_SHARED_MEM_SEL_LSB_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - This register provides memss RAM column configuration for column 0 to 9. 3 bits are allocated per column. This register is required to be configured before starting RAM access. Changing register setting while code is running will result into unpredictable memory behaviour. Register is supported to configured ones after core is booted up. 3 bit encoding per column is as follows: when 000 : WLAN, 001: NWP, 010: APPS, 011: PHY, 100: OCLA column 0 select: bit \\[2:0\\]
:when 000 -> WLAN,001 -> NWP,010 -> APPS, 011 -> PHY, 100 -> OCLA column 1 select: bit \\[5:3\\]
:column 2 select: bit \\[8 : 6\\]: column 3 select : bit \\[11: 9\\]
column 4 select : bit \\[14:12\\]
column 5 select : bit \\[17:15\\]
column 6 select : bit \\[20:18\\]
column 7 select : bit \\[23:21\\]
column 8 select : bit \\[26:24\\]
column 9 select : bit \\[29:27\\]
column 10 select"]
    #[inline(always)]
    pub fn mem_shared_mem_sel_lsb(&mut self) -> MEM_SHARED_MEM_SEL_LSB_W {
        MEM_SHARED_MEM_SEL_LSB_W { w: self }
    }
}
