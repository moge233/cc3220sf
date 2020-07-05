#[doc = "Reader of register BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG"]
pub type R = crate::R<u32, super::BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG>;
#[doc = "Writer for register BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG"]
pub type W = crate::W<u32, super::BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG>;
#[doc = "Register BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_Bus_matrix_M3_segment_access_config_Bus_matrix_M3_segment_access_config`"]
pub type COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_R =
    crate::R<u32, u32>;
#[doc = "Write proxy for field `COMMON_REG_Bus_matrix_M3_segment_access_config_Bus_matrix_M3_segment_access_config`"]
pub struct COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_W<'a>
{
    w: &'a mut W,
}
impl<'a> COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Master 3 control word matrix to each segment. Tieoff. Bit value 1 indicates segment is accesable."]
    #[inline(always)]
    pub fn common_reg_bus_matrix_m3_segment_access_config_bus_matrix_m3_segment_access_config(
        &self,
    ) -> COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_R {
        COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_R::new(
            (self.bits & 0x0003_ffff) as u32,
        )
    }
}
impl W {
    #[doc = "Bits 0:17 - Master 3 control word matrix to each segment. Tieoff. Bit value 1 indicates segment is accesable."]
    #[inline(always)]
    pub fn common_reg_bus_matrix_m3_segment_access_config_bus_matrix_m3_segment_access_config(
        &mut self,
    ) -> COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_W {
        COMMON_REG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_BUS_MATRIX_M3_SEGMENT_ACCESS_CONFIG_W {
            w: self,
        }
    }
}
