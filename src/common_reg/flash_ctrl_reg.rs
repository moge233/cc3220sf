#[doc = "Reader of register FLASH_CTRL_REG"]
pub type R = crate::R<u32, super::FLASH_CTRL_REG>;
#[doc = "Writer for register FLASH_CTRL_REG"]
pub type W = crate::W<u32, super::FLASH_CTRL_REG>;
#[doc = "Register FLASH_CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_Flash_ctrl_reg_Flash_ctrl_reg`"]
pub type COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON_REG_Flash_ctrl_reg_Flash_ctrl_reg`"]
pub struct COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_W<'a> {
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
    pub fn common_reg_flash_ctrl_reg_flash_ctrl_reg(
        &self,
    ) -> COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_R {
        COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Each semaphore register is of 2 bit. When this register is set to 2b01 Apps have access and when set to 2b10 NW have access. Ideally both the master can modify any of this 2 bit, but assumption apps will write only 2b01 or 2b00 to this register and nw will write only 2b10 or 2b00. Implementation is when any of the bit of this register is set, only next write allowedvis 2b00 Again assumption is one master will not write 2b00 if other is already holding the semaphore."]
    #[inline(always)]
    pub fn common_reg_flash_ctrl_reg_flash_ctrl_reg(
        &mut self,
    ) -> COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_W {
        COMMON_REG_FLASH_CTRL_REG_FLASH_CTRL_REG_W { w: self }
    }
}
