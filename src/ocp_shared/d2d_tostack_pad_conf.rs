#[doc = "Reader of register D2D_TOSTACK_PAD_CONF"]
pub type R = crate::R<u32, super::D2D_TOSTACK_PAD_CONF>;
#[doc = "Writer for register D2D_TOSTACK_PAD_CONF"]
pub type W = crate::W<u32, super::D2D_TOSTACK_PAD_CONF>;
#[doc = "Register D2D_TOSTACK_PAD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::D2D_TOSTACK_PAD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_D2D_TOSTACK_PAD_CONF`"]
pub type MEM_D2D_TOSTACK_PAD_CONF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_D2D_TOSTACK_PAD_CONF`"]
pub struct MEM_D2D_TOSTACK_PAD_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_D2D_TOSTACK_PAD_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - OEN/OEN2X control. When 0 : Act as input buffer else output buffer with drive strength 2. this register control OEN2X pin of D2D TOSTACK PAD: OEN1X and OEN2X decoding is as follows: &quot;when &quot;&quot;00&quot;&quot; :&quot; &quot;when &quot;&quot;01&quot;&quot; : dirve strength is '1' and output buffer enabled.&quot; &quot;when &quot;&quot;10&quot;&quot; : drive strength is 2 and output buffer is disabled.&quot; &quot;when &quot;&quot;11&quot;&quot; : dirve strength is '3' and output buffer enabled.&quot;"]
    #[inline(always)]
    pub fn mem_d2d_tostack_pad_conf(&self) -> MEM_D2D_TOSTACK_PAD_CONF_R {
        MEM_D2D_TOSTACK_PAD_CONF_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - OEN/OEN2X control. When 0 : Act as input buffer else output buffer with drive strength 2. this register control OEN2X pin of D2D TOSTACK PAD: OEN1X and OEN2X decoding is as follows: &quot;when &quot;&quot;00&quot;&quot; :&quot; &quot;when &quot;&quot;01&quot;&quot; : dirve strength is '1' and output buffer enabled.&quot; &quot;when &quot;&quot;10&quot;&quot; : drive strength is 2 and output buffer is disabled.&quot; &quot;when &quot;&quot;11&quot;&quot; : dirve strength is '3' and output buffer enabled.&quot;"]
    #[inline(always)]
    pub fn mem_d2d_tostack_pad_conf(&mut self) -> MEM_D2D_TOSTACK_PAD_CONF_W {
        MEM_D2D_TOSTACK_PAD_CONF_W { w: self }
    }
}
