#[doc = "Reader of register CC_SYSCONFIG"]
pub type R = crate::R<u32, super::CC_SYSCONFIG>;
#[doc = "Writer for register CC_SYSCONFIG"]
pub type W = crate::W<u32, super::CC_SYSCONFIG>;
#[doc = "Register CC_SYSCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_SYSCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S_IDLE_MODE`"]
pub type S_IDLE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S_IDLE_MODE`"]
pub struct S_IDLE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_IDLE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - Slave interface power management req/ack control &quot;&quot;&quot;00&quot;&quot; Force-idle. An idle request is acknoledged unconditionally&quot; &quot;&quot;&quot;01&quot;&quot; No-idle. An idle request is never acknowledged&quot; &quot;&quot;&quot;10&quot;&quot; reserved (Smart-idle not implemented)&quot;"]
    #[inline(always)]
    pub fn s_idle_mode(&self) -> S_IDLE_MODE_R {
        S_IDLE_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - Slave interface power management req/ack control &quot;&quot;&quot;00&quot;&quot; Force-idle. An idle request is acknoledged unconditionally&quot; &quot;&quot;&quot;01&quot;&quot; No-idle. An idle request is never acknowledged&quot; &quot;&quot;&quot;10&quot;&quot; reserved (Smart-idle not implemented)&quot;"]
    #[inline(always)]
    pub fn s_idle_mode(&mut self) -> S_IDLE_MODE_W {
        S_IDLE_MODE_W { w: self }
    }
}
