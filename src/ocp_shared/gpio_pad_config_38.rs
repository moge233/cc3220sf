#[doc = "Reader of register GPIO_PAD_CONFIG_38"]
pub type R = crate::R<u32, super::GPIO_PAD_CONFIG_38>;
#[doc = "Writer for register GPIO_PAD_CONFIG_38"]
pub type W = crate::W<u32, super::GPIO_PAD_CONFIG_38>;
#[doc = "Register GPIO_PAD_CONFIG_38 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_PAD_CONFIG_38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_GPIO_PAD_CONFIG_38`"]
pub type MEM_GPIO_PAD_CONFIG_38_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_GPIO_PAD_CONFIG_38`"]
pub struct MEM_GPIO_PAD_CONFIG_38_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_GPIO_PAD_CONFIG_38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - GPIO 0 register: &quot;Bit 0 - 3 is used for PAD IO mode selection. io_register={ &quot;&quot; 0 => &quot;&quot;&quot;&quot;CONFMODE\\[0\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 1 => &quot;&quot;&quot;&quot;CONFMODE\\[1\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 2 => &quot;&quot;&quot;&quot;CONFMODE\\[2\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 3 => &quot;&quot;&quot;&quot;CONFMODE\\[3\\]&quot;&quot;&quot;&quot; 4 => &quot;&quot;&quot;&quot;IOE_N&quot;&quot;&quot;&quot; --> output enable value. level 0 enables the IDO to PAD path. Else PAD is tristated (except for the PU/PD which are independent).&quot; &quot;Value gets latched at rising edge of RET33&quot;&quot;&quot; &quot;&quot;&quot; 5 =>&quot;&quot;&quot;&quot; IOE_N_OV&quot;&quot;&quot;&quot; --> output enable overirde. when bit is set to logic '1' IOE_N (bit 4) value will control IO IOE_N signal else IOE_N is control via selected HW logic. strong PULL UP and PULL Down control is disabled for all IO's. both controls are tied to logic level '0'. IODEN and I8MAEN is diesabled for all development IO's. These signals are tied to logic level '0'. common control is implemented for I2MAEN, I4MAEN, WKPU, WKPD control . refer dev_pad_cmn_config register bits."]
    #[inline(always)]
    pub fn mem_gpio_pad_config_38(&self) -> MEM_GPIO_PAD_CONFIG_38_R {
        MEM_GPIO_PAD_CONFIG_38_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - GPIO 0 register: &quot;Bit 0 - 3 is used for PAD IO mode selection. io_register={ &quot;&quot; 0 => &quot;&quot;&quot;&quot;CONFMODE\\[0\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 1 => &quot;&quot;&quot;&quot;CONFMODE\\[1\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 2 => &quot;&quot;&quot;&quot;CONFMODE\\[2\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 3 => &quot;&quot;&quot;&quot;CONFMODE\\[3\\]&quot;&quot;&quot;&quot; 4 => &quot;&quot;&quot;&quot;IOE_N&quot;&quot;&quot;&quot; --> output enable value. level 0 enables the IDO to PAD path. Else PAD is tristated (except for the PU/PD which are independent).&quot; &quot;Value gets latched at rising edge of RET33&quot;&quot;&quot; &quot;&quot;&quot; 5 =>&quot;&quot;&quot;&quot; IOE_N_OV&quot;&quot;&quot;&quot; --> output enable overirde. when bit is set to logic '1' IOE_N (bit 4) value will control IO IOE_N signal else IOE_N is control via selected HW logic. strong PULL UP and PULL Down control is disabled for all IO's. both controls are tied to logic level '0'. IODEN and I8MAEN is diesabled for all development IO's. These signals are tied to logic level '0'. common control is implemented for I2MAEN, I4MAEN, WKPU, WKPD control . refer dev_pad_cmn_config register bits."]
    #[inline(always)]
    pub fn mem_gpio_pad_config_38(&mut self) -> MEM_GPIO_PAD_CONFIG_38_W {
        MEM_GPIO_PAD_CONFIG_38_W { w: self }
    }
}
