#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XTS second key / CBC-MAC third key"]
    pub key2_6: KEY2_6,
    #[doc = "0x04 - XTS second key (MSW for 256-bit key) / CBC-MAC third key (MSW)"]
    pub key2_7: KEY2_7,
    #[doc = "0x08 - XTS / CCM second key / CBC-MAC third key (LSW)"]
    pub key2_4: KEY2_4,
    #[doc = "0x0c - XTS second key (MSW for 192-bit key) / CBC-MAC third key"]
    pub key2_5: KEY2_5,
    #[doc = "0x10 - XTS / CCM / CBC-MAC second key / Hash Key input"]
    pub key2_2: KEY2_2,
    #[doc = "0x14 - XTS second key (MSW for 128-bit key) + CCM/CBC-MAC second key (MSW) / Hash Key input (MSW)"]
    pub key2_3: KEY2_3,
    #[doc = "0x18 - XTS / CCM / CBC-MAC second key (LSW) / Hash Key input (LSW)"]
    pub key2_0: KEY2_0,
    #[doc = "0x1c - XTS / CCM / CBC-MAC second key / Hash Key input"]
    pub key2_1: KEY2_1,
    #[doc = "0x20 - Key (LSW for 256-bit key)"]
    pub key1_6: KEY1_6,
    #[doc = "0x24 - Key (MSW for 256-bit key)"]
    pub key1_7: KEY1_7,
    #[doc = "0x28 - Key (LSW for 192-bit key)"]
    pub key1_4: KEY1_4,
    #[doc = "0x2c - Key (MSW for 192-bit key)"]
    pub key1_5: KEY1_5,
    #[doc = "0x30 - Key"]
    pub key1_2: KEY1_2,
    #[doc = "0x34 - Key (MSW for 128-bit key)"]
    pub key1_3: KEY1_3,
    #[doc = "0x38 - Key (LSW for 128-bit key)"]
    pub key1_0: KEY1_0,
    #[doc = "0x3c - Key"]
    pub key1_1: KEY1_1,
    #[doc = "0x40 - Initialization Vector input (LSW)"]
    pub iv_in_0: IV_IN_0,
    #[doc = "0x44 - Initialization vector input"]
    pub iv_in_1: IV_IN_1,
    #[doc = "0x48 - Initialization vector input"]
    pub iv_in_2: IV_IN_2,
    #[doc = "0x4c - Initialization Vector input (MSW)"]
    pub iv_in_3: IV_IN_3,
    #[doc = "0x50 - register determines the mode of operation of the AES Engine"]
    pub ctrl: CTRL,
    #[doc = "0x54 - Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
    pub c_length_0: C_LENGTH_0,
    #[doc = "0x58 - Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
    pub c_length_1: C_LENGTH_1,
    #[doc = "0x5c - AAD data length. The authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM) Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started@@ this length decrements to zero. A write to this register triggers the engine to start using this context for GCM and CCM. For XTS this register is optionally used to load j. Loading of j is only required if j != 0. j is a 28-bit value and must be written to bits \\[31-4\\]
of this register. j represents the sequential number of the 128-bit block inside the data unit. For the first block in a unit@@ this value is zero. It is not required to provide a j for each new data block within a unit. Note that it is possible to start with a j unequal to zero; refer to Table 4 for more details. For a Host read operation@@ these registers return all-zeroes."]
    pub auth_length: AUTH_LENGTH,
    #[doc = "0x60 - Data register to read and write plaintext/ciphertext (MSW)"]
    pub data_in_0: DATA_IN_0,
    #[doc = "0x64 - Data register to read and write plaintext/ciphertext"]
    pub data_in_1: DATA_IN_1,
    #[doc = "0x68 - Data register to read and write plaintext/ciphertext"]
    pub data_in_2: DATA_IN_2,
    #[doc = "0x6c - Data register to read and write plaintext/ciphertext (LSW)"]
    pub data_in_3: DATA_IN_3,
    #[doc = "0x70 - TAG_OUT_0"]
    pub tag_out_0: TAG_OUT_0,
    #[doc = "0x74 - TAG_OUT_1"]
    pub tag_out_1: TAG_OUT_1,
    #[doc = "0x78 - TAG_OUT_2"]
    pub tag_out_2: TAG_OUT_2,
    #[doc = "0x7c - TAG_OUT_3"]
    pub tag_out_3: TAG_OUT_3,
    #[doc = "0x80 - Register AES_REVISION"]
    pub revision: REVISION,
    #[doc = "0x84 - Register AES_SYSCONFIG.This register configures the DMA signals and controls the IDLE and reset logic"]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x88 - SYSSTATUS"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x8c - This register indicates the interrupt status. If one of the interrupt bits is set the interrupt output will be asserted"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x90 - This register contains an enable bit for each unique interrupt generated by the module. It matches the layout of AES_IRQSTATUS register. An interrupt is enabled when the bit in this register is set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_x output. All interrupts need to be enabled explicitly by writing this register. ****************************************************************************"]
    pub irqenable: IRQENABLE,
}
#[doc = "XTS second key / CBC-MAC third key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_6](key2_6) module"]
pub type KEY2_6 = crate::Reg<u32, _KEY2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_6;
#[doc = "`read()` method returns [key2_6::R](key2_6::R) reader structure"]
impl crate::Readable for KEY2_6 {}
#[doc = "`write(|w| ..)` method takes [key2_6::W](key2_6::W) writer structure"]
impl crate::Writable for KEY2_6 {}
#[doc = "XTS second key / CBC-MAC third key"]
pub mod key2_6;
#[doc = "XTS second key (MSW for 256-bit key) / CBC-MAC third key (MSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_7](key2_7) module"]
pub type KEY2_7 = crate::Reg<u32, _KEY2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_7;
#[doc = "`read()` method returns [key2_7::R](key2_7::R) reader structure"]
impl crate::Readable for KEY2_7 {}
#[doc = "`write(|w| ..)` method takes [key2_7::W](key2_7::W) writer structure"]
impl crate::Writable for KEY2_7 {}
#[doc = "XTS second key (MSW for 256-bit key) / CBC-MAC third key (MSW)"]
pub mod key2_7;
#[doc = "XTS / CCM second key / CBC-MAC third key (LSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_4](key2_4) module"]
pub type KEY2_4 = crate::Reg<u32, _KEY2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_4;
#[doc = "`read()` method returns [key2_4::R](key2_4::R) reader structure"]
impl crate::Readable for KEY2_4 {}
#[doc = "`write(|w| ..)` method takes [key2_4::W](key2_4::W) writer structure"]
impl crate::Writable for KEY2_4 {}
#[doc = "XTS / CCM second key / CBC-MAC third key (LSW)"]
pub mod key2_4;
#[doc = "XTS second key (MSW for 192-bit key) / CBC-MAC third key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_5](key2_5) module"]
pub type KEY2_5 = crate::Reg<u32, _KEY2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_5;
#[doc = "`read()` method returns [key2_5::R](key2_5::R) reader structure"]
impl crate::Readable for KEY2_5 {}
#[doc = "`write(|w| ..)` method takes [key2_5::W](key2_5::W) writer structure"]
impl crate::Writable for KEY2_5 {}
#[doc = "XTS second key (MSW for 192-bit key) / CBC-MAC third key"]
pub mod key2_5;
#[doc = "XTS / CCM / CBC-MAC second key / Hash Key input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_2](key2_2) module"]
pub type KEY2_2 = crate::Reg<u32, _KEY2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_2;
#[doc = "`read()` method returns [key2_2::R](key2_2::R) reader structure"]
impl crate::Readable for KEY2_2 {}
#[doc = "`write(|w| ..)` method takes [key2_2::W](key2_2::W) writer structure"]
impl crate::Writable for KEY2_2 {}
#[doc = "XTS / CCM / CBC-MAC second key / Hash Key input"]
pub mod key2_2;
#[doc = "XTS second key (MSW for 128-bit key) + CCM/CBC-MAC second key (MSW) / Hash Key input (MSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_3](key2_3) module"]
pub type KEY2_3 = crate::Reg<u32, _KEY2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_3;
#[doc = "`read()` method returns [key2_3::R](key2_3::R) reader structure"]
impl crate::Readable for KEY2_3 {}
#[doc = "`write(|w| ..)` method takes [key2_3::W](key2_3::W) writer structure"]
impl crate::Writable for KEY2_3 {}
#[doc = "XTS second key (MSW for 128-bit key) + CCM/CBC-MAC second key (MSW) / Hash Key input (MSW)"]
pub mod key2_3;
#[doc = "XTS / CCM / CBC-MAC second key (LSW) / Hash Key input (LSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_0](key2_0) module"]
pub type KEY2_0 = crate::Reg<u32, _KEY2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_0;
#[doc = "`read()` method returns [key2_0::R](key2_0::R) reader structure"]
impl crate::Readable for KEY2_0 {}
#[doc = "`write(|w| ..)` method takes [key2_0::W](key2_0::W) writer structure"]
impl crate::Writable for KEY2_0 {}
#[doc = "XTS / CCM / CBC-MAC second key (LSW) / Hash Key input (LSW)"]
pub mod key2_0;
#[doc = "XTS / CCM / CBC-MAC second key / Hash Key input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key2_1](key2_1) module"]
pub type KEY2_1 = crate::Reg<u32, _KEY2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2_1;
#[doc = "`read()` method returns [key2_1::R](key2_1::R) reader structure"]
impl crate::Readable for KEY2_1 {}
#[doc = "`write(|w| ..)` method takes [key2_1::W](key2_1::W) writer structure"]
impl crate::Writable for KEY2_1 {}
#[doc = "XTS / CCM / CBC-MAC second key / Hash Key input"]
pub mod key2_1;
#[doc = "Key (LSW for 256-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_6](key1_6) module"]
pub type KEY1_6 = crate::Reg<u32, _KEY1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_6;
#[doc = "`read()` method returns [key1_6::R](key1_6::R) reader structure"]
impl crate::Readable for KEY1_6 {}
#[doc = "`write(|w| ..)` method takes [key1_6::W](key1_6::W) writer structure"]
impl crate::Writable for KEY1_6 {}
#[doc = "Key (LSW for 256-bit key)"]
pub mod key1_6;
#[doc = "Key (MSW for 256-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_7](key1_7) module"]
pub type KEY1_7 = crate::Reg<u32, _KEY1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_7;
#[doc = "`read()` method returns [key1_7::R](key1_7::R) reader structure"]
impl crate::Readable for KEY1_7 {}
#[doc = "`write(|w| ..)` method takes [key1_7::W](key1_7::W) writer structure"]
impl crate::Writable for KEY1_7 {}
#[doc = "Key (MSW for 256-bit key)"]
pub mod key1_7;
#[doc = "Key (LSW for 192-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_4](key1_4) module"]
pub type KEY1_4 = crate::Reg<u32, _KEY1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_4;
#[doc = "`read()` method returns [key1_4::R](key1_4::R) reader structure"]
impl crate::Readable for KEY1_4 {}
#[doc = "`write(|w| ..)` method takes [key1_4::W](key1_4::W) writer structure"]
impl crate::Writable for KEY1_4 {}
#[doc = "Key (LSW for 192-bit key)"]
pub mod key1_4;
#[doc = "Key (MSW for 192-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_5](key1_5) module"]
pub type KEY1_5 = crate::Reg<u32, _KEY1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_5;
#[doc = "`read()` method returns [key1_5::R](key1_5::R) reader structure"]
impl crate::Readable for KEY1_5 {}
#[doc = "`write(|w| ..)` method takes [key1_5::W](key1_5::W) writer structure"]
impl crate::Writable for KEY1_5 {}
#[doc = "Key (MSW for 192-bit key)"]
pub mod key1_5;
#[doc = "Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_2](key1_2) module"]
pub type KEY1_2 = crate::Reg<u32, _KEY1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_2;
#[doc = "`read()` method returns [key1_2::R](key1_2::R) reader structure"]
impl crate::Readable for KEY1_2 {}
#[doc = "`write(|w| ..)` method takes [key1_2::W](key1_2::W) writer structure"]
impl crate::Writable for KEY1_2 {}
#[doc = "Key"]
pub mod key1_2;
#[doc = "Key (MSW for 128-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_3](key1_3) module"]
pub type KEY1_3 = crate::Reg<u32, _KEY1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_3;
#[doc = "`read()` method returns [key1_3::R](key1_3::R) reader structure"]
impl crate::Readable for KEY1_3 {}
#[doc = "`write(|w| ..)` method takes [key1_3::W](key1_3::W) writer structure"]
impl crate::Writable for KEY1_3 {}
#[doc = "Key (MSW for 128-bit key)"]
pub mod key1_3;
#[doc = "Key (LSW for 128-bit key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_0](key1_0) module"]
pub type KEY1_0 = crate::Reg<u32, _KEY1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_0;
#[doc = "`read()` method returns [key1_0::R](key1_0::R) reader structure"]
impl crate::Readable for KEY1_0 {}
#[doc = "`write(|w| ..)` method takes [key1_0::W](key1_0::W) writer structure"]
impl crate::Writable for KEY1_0 {}
#[doc = "Key (LSW for 128-bit key)"]
pub mod key1_0;
#[doc = "Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_1](key1_1) module"]
pub type KEY1_1 = crate::Reg<u32, _KEY1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1_1;
#[doc = "`read()` method returns [key1_1::R](key1_1::R) reader structure"]
impl crate::Readable for KEY1_1 {}
#[doc = "`write(|w| ..)` method takes [key1_1::W](key1_1::W) writer structure"]
impl crate::Writable for KEY1_1 {}
#[doc = "Key"]
pub mod key1_1;
#[doc = "Initialization Vector input (LSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_in_0](iv_in_0) module"]
pub type IV_IN_0 = crate::Reg<u32, _IV_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_IN_0;
#[doc = "`read()` method returns [iv_in_0::R](iv_in_0::R) reader structure"]
impl crate::Readable for IV_IN_0 {}
#[doc = "`write(|w| ..)` method takes [iv_in_0::W](iv_in_0::W) writer structure"]
impl crate::Writable for IV_IN_0 {}
#[doc = "Initialization Vector input (LSW)"]
pub mod iv_in_0;
#[doc = "Initialization vector input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_in_1](iv_in_1) module"]
pub type IV_IN_1 = crate::Reg<u32, _IV_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_IN_1;
#[doc = "`read()` method returns [iv_in_1::R](iv_in_1::R) reader structure"]
impl crate::Readable for IV_IN_1 {}
#[doc = "`write(|w| ..)` method takes [iv_in_1::W](iv_in_1::W) writer structure"]
impl crate::Writable for IV_IN_1 {}
#[doc = "Initialization vector input"]
pub mod iv_in_1;
#[doc = "Initialization vector input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_in_2](iv_in_2) module"]
pub type IV_IN_2 = crate::Reg<u32, _IV_IN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_IN_2;
#[doc = "`read()` method returns [iv_in_2::R](iv_in_2::R) reader structure"]
impl crate::Readable for IV_IN_2 {}
#[doc = "`write(|w| ..)` method takes [iv_in_2::W](iv_in_2::W) writer structure"]
impl crate::Writable for IV_IN_2 {}
#[doc = "Initialization vector input"]
pub mod iv_in_2;
#[doc = "Initialization Vector input (MSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_in_3](iv_in_3) module"]
pub type IV_IN_3 = crate::Reg<u32, _IV_IN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_IN_3;
#[doc = "`read()` method returns [iv_in_3::R](iv_in_3::R) reader structure"]
impl crate::Readable for IV_IN_3 {}
#[doc = "`write(|w| ..)` method takes [iv_in_3::W](iv_in_3::W) writer structure"]
impl crate::Writable for IV_IN_3 {}
#[doc = "Initialization Vector input (MSW)"]
pub mod iv_in_3;
#[doc = "register determines the mode of operation of the AES Engine\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "register determines the mode of operation of the AES Engine"]
pub mod ctrl;
#[doc = "Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_length_0](c_length_0) module"]
pub type C_LENGTH_0 = crate::Reg<u32, _C_LENGTH_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C_LENGTH_0;
#[doc = "`read()` method returns [c_length_0::R](c_length_0::R) reader structure"]
impl crate::Readable for C_LENGTH_0 {}
#[doc = "`write(|w| ..)` method takes [c_length_0::W](c_length_0::W) writer structure"]
impl crate::Writable for C_LENGTH_0 {}
#[doc = "Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
pub mod c_length_0;
#[doc = "Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_length_1](c_length_1) module"]
pub type C_LENGTH_1 = crate::Reg<u32, _C_LENGTH_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C_LENGTH_1;
#[doc = "`read()` method returns [c_length_1::R](c_length_1::R) reader structure"]
impl crate::Readable for C_LENGTH_1 {}
#[doc = "`write(|w| ..)` method takes [c_length_1::W](c_length_1::W) writer structure"]
impl crate::Writable for C_LENGTH_1 {}
#[doc = "Crypto data length registers (LSW and MSW) store the cryptographic data length in bytes for all modes. Once processing with this context is started@@ this length decrements to zero. Data lengths up to (2^61 1) bytes are allowed. For GCM@@ any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 2@@ resulting in a maximum number of bytes of 2^36 - 32. A write to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note that for the combined modes@@ this length does not include the authentication only data; the authentication length is specified in the AES_AUTH_LENGTH register below. All modes must have a length > 0. For the combined modes@@ it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR/ICM/CFB128) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned; bit aligned data streams are not supported by the AES Engine. For a Host read operation@@ these registers return all-zeroes."]
pub mod c_length_1;
#[doc = "AAD data length. The authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM) Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started@@ this length decrements to zero. A write to this register triggers the engine to start using this context for GCM and CCM. For XTS this register is optionally used to load j. Loading of j is only required if j != 0. j is a 28-bit value and must be written to bits \\[31-4\\]
of this register. j represents the sequential number of the 128-bit block inside the data unit. For the first block in a unit@@ this value is zero. It is not required to provide a j for each new data block within a unit. Note that it is possible to start with a j unequal to zero; refer to Table 4 for more details. For a Host read operation@@ these registers return all-zeroes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auth_length](auth_length) module"]
pub type AUTH_LENGTH = crate::Reg<u32, _AUTH_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTH_LENGTH;
#[doc = "`read()` method returns [auth_length::R](auth_length::R) reader structure"]
impl crate::Readable for AUTH_LENGTH {}
#[doc = "`write(|w| ..)` method takes [auth_length::W](auth_length::W) writer structure"]
impl crate::Writable for AUTH_LENGTH {}
#[doc = "AAD data length. The authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM) Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started@@ this length decrements to zero. A write to this register triggers the engine to start using this context for GCM and CCM. For XTS this register is optionally used to load j. Loading of j is only required if j != 0. j is a 28-bit value and must be written to bits \\[31-4\\]
of this register. j represents the sequential number of the 128-bit block inside the data unit. For the first block in a unit@@ this value is zero. It is not required to provide a j for each new data block within a unit. Note that it is possible to start with a j unequal to zero; refer to Table 4 for more details. For a Host read operation@@ these registers return all-zeroes."]
pub mod auth_length;
#[doc = "Data register to read and write plaintext/ciphertext (MSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in_0](data_in_0) module"]
pub type DATA_IN_0 = crate::Reg<u32, _DATA_IN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN_0;
#[doc = "`read()` method returns [data_in_0::R](data_in_0::R) reader structure"]
impl crate::Readable for DATA_IN_0 {}
#[doc = "`write(|w| ..)` method takes [data_in_0::W](data_in_0::W) writer structure"]
impl crate::Writable for DATA_IN_0 {}
#[doc = "Data register to read and write plaintext/ciphertext (MSW)"]
pub mod data_in_0;
#[doc = "Data register to read and write plaintext/ciphertext\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in_1](data_in_1) module"]
pub type DATA_IN_1 = crate::Reg<u32, _DATA_IN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN_1;
#[doc = "`read()` method returns [data_in_1::R](data_in_1::R) reader structure"]
impl crate::Readable for DATA_IN_1 {}
#[doc = "`write(|w| ..)` method takes [data_in_1::W](data_in_1::W) writer structure"]
impl crate::Writable for DATA_IN_1 {}
#[doc = "Data register to read and write plaintext/ciphertext"]
pub mod data_in_1;
#[doc = "Data register to read and write plaintext/ciphertext\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in_2](data_in_2) module"]
pub type DATA_IN_2 = crate::Reg<u32, _DATA_IN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN_2;
#[doc = "`read()` method returns [data_in_2::R](data_in_2::R) reader structure"]
impl crate::Readable for DATA_IN_2 {}
#[doc = "`write(|w| ..)` method takes [data_in_2::W](data_in_2::W) writer structure"]
impl crate::Writable for DATA_IN_2 {}
#[doc = "Data register to read and write plaintext/ciphertext"]
pub mod data_in_2;
#[doc = "Data register to read and write plaintext/ciphertext (LSW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in_3](data_in_3) module"]
pub type DATA_IN_3 = crate::Reg<u32, _DATA_IN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN_3;
#[doc = "`read()` method returns [data_in_3::R](data_in_3::R) reader structure"]
impl crate::Readable for DATA_IN_3 {}
#[doc = "`write(|w| ..)` method takes [data_in_3::W](data_in_3::W) writer structure"]
impl crate::Writable for DATA_IN_3 {}
#[doc = "Data register to read and write plaintext/ciphertext (LSW)"]
pub mod data_in_3;
#[doc = "TAG_OUT_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_out_0](tag_out_0) module"]
pub type TAG_OUT_0 = crate::Reg<u32, _TAG_OUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_OUT_0;
#[doc = "`read()` method returns [tag_out_0::R](tag_out_0::R) reader structure"]
impl crate::Readable for TAG_OUT_0 {}
#[doc = "`write(|w| ..)` method takes [tag_out_0::W](tag_out_0::W) writer structure"]
impl crate::Writable for TAG_OUT_0 {}
#[doc = "TAG_OUT_0"]
pub mod tag_out_0;
#[doc = "TAG_OUT_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_out_1](tag_out_1) module"]
pub type TAG_OUT_1 = crate::Reg<u32, _TAG_OUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_OUT_1;
#[doc = "`read()` method returns [tag_out_1::R](tag_out_1::R) reader structure"]
impl crate::Readable for TAG_OUT_1 {}
#[doc = "`write(|w| ..)` method takes [tag_out_1::W](tag_out_1::W) writer structure"]
impl crate::Writable for TAG_OUT_1 {}
#[doc = "TAG_OUT_1"]
pub mod tag_out_1;
#[doc = "TAG_OUT_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_out_2](tag_out_2) module"]
pub type TAG_OUT_2 = crate::Reg<u32, _TAG_OUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_OUT_2;
#[doc = "`read()` method returns [tag_out_2::R](tag_out_2::R) reader structure"]
impl crate::Readable for TAG_OUT_2 {}
#[doc = "`write(|w| ..)` method takes [tag_out_2::W](tag_out_2::W) writer structure"]
impl crate::Writable for TAG_OUT_2 {}
#[doc = "TAG_OUT_2"]
pub mod tag_out_2;
#[doc = "TAG_OUT_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_out_3](tag_out_3) module"]
pub type TAG_OUT_3 = crate::Reg<u32, _TAG_OUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_OUT_3;
#[doc = "`read()` method returns [tag_out_3::R](tag_out_3::R) reader structure"]
impl crate::Readable for TAG_OUT_3 {}
#[doc = "`write(|w| ..)` method takes [tag_out_3::W](tag_out_3::W) writer structure"]
impl crate::Writable for TAG_OUT_3 {}
#[doc = "TAG_OUT_3"]
pub mod tag_out_3;
#[doc = "Register AES_REVISION\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "`write(|w| ..)` method takes [revision::W](revision::W) writer structure"]
impl crate::Writable for REVISION {}
#[doc = "Register AES_REVISION"]
pub mod revision;
#[doc = "Register AES_SYSCONFIG.This register configures the DMA signals and controls the IDLE and reset logic\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](sysconfig) module"]
pub type SYSCONFIG = crate::Reg<u32, _SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCONFIG;
#[doc = "`read()` method returns [sysconfig::R](sysconfig::R) reader structure"]
impl crate::Readable for SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](sysconfig::W) writer structure"]
impl crate::Writable for SYSCONFIG {}
#[doc = "Register AES_SYSCONFIG.This register configures the DMA signals and controls the IDLE and reset logic"]
pub mod sysconfig;
#[doc = "SYSSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysstatus](sysstatus) module"]
pub type SYSSTATUS = crate::Reg<u32, _SYSSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSTATUS;
#[doc = "`read()` method returns [sysstatus::R](sysstatus::R) reader structure"]
impl crate::Readable for SYSSTATUS {}
#[doc = "`write(|w| ..)` method takes [sysstatus::W](sysstatus::W) writer structure"]
impl crate::Writable for SYSSTATUS {}
#[doc = "SYSSTATUS"]
pub mod sysstatus;
#[doc = "This register indicates the interrupt status. If one of the interrupt bits is set the interrupt output will be asserted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](irqstatus) module"]
pub type IRQSTATUS = crate::Reg<u32, _IRQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATUS;
#[doc = "`read()` method returns [irqstatus::R](irqstatus::R) reader structure"]
impl crate::Readable for IRQSTATUS {}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](irqstatus::W) writer structure"]
impl crate::Writable for IRQSTATUS {}
#[doc = "This register indicates the interrupt status. If one of the interrupt bits is set the interrupt output will be asserted"]
pub mod irqstatus;
#[doc = "This register contains an enable bit for each unique interrupt generated by the module. It matches the layout of AES_IRQSTATUS register. An interrupt is enabled when the bit in this register is set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_x output. All interrupts need to be enabled explicitly by writing this register. ****************************************************************************\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](irqenable) module"]
pub type IRQENABLE = crate::Reg<u32, _IRQENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQENABLE;
#[doc = "`read()` method returns [irqenable::R](irqenable::R) reader structure"]
impl crate::Readable for IRQENABLE {}
#[doc = "`write(|w| ..)` method takes [irqenable::W](irqenable::W) writer structure"]
impl crate::Writable for IRQENABLE {}
#[doc = "This register contains an enable bit for each unique interrupt generated by the module. It matches the layout of AES_IRQSTATUS register. An interrupt is enabled when the bit in this register is set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_x output. All interrupts need to be enabled explicitly by writing this register. ****************************************************************************"]
pub mod irqenable;
