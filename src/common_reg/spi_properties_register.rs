#[doc = "Reader of register SPI_PROPERTIES_REGISTER"]
pub type R = crate::R<u32, super::SPI_PROPERTIES_REGISTER>;
#[doc = "Writer for register SPI_PROPERTIES_REGISTER"]
pub type W = crate::W<u32, super::SPI_PROPERTIES_REGISTER>;
#[doc = "Register SPI_PROPERTIES_REGISTER `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_PROPERTIES_REGISTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_SPI_Properties_Register_SPI_Properties_Register`"]
pub type COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON_REG_SPI_Properties_Register_SPI_Properties_Register`"]
pub struct COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Each semaphore register is of 2 bit. When this register is set to 2b01 Apps have access and when set to 2b10 NW have access. Ideally both the master can modify any of this 2 bit, but assumption apps will write only 2b01 or 2b00 to this register and nw will write only 2b10 or 2b00. Implementation is when any of the bit of this register is set, only next write allowedvis 2b00 Again assumption is one master will not write 2b00 if other is already holding the semaphore."]
    #[inline(always)]
    pub fn common_reg_spi_properties_register_spi_properties_register(
        &self,
    ) -> COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_R {
        COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Each semaphore register is of 2 bit. When this register is set to 2b01 Apps have access and when set to 2b10 NW have access. Ideally both the master can modify any of this 2 bit, but assumption apps will write only 2b01 or 2b00 to this register and nw will write only 2b10 or 2b00. Implementation is when any of the bit of this register is set, only next write allowedvis 2b00 Again assumption is one master will not write 2b00 if other is already holding the semaphore."]
    #[inline(always)]
    pub fn common_reg_spi_properties_register_spi_properties_register(
        &mut self,
    ) -> COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_W {
        COMMON_REG_SPI_PROPERTIES_REGISTER_SPI_PROPERTIES_REGISTER_W { w: self }
    }
}
