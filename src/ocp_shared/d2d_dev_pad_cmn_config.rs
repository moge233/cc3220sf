#[doc = "Reader of register D2D_DEV_PAD_CMN_CONFIG"]
pub type R = crate::R<u32, super::D2D_DEV_PAD_CMN_CONFIG>;
#[doc = "Writer for register D2D_DEV_PAD_CMN_CONFIG"]
pub type W = crate::W<u32, super::D2D_DEV_PAD_CMN_CONFIG>;
#[doc = "Register D2D_DEV_PAD_CMN_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::D2D_DEV_PAD_CMN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DEV_PAD_CMN_CONF`"]
pub type MEM_DEV_PAD_CMN_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DEV_PAD_CMN_CONF`"]
pub struct MEM_DEV_PAD_CMN_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DEV_PAD_CMN_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - this register implements common IO control to all devement mode PADs; these PADs are DEV_PAD33 to DEV_PAD39. Bit \\[1:0\\]
: Drive strength control. These 2 bits are connected to DEV PAD drive strength control. possible drive stregnths are 2MA, 4MA and 6 MA for the these IO's. bit 0: when set to logic value '1' enable 2MA drive strength for DEVPAD01 to 07 bit 1: when set to logic value '1' enable 4MA drive strength for DEVPAD01 to 07. bit\\[3:2\\]
: WK PULL UP and PULL down control. These 2 bits provide IWKPUEN and IWKPDEN control for all DEV IO's. bit 2: when set to logic value '1' enable WKPU to DEVPAD01 to 07 bit 3: when set to logic value '1' enable WKPD to DEVPAD01 to 07. bit 4: WK PULL control for DEV_PKG_DETECT pin. when '1' pullup enabled else it is disable. bit 5: when set to logic value '1' enable 8MA drive strength for DEVPAD01 to 07."]
    #[inline(always)]
    pub fn mem_dev_pad_cmn_conf(&self) -> MEM_DEV_PAD_CMN_CONF_R {
        MEM_DEV_PAD_CMN_CONF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - this register implements common IO control to all devement mode PADs; these PADs are DEV_PAD33 to DEV_PAD39. Bit \\[1:0\\]
: Drive strength control. These 2 bits are connected to DEV PAD drive strength control. possible drive stregnths are 2MA, 4MA and 6 MA for the these IO's. bit 0: when set to logic value '1' enable 2MA drive strength for DEVPAD01 to 07 bit 1: when set to logic value '1' enable 4MA drive strength for DEVPAD01 to 07. bit\\[3:2\\]
: WK PULL UP and PULL down control. These 2 bits provide IWKPUEN and IWKPDEN control for all DEV IO's. bit 2: when set to logic value '1' enable WKPU to DEVPAD01 to 07 bit 3: when set to logic value '1' enable WKPD to DEVPAD01 to 07. bit 4: WK PULL control for DEV_PKG_DETECT pin. when '1' pullup enabled else it is disable. bit 5: when set to logic value '1' enable 8MA drive strength for DEVPAD01 to 07."]
    #[inline(always)]
    pub fn mem_dev_pad_cmn_conf(&mut self) -> MEM_DEV_PAD_CMN_CONF_W {
        MEM_DEV_PAD_CMN_CONF_W { w: self }
    }
}
