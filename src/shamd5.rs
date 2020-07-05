#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WRITE: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[31:0\\]
for HMAC key proc READ: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2"]
    pub odigest_a: ODIGEST_A,
    #[doc = "0x04 - WRITE: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[63:32\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2"]
    pub odigest_b: ODIGEST_B,
    #[doc = "0x08 - WRITE: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / HMAC Key \\[95:64\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2"]
    pub odigest_c: ODIGEST_C,
    #[doc = "0x0c - WRITE: Outer Digest \\[31:0\\]
for MD5 \\[63:31\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[127:96\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2"]
    pub odigest_d: ODIGEST_D,
    #[doc = "0x10 - WRITE: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[159:128\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2"]
    pub odigest_e: ODIGEST_E,
    #[doc = "0x14 - WRITE: Outer Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[191:160\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for SHA-2"]
    pub odigest_f: ODIGEST_F,
    #[doc = "0x18 - WRITE: Outer Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[223:192\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for SHA-2"]
    pub odigest_g: ODIGEST_G,
    #[doc = "0x1c - WRITE: Outer Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[255:224\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-2"]
    pub odigest_h: ODIGEST_H,
    #[doc = "0x20 - WRITE: Inner / Initial Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[287:256\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / Result Digest/MAC \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[223:192\\]
for SHA-2 224 \\[255:224\\]
for SHA-2 256"]
    pub idigest_a: IDIGEST_A,
    #[doc = "0x24 - WRITE: Inner / Initial Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[319:288\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / Result Digest/MAC \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[191:160\\]
for SHA-2 224 \\[223:192\\]
for SHA-2 256"]
    pub idigest_b: IDIGEST_B,
    #[doc = "0x28 - WRITE: Inner / Initial Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA- 2 / HMAC Key \\[351:320\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[159:128\\]
for SHA-2 224 \\[191:160\\]
for SHA-2 256"]
    pub idigest_c: IDIGEST_C,
    #[doc = "0x2c - WRITE: Inner / Initial Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[383:352\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[127:96\\]
for SHA-2 224 \\[159:128\\]
for SHA-2 256"]
    pub idigest_d: IDIGEST_D,
    #[doc = "0x30 - WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[415:384\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-1 \\[95:64\\]
for SHA-2 224 \\[127:96\\]
for SHA-2 256"]
    pub idigest_e: IDIGEST_E,
    #[doc = "0x34 - WRITE: Inner / Initial Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[447:416\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for SHA-2 224 \\[95:64\\]
for SHA-2 256"]
    pub idigest_f: IDIGEST_F,
    #[doc = "0x38 - WRITE: Inner / Initial Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[479:448\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 224 \\[63:32\\]
for SHA-2 256"]
    pub idigest_g: IDIGEST_G,
    #[doc = "0x3c - WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[511:480\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 256"]
    pub idigest_h: IDIGEST_H,
    #[doc = "0x40 - WRITE: Initial Digest Count (\\[31:6\\]
only \\[5:0\\]
assumed 0) READ: Result / IntermediateDigest Count The initial digest byte count for hash/HMAC continue operations (HMAC Key Processing = 0 and Use Algorithm Constants = 0) on the Secure World must be written to this register prior to starting the operation by writing to S_HASH_MODE. When either HMAC Key Processing is 1 or Use Algorithm Constants is 1 this register does not need to be written it will be overwritten with 64 (1 hash block of key XOR ipad) or 0 respectively automatically. When starting a HMAC operation from pre-computes (HMAC Key Processing is 0) then the value 64 must be written here to compensate for the appended key XOR ipad block. Note that the value written should always be a 64 byte multiple the lower 6 bits written are ignored. The updated digest byte count (initial digest byte count + bytes processed) can be read from this register when the status register indicates that the operation is done or suspended due to a context switch request or when a Secure World context out DMA is requested. In Advanced DMA mode when not suspended with a partial result reading the SHAMD5_DIGEST_COUNT register triggers the Hash/HMAC Engine to start the next context input DMA. Therefore reading the SHAMD5_DIGEST_COUNT register should always be the last context-read action if not suspended with a partial result (i.e. PartHashReady interrupt not pending)."]
    pub digest_count: DIGEST_COUNT,
    #[doc = "0x44 - Register SHAMD5_MODE"]
    pub mode: MODE,
    #[doc = "0x48 - WRITE: Block Length / Remaining Byte Count (bytes) READ: Remaining Byte Count. The value programmed MUST be a 64-byte multiple if Close Hash is set to 0. This register is also the trigger to start processing: once this register is written the core will commence requesting input data via DMA or IRQ (if programmed length > 0) and start processing. The remaining byte count for the active operation can be read from this register when the interrupt status register indicates that the operation is suspended due to a context switch request."]
    pub length: LENGTH,
    _reserved19: [u8; 52usize],
    #[doc = "0x80 - Data input message 0"]
    pub data0_in: DATA0_IN,
    #[doc = "0x84 - Data input message 1"]
    pub data1_in: DATA1_IN,
    #[doc = "0x88 - Data input message 2"]
    pub data2_in: DATA2_IN,
    #[doc = "0x8c - Data input message 3"]
    pub data3_in: DATA3_IN,
    #[doc = "0x90 - Data input message 4"]
    pub data4_in: DATA4_IN,
    #[doc = "0x94 - Data input message 5"]
    pub data5_in: DATA5_IN,
    #[doc = "0x98 - Data input message 6"]
    pub data6_in: DATA6_IN,
    #[doc = "0x9c - Data input message 7"]
    pub data7_in: DATA7_IN,
    #[doc = "0xa0 - Data input message 8"]
    pub data8_in: DATA8_IN,
    #[doc = "0xa4 - Data input message 9"]
    pub data9_in: DATA9_IN,
    #[doc = "0xa8 - Data input message 10"]
    pub data10_in: DATA10_IN,
    #[doc = "0xac - Data input message 11"]
    pub data11_in: DATA11_IN,
    #[doc = "0xb0 - Data input message 12"]
    pub data12_in: DATA12_IN,
    #[doc = "0xb4 - Data input message 13"]
    pub data13_in: DATA13_IN,
    #[doc = "0xb8 - Data input message 14"]
    pub data14_in: DATA14_IN,
    #[doc = "0xbc - Data input message 15"]
    pub data15_in: DATA15_IN,
    _reserved35: [u8; 64usize],
    #[doc = "0x100 - Register SHAMD5_REV"]
    pub revision: REVISION,
    _reserved36: [u8; 12usize],
    #[doc = "0x110 - Register SHAMD5_SYSCONFIG"]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x114 - Register SHAMD5_SYSSTATUS"]
    pub sysstatus: SYSSTATUS,
    #[doc = "0x118 - Register SHAMD5_IRQSTATUS"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x11c - Register SHAMD5_IRQENABLE. The SHAMD5_IRQENABLE register contains an enable bit for each unique interrupt for the public side. An interrupt is enabled when both the global enable in SHAMD5_SYSCONFIG (PIT_en) and the bit in this register are both set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_P output. Please note that the dedicated partial hash output (SINTREQUEST_PART_P) is not affected by this register it is only affected by the global enable SHAMD5_SYSCONFIG (PIT_en)."]
    pub irqenable: IRQENABLE,
    _reserved40: [u8; 224usize],
    #[doc = "0x200 - HASH512_ODIGEST_A"]
    pub hash512_odigest_a: HASH512_ODIGEST_A,
    #[doc = "0x204 - HASH512_ODIGEST_B"]
    pub hash512_odigest_b: HASH512_ODIGEST_B,
    #[doc = "0x208 - HASH512_ODIGEST_C"]
    pub hash512_odigest_c: HASH512_ODIGEST_C,
    #[doc = "0x20c - HASH512_ODIGEST_D"]
    pub hash512_odigest_d: HASH512_ODIGEST_D,
    #[doc = "0x210 - HASH512_ODIGEST_E"]
    pub hash512_odigest_e: HASH512_ODIGEST_E,
    #[doc = "0x214 - HASH512_ODIGEST_F"]
    pub hash512_odigest_f: HASH512_ODIGEST_F,
    #[doc = "0x218 - HASH512_ODIGEST_G"]
    pub hash512_odigest_g: HASH512_ODIGEST_G,
    #[doc = "0x21c - HASH512_ODIGEST_H"]
    pub hash512_odigest_h: HASH512_ODIGEST_H,
    #[doc = "0x220 - HASH512_ODIGEST_I"]
    pub hash512_odigest_i: HASH512_ODIGEST_I,
    #[doc = "0x224 - HASH512_ODIGEST_J"]
    pub hash512_odigest_j: HASH512_ODIGEST_J,
    #[doc = "0x228 - HASH512_ODIGEST_K"]
    pub hash512_odigest_k: HASH512_ODIGEST_K,
    #[doc = "0x22c - HASH512_ODIGEST_L"]
    pub hash512_odigest_l: HASH512_ODIGEST_L,
    #[doc = "0x230 - HASH512_ODIGEST_M"]
    pub hash512_odigest_m: HASH512_ODIGEST_M,
    #[doc = "0x234 - HASH512_ODIGEST_N"]
    pub hash512_odigest_n: HASH512_ODIGEST_N,
    #[doc = "0x238 - HASH512_ODIGEST_O"]
    pub hash512_odigest_o: HASH512_ODIGEST_O,
    #[doc = "0x23c - HASH512_ODIGEST_P"]
    pub hash512_odigest_p: HASH512_ODIGEST_P,
    #[doc = "0x240 - HASH512_IDIGEST_A"]
    pub hash512_idigest_a: HASH512_IDIGEST_A,
    #[doc = "0x244 - HASH512_IDIGEST_B"]
    pub hash512_idigest_b: HASH512_IDIGEST_B,
    #[doc = "0x248 - HASH512_IDIGEST_C"]
    pub hash512_idigest_c: HASH512_IDIGEST_C,
    #[doc = "0x24c - HASH512_IDIGEST_D"]
    pub hash512_idigest_d: HASH512_IDIGEST_D,
    #[doc = "0x250 - HASH512_IDIGEST_E"]
    pub hash512_idigest_e: HASH512_IDIGEST_E,
    #[doc = "0x254 - HASH512_IDIGEST_F"]
    pub hash512_idigest_f: HASH512_IDIGEST_F,
    #[doc = "0x258 - HASH512_IDIGEST_G"]
    pub hash512_idigest_g: HASH512_IDIGEST_G,
    #[doc = "0x25c - HASH512_IDIGEST_H"]
    pub hash512_idigest_h: HASH512_IDIGEST_H,
    #[doc = "0x260 - HASH512_IDIGEST_I"]
    pub hash512_idigest_i: HASH512_IDIGEST_I,
    #[doc = "0x264 - HASH512_IDIGEST_J"]
    pub hash512_idigest_j: HASH512_IDIGEST_J,
    #[doc = "0x268 - HASH512_IDIGEST_K"]
    pub hash512_idigest_k: HASH512_IDIGEST_K,
    #[doc = "0x26c - HASH512_IDIGEST_L"]
    pub hash512_idigest_l: HASH512_IDIGEST_L,
    #[doc = "0x270 - HASH512_IDIGEST_M"]
    pub hash512_idigest_m: HASH512_IDIGEST_M,
    #[doc = "0x274 - HASH512_IDIGEST_N"]
    pub hash512_idigest_n: HASH512_IDIGEST_N,
    #[doc = "0x278 - HASH512_IDIGEST_O"]
    pub hash512_idigest_o: HASH512_IDIGEST_O,
    #[doc = "0x27c - HASH512_IDIGEST_P"]
    pub hash512_idigest_p: HASH512_IDIGEST_P,
    #[doc = "0x280 - HASH512_DIGEST_COUNT"]
    pub hash512_digest_count: HASH512_DIGEST_COUNT,
    #[doc = "0x284 - HASH512_MODE"]
    pub hash512_mode: HASH512_MODE,
    #[doc = "0x288 - HASH512_LENGTH"]
    pub hash512_length: HASH512_LENGTH,
}
#[doc = "WRITE: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[31:0\\]
for HMAC key proc READ: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_a](odigest_a) module"]
pub type ODIGEST_A = crate::Reg<u32, _ODIGEST_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_A;
#[doc = "`read()` method returns [odigest_a::R](odigest_a::R) reader structure"]
impl crate::Readable for ODIGEST_A {}
#[doc = "`write(|w| ..)` method takes [odigest_a::W](odigest_a::W) writer structure"]
impl crate::Writable for ODIGEST_A {}
#[doc = "WRITE: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[31:0\\]
for HMAC key proc READ: Outer Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2"]
pub mod odigest_a;
#[doc = "WRITE: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[63:32\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_b](odigest_b) module"]
pub type ODIGEST_B = crate::Reg<u32, _ODIGEST_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_B;
#[doc = "`read()` method returns [odigest_b::R](odigest_b::R) reader structure"]
impl crate::Readable for ODIGEST_B {}
#[doc = "`write(|w| ..)` method takes [odigest_b::W](odigest_b::W) writer structure"]
impl crate::Writable for ODIGEST_B {}
#[doc = "WRITE: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[63:32\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2"]
pub mod odigest_b;
#[doc = "WRITE: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / HMAC Key \\[95:64\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_c](odigest_c) module"]
pub type ODIGEST_C = crate::Reg<u32, _ODIGEST_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_C;
#[doc = "`read()` method returns [odigest_c::R](odigest_c::R) reader structure"]
impl crate::Readable for ODIGEST_C {}
#[doc = "`write(|w| ..)` method takes [odigest_c::W](odigest_c::W) writer structure"]
impl crate::Writable for ODIGEST_C {}
#[doc = "WRITE: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / HMAC Key \\[95:64\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2"]
pub mod odigest_c;
#[doc = "WRITE: Outer Digest \\[31:0\\]
for MD5 \\[63:31\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[127:96\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_d](odigest_d) module"]
pub type ODIGEST_D = crate::Reg<u32, _ODIGEST_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_D;
#[doc = "`read()` method returns [odigest_d::R](odigest_d::R) reader structure"]
impl crate::Readable for ODIGEST_D {}
#[doc = "`write(|w| ..)` method takes [odigest_d::W](odigest_d::W) writer structure"]
impl crate::Writable for ODIGEST_D {}
#[doc = "WRITE: Outer Digest \\[31:0\\]
for MD5 \\[63:31\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[127:96\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2"]
pub mod odigest_d;
#[doc = "WRITE: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[159:128\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_e](odigest_e) module"]
pub type ODIGEST_E = crate::Reg<u32, _ODIGEST_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_E;
#[doc = "`read()` method returns [odigest_e::R](odigest_e::R) reader structure"]
impl crate::Readable for ODIGEST_E {}
#[doc = "`write(|w| ..)` method takes [odigest_e::W](odigest_e::W) writer structure"]
impl crate::Writable for ODIGEST_E {}
#[doc = "WRITE: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[159:128\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2"]
pub mod odigest_e;
#[doc = "WRITE: Outer Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[191:160\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_f](odigest_f) module"]
pub type ODIGEST_F = crate::Reg<u32, _ODIGEST_F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_F;
#[doc = "`read()` method returns [odigest_f::R](odigest_f::R) reader structure"]
impl crate::Readable for ODIGEST_F {}
#[doc = "`write(|w| ..)` method takes [odigest_f::W](odigest_f::W) writer structure"]
impl crate::Writable for ODIGEST_F {}
#[doc = "WRITE: Outer Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[191:160\\]
for HMAC key proc READ: Outer Digest \\[95:64\\]
for SHA-2"]
pub mod odigest_f;
#[doc = "WRITE: Outer Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[223:192\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_g](odigest_g) module"]
pub type ODIGEST_G = crate::Reg<u32, _ODIGEST_G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_G;
#[doc = "`read()` method returns [odigest_g::R](odigest_g::R) reader structure"]
impl crate::Readable for ODIGEST_G {}
#[doc = "`write(|w| ..)` method takes [odigest_g::W](odigest_g::W) writer structure"]
impl crate::Writable for ODIGEST_G {}
#[doc = "WRITE: Outer Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[223:192\\]
for HMAC key proc READ: Outer Digest \\[63:32\\]
for SHA-2"]
pub mod odigest_g;
#[doc = "WRITE: Outer Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[255:224\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_h](odigest_h) module"]
pub type ODIGEST_H = crate::Reg<u32, _ODIGEST_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODIGEST_H;
#[doc = "`read()` method returns [odigest_h::R](odigest_h::R) reader structure"]
impl crate::Readable for ODIGEST_H {}
#[doc = "`write(|w| ..)` method takes [odigest_h::W](odigest_h::W) writer structure"]
impl crate::Writable for ODIGEST_H {}
#[doc = "WRITE: Outer Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[255:224\\]
for HMAC key proc READ: Outer Digest \\[31:0\\]
for SHA-2"]
pub mod odigest_h;
#[doc = "WRITE: Inner / Initial Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[287:256\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / Result Digest/MAC \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[223:192\\]
for SHA-2 224 \\[255:224\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_a](idigest_a) module"]
pub type IDIGEST_A = crate::Reg<u32, _IDIGEST_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_A;
#[doc = "`read()` method returns [idigest_a::R](idigest_a::R) reader structure"]
impl crate::Readable for IDIGEST_A {}
#[doc = "`write(|w| ..)` method takes [idigest_a::W](idigest_a::W) writer structure"]
impl crate::Writable for IDIGEST_A {}
#[doc = "WRITE: Inner / Initial Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / HMAC Key \\[287:256\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[255:224\\]
for SHA-2 / Result Digest/MAC \\[127:96\\]
for MD5 \\[159:128\\]
for SHA-1 \\[223:192\\]
for SHA-2 224 \\[255:224\\]
for SHA-2 256"]
pub mod idigest_a;
#[doc = "WRITE: Inner / Initial Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[319:288\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / Result Digest/MAC \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[191:160\\]
for SHA-2 224 \\[223:192\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_b](idigest_b) module"]
pub type IDIGEST_B = crate::Reg<u32, _IDIGEST_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_B;
#[doc = "`read()` method returns [idigest_b::R](idigest_b::R) reader structure"]
impl crate::Readable for IDIGEST_B {}
#[doc = "`write(|w| ..)` method takes [idigest_b::W](idigest_b::W) writer structure"]
impl crate::Writable for IDIGEST_B {}
#[doc = "WRITE: Inner / Initial Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / HMAC Key \\[319:288\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[223:192\\]
for SHA-2 / Result Digest/MAC \\[95:64\\]
for MD5 \\[127:96\\]
for SHA-1 \\[191:160\\]
for SHA-2 224 \\[223:192\\]
for SHA-2 256"]
pub mod idigest_b;
#[doc = "WRITE: Inner / Initial Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA- 2 / HMAC Key \\[351:320\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[159:128\\]
for SHA-2 224 \\[191:160\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_c](idigest_c) module"]
pub type IDIGEST_C = crate::Reg<u32, _IDIGEST_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_C;
#[doc = "`read()` method returns [idigest_c::R](idigest_c::R) reader structure"]
impl crate::Readable for IDIGEST_C {}
#[doc = "`write(|w| ..)` method takes [idigest_c::W](idigest_c::W) writer structure"]
impl crate::Writable for IDIGEST_C {}
#[doc = "WRITE: Inner / Initial Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA- 2 / HMAC Key \\[351:320\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[191:160\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for MD5 \\[95:64\\]
for SHA-1 \\[159:128\\]
for SHA-2 224 \\[191:160\\]
for SHA-2 256"]
pub mod idigest_c;
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[383:352\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[127:96\\]
for SHA-2 224 \\[159:128\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_d](idigest_d) module"]
pub type IDIGEST_D = crate::Reg<u32, _IDIGEST_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_D;
#[doc = "`read()` method returns [idigest_d::R](idigest_d::R) reader structure"]
impl crate::Readable for IDIGEST_D {}
#[doc = "`write(|w| ..)` method takes [idigest_d::W](idigest_d::W) writer structure"]
impl crate::Writable for IDIGEST_D {}
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / HMAC Key \\[383:352\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[159:128\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for MD5 \\[63:32\\]
for SHA-1 \\[127:96\\]
for SHA-2 224 \\[159:128\\]
for SHA-2 256"]
pub mod idigest_d;
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[415:384\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-1 \\[95:64\\]
for SHA-2 224 \\[127:96\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_e](idigest_e) module"]
pub type IDIGEST_E = crate::Reg<u32, _IDIGEST_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_E;
#[doc = "`read()` method returns [idigest_e::R](idigest_e::R) reader structure"]
impl crate::Readable for IDIGEST_E {}
#[doc = "`write(|w| ..)` method takes [idigest_e::W](idigest_e::W) writer structure"]
impl crate::Writable for IDIGEST_E {}
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / HMAC Key \\[415:384\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-1 \\[127:96\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-1 \\[95:64\\]
for SHA-2 224 \\[127:96\\]
for SHA-2 256"]
pub mod idigest_e;
#[doc = "WRITE: Inner / Initial Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[447:416\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for SHA-2 224 \\[95:64\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_f](idigest_f) module"]
pub type IDIGEST_F = crate::Reg<u32, _IDIGEST_F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_F;
#[doc = "`read()` method returns [idigest_f::R](idigest_f::R) reader structure"]
impl crate::Readable for IDIGEST_F {}
#[doc = "`write(|w| ..)` method takes [idigest_f::W](idigest_f::W) writer structure"]
impl crate::Writable for IDIGEST_F {}
#[doc = "WRITE: Inner / Initial Digest \\[95:64\\]
for SHA-2 / HMAC Key \\[447:416\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[95:64\\]
for SHA-2 / Result Digest/MAC \\[63:32\\]
for SHA-2 224 \\[95:64\\]
for SHA-2 256"]
pub mod idigest_f;
#[doc = "WRITE: Inner / Initial Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[479:448\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 224 \\[63:32\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_g](idigest_g) module"]
pub type IDIGEST_G = crate::Reg<u32, _IDIGEST_G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_G;
#[doc = "`read()` method returns [idigest_g::R](idigest_g::R) reader structure"]
impl crate::Readable for IDIGEST_G {}
#[doc = "`write(|w| ..)` method takes [idigest_g::W](idigest_g::W) writer structure"]
impl crate::Writable for IDIGEST_G {}
#[doc = "WRITE: Inner / Initial Digest \\[63:32\\]
for SHA-2 / HMAC Key \\[479:448\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[63:32\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 224 \\[63:32\\]
for SHA-2 256"]
pub mod idigest_g;
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[511:480\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 256\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_h](idigest_h) module"]
pub type IDIGEST_H = crate::Reg<u32, _IDIGEST_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDIGEST_H;
#[doc = "`read()` method returns [idigest_h::R](idigest_h::R) reader structure"]
impl crate::Readable for IDIGEST_H {}
#[doc = "`write(|w| ..)` method takes [idigest_h::W](idigest_h::W) writer structure"]
impl crate::Writable for IDIGEST_H {}
#[doc = "WRITE: Inner / Initial Digest \\[31:0\\]
for SHA-2 / HMAC Key \\[511:480\\]
for HMAC key proc READ: Intermediate / Inner Digest \\[31:0\\]
for SHA-2 / Result Digest/MAC \\[31:0\\]
for SHA-2 256"]
pub mod idigest_h;
#[doc = "WRITE: Initial Digest Count (\\[31:6\\]
only \\[5:0\\]
assumed 0) READ: Result / IntermediateDigest Count The initial digest byte count for hash/HMAC continue operations (HMAC Key Processing = 0 and Use Algorithm Constants = 0) on the Secure World must be written to this register prior to starting the operation by writing to S_HASH_MODE. When either HMAC Key Processing is 1 or Use Algorithm Constants is 1 this register does not need to be written it will be overwritten with 64 (1 hash block of key XOR ipad) or 0 respectively automatically. When starting a HMAC operation from pre-computes (HMAC Key Processing is 0) then the value 64 must be written here to compensate for the appended key XOR ipad block. Note that the value written should always be a 64 byte multiple the lower 6 bits written are ignored. The updated digest byte count (initial digest byte count + bytes processed) can be read from this register when the status register indicates that the operation is done or suspended due to a context switch request or when a Secure World context out DMA is requested. In Advanced DMA mode when not suspended with a partial result reading the SHAMD5_DIGEST_COUNT register triggers the Hash/HMAC Engine to start the next context input DMA. Therefore reading the SHAMD5_DIGEST_COUNT register should always be the last context-read action if not suspended with a partial result (i.e. PartHashReady interrupt not pending).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digest_count](digest_count) module"]
pub type DIGEST_COUNT = crate::Reg<u32, _DIGEST_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIGEST_COUNT;
#[doc = "`read()` method returns [digest_count::R](digest_count::R) reader structure"]
impl crate::Readable for DIGEST_COUNT {}
#[doc = "`write(|w| ..)` method takes [digest_count::W](digest_count::W) writer structure"]
impl crate::Writable for DIGEST_COUNT {}
#[doc = "WRITE: Initial Digest Count (\\[31:6\\]
only \\[5:0\\]
assumed 0) READ: Result / IntermediateDigest Count The initial digest byte count for hash/HMAC continue operations (HMAC Key Processing = 0 and Use Algorithm Constants = 0) on the Secure World must be written to this register prior to starting the operation by writing to S_HASH_MODE. When either HMAC Key Processing is 1 or Use Algorithm Constants is 1 this register does not need to be written it will be overwritten with 64 (1 hash block of key XOR ipad) or 0 respectively automatically. When starting a HMAC operation from pre-computes (HMAC Key Processing is 0) then the value 64 must be written here to compensate for the appended key XOR ipad block. Note that the value written should always be a 64 byte multiple the lower 6 bits written are ignored. The updated digest byte count (initial digest byte count + bytes processed) can be read from this register when the status register indicates that the operation is done or suspended due to a context switch request or when a Secure World context out DMA is requested. In Advanced DMA mode when not suspended with a partial result reading the SHAMD5_DIGEST_COUNT register triggers the Hash/HMAC Engine to start the next context input DMA. Therefore reading the SHAMD5_DIGEST_COUNT register should always be the last context-read action if not suspended with a partial result (i.e. PartHashReady interrupt not pending)."]
pub mod digest_count;
#[doc = "Register SHAMD5_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Register SHAMD5_MODE"]
pub mod mode;
#[doc = "WRITE: Block Length / Remaining Byte Count (bytes) READ: Remaining Byte Count. The value programmed MUST be a 64-byte multiple if Close Hash is set to 0. This register is also the trigger to start processing: once this register is written the core will commence requesting input data via DMA or IRQ (if programmed length > 0) and start processing. The remaining byte count for the active operation can be read from this register when the interrupt status register indicates that the operation is suspended due to a context switch request.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](length) module"]
pub type LENGTH = crate::Reg<u32, _LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LENGTH;
#[doc = "`read()` method returns [length::R](length::R) reader structure"]
impl crate::Readable for LENGTH {}
#[doc = "`write(|w| ..)` method takes [length::W](length::W) writer structure"]
impl crate::Writable for LENGTH {}
#[doc = "WRITE: Block Length / Remaining Byte Count (bytes) READ: Remaining Byte Count. The value programmed MUST be a 64-byte multiple if Close Hash is set to 0. This register is also the trigger to start processing: once this register is written the core will commence requesting input data via DMA or IRQ (if programmed length > 0) and start processing. The remaining byte count for the active operation can be read from this register when the interrupt status register indicates that the operation is suspended due to a context switch request."]
pub mod length;
#[doc = "Data input message 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0_in](data0_in) module"]
pub type DATA0_IN = crate::Reg<u32, _DATA0_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0_IN;
#[doc = "`read()` method returns [data0_in::R](data0_in::R) reader structure"]
impl crate::Readable for DATA0_IN {}
#[doc = "`write(|w| ..)` method takes [data0_in::W](data0_in::W) writer structure"]
impl crate::Writable for DATA0_IN {}
#[doc = "Data input message 0"]
pub mod data0_in;
#[doc = "Data input message 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1_in](data1_in) module"]
pub type DATA1_IN = crate::Reg<u32, _DATA1_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1_IN;
#[doc = "`read()` method returns [data1_in::R](data1_in::R) reader structure"]
impl crate::Readable for DATA1_IN {}
#[doc = "`write(|w| ..)` method takes [data1_in::W](data1_in::W) writer structure"]
impl crate::Writable for DATA1_IN {}
#[doc = "Data input message 1"]
pub mod data1_in;
#[doc = "Data input message 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2_in](data2_in) module"]
pub type DATA2_IN = crate::Reg<u32, _DATA2_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA2_IN;
#[doc = "`read()` method returns [data2_in::R](data2_in::R) reader structure"]
impl crate::Readable for DATA2_IN {}
#[doc = "`write(|w| ..)` method takes [data2_in::W](data2_in::W) writer structure"]
impl crate::Writable for DATA2_IN {}
#[doc = "Data input message 2"]
pub mod data2_in;
#[doc = "Data input message 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data3_in](data3_in) module"]
pub type DATA3_IN = crate::Reg<u32, _DATA3_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA3_IN;
#[doc = "`read()` method returns [data3_in::R](data3_in::R) reader structure"]
impl crate::Readable for DATA3_IN {}
#[doc = "`write(|w| ..)` method takes [data3_in::W](data3_in::W) writer structure"]
impl crate::Writable for DATA3_IN {}
#[doc = "Data input message 3"]
pub mod data3_in;
#[doc = "Data input message 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data4_in](data4_in) module"]
pub type DATA4_IN = crate::Reg<u32, _DATA4_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA4_IN;
#[doc = "`read()` method returns [data4_in::R](data4_in::R) reader structure"]
impl crate::Readable for DATA4_IN {}
#[doc = "`write(|w| ..)` method takes [data4_in::W](data4_in::W) writer structure"]
impl crate::Writable for DATA4_IN {}
#[doc = "Data input message 4"]
pub mod data4_in;
#[doc = "Data input message 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data5_in](data5_in) module"]
pub type DATA5_IN = crate::Reg<u32, _DATA5_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA5_IN;
#[doc = "`read()` method returns [data5_in::R](data5_in::R) reader structure"]
impl crate::Readable for DATA5_IN {}
#[doc = "`write(|w| ..)` method takes [data5_in::W](data5_in::W) writer structure"]
impl crate::Writable for DATA5_IN {}
#[doc = "Data input message 5"]
pub mod data5_in;
#[doc = "Data input message 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data6_in](data6_in) module"]
pub type DATA6_IN = crate::Reg<u32, _DATA6_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA6_IN;
#[doc = "`read()` method returns [data6_in::R](data6_in::R) reader structure"]
impl crate::Readable for DATA6_IN {}
#[doc = "`write(|w| ..)` method takes [data6_in::W](data6_in::W) writer structure"]
impl crate::Writable for DATA6_IN {}
#[doc = "Data input message 6"]
pub mod data6_in;
#[doc = "Data input message 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data7_in](data7_in) module"]
pub type DATA7_IN = crate::Reg<u32, _DATA7_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA7_IN;
#[doc = "`read()` method returns [data7_in::R](data7_in::R) reader structure"]
impl crate::Readable for DATA7_IN {}
#[doc = "`write(|w| ..)` method takes [data7_in::W](data7_in::W) writer structure"]
impl crate::Writable for DATA7_IN {}
#[doc = "Data input message 7"]
pub mod data7_in;
#[doc = "Data input message 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8_in](data8_in) module"]
pub type DATA8_IN = crate::Reg<u32, _DATA8_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA8_IN;
#[doc = "`read()` method returns [data8_in::R](data8_in::R) reader structure"]
impl crate::Readable for DATA8_IN {}
#[doc = "`write(|w| ..)` method takes [data8_in::W](data8_in::W) writer structure"]
impl crate::Writable for DATA8_IN {}
#[doc = "Data input message 8"]
pub mod data8_in;
#[doc = "Data input message 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data9_in](data9_in) module"]
pub type DATA9_IN = crate::Reg<u32, _DATA9_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA9_IN;
#[doc = "`read()` method returns [data9_in::R](data9_in::R) reader structure"]
impl crate::Readable for DATA9_IN {}
#[doc = "`write(|w| ..)` method takes [data9_in::W](data9_in::W) writer structure"]
impl crate::Writable for DATA9_IN {}
#[doc = "Data input message 9"]
pub mod data9_in;
#[doc = "Data input message 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data10_in](data10_in) module"]
pub type DATA10_IN = crate::Reg<u32, _DATA10_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA10_IN;
#[doc = "`read()` method returns [data10_in::R](data10_in::R) reader structure"]
impl crate::Readable for DATA10_IN {}
#[doc = "`write(|w| ..)` method takes [data10_in::W](data10_in::W) writer structure"]
impl crate::Writable for DATA10_IN {}
#[doc = "Data input message 10"]
pub mod data10_in;
#[doc = "Data input message 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data11_in](data11_in) module"]
pub type DATA11_IN = crate::Reg<u32, _DATA11_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA11_IN;
#[doc = "`read()` method returns [data11_in::R](data11_in::R) reader structure"]
impl crate::Readable for DATA11_IN {}
#[doc = "`write(|w| ..)` method takes [data11_in::W](data11_in::W) writer structure"]
impl crate::Writable for DATA11_IN {}
#[doc = "Data input message 11"]
pub mod data11_in;
#[doc = "Data input message 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data12_in](data12_in) module"]
pub type DATA12_IN = crate::Reg<u32, _DATA12_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA12_IN;
#[doc = "`read()` method returns [data12_in::R](data12_in::R) reader structure"]
impl crate::Readable for DATA12_IN {}
#[doc = "`write(|w| ..)` method takes [data12_in::W](data12_in::W) writer structure"]
impl crate::Writable for DATA12_IN {}
#[doc = "Data input message 12"]
pub mod data12_in;
#[doc = "Data input message 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data13_in](data13_in) module"]
pub type DATA13_IN = crate::Reg<u32, _DATA13_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA13_IN;
#[doc = "`read()` method returns [data13_in::R](data13_in::R) reader structure"]
impl crate::Readable for DATA13_IN {}
#[doc = "`write(|w| ..)` method takes [data13_in::W](data13_in::W) writer structure"]
impl crate::Writable for DATA13_IN {}
#[doc = "Data input message 13"]
pub mod data13_in;
#[doc = "Data input message 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data14_in](data14_in) module"]
pub type DATA14_IN = crate::Reg<u32, _DATA14_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA14_IN;
#[doc = "`read()` method returns [data14_in::R](data14_in::R) reader structure"]
impl crate::Readable for DATA14_IN {}
#[doc = "`write(|w| ..)` method takes [data14_in::W](data14_in::W) writer structure"]
impl crate::Writable for DATA14_IN {}
#[doc = "Data input message 14"]
pub mod data14_in;
#[doc = "Data input message 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data15_in](data15_in) module"]
pub type DATA15_IN = crate::Reg<u32, _DATA15_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA15_IN;
#[doc = "`read()` method returns [data15_in::R](data15_in::R) reader structure"]
impl crate::Readable for DATA15_IN {}
#[doc = "`write(|w| ..)` method takes [data15_in::W](data15_in::W) writer structure"]
impl crate::Writable for DATA15_IN {}
#[doc = "Data input message 15"]
pub mod data15_in;
#[doc = "Register SHAMD5_REV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "`write(|w| ..)` method takes [revision::W](revision::W) writer structure"]
impl crate::Writable for REVISION {}
#[doc = "Register SHAMD5_REV"]
pub mod revision;
#[doc = "Register SHAMD5_SYSCONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](sysconfig) module"]
pub type SYSCONFIG = crate::Reg<u32, _SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCONFIG;
#[doc = "`read()` method returns [sysconfig::R](sysconfig::R) reader structure"]
impl crate::Readable for SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](sysconfig::W) writer structure"]
impl crate::Writable for SYSCONFIG {}
#[doc = "Register SHAMD5_SYSCONFIG"]
pub mod sysconfig;
#[doc = "Register SHAMD5_SYSSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysstatus](sysstatus) module"]
pub type SYSSTATUS = crate::Reg<u32, _SYSSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSTATUS;
#[doc = "`read()` method returns [sysstatus::R](sysstatus::R) reader structure"]
impl crate::Readable for SYSSTATUS {}
#[doc = "`write(|w| ..)` method takes [sysstatus::W](sysstatus::W) writer structure"]
impl crate::Writable for SYSSTATUS {}
#[doc = "Register SHAMD5_SYSSTATUS"]
pub mod sysstatus;
#[doc = "Register SHAMD5_IRQSTATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](irqstatus) module"]
pub type IRQSTATUS = crate::Reg<u32, _IRQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATUS;
#[doc = "`read()` method returns [irqstatus::R](irqstatus::R) reader structure"]
impl crate::Readable for IRQSTATUS {}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](irqstatus::W) writer structure"]
impl crate::Writable for IRQSTATUS {}
#[doc = "Register SHAMD5_IRQSTATUS"]
pub mod irqstatus;
#[doc = "Register SHAMD5_IRQENABLE. The SHAMD5_IRQENABLE register contains an enable bit for each unique interrupt for the public side. An interrupt is enabled when both the global enable in SHAMD5_SYSCONFIG (PIT_en) and the bit in this register are both set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_P output. Please note that the dedicated partial hash output (SINTREQUEST_PART_P) is not affected by this register it is only affected by the global enable SHAMD5_SYSCONFIG (PIT_en).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqenable](irqenable) module"]
pub type IRQENABLE = crate::Reg<u32, _IRQENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQENABLE;
#[doc = "`read()` method returns [irqenable::R](irqenable::R) reader structure"]
impl crate::Readable for IRQENABLE {}
#[doc = "`write(|w| ..)` method takes [irqenable::W](irqenable::W) writer structure"]
impl crate::Writable for IRQENABLE {}
#[doc = "Register SHAMD5_IRQENABLE. The SHAMD5_IRQENABLE register contains an enable bit for each unique interrupt for the public side. An interrupt is enabled when both the global enable in SHAMD5_SYSCONFIG (PIT_en) and the bit in this register are both set to 1. An interrupt that is enabled is propagated to the SINTREQUEST_P output. Please note that the dedicated partial hash output (SINTREQUEST_PART_P) is not affected by this register it is only affected by the global enable SHAMD5_SYSCONFIG (PIT_en)."]
pub mod irqenable;
#[doc = "HASH512_ODIGEST_A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_a](hash512_odigest_a) module"]
pub type HASH512_ODIGEST_A = crate::Reg<u32, _HASH512_ODIGEST_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_A;
#[doc = "`read()` method returns [hash512_odigest_a::R](hash512_odigest_a::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_A {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_a::W](hash512_odigest_a::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_A {}
#[doc = "HASH512_ODIGEST_A"]
pub mod hash512_odigest_a;
#[doc = "HASH512_ODIGEST_B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_b](hash512_odigest_b) module"]
pub type HASH512_ODIGEST_B = crate::Reg<u32, _HASH512_ODIGEST_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_B;
#[doc = "`read()` method returns [hash512_odigest_b::R](hash512_odigest_b::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_B {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_b::W](hash512_odigest_b::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_B {}
#[doc = "HASH512_ODIGEST_B"]
pub mod hash512_odigest_b;
#[doc = "HASH512_ODIGEST_C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_c](hash512_odigest_c) module"]
pub type HASH512_ODIGEST_C = crate::Reg<u32, _HASH512_ODIGEST_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_C;
#[doc = "`read()` method returns [hash512_odigest_c::R](hash512_odigest_c::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_C {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_c::W](hash512_odigest_c::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_C {}
#[doc = "HASH512_ODIGEST_C"]
pub mod hash512_odigest_c;
#[doc = "HASH512_ODIGEST_D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_d](hash512_odigest_d) module"]
pub type HASH512_ODIGEST_D = crate::Reg<u32, _HASH512_ODIGEST_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_D;
#[doc = "`read()` method returns [hash512_odigest_d::R](hash512_odigest_d::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_D {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_d::W](hash512_odigest_d::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_D {}
#[doc = "HASH512_ODIGEST_D"]
pub mod hash512_odigest_d;
#[doc = "HASH512_ODIGEST_E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_e](hash512_odigest_e) module"]
pub type HASH512_ODIGEST_E = crate::Reg<u32, _HASH512_ODIGEST_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_E;
#[doc = "`read()` method returns [hash512_odigest_e::R](hash512_odigest_e::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_E {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_e::W](hash512_odigest_e::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_E {}
#[doc = "HASH512_ODIGEST_E"]
pub mod hash512_odigest_e;
#[doc = "HASH512_ODIGEST_F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_f](hash512_odigest_f) module"]
pub type HASH512_ODIGEST_F = crate::Reg<u32, _HASH512_ODIGEST_F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_F;
#[doc = "`read()` method returns [hash512_odigest_f::R](hash512_odigest_f::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_F {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_f::W](hash512_odigest_f::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_F {}
#[doc = "HASH512_ODIGEST_F"]
pub mod hash512_odigest_f;
#[doc = "HASH512_ODIGEST_G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_g](hash512_odigest_g) module"]
pub type HASH512_ODIGEST_G = crate::Reg<u32, _HASH512_ODIGEST_G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_G;
#[doc = "`read()` method returns [hash512_odigest_g::R](hash512_odigest_g::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_G {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_g::W](hash512_odigest_g::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_G {}
#[doc = "HASH512_ODIGEST_G"]
pub mod hash512_odigest_g;
#[doc = "HASH512_ODIGEST_H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_h](hash512_odigest_h) module"]
pub type HASH512_ODIGEST_H = crate::Reg<u32, _HASH512_ODIGEST_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_H;
#[doc = "`read()` method returns [hash512_odigest_h::R](hash512_odigest_h::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_H {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_h::W](hash512_odigest_h::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_H {}
#[doc = "HASH512_ODIGEST_H"]
pub mod hash512_odigest_h;
#[doc = "HASH512_ODIGEST_I\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_i](hash512_odigest_i) module"]
pub type HASH512_ODIGEST_I = crate::Reg<u32, _HASH512_ODIGEST_I>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_I;
#[doc = "`read()` method returns [hash512_odigest_i::R](hash512_odigest_i::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_I {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_i::W](hash512_odigest_i::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_I {}
#[doc = "HASH512_ODIGEST_I"]
pub mod hash512_odigest_i;
#[doc = "HASH512_ODIGEST_J\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_j](hash512_odigest_j) module"]
pub type HASH512_ODIGEST_J = crate::Reg<u32, _HASH512_ODIGEST_J>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_J;
#[doc = "`read()` method returns [hash512_odigest_j::R](hash512_odigest_j::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_J {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_j::W](hash512_odigest_j::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_J {}
#[doc = "HASH512_ODIGEST_J"]
pub mod hash512_odigest_j;
#[doc = "HASH512_ODIGEST_K\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_k](hash512_odigest_k) module"]
pub type HASH512_ODIGEST_K = crate::Reg<u32, _HASH512_ODIGEST_K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_K;
#[doc = "`read()` method returns [hash512_odigest_k::R](hash512_odigest_k::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_K {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_k::W](hash512_odigest_k::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_K {}
#[doc = "HASH512_ODIGEST_K"]
pub mod hash512_odigest_k;
#[doc = "HASH512_ODIGEST_L\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_l](hash512_odigest_l) module"]
pub type HASH512_ODIGEST_L = crate::Reg<u32, _HASH512_ODIGEST_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_L;
#[doc = "`read()` method returns [hash512_odigest_l::R](hash512_odigest_l::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_L {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_l::W](hash512_odigest_l::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_L {}
#[doc = "HASH512_ODIGEST_L"]
pub mod hash512_odigest_l;
#[doc = "HASH512_ODIGEST_M\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_m](hash512_odigest_m) module"]
pub type HASH512_ODIGEST_M = crate::Reg<u32, _HASH512_ODIGEST_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_M;
#[doc = "`read()` method returns [hash512_odigest_m::R](hash512_odigest_m::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_M {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_m::W](hash512_odigest_m::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_M {}
#[doc = "HASH512_ODIGEST_M"]
pub mod hash512_odigest_m;
#[doc = "HASH512_ODIGEST_N\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_n](hash512_odigest_n) module"]
pub type HASH512_ODIGEST_N = crate::Reg<u32, _HASH512_ODIGEST_N>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_N;
#[doc = "`read()` method returns [hash512_odigest_n::R](hash512_odigest_n::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_N {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_n::W](hash512_odigest_n::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_N {}
#[doc = "HASH512_ODIGEST_N"]
pub mod hash512_odigest_n;
#[doc = "HASH512_ODIGEST_O\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_o](hash512_odigest_o) module"]
pub type HASH512_ODIGEST_O = crate::Reg<u32, _HASH512_ODIGEST_O>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_O;
#[doc = "`read()` method returns [hash512_odigest_o::R](hash512_odigest_o::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_O {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_o::W](hash512_odigest_o::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_O {}
#[doc = "HASH512_ODIGEST_O"]
pub mod hash512_odigest_o;
#[doc = "HASH512_ODIGEST_P\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_odigest_p](hash512_odigest_p) module"]
pub type HASH512_ODIGEST_P = crate::Reg<u32, _HASH512_ODIGEST_P>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_ODIGEST_P;
#[doc = "`read()` method returns [hash512_odigest_p::R](hash512_odigest_p::R) reader structure"]
impl crate::Readable for HASH512_ODIGEST_P {}
#[doc = "`write(|w| ..)` method takes [hash512_odigest_p::W](hash512_odigest_p::W) writer structure"]
impl crate::Writable for HASH512_ODIGEST_P {}
#[doc = "HASH512_ODIGEST_P"]
pub mod hash512_odigest_p;
#[doc = "HASH512_IDIGEST_A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_a](hash512_idigest_a) module"]
pub type HASH512_IDIGEST_A = crate::Reg<u32, _HASH512_IDIGEST_A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_A;
#[doc = "`read()` method returns [hash512_idigest_a::R](hash512_idigest_a::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_A {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_a::W](hash512_idigest_a::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_A {}
#[doc = "HASH512_IDIGEST_A"]
pub mod hash512_idigest_a;
#[doc = "HASH512_IDIGEST_B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_b](hash512_idigest_b) module"]
pub type HASH512_IDIGEST_B = crate::Reg<u32, _HASH512_IDIGEST_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_B;
#[doc = "`read()` method returns [hash512_idigest_b::R](hash512_idigest_b::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_B {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_b::W](hash512_idigest_b::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_B {}
#[doc = "HASH512_IDIGEST_B"]
pub mod hash512_idigest_b;
#[doc = "HASH512_IDIGEST_C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_c](hash512_idigest_c) module"]
pub type HASH512_IDIGEST_C = crate::Reg<u32, _HASH512_IDIGEST_C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_C;
#[doc = "`read()` method returns [hash512_idigest_c::R](hash512_idigest_c::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_C {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_c::W](hash512_idigest_c::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_C {}
#[doc = "HASH512_IDIGEST_C"]
pub mod hash512_idigest_c;
#[doc = "HASH512_IDIGEST_D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_d](hash512_idigest_d) module"]
pub type HASH512_IDIGEST_D = crate::Reg<u32, _HASH512_IDIGEST_D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_D;
#[doc = "`read()` method returns [hash512_idigest_d::R](hash512_idigest_d::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_D {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_d::W](hash512_idigest_d::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_D {}
#[doc = "HASH512_IDIGEST_D"]
pub mod hash512_idigest_d;
#[doc = "HASH512_IDIGEST_E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_e](hash512_idigest_e) module"]
pub type HASH512_IDIGEST_E = crate::Reg<u32, _HASH512_IDIGEST_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_E;
#[doc = "`read()` method returns [hash512_idigest_e::R](hash512_idigest_e::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_E {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_e::W](hash512_idigest_e::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_E {}
#[doc = "HASH512_IDIGEST_E"]
pub mod hash512_idigest_e;
#[doc = "HASH512_IDIGEST_F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_f](hash512_idigest_f) module"]
pub type HASH512_IDIGEST_F = crate::Reg<u32, _HASH512_IDIGEST_F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_F;
#[doc = "`read()` method returns [hash512_idigest_f::R](hash512_idigest_f::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_F {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_f::W](hash512_idigest_f::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_F {}
#[doc = "HASH512_IDIGEST_F"]
pub mod hash512_idigest_f;
#[doc = "HASH512_IDIGEST_G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_g](hash512_idigest_g) module"]
pub type HASH512_IDIGEST_G = crate::Reg<u32, _HASH512_IDIGEST_G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_G;
#[doc = "`read()` method returns [hash512_idigest_g::R](hash512_idigest_g::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_G {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_g::W](hash512_idigest_g::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_G {}
#[doc = "HASH512_IDIGEST_G"]
pub mod hash512_idigest_g;
#[doc = "HASH512_IDIGEST_H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_h](hash512_idigest_h) module"]
pub type HASH512_IDIGEST_H = crate::Reg<u32, _HASH512_IDIGEST_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_H;
#[doc = "`read()` method returns [hash512_idigest_h::R](hash512_idigest_h::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_H {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_h::W](hash512_idigest_h::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_H {}
#[doc = "HASH512_IDIGEST_H"]
pub mod hash512_idigest_h;
#[doc = "HASH512_IDIGEST_I\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_i](hash512_idigest_i) module"]
pub type HASH512_IDIGEST_I = crate::Reg<u32, _HASH512_IDIGEST_I>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_I;
#[doc = "`read()` method returns [hash512_idigest_i::R](hash512_idigest_i::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_I {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_i::W](hash512_idigest_i::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_I {}
#[doc = "HASH512_IDIGEST_I"]
pub mod hash512_idigest_i;
#[doc = "HASH512_IDIGEST_J\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_j](hash512_idigest_j) module"]
pub type HASH512_IDIGEST_J = crate::Reg<u32, _HASH512_IDIGEST_J>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_J;
#[doc = "`read()` method returns [hash512_idigest_j::R](hash512_idigest_j::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_J {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_j::W](hash512_idigest_j::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_J {}
#[doc = "HASH512_IDIGEST_J"]
pub mod hash512_idigest_j;
#[doc = "HASH512_IDIGEST_K\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_k](hash512_idigest_k) module"]
pub type HASH512_IDIGEST_K = crate::Reg<u32, _HASH512_IDIGEST_K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_K;
#[doc = "`read()` method returns [hash512_idigest_k::R](hash512_idigest_k::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_K {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_k::W](hash512_idigest_k::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_K {}
#[doc = "HASH512_IDIGEST_K"]
pub mod hash512_idigest_k;
#[doc = "HASH512_IDIGEST_L\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_l](hash512_idigest_l) module"]
pub type HASH512_IDIGEST_L = crate::Reg<u32, _HASH512_IDIGEST_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_L;
#[doc = "`read()` method returns [hash512_idigest_l::R](hash512_idigest_l::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_L {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_l::W](hash512_idigest_l::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_L {}
#[doc = "HASH512_IDIGEST_L"]
pub mod hash512_idigest_l;
#[doc = "HASH512_IDIGEST_M\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_m](hash512_idigest_m) module"]
pub type HASH512_IDIGEST_M = crate::Reg<u32, _HASH512_IDIGEST_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_M;
#[doc = "`read()` method returns [hash512_idigest_m::R](hash512_idigest_m::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_M {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_m::W](hash512_idigest_m::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_M {}
#[doc = "HASH512_IDIGEST_M"]
pub mod hash512_idigest_m;
#[doc = "HASH512_IDIGEST_N\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_n](hash512_idigest_n) module"]
pub type HASH512_IDIGEST_N = crate::Reg<u32, _HASH512_IDIGEST_N>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_N;
#[doc = "`read()` method returns [hash512_idigest_n::R](hash512_idigest_n::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_N {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_n::W](hash512_idigest_n::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_N {}
#[doc = "HASH512_IDIGEST_N"]
pub mod hash512_idigest_n;
#[doc = "HASH512_IDIGEST_O\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_o](hash512_idigest_o) module"]
pub type HASH512_IDIGEST_O = crate::Reg<u32, _HASH512_IDIGEST_O>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_O;
#[doc = "`read()` method returns [hash512_idigest_o::R](hash512_idigest_o::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_O {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_o::W](hash512_idigest_o::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_O {}
#[doc = "HASH512_IDIGEST_O"]
pub mod hash512_idigest_o;
#[doc = "HASH512_IDIGEST_P\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_idigest_p](hash512_idigest_p) module"]
pub type HASH512_IDIGEST_P = crate::Reg<u32, _HASH512_IDIGEST_P>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_IDIGEST_P;
#[doc = "`read()` method returns [hash512_idigest_p::R](hash512_idigest_p::R) reader structure"]
impl crate::Readable for HASH512_IDIGEST_P {}
#[doc = "`write(|w| ..)` method takes [hash512_idigest_p::W](hash512_idigest_p::W) writer structure"]
impl crate::Writable for HASH512_IDIGEST_P {}
#[doc = "HASH512_IDIGEST_P"]
pub mod hash512_idigest_p;
#[doc = "HASH512_DIGEST_COUNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_digest_count](hash512_digest_count) module"]
pub type HASH512_DIGEST_COUNT = crate::Reg<u32, _HASH512_DIGEST_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_DIGEST_COUNT;
#[doc = "`read()` method returns [hash512_digest_count::R](hash512_digest_count::R) reader structure"]
impl crate::Readable for HASH512_DIGEST_COUNT {}
#[doc = "`write(|w| ..)` method takes [hash512_digest_count::W](hash512_digest_count::W) writer structure"]
impl crate::Writable for HASH512_DIGEST_COUNT {}
#[doc = "HASH512_DIGEST_COUNT"]
pub mod hash512_digest_count;
#[doc = "HASH512_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_mode](hash512_mode) module"]
pub type HASH512_MODE = crate::Reg<u32, _HASH512_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_MODE;
#[doc = "`read()` method returns [hash512_mode::R](hash512_mode::R) reader structure"]
impl crate::Readable for HASH512_MODE {}
#[doc = "`write(|w| ..)` method takes [hash512_mode::W](hash512_mode::W) writer structure"]
impl crate::Writable for HASH512_MODE {}
#[doc = "HASH512_MODE"]
pub mod hash512_mode;
#[doc = "HASH512_LENGTH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash512_length](hash512_length) module"]
pub type HASH512_LENGTH = crate::Reg<u32, _HASH512_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH512_LENGTH;
#[doc = "`read()` method returns [hash512_length::R](hash512_length::R) reader structure"]
impl crate::Readable for HASH512_LENGTH {}
#[doc = "`write(|w| ..)` method takes [hash512_length::W](hash512_length::W) writer structure"]
impl crate::Writable for HASH512_LENGTH {}
#[doc = "HASH512_LENGTH"]
pub mod hash512_length;
