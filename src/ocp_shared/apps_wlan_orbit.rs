#[doc = "Reader of register APPS_WLAN_ORBIT"]
pub type R = crate::R<u32, super::APPS_WLAN_ORBIT>;
#[doc = "Writer for register APPS_WLAN_ORBIT"]
pub type W = crate::W<u32, super::APPS_WLAN_ORBIT>;
#[doc = "Register APPS_WLAN_ORBIT `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_WLAN_ORBIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_ORBIT_SPARE`"]
pub type MEM_ORBIT_SPARE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_ORBIT_SPARE`"]
pub struct MEM_ORBIT_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ORBIT_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_ORBIT_TEST_ID`"]
pub type MEM_ORBIT_TEST_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_ORBIT_TEST_ID`"]
pub struct MEM_ORBIT_TEST_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ORBIT_TEST_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Spare bit"]
    #[inline(always)]
    pub fn mem_orbit_spare(&self) -> MEM_ORBIT_SPARE_R {
        MEM_ORBIT_SPARE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 2:7 - Implies the test case ID that needs to run."]
    #[inline(always)]
    pub fn mem_orbit_test_id(&self) -> MEM_ORBIT_TEST_ID_R {
        MEM_ORBIT_TEST_ID_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - Spare bit"]
    #[inline(always)]
    pub fn mem_orbit_spare(&mut self) -> MEM_ORBIT_SPARE_W {
        MEM_ORBIT_SPARE_W { w: self }
    }
    #[doc = "Bits 2:7 - Implies the test case ID that needs to run."]
    #[inline(always)]
    pub fn mem_orbit_test_id(&mut self) -> MEM_ORBIT_TEST_ID_W {
        MEM_ORBIT_TEST_ID_W { w: self }
    }
}
