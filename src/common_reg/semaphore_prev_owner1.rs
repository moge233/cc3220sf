#[doc = "Reader of register SEMAPHORE_PREV_OWNER1"]
pub type R = crate::R<u32, super::SEMAPHORE_PREV_OWNER1>;
#[doc = "Writer for register SEMAPHORE_PREV_OWNER1"]
pub type W = crate::W<u32, super::SEMAPHORE_PREV_OWNER1>;
#[doc = "Register SEMAPHORE_PREV_OWNER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORE_PREV_OWNER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEMAPHORE_PREV_OWNER1`"]
pub type SEMAPHORE_PREV_OWNER1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEMAPHORE_PREV_OWNER1`"]
pub struct SEMAPHORE_PREV_OWNER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMAPHORE_PREV_OWNER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - 1:0 : prvious owner of i2c_properties_reg\\[1:0\\]
3:2 : prvious owner of spi_properties_reg\\[1:0\\]
5:4 : prvious owner of gpio_properties_reg\\[1:0\\]
9:8 : prvious owner of gpio_properties_reg\\[3:2\\]
11:10 : prvious owner of gpio_properties_reg\\[5:4\\]
13:12 : prvious owner of gpio_properties_reg\\[7:6\\]
15:14 : prvious owner of gpio_properties_reg\\[9:8\\]
17:16 : prvious owner of flash_control_reg\\[1:0\\]"]
    #[inline(always)]
    pub fn semaphore_prev_owner1(&self) -> SEMAPHORE_PREV_OWNER1_R {
        SEMAPHORE_PREV_OWNER1_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - 1:0 : prvious owner of i2c_properties_reg\\[1:0\\]
3:2 : prvious owner of spi_properties_reg\\[1:0\\]
5:4 : prvious owner of gpio_properties_reg\\[1:0\\]
9:8 : prvious owner of gpio_properties_reg\\[3:2\\]
11:10 : prvious owner of gpio_properties_reg\\[5:4\\]
13:12 : prvious owner of gpio_properties_reg\\[7:6\\]
15:14 : prvious owner of gpio_properties_reg\\[9:8\\]
17:16 : prvious owner of flash_control_reg\\[1:0\\]"]
    #[inline(always)]
    pub fn semaphore_prev_owner1(&mut self) -> SEMAPHORE_PREV_OWNER1_W {
        SEMAPHORE_PREV_OWNER1_W { w: self }
    }
}
