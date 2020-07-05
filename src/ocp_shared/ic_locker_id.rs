#[doc = "Reader of register IC_LOCKER_ID"]
pub type R = crate::R<u32, super::IC_LOCKER_ID>;
#[doc = "Writer for register IC_LOCKER_ID"]
pub type W = crate::W<u32, super::IC_LOCKER_ID>;
#[doc = "Register IC_LOCKER_ID `reset()`'s with value 0"]
impl crate::ResetValue for super::IC_LOCKER_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_IC_LOCKER_ID`"]
pub type MEM_IC_LOCKER_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_IC_LOCKER_ID`"]
pub struct MEM_IC_LOCKER_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_IC_LOCKER_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This register is used for allowing only one master OCP to perform write transactions to the OCP slaves. Each bit represents an IP in the following format: { JTAG,WLAN, NWP mcu}. As any of the bits is set to one, the correlating IP is preventing the other IP's from performing write transactions to the slaves. As the Inter Connect is locked, the only the locking IP can write to the register and by that releasing the lock. 3'b000 => IC is not locked. 3'b001 => IC is locked by NWP mcu. 3'b010 => IC is locked by WLAN. 3'b100 => IC is locked by JTAG."]
    #[inline(always)]
    pub fn mem_ic_locker_id(&self) -> MEM_IC_LOCKER_ID_R {
        MEM_IC_LOCKER_ID_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used for allowing only one master OCP to perform write transactions to the OCP slaves. Each bit represents an IP in the following format: { JTAG,WLAN, NWP mcu}. As any of the bits is set to one, the correlating IP is preventing the other IP's from performing write transactions to the slaves. As the Inter Connect is locked, the only the locking IP can write to the register and by that releasing the lock. 3'b000 => IC is not locked. 3'b001 => IC is locked by NWP mcu. 3'b010 => IC is locked by WLAN. 3'b100 => IC is locked by JTAG."]
    #[inline(always)]
    pub fn mem_ic_locker_id(&mut self) -> MEM_IC_LOCKER_ID_W {
        MEM_IC_LOCKER_ID_W { w: self }
    }
}
