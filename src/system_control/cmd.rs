#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INDX`"]
pub type INDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INDX`"]
pub struct INDX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CMD_TYPE`"]
pub type CMD_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_TYPE`"]
pub struct CMD_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `RSP_TYPE`"]
pub type RSP_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSP_TYPE`"]
pub struct RSP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Command index Binary encoded value from 0 to 63 specifying the command number send to card 0x00 CMD0 or ACMD0 0x01 CMD1 or ACMD1 0x02 CMD2 or ACMD2 0x03 CMD3 or ACMD3 0x04 CMD4 or ACMD4 0x05 CMD5 or ACMD5 0x06 CMD6 or ACMD6 0x07 CMD7 or ACMD7 0x08 CMD8 or ACMD8 0x09 CMD9 or ACMD9 0x0A CMD10 or ACMD10 0x0B CMD11 or ACMD11 0x0C CMD12 or ACMD12 0x0D CMD13 or ACMD13 0x0E CMD14 or ACMD14 0x0F CMD15 or ACMD15 0x10 CMD16 or ACMD16 0x11 CMD17 or ACMD17 0x12 CMD18 or ACMD18 0x13 CMD19 or ACMD19 0x14 CMD20 or ACMD20 0x15 CMD21 or ACMD21 0x16 CMD22 or ACMD22 0x17 CMD23 or ACMD23 0x18 CMD24 or ACMD24 0x19 CMD25 or ACMD25 0x1A CMD26 or ACMD26 0x1B CMD27 or ACMD27 0x1C CMD28 or ACMD28 0x1D CMD29 or ACMD29 0x1E CMD30 or ACMD30 0x1F CMD31 or ACMD31 0x20 CMD32 or ACMD32 0x21 CMD33 or ACMD33 0x22 CMD34 or ACMD34 0x23 CMD35 or ACMD35 0x24 CMD36 or ACMD36 0x25 CMD37 or ACMD37 0x26 CMD38 or ACMD38 0x27 CMD39 or ACMD39 0x28 CMD40 or ACMD40 0x29 CMD41 or ACMD41 0x2A CMD42 or ACMD42 0x2B CMD43 or ACMD43 0x2C CMD44 or ACMD44 0x2D CMD45 or ACMD45 0x2E CMD46 or ACMD46 0x2F CMD47 or ACMD47 0x30 CMD48 or ACMD48 0x31 CMD49 or ACMD49 0x32 CMD50 or ACMD50 0x33 CMD51 or ACMD51 0x34 CMD52 or ACMD52 0x35 CMD53 or ACMD53 0x36 CMD54 or ACMD54 0x37 CMD55 or ACMD55 0x38 CMD56 or ACMD56 0x39 CMD57 or ACMD57 0x3A CMD58 or ACMD58 0x3B CMD59 or ACMD59 0x3C CMD60 or ACMD60 0x3D CMD61 or ACMD61 0x3E CMD62 or ACMD62 0x3F CMD63 or ACMD63"]
    #[inline(always)]
    pub fn indx(&self) -> INDX_R {
        INDX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Command type This register specifies three types of special command: Suspend Resume and Abort. These bits shall be set to 00b for all other commands. 0x0 Others Commands 0x1 &quot;CMD52 for writing &quot;&quot;Bus Suspend&quot;&quot; in CCCR&quot; 0x2 &quot;CMD52 for writing &quot;&quot;Function Select&quot;&quot; in CCCR&quot; 0x3 &quot;Abort command CMD12 CMD52 for writing &quot;&quot; I/O Abort&quot;&quot; in CCCR&quot;"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Response type This bits defines the response type of the command 0x0 No response 0x1 Response Length 136 bits 0x2 Response Length 48 bits 0x3 Response Length 48 bits with busy after response"]
    #[inline(always)]
    pub fn rsp_type(&self) -> RSP_TYPE_R {
        RSP_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Command index Binary encoded value from 0 to 63 specifying the command number send to card 0x00 CMD0 or ACMD0 0x01 CMD1 or ACMD1 0x02 CMD2 or ACMD2 0x03 CMD3 or ACMD3 0x04 CMD4 or ACMD4 0x05 CMD5 or ACMD5 0x06 CMD6 or ACMD6 0x07 CMD7 or ACMD7 0x08 CMD8 or ACMD8 0x09 CMD9 or ACMD9 0x0A CMD10 or ACMD10 0x0B CMD11 or ACMD11 0x0C CMD12 or ACMD12 0x0D CMD13 or ACMD13 0x0E CMD14 or ACMD14 0x0F CMD15 or ACMD15 0x10 CMD16 or ACMD16 0x11 CMD17 or ACMD17 0x12 CMD18 or ACMD18 0x13 CMD19 or ACMD19 0x14 CMD20 or ACMD20 0x15 CMD21 or ACMD21 0x16 CMD22 or ACMD22 0x17 CMD23 or ACMD23 0x18 CMD24 or ACMD24 0x19 CMD25 or ACMD25 0x1A CMD26 or ACMD26 0x1B CMD27 or ACMD27 0x1C CMD28 or ACMD28 0x1D CMD29 or ACMD29 0x1E CMD30 or ACMD30 0x1F CMD31 or ACMD31 0x20 CMD32 or ACMD32 0x21 CMD33 or ACMD33 0x22 CMD34 or ACMD34 0x23 CMD35 or ACMD35 0x24 CMD36 or ACMD36 0x25 CMD37 or ACMD37 0x26 CMD38 or ACMD38 0x27 CMD39 or ACMD39 0x28 CMD40 or ACMD40 0x29 CMD41 or ACMD41 0x2A CMD42 or ACMD42 0x2B CMD43 or ACMD43 0x2C CMD44 or ACMD44 0x2D CMD45 or ACMD45 0x2E CMD46 or ACMD46 0x2F CMD47 or ACMD47 0x30 CMD48 or ACMD48 0x31 CMD49 or ACMD49 0x32 CMD50 or ACMD50 0x33 CMD51 or ACMD51 0x34 CMD52 or ACMD52 0x35 CMD53 or ACMD53 0x36 CMD54 or ACMD54 0x37 CMD55 or ACMD55 0x38 CMD56 or ACMD56 0x39 CMD57 or ACMD57 0x3A CMD58 or ACMD58 0x3B CMD59 or ACMD59 0x3C CMD60 or ACMD60 0x3D CMD61 or ACMD61 0x3E CMD62 or ACMD62 0x3F CMD63 or ACMD63"]
    #[inline(always)]
    pub fn indx(&mut self) -> INDX_W {
        INDX_W { w: self }
    }
    #[doc = "Bits 22:23 - Command type This register specifies three types of special command: Suspend Resume and Abort. These bits shall be set to 00b for all other commands. 0x0 Others Commands 0x1 &quot;CMD52 for writing &quot;&quot;Bus Suspend&quot;&quot; in CCCR&quot; 0x2 &quot;CMD52 for writing &quot;&quot;Function Select&quot;&quot; in CCCR&quot; 0x3 &quot;Abort command CMD12 CMD52 for writing &quot;&quot; I/O Abort&quot;&quot; in CCCR&quot;"]
    #[inline(always)]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W {
        CMD_TYPE_W { w: self }
    }
    #[doc = "Bits 16:17 - Response type This bits defines the response type of the command 0x0 No response 0x1 Response Length 136 bits 0x2 Response Length 48 bits 0x3 Response Length 48 bits with busy after response"]
    #[inline(always)]
    pub fn rsp_type(&mut self) -> RSP_TYPE_W {
        RSP_TYPE_W { w: self }
    }
}
