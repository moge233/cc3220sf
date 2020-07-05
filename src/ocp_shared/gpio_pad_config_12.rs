#[doc = "Reader of register GPIO_PAD_CONFIG_12"]
pub type R = crate::R<u32, super::GPIO_PAD_CONFIG_12>;
#[doc = "Writer for register GPIO_PAD_CONFIG_12"]
pub type W = crate::W<u32, super::GPIO_PAD_CONFIG_12>;
#[doc = "Register GPIO_PAD_CONFIG_12 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_PAD_CONFIG_12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_GPIO_PAD_CONFIG_12`"]
pub type MEM_GPIO_PAD_CONFIG_12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_GPIO_PAD_CONFIG_12`"]
pub struct MEM_GPIO_PAD_CONFIG_12_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_GPIO_PAD_CONFIG_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - GPIO 0 register: &quot;Bit 0 - 3 is used for PAD IO mode selection. io_register={ &quot;&quot; 0 => &quot;&quot;&quot;&quot;CONFMODE\\[0\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 1 => &quot;&quot;&quot;&quot;CONFMODE\\[1\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 2 => &quot;&quot;&quot;&quot;CONFMODE\\[2\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 3 => &quot;&quot;&quot;&quot;CONFMODE\\[3\\]&quot;&quot;&quot;&quot; 4 => &quot;&quot;&quot;&quot;IODEN&quot;&quot;&quot;&quot; --> When level 1 this disables the PMOS xtors of the output stages making them open-drain type.&quot; it can be used for I2C type of peripherals. 5 => &quot;&quot;&quot;&quot;I2MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 2mA output stage&quot;&quot;&quot; &quot;&quot;&quot; 6 => &quot;&quot;&quot;&quot;I4MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 4mA output stage&quot;&quot;&quot; &quot;&quot;&quot; 7 => &quot;&quot;&quot;&quot;I8MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 8mA output stage. Note: any drive strength between 2mA and 14mA can be obtained with combination of 2mA 4mA and 8mA.&quot;&quot;&quot; &quot;&quot;&quot; 8 => &quot;&quot;&quot;&quot;IWKPUEN&quot;&quot;&quot;&quot; --> 10uA pull up (weak strength)&quot;&quot;&quot; &quot;&quot;&quot; 9 => &quot;&quot;&quot;&quot;IWKPDEN&quot;&quot;&quot;&quot; --> 10uA pull down (weak strength)&quot;&quot;&quot; &quot;&quot;&quot; 10 => &quot;&quot;&quot;&quot;IOE_N&quot;&quot;&quot;&quot; --> output enable value. level 0 enables the IDO to PAD path. Else PAD is tristated (except for the PU/PD which are independent).&quot; &quot;Value gets latched at rising edge of RET33&quot;&quot;&quot; &quot;&quot;&quot; 11 =>&quot;&quot;&quot;&quot; IOE_N_OV&quot;&quot;&quot;&quot; --> output enable overirde. when bit is set to logic '1' IOE_N (bit 4) value will control IO IOE_N signal else IOE_N is control via selected HW logic. strong PULL UP and PULL Down control is disabled for all IO's. both controls are tied to logic level '0'."]
    #[inline(always)]
    pub fn mem_gpio_pad_config_12(&self) -> MEM_GPIO_PAD_CONFIG_12_R {
        MEM_GPIO_PAD_CONFIG_12_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - GPIO 0 register: &quot;Bit 0 - 3 is used for PAD IO mode selection. io_register={ &quot;&quot; 0 => &quot;&quot;&quot;&quot;CONFMODE\\[0\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 1 => &quot;&quot;&quot;&quot;CONFMODE\\[1\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 2 => &quot;&quot;&quot;&quot;CONFMODE\\[2\\]&quot;&quot;&quot;&quot;&quot;&quot; &quot;&quot; 3 => &quot;&quot;&quot;&quot;CONFMODE\\[3\\]&quot;&quot;&quot;&quot; 4 => &quot;&quot;&quot;&quot;IODEN&quot;&quot;&quot;&quot; --> When level 1 this disables the PMOS xtors of the output stages making them open-drain type.&quot; it can be used for I2C type of peripherals. 5 => &quot;&quot;&quot;&quot;I2MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 2mA output stage&quot;&quot;&quot; &quot;&quot;&quot; 6 => &quot;&quot;&quot;&quot;I4MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 4mA output stage&quot;&quot;&quot; &quot;&quot;&quot; 7 => &quot;&quot;&quot;&quot;I8MAEN&quot;&quot;&quot;&quot; --> Level 1 enables the approx 8mA output stage. Note: any drive strength between 2mA and 14mA can be obtained with combination of 2mA 4mA and 8mA.&quot;&quot;&quot; &quot;&quot;&quot; 8 => &quot;&quot;&quot;&quot;IWKPUEN&quot;&quot;&quot;&quot; --> 10uA pull up (weak strength)&quot;&quot;&quot; &quot;&quot;&quot; 9 => &quot;&quot;&quot;&quot;IWKPDEN&quot;&quot;&quot;&quot; --> 10uA pull down (weak strength)&quot;&quot;&quot; &quot;&quot;&quot; 10 => &quot;&quot;&quot;&quot;IOE_N&quot;&quot;&quot;&quot; --> output enable value. level 0 enables the IDO to PAD path. Else PAD is tristated (except for the PU/PD which are independent).&quot; &quot;Value gets latched at rising edge of RET33&quot;&quot;&quot; &quot;&quot;&quot; 11 =>&quot;&quot;&quot;&quot; IOE_N_OV&quot;&quot;&quot;&quot; --> output enable overirde. when bit is set to logic '1' IOE_N (bit 4) value will control IO IOE_N signal else IOE_N is control via selected HW logic. strong PULL UP and PULL Down control is disabled for all IO's. both controls are tied to logic level '0'."]
    #[inline(always)]
    pub fn mem_gpio_pad_config_12(&mut self) -> MEM_GPIO_PAD_CONFIG_12_W {
        MEM_GPIO_PAD_CONFIG_12_W { w: self }
    }
}
