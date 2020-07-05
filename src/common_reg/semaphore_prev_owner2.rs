#[doc = "Reader of register SEMAPHORE_PREV_OWNER2"]
pub type R = crate::R<u32, super::SEMAPHORE_PREV_OWNER2>;
#[doc = "Writer for register SEMAPHORE_PREV_OWNER2"]
pub type W = crate::W<u32, super::SEMAPHORE_PREV_OWNER2>;
#[doc = "Register SEMAPHORE_PREV_OWNER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORE_PREV_OWNER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEMAPHORE_PREV_OWNER2`"]
pub type SEMAPHORE_PREV_OWNER2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEMAPHORE_PREV_OWNER2`"]
pub struct SEMAPHORE_PREV_OWNER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMAPHORE_PREV_OWNER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - 1:0 : previous owner of apps_nw_semaphore1_reg\\[1:0\\]
3:2 : previous owner of apps_nw_semaphore2_reg\\[1:0\\]
5:4 : previous owner of apps_nw_semaphore3_reg\\[1:0\\]
7:6 : previous owner of apps_nw_semaphore4_reg\\[1:0\\]
9:8 : previous owner of apps_nw_semaphore5_reg\\[1:0\\]
11:10 : previous owner of apps_nw_semaphore6_reg\\[1:0\\]
13:12 : previous owner of apps_nw_semaphore7_reg\\[1:0\\]
15:14 : previous owner of apps_nw_semaphore8_reg\\[1:0\\]
17:16 : previous owner of apps_nw_semaphore9_reg\\[1:0\\]
19:18 : previous owner of apps_nw_semaphore10_reg\\[1:0\\]
21:20 : previous owner of apps_nw_semaphore11_reg\\[1:0\\]
23:22 : previous owner of apps_nw_semaphore12_reg\\[1:0\\]"]
    #[inline(always)]
    pub fn semaphore_prev_owner2(&self) -> SEMAPHORE_PREV_OWNER2_R {
        SEMAPHORE_PREV_OWNER2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - 1:0 : previous owner of apps_nw_semaphore1_reg\\[1:0\\]
3:2 : previous owner of apps_nw_semaphore2_reg\\[1:0\\]
5:4 : previous owner of apps_nw_semaphore3_reg\\[1:0\\]
7:6 : previous owner of apps_nw_semaphore4_reg\\[1:0\\]
9:8 : previous owner of apps_nw_semaphore5_reg\\[1:0\\]
11:10 : previous owner of apps_nw_semaphore6_reg\\[1:0\\]
13:12 : previous owner of apps_nw_semaphore7_reg\\[1:0\\]
15:14 : previous owner of apps_nw_semaphore8_reg\\[1:0\\]
17:16 : previous owner of apps_nw_semaphore9_reg\\[1:0\\]
19:18 : previous owner of apps_nw_semaphore10_reg\\[1:0\\]
21:20 : previous owner of apps_nw_semaphore11_reg\\[1:0\\]
23:22 : previous owner of apps_nw_semaphore12_reg\\[1:0\\]"]
    #[inline(always)]
    pub fn semaphore_prev_owner2(&mut self) -> SEMAPHORE_PREV_OWNER2_W {
        SEMAPHORE_PREV_OWNER2_W { w: self }
    }
}
