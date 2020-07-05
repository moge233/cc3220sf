#[doc = "Reader of register BLK"]
pub type R = crate::R<u32, super::BLK>;
#[doc = "Writer for register BLK"]
pub type W = crate::W<u32, super::BLK>;
#[doc = "Register BLK `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBLK`"]
pub type NBLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NBLK`"]
pub struct NBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> NBLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BLEN`"]
pub type BLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLEN`"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Blocks count for current transfer This register is enabled when Block count Enable (MMCHS_CMD\\[BCE\\]) is set to 1 and is valid only for multiple block transfers. Setting the block count to 0 results no data blocks being transferred. Note: The host controller decrements the block count after each block transfer and stops when the count reaches zero. This register can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfers may return an invalid value and write operation will be ignored. In suspend context the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command The local host shall restore the previously saved block count. 0x0000 Stop count 0x0001 1 block 0x0002 2 blocks 0xFFFF 65535 blocks"]
    #[inline(always)]
    pub fn nblk(&self) -> NBLK_R {
        NBLK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:11 - Transfer Block Size. This register specifies the block size for block data transfers. Read operations during transfers may return an invalid value and write operations are ignored. When a CMD12 command is issued to stop the transfer a read of the BLEN field after transfer completion (MMCHS_STAT\\[TC\\]
set to 1) will not return the true byte number of data length while the stop occurs but the value written in this register before transfer is launched. 0x000 No data transfer 0x001 1 byte block length 0x002 2 bytes block length 0x003 3 bytes block length 0x1FF 511 bytes block length 0x200 512 bytes block length 0x7FF 2047 bytes block length 0x800 2048 bytes block length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Blocks count for current transfer This register is enabled when Block count Enable (MMCHS_CMD\\[BCE\\]) is set to 1 and is valid only for multiple block transfers. Setting the block count to 0 results no data blocks being transferred. Note: The host controller decrements the block count after each block transfer and stops when the count reaches zero. This register can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfers may return an invalid value and write operation will be ignored. In suspend context the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command The local host shall restore the previously saved block count. 0x0000 Stop count 0x0001 1 block 0x0002 2 blocks 0xFFFF 65535 blocks"]
    #[inline(always)]
    pub fn nblk(&mut self) -> NBLK_W {
        NBLK_W { w: self }
    }
    #[doc = "Bits 0:11 - Transfer Block Size. This register specifies the block size for block data transfers. Read operations during transfers may return an invalid value and write operations are ignored. When a CMD12 command is issued to stop the transfer a read of the BLEN field after transfer completion (MMCHS_STAT\\[TC\\]
set to 1) will not return the true byte number of data length while the stop occurs but the value written in this register before transfer is launched. 0x000 No data transfer 0x001 1 byte block length 0x002 2 bytes block length 0x003 3 bytes block length 0x1FF 511 bytes block length 0x200 512 bytes block length 0x7FF 2047 bytes block length 0x800 2048 bytes block length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
}
