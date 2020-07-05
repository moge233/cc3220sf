#[doc = "Reader of register MEM_SYSCLK_DIV_CFG"]
pub type R = crate::R<u32, super::MEM_SYSCLK_DIV_CFG>;
#[doc = "Writer for register MEM_SYSCLK_DIV_CFG"]
pub type W = crate::W<u32, super::MEM_SYSCLK_DIV_CFG>;
#[doc = "Register MEM_SYSCLK_DIV_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_SYSCLK_DIV_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SYSCLK_DIV_OFF_TIME`"]
pub type MEM_SYSCLK_DIV_OFF_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SYSCLK_DIV_OFF_TIME`"]
pub struct MEM_SYSCLK_DIV_OFF_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SYSCLK_DIV_OFF_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MEM_SYSCLK_DIV_ON_TIME`"]
pub type MEM_SYSCLK_DIV_ON_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SYSCLK_DIV_ON_TIME`"]
pub struct MEM_SYSCLK_DIV_ON_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SYSCLK_DIV_ON_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - MEM_SYSCLK_DIV_OFF_TIME"]
    #[inline(always)]
    pub fn mem_sysclk_div_off_time(&self) -> MEM_SYSCLK_DIV_OFF_TIME_R {
        MEM_SYSCLK_DIV_OFF_TIME_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - MEM_SYSCLK_DIV_ON_TIME"]
    #[inline(always)]
    pub fn mem_sysclk_div_on_time(&self) -> MEM_SYSCLK_DIV_ON_TIME_R {
        MEM_SYSCLK_DIV_ON_TIME_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - MEM_SYSCLK_DIV_OFF_TIME"]
    #[inline(always)]
    pub fn mem_sysclk_div_off_time(&mut self) -> MEM_SYSCLK_DIV_OFF_TIME_W {
        MEM_SYSCLK_DIV_OFF_TIME_W { w: self }
    }
    #[doc = "Bits 0:2 - MEM_SYSCLK_DIV_ON_TIME"]
    #[inline(always)]
    pub fn mem_sysclk_div_on_time(&mut self) -> MEM_SYSCLK_DIV_ON_TIME_W {
        MEM_SYSCLK_DIV_ON_TIME_W { w: self }
    }
}
