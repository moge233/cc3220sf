#[doc = "Reader of register MEM_TOPMUXCTRL_IFORCE"]
pub type R = crate::R<u32, super::MEM_TOPMUXCTRL_IFORCE>;
#[doc = "Writer for register MEM_TOPMUXCTRL_IFORCE"]
pub type W = crate::W<u32, super::MEM_TOPMUXCTRL_IFORCE>;
#[doc = "Register MEM_TOPMUXCTRL_IFORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_TOPMUXCTRL_IFORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_TOPMUXCTRL_IFORCE1`"]
pub type MEM_TOPMUXCTRL_IFORCE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_TOPMUXCTRL_IFORCE1`"]
pub struct MEM_TOPMUXCTRL_IFORCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TOPMUXCTRL_IFORCE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_TOPMUXCTRL_IFORCE`"]
pub type MEM_TOPMUXCTRL_IFORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_TOPMUXCTRL_IFORCE`"]
pub struct MEM_TOPMUXCTRL_IFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TOPMUXCTRL_IFORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - 4\\]
1: switch between WLAN_I2C_SCL and TOP_GPIO_PORT4_I2C closes 0: switch opens \\[5\\]
1: switch between WLAN_I2C_SCL and TOP_VSENSE_PORT closes 0: switch opens \\[6\\]
1: switch between WLAN_I2C_SCL and WLAN_ANA_TP4 closes 0: switch opens \\[7\\]
Reserved"]
    #[inline(always)]
    pub fn mem_topmuxctrl_iforce1(&self) -> MEM_TOPMUXCTRL_IFORCE1_R {
        MEM_TOPMUXCTRL_IFORCE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 0\\]
1: switch between WLAN_I2C_SDA and TOP_GPIO_PORT3_I2C closes 0: switch opens \\[1\\]
1: switch between WLAN_I2C_SDA and TOP_IFORCE_PORT closes 0: switch opens \\[2\\]
1: switch between WLAN_I2C_SDA and WLAN_ANA_TP3 closes 0: switch opens \\[3\\]
Reserved"]
    #[inline(always)]
    pub fn mem_topmuxctrl_iforce(&self) -> MEM_TOPMUXCTRL_IFORCE_R {
        MEM_TOPMUXCTRL_IFORCE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - 4\\]
1: switch between WLAN_I2C_SCL and TOP_GPIO_PORT4_I2C closes 0: switch opens \\[5\\]
1: switch between WLAN_I2C_SCL and TOP_VSENSE_PORT closes 0: switch opens \\[6\\]
1: switch between WLAN_I2C_SCL and WLAN_ANA_TP4 closes 0: switch opens \\[7\\]
Reserved"]
    #[inline(always)]
    pub fn mem_topmuxctrl_iforce1(&mut self) -> MEM_TOPMUXCTRL_IFORCE1_W {
        MEM_TOPMUXCTRL_IFORCE1_W { w: self }
    }
    #[doc = "Bits 0:3 - 0\\]
1: switch between WLAN_I2C_SDA and TOP_GPIO_PORT3_I2C closes 0: switch opens \\[1\\]
1: switch between WLAN_I2C_SDA and TOP_IFORCE_PORT closes 0: switch opens \\[2\\]
1: switch between WLAN_I2C_SDA and WLAN_ANA_TP3 closes 0: switch opens \\[3\\]
Reserved"]
    #[inline(always)]
    pub fn mem_topmuxctrl_iforce(&mut self) -> MEM_TOPMUXCTRL_IFORCE_W {
        MEM_TOPMUXCTRL_IFORCE_W { w: self }
    }
}
