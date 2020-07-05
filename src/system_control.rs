#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
    pub hl_rev: HL_REV,
    #[doc = "0x04 - Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
    pub hl_hwinfo: HL_HWINFO,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Clock management configuration"]
    pub hl_sysconfig: HL_SYSCONFIG,
    _reserved3: [u8; 252usize],
    #[doc = "0x110 - System Configuration Register This register allows controlling various parameters of the OCP interface."]
    pub sysconfig: SYSCONFIG,
    #[doc = "0x114 - System Status Register This register provides status information about the module excluding the interrupt status information"]
    pub sysstatus: SYSSTATUS,
    _reserved5: [u8; 12usize],
    #[doc = "0x124 - Card status response error This register enables the host controller to detect card status errors of response type R1 R1b for all cards and of R5 R5b and R6 response for cards types SD or SDIO. When a bit MMCHS_CSRE\\[i\\]
is set to 1 if the corresponding bit at the same position in the response MMCHS_RSP0\\[i\\]
is set to 1 the host controller indicates a card error (MMCHS_STAT\\[CERR\\]) interrupt status to avoid the host driver reading the response register (MMCHS_RSP0). Note: No automatic card error detection for autoCMD12 is implemented; the host system has to check autoCMD12 response register (MMCHS_RESP76) for possible card errors."]
    pub csre: CSRE,
    #[doc = "0x128 - System Test register This register is used to control the signals that connect to I/O pins when the module is configured in system test (SYSTEST) mode for boundary connectivity verification. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer. The buffer behaves as a stack accessible only by the local host (push and pop operations). In this mode the Transfer Block Size (MMCHS_BLK\\[BLEN\\]) and the Blocks count for current transfer (MMCHS_BLK\\[NBLK\\]) are needed to generate a Buffer write ready interrupt (MMCHS_STAT\\[BWR\\]) or a Buffer read ready interrupt (MMCHS_STAT\\[BRR\\]) and DMA requests if enabled."]
    pub systest: SYSTEST,
    #[doc = "0x12c - Configuration register This register is used: - to select the functional mode or the SYSTEST mode for any card. - to send an initialization sequence to any card. - to enable the detection on DAT\\[1\\]
of a card interrupt for SDIO cards only. and also to configure : - specific data and command transfers for MMC cards only. - the parameters related to the card detect and write protect input signals."]
    pub con: CON,
    #[doc = "0x130 - Power counter register This register is used to program a mmc counter to delay command transfers after activating the PAD power this value depends on PAD characteristics and voltage."]
    pub pwcnt: PWCNT,
    _reserved9: [u8; 208usize],
    #[doc = "0x204 - Transfer Length Configuration register MMCHS_BLK\\[BLEN\\]
is the block size register. MMCHS_BLK\\[NBLK\\]
is the block count register. This register shall be used for any card."]
    pub blk: BLK,
    #[doc = "0x208 - Command argument Register This register contains command argument specified as bit 39-8 of Command-Format These registers must be initialized prior to sending the command itself to the card (write action into the register MMCHS_CMD register). Only exception is for a command index specifying stuff bits in arguments making a write unnecessary."]
    pub arg: ARG,
    #[doc = "0x20c - Command and transfer mode register MMCHS_CMD\\[31:16\\]
= the command register MMCHS_CMD\\[15:0\\]
= the transfer mode. This register configures the data and command transfers. A write into the most significant byte send the command. A write into MMCHS_CMD\\[15:0\\]
registers during data transfer has no effect. This register shall be used for any card. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer."]
    pub cmd: CMD,
    #[doc = "0x210 - Command response\\[31:0\\]
Register This 32-bit register holds bits positions \\[31:0\\]
of command response type R1/R1b/R2/R3/R4/R5/R5b/R6"]
    pub rsp10: RSP10,
    #[doc = "0x214 - Command response\\[63:32\\]
Register This 32-bit register holds bits positions \\[63:32\\]
of command response type R2"]
    pub rsp32: RSP32,
    #[doc = "0x218 - Command response\\[95:64\\]
Register This 32-bit register holds bits positions \\[95:64\\]
of command response type R2"]
    pub rsp54: RSP54,
    #[doc = "0x21c - Command response\\[127:96\\]
Register This 32-bit register holds bits positions \\[127:96\\]
of command response type R2"]
    pub rsp76: RSP76,
    #[doc = "0x220 - Data Register This register is the 32-bit entry point of the buffer for read or write data transfers. The buffer size is 32bits x256(1024 bytes). Bytes within a word are stored and read in little endian format. This buffer can be used as two 512 byte buffers to transfer data efficiently without reducing the throughput. Sequential and contiguous access is necessary to increment the pointer correctly. Random or skipped access is not allowed. In little endian if the local host accesses this register byte-wise or 16bit-wise the least significant byte (bits \\[7:0\\]) must always be written/read first. The update of the buffer address is done on the most significant byte write for full 32-bit DATA register or on the most significant byte of the last word of block transfer. Example 1: Byte or 16-bit access Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1100 (2-bytes) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=0100 (1-byte) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1000 (1-byte) Bad"]
    pub data: DATA,
    #[doc = "0x224 - Present state register The Host can get status of the Host Controller from this 32-bit read only register."]
    pub pstate: PSTATE,
    #[doc = "0x228 - Control register This register defines the host controls to set power wakeup and transfer parameters. MMCHS_HCTL\\[31:24\\]
= Wakeup control MMCHS_HCTL\\[23:16\\]
= Block gap control MMCHS_HCTL\\[15:8\\]
= Power control MMCHS_HCTL\\[7:0\\]
= Host control"]
    pub hctl: HCTL,
    #[doc = "0x22c - SD system control register This register defines the system controls to set software resets clock frequency management and data timeout. MMCHS_SYSCTL\\[31:24\\]
= Software resets MMCHS_SYSCTL\\[23:16\\]
= Timeout control MMCHS_SYSCTL\\[15:0\\]
= Clock control"]
    pub sysctl: SYSCTL,
    #[doc = "0x230 - Interrupt status register The interrupt status regroups all the status of the module internal events that can generate an interrupt. MMCHS_STAT\\[31:16\\]
= Error Interrupt Status MMCHS_STAT\\[15:0\\]
= Normal Interrupt Status"]
    pub stat: STAT,
    #[doc = "0x234 - Interrupt SD enable register This register allows to enable/disable the module to set status bits on an event-by-event basis. MMCHS_IE\\[31:16\\]
= Error Interrupt Status Enable MMCHS_IE\\[15:0\\]
= Normal Interrupt Status Enable"]
    pub ie: IE,
    #[doc = "0x238 - Interrupt signal enable register This register allows to enable/disable the module internal sources of status on an event-by-event basis. MMCHS_ISE\\[31:16\\]
= Error Interrupt Signal Enable MMCHS_ISE\\[15:0\\]
= Normal Interrupt Signal Enable"]
    pub ise: ISE,
    #[doc = "0x23c - Auto CMD12 Error Status Register The host driver may determine which of the errors cases related to Auto CMD12 has occurred by checking this MMCHS_AC12 register when an Auto CMD12 Error interrupt occurs. This register is valid only when Auto CMD12 is enabled (MMCHS_CMD\\[ACEN\\]) and Auto CMD12Error (MMCHS_STAT\\[ACE\\]) is set to 1. Note: These bits are automatically reset when starting a new adtc command with data."]
    pub ac12: AC12,
    #[doc = "0x240 - Capabilities register This register lists the capabilities of the MMC/SD/SDIO host controller."]
    pub capa: CAPA,
    _reserved25: [u8; 4usize],
    #[doc = "0x248 - Maximum current capabilities Register This register indicates the maximum current capability for each voltage. The value is meaningful if the voltage support is set in the capabilities register (MMCHS_CAPA). Initialization of this register (via a write access to this register) depends on the system capabilities. The host driver shall not modify this register after the initilaization. This register is only reinitialized by a hard reset (via RESETN signal)"]
    pub cur_capa: CUR_CAPA,
    _reserved26: [u8; 4usize],
    #[doc = "0x250 - Force Event Register for Error Interrupt status The force Event Register is not a physically implemented register. Rather it is an address at which the Error Interrupt Status register can be written. The effect of a write to this address will be reflected in the Error Interrupt Status Register if corresponding bit of the Error Interrupt Status Enable Register is set."]
    pub fe: FE,
    #[doc = "0x254 - ADMA Error Status Register When ADMA Error Interrupt is occurred the ADMA Error States field in this register holds the ADMA state and the ADMA System Address Register holds the address around the error descriptor. For recovering the error the Host Driver requires the ADMA state to identify the error descriptor address as follows: ST_STOP: Previous location set in the ADMA System Address register is the error descriptor address ST_FDS: Current location set in the ADMA System Address register is the error descriptor address ST_CADR: This sate is never set because do not generate ADMA error in this state. ST_TFR: Previous location set in the ADMA System Address register is the error descriptor address In case of write operation the Host Driver should use ACMD22 to get the number of written block rather than using this information since unwritten data may exist in the Host Controller. The Host Controller generates the ADMA Error Interrupt when it detects invalid descriptor data (Valid=0) at the ST_FDS state. In this case ADMA Error State indicates that an error occurs at ST_FDS state. The Host Driver may find that the Valid bit is not set in the error descriptor."]
    pub admaes: ADMAES,
    #[doc = "0x258 - ADMA System address Low bits"]
    pub admasal: ADMASAL,
    _reserved29: [u8; 160usize],
    #[doc = "0x2fc - Versions Register This register contains the hard coded RTL vendor revision number the version number of SD specification compliancy and a slot status bit. MMCHS_REV\\[31:16\\]
= Host controller version MMCHS_REV\\[15:0\\]
= Slot Interrupt Status ****************************************************************************"]
    pub rev: REV,
}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_rev](hl_rev) module"]
pub type HL_REV = crate::Reg<u32, _HL_REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_REV;
#[doc = "`read()` method returns [hl_rev::R](hl_rev::R) reader structure"]
impl crate::Readable for HL_REV {}
#[doc = "`write(|w| ..)` method takes [hl_rev::W](hl_rev::W) writer structure"]
impl crate::Writable for HL_REV {}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility"]
pub mod hl_rev;
#[doc = "Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_hwinfo](hl_hwinfo) module"]
pub type HL_HWINFO = crate::Reg<u32, _HL_HWINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_HWINFO;
#[doc = "`read()` method returns [hl_hwinfo::R](hl_hwinfo::R) reader structure"]
impl crate::Readable for HL_HWINFO {}
#[doc = "`write(|w| ..)` method takes [hl_hwinfo::W](hl_hwinfo::W) writer structure"]
impl crate::Writable for HL_HWINFO {}
#[doc = "Information about the IP module's hardware configuration i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
pub mod hl_hwinfo;
#[doc = "Clock management configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hl_sysconfig](hl_sysconfig) module"]
pub type HL_SYSCONFIG = crate::Reg<u32, _HL_SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HL_SYSCONFIG;
#[doc = "`read()` method returns [hl_sysconfig::R](hl_sysconfig::R) reader structure"]
impl crate::Readable for HL_SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [hl_sysconfig::W](hl_sysconfig::W) writer structure"]
impl crate::Writable for HL_SYSCONFIG {}
#[doc = "Clock management configuration"]
pub mod hl_sysconfig;
#[doc = "System Configuration Register This register allows controlling various parameters of the OCP interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](sysconfig) module"]
pub type SYSCONFIG = crate::Reg<u32, _SYSCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCONFIG;
#[doc = "`read()` method returns [sysconfig::R](sysconfig::R) reader structure"]
impl crate::Readable for SYSCONFIG {}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](sysconfig::W) writer structure"]
impl crate::Writable for SYSCONFIG {}
#[doc = "System Configuration Register This register allows controlling various parameters of the OCP interface."]
pub mod sysconfig;
#[doc = "System Status Register This register provides status information about the module excluding the interrupt status information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysstatus](sysstatus) module"]
pub type SYSSTATUS = crate::Reg<u32, _SYSSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSTATUS;
#[doc = "`read()` method returns [sysstatus::R](sysstatus::R) reader structure"]
impl crate::Readable for SYSSTATUS {}
#[doc = "`write(|w| ..)` method takes [sysstatus::W](sysstatus::W) writer structure"]
impl crate::Writable for SYSSTATUS {}
#[doc = "System Status Register This register provides status information about the module excluding the interrupt status information"]
pub mod sysstatus;
#[doc = "Card status response error This register enables the host controller to detect card status errors of response type R1 R1b for all cards and of R5 R5b and R6 response for cards types SD or SDIO. When a bit MMCHS_CSRE\\[i\\]
is set to 1 if the corresponding bit at the same position in the response MMCHS_RSP0\\[i\\]
is set to 1 the host controller indicates a card error (MMCHS_STAT\\[CERR\\]) interrupt status to avoid the host driver reading the response register (MMCHS_RSP0). Note: No automatic card error detection for autoCMD12 is implemented; the host system has to check autoCMD12 response register (MMCHS_RESP76) for possible card errors.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csre](csre) module"]
pub type CSRE = crate::Reg<u32, _CSRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSRE;
#[doc = "`read()` method returns [csre::R](csre::R) reader structure"]
impl crate::Readable for CSRE {}
#[doc = "`write(|w| ..)` method takes [csre::W](csre::W) writer structure"]
impl crate::Writable for CSRE {}
#[doc = "Card status response error This register enables the host controller to detect card status errors of response type R1 R1b for all cards and of R5 R5b and R6 response for cards types SD or SDIO. When a bit MMCHS_CSRE\\[i\\]
is set to 1 if the corresponding bit at the same position in the response MMCHS_RSP0\\[i\\]
is set to 1 the host controller indicates a card error (MMCHS_STAT\\[CERR\\]) interrupt status to avoid the host driver reading the response register (MMCHS_RSP0). Note: No automatic card error detection for autoCMD12 is implemented; the host system has to check autoCMD12 response register (MMCHS_RESP76) for possible card errors."]
pub mod csre;
#[doc = "System Test register This register is used to control the signals that connect to I/O pins when the module is configured in system test (SYSTEST) mode for boundary connectivity verification. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer. The buffer behaves as a stack accessible only by the local host (push and pop operations). In this mode the Transfer Block Size (MMCHS_BLK\\[BLEN\\]) and the Blocks count for current transfer (MMCHS_BLK\\[NBLK\\]) are needed to generate a Buffer write ready interrupt (MMCHS_STAT\\[BWR\\]) or a Buffer read ready interrupt (MMCHS_STAT\\[BRR\\]) and DMA requests if enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systest](systest) module"]
pub type SYSTEST = crate::Reg<u32, _SYSTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEST;
#[doc = "`read()` method returns [systest::R](systest::R) reader structure"]
impl crate::Readable for SYSTEST {}
#[doc = "`write(|w| ..)` method takes [systest::W](systest::W) writer structure"]
impl crate::Writable for SYSTEST {}
#[doc = "System Test register This register is used to control the signals that connect to I/O pins when the module is configured in system test (SYSTEST) mode for boundary connectivity verification. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer. The buffer behaves as a stack accessible only by the local host (push and pop operations). In this mode the Transfer Block Size (MMCHS_BLK\\[BLEN\\]) and the Blocks count for current transfer (MMCHS_BLK\\[NBLK\\]) are needed to generate a Buffer write ready interrupt (MMCHS_STAT\\[BWR\\]) or a Buffer read ready interrupt (MMCHS_STAT\\[BRR\\]) and DMA requests if enabled."]
pub mod systest;
#[doc = "Configuration register This register is used: - to select the functional mode or the SYSTEST mode for any card. - to send an initialization sequence to any card. - to enable the detection on DAT\\[1\\]
of a card interrupt for SDIO cards only. and also to configure : - specific data and command transfers for MMC cards only. - the parameters related to the card detect and write protect input signals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`read()` method returns [con::R](con::R) reader structure"]
impl crate::Readable for CON {}
#[doc = "`write(|w| ..)` method takes [con::W](con::W) writer structure"]
impl crate::Writable for CON {}
#[doc = "Configuration register This register is used: - to select the functional mode or the SYSTEST mode for any card. - to send an initialization sequence to any card. - to enable the detection on DAT\\[1\\]
of a card interrupt for SDIO cards only. and also to configure : - specific data and command transfers for MMC cards only. - the parameters related to the card detect and write protect input signals."]
pub mod con;
#[doc = "Power counter register This register is used to program a mmc counter to delay command transfers after activating the PAD power this value depends on PAD characteristics and voltage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwcnt](pwcnt) module"]
pub type PWCNT = crate::Reg<u32, _PWCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWCNT;
#[doc = "`read()` method returns [pwcnt::R](pwcnt::R) reader structure"]
impl crate::Readable for PWCNT {}
#[doc = "`write(|w| ..)` method takes [pwcnt::W](pwcnt::W) writer structure"]
impl crate::Writable for PWCNT {}
#[doc = "Power counter register This register is used to program a mmc counter to delay command transfers after activating the PAD power this value depends on PAD characteristics and voltage."]
pub mod pwcnt;
#[doc = "Transfer Length Configuration register MMCHS_BLK\\[BLEN\\]
is the block size register. MMCHS_BLK\\[NBLK\\]
is the block count register. This register shall be used for any card.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk](blk) module"]
pub type BLK = crate::Reg<u32, _BLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK;
#[doc = "`read()` method returns [blk::R](blk::R) reader structure"]
impl crate::Readable for BLK {}
#[doc = "`write(|w| ..)` method takes [blk::W](blk::W) writer structure"]
impl crate::Writable for BLK {}
#[doc = "Transfer Length Configuration register MMCHS_BLK\\[BLEN\\]
is the block size register. MMCHS_BLK\\[NBLK\\]
is the block count register. This register shall be used for any card."]
pub mod blk;
#[doc = "Command argument Register This register contains command argument specified as bit 39-8 of Command-Format These registers must be initialized prior to sending the command itself to the card (write action into the register MMCHS_CMD register). Only exception is for a command index specifying stuff bits in arguments making a write unnecessary.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg](arg) module"]
pub type ARG = crate::Reg<u32, _ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG;
#[doc = "`read()` method returns [arg::R](arg::R) reader structure"]
impl crate::Readable for ARG {}
#[doc = "`write(|w| ..)` method takes [arg::W](arg::W) writer structure"]
impl crate::Writable for ARG {}
#[doc = "Command argument Register This register contains command argument specified as bit 39-8 of Command-Format These registers must be initialized prior to sending the command itself to the card (write action into the register MMCHS_CMD register). Only exception is for a command index specifying stuff bits in arguments making a write unnecessary."]
pub mod arg;
#[doc = "Command and transfer mode register MMCHS_CMD\\[31:16\\]
= the command register MMCHS_CMD\\[15:0\\]
= the transfer mode. This register configures the data and command transfers. A write into the most significant byte send the command. A write into MMCHS_CMD\\[15:0\\]
registers during data transfer has no effect. This register shall be used for any card. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command and transfer mode register MMCHS_CMD\\[31:16\\]
= the command register MMCHS_CMD\\[15:0\\]
= the transfer mode. This register configures the data and command transfers. A write into the most significant byte send the command. A write into MMCHS_CMD\\[15:0\\]
registers during data transfer has no effect. This register shall be used for any card. Note: In SYSTEST mode a write into MMCHS_CMD register will not start a transfer."]
pub mod cmd;
#[doc = "Command response\\[31:0\\]
Register This 32-bit register holds bits positions \\[31:0\\]
of command response type R1/R1b/R2/R3/R4/R5/R5b/R6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp10](rsp10) module"]
pub type RSP10 = crate::Reg<u32, _RSP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSP10;
#[doc = "`read()` method returns [rsp10::R](rsp10::R) reader structure"]
impl crate::Readable for RSP10 {}
#[doc = "`write(|w| ..)` method takes [rsp10::W](rsp10::W) writer structure"]
impl crate::Writable for RSP10 {}
#[doc = "Command response\\[31:0\\]
Register This 32-bit register holds bits positions \\[31:0\\]
of command response type R1/R1b/R2/R3/R4/R5/R5b/R6"]
pub mod rsp10;
#[doc = "Command response\\[63:32\\]
Register This 32-bit register holds bits positions \\[63:32\\]
of command response type R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp32](rsp32) module"]
pub type RSP32 = crate::Reg<u32, _RSP32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSP32;
#[doc = "`read()` method returns [rsp32::R](rsp32::R) reader structure"]
impl crate::Readable for RSP32 {}
#[doc = "`write(|w| ..)` method takes [rsp32::W](rsp32::W) writer structure"]
impl crate::Writable for RSP32 {}
#[doc = "Command response\\[63:32\\]
Register This 32-bit register holds bits positions \\[63:32\\]
of command response type R2"]
pub mod rsp32;
#[doc = "Command response\\[95:64\\]
Register This 32-bit register holds bits positions \\[95:64\\]
of command response type R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp54](rsp54) module"]
pub type RSP54 = crate::Reg<u32, _RSP54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSP54;
#[doc = "`read()` method returns [rsp54::R](rsp54::R) reader structure"]
impl crate::Readable for RSP54 {}
#[doc = "`write(|w| ..)` method takes [rsp54::W](rsp54::W) writer structure"]
impl crate::Writable for RSP54 {}
#[doc = "Command response\\[95:64\\]
Register This 32-bit register holds bits positions \\[95:64\\]
of command response type R2"]
pub mod rsp54;
#[doc = "Command response\\[127:96\\]
Register This 32-bit register holds bits positions \\[127:96\\]
of command response type R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsp76](rsp76) module"]
pub type RSP76 = crate::Reg<u32, _RSP76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSP76;
#[doc = "`read()` method returns [rsp76::R](rsp76::R) reader structure"]
impl crate::Readable for RSP76 {}
#[doc = "`write(|w| ..)` method takes [rsp76::W](rsp76::W) writer structure"]
impl crate::Writable for RSP76 {}
#[doc = "Command response\\[127:96\\]
Register This 32-bit register holds bits positions \\[127:96\\]
of command response type R2"]
pub mod rsp76;
#[doc = "Data Register This register is the 32-bit entry point of the buffer for read or write data transfers. The buffer size is 32bits x256(1024 bytes). Bytes within a word are stored and read in little endian format. This buffer can be used as two 512 byte buffers to transfer data efficiently without reducing the throughput. Sequential and contiguous access is necessary to increment the pointer correctly. Random or skipped access is not allowed. In little endian if the local host accesses this register byte-wise or 16bit-wise the least significant byte (bits \\[7:0\\]) must always be written/read first. The update of the buffer address is done on the most significant byte write for full 32-bit DATA register or on the most significant byte of the last word of block transfer. Example 1: Byte or 16-bit access Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1100 (2-bytes) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=0100 (1-byte) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1000 (1-byte) Bad\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data Register This register is the 32-bit entry point of the buffer for read or write data transfers. The buffer size is 32bits x256(1024 bytes). Bytes within a word are stored and read in little endian format. This buffer can be used as two 512 byte buffers to transfer data efficiently without reducing the throughput. Sequential and contiguous access is necessary to increment the pointer correctly. Random or skipped access is not allowed. In little endian if the local host accesses this register byte-wise or 16bit-wise the least significant byte (bits \\[7:0\\]) must always be written/read first. The update of the buffer address is done on the most significant byte write for full 32-bit DATA register or on the most significant byte of the last word of block transfer. Example 1: Byte or 16-bit access Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1100 (2-bytes) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=0100 (1-byte) OK Mbyteen\\[3:0\\]=0001 (1-byte) => Mbyteen\\[3:0\\]=0010 (1-byte) => Mbyteen\\[3:0\\]=1000 (1-byte) Bad"]
pub mod data;
#[doc = "Present state register The Host can get status of the Host Controller from this 32-bit read only register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstate](pstate) module"]
pub type PSTATE = crate::Reg<u32, _PSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSTATE;
#[doc = "`read()` method returns [pstate::R](pstate::R) reader structure"]
impl crate::Readable for PSTATE {}
#[doc = "`write(|w| ..)` method takes [pstate::W](pstate::W) writer structure"]
impl crate::Writable for PSTATE {}
#[doc = "Present state register The Host can get status of the Host Controller from this 32-bit read only register."]
pub mod pstate;
#[doc = "Control register This register defines the host controls to set power wakeup and transfer parameters. MMCHS_HCTL\\[31:24\\]
= Wakeup control MMCHS_HCTL\\[23:16\\]
= Block gap control MMCHS_HCTL\\[15:8\\]
= Power control MMCHS_HCTL\\[7:0\\]
= Host control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctl](hctl) module"]
pub type HCTL = crate::Reg<u32, _HCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTL;
#[doc = "`read()` method returns [hctl::R](hctl::R) reader structure"]
impl crate::Readable for HCTL {}
#[doc = "`write(|w| ..)` method takes [hctl::W](hctl::W) writer structure"]
impl crate::Writable for HCTL {}
#[doc = "Control register This register defines the host controls to set power wakeup and transfer parameters. MMCHS_HCTL\\[31:24\\]
= Wakeup control MMCHS_HCTL\\[23:16\\]
= Block gap control MMCHS_HCTL\\[15:8\\]
= Power control MMCHS_HCTL\\[7:0\\]
= Host control"]
pub mod hctl;
#[doc = "SD system control register This register defines the system controls to set software resets clock frequency management and data timeout. MMCHS_SYSCTL\\[31:24\\]
= Software resets MMCHS_SYSCTL\\[23:16\\]
= Timeout control MMCHS_SYSCTL\\[15:0\\]
= Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](sysctl) module"]
pub type SYSCTL = crate::Reg<u32, _SYSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTL;
#[doc = "`read()` method returns [sysctl::R](sysctl::R) reader structure"]
impl crate::Readable for SYSCTL {}
#[doc = "`write(|w| ..)` method takes [sysctl::W](sysctl::W) writer structure"]
impl crate::Writable for SYSCTL {}
#[doc = "SD system control register This register defines the system controls to set software resets clock frequency management and data timeout. MMCHS_SYSCTL\\[31:24\\]
= Software resets MMCHS_SYSCTL\\[23:16\\]
= Timeout control MMCHS_SYSCTL\\[15:0\\]
= Clock control"]
pub mod sysctl;
#[doc = "Interrupt status register The interrupt status regroups all the status of the module internal events that can generate an interrupt. MMCHS_STAT\\[31:16\\]
= Error Interrupt Status MMCHS_STAT\\[15:0\\]
= Normal Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Interrupt status register The interrupt status regroups all the status of the module internal events that can generate an interrupt. MMCHS_STAT\\[31:16\\]
= Error Interrupt Status MMCHS_STAT\\[15:0\\]
= Normal Interrupt Status"]
pub mod stat;
#[doc = "Interrupt SD enable register This register allows to enable/disable the module to set status bits on an event-by-event basis. MMCHS_IE\\[31:16\\]
= Error Interrupt Status Enable MMCHS_IE\\[15:0\\]
= Normal Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt SD enable register This register allows to enable/disable the module to set status bits on an event-by-event basis. MMCHS_IE\\[31:16\\]
= Error Interrupt Status Enable MMCHS_IE\\[15:0\\]
= Normal Interrupt Status Enable"]
pub mod ie;
#[doc = "Interrupt signal enable register This register allows to enable/disable the module internal sources of status on an event-by-event basis. MMCHS_ISE\\[31:16\\]
= Error Interrupt Signal Enable MMCHS_ISE\\[15:0\\]
= Normal Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ise](ise) module"]
pub type ISE = crate::Reg<u32, _ISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISE;
#[doc = "`read()` method returns [ise::R](ise::R) reader structure"]
impl crate::Readable for ISE {}
#[doc = "`write(|w| ..)` method takes [ise::W](ise::W) writer structure"]
impl crate::Writable for ISE {}
#[doc = "Interrupt signal enable register This register allows to enable/disable the module internal sources of status on an event-by-event basis. MMCHS_ISE\\[31:16\\]
= Error Interrupt Signal Enable MMCHS_ISE\\[15:0\\]
= Normal Interrupt Signal Enable"]
pub mod ise;
#[doc = "Auto CMD12 Error Status Register The host driver may determine which of the errors cases related to Auto CMD12 has occurred by checking this MMCHS_AC12 register when an Auto CMD12 Error interrupt occurs. This register is valid only when Auto CMD12 is enabled (MMCHS_CMD\\[ACEN\\]) and Auto CMD12Error (MMCHS_STAT\\[ACE\\]) is set to 1. Note: These bits are automatically reset when starting a new adtc command with data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac12](ac12) module"]
pub type AC12 = crate::Reg<u32, _AC12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AC12;
#[doc = "`read()` method returns [ac12::R](ac12::R) reader structure"]
impl crate::Readable for AC12 {}
#[doc = "`write(|w| ..)` method takes [ac12::W](ac12::W) writer structure"]
impl crate::Writable for AC12 {}
#[doc = "Auto CMD12 Error Status Register The host driver may determine which of the errors cases related to Auto CMD12 has occurred by checking this MMCHS_AC12 register when an Auto CMD12 Error interrupt occurs. This register is valid only when Auto CMD12 is enabled (MMCHS_CMD\\[ACEN\\]) and Auto CMD12Error (MMCHS_STAT\\[ACE\\]) is set to 1. Note: These bits are automatically reset when starting a new adtc command with data."]
pub mod ac12;
#[doc = "Capabilities register This register lists the capabilities of the MMC/SD/SDIO host controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capa](capa) module"]
pub type CAPA = crate::Reg<u32, _CAPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPA;
#[doc = "`read()` method returns [capa::R](capa::R) reader structure"]
impl crate::Readable for CAPA {}
#[doc = "`write(|w| ..)` method takes [capa::W](capa::W) writer structure"]
impl crate::Writable for CAPA {}
#[doc = "Capabilities register This register lists the capabilities of the MMC/SD/SDIO host controller."]
pub mod capa;
#[doc = "Maximum current capabilities Register This register indicates the maximum current capability for each voltage. The value is meaningful if the voltage support is set in the capabilities register (MMCHS_CAPA). Initialization of this register (via a write access to this register) depends on the system capabilities. The host driver shall not modify this register after the initilaization. This register is only reinitialized by a hard reset (via RESETN signal)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cur_capa](cur_capa) module"]
pub type CUR_CAPA = crate::Reg<u32, _CUR_CAPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUR_CAPA;
#[doc = "`read()` method returns [cur_capa::R](cur_capa::R) reader structure"]
impl crate::Readable for CUR_CAPA {}
#[doc = "`write(|w| ..)` method takes [cur_capa::W](cur_capa::W) writer structure"]
impl crate::Writable for CUR_CAPA {}
#[doc = "Maximum current capabilities Register This register indicates the maximum current capability for each voltage. The value is meaningful if the voltage support is set in the capabilities register (MMCHS_CAPA). Initialization of this register (via a write access to this register) depends on the system capabilities. The host driver shall not modify this register after the initilaization. This register is only reinitialized by a hard reset (via RESETN signal)"]
pub mod cur_capa;
#[doc = "Force Event Register for Error Interrupt status The force Event Register is not a physically implemented register. Rather it is an address at which the Error Interrupt Status register can be written. The effect of a write to this address will be reflected in the Error Interrupt Status Register if corresponding bit of the Error Interrupt Status Enable Register is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fe](fe) module"]
pub type FE = crate::Reg<u32, _FE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FE;
#[doc = "`read()` method returns [fe::R](fe::R) reader structure"]
impl crate::Readable for FE {}
#[doc = "`write(|w| ..)` method takes [fe::W](fe::W) writer structure"]
impl crate::Writable for FE {}
#[doc = "Force Event Register for Error Interrupt status The force Event Register is not a physically implemented register. Rather it is an address at which the Error Interrupt Status register can be written. The effect of a write to this address will be reflected in the Error Interrupt Status Register if corresponding bit of the Error Interrupt Status Enable Register is set."]
pub mod fe;
#[doc = "ADMA Error Status Register When ADMA Error Interrupt is occurred the ADMA Error States field in this register holds the ADMA state and the ADMA System Address Register holds the address around the error descriptor. For recovering the error the Host Driver requires the ADMA state to identify the error descriptor address as follows: ST_STOP: Previous location set in the ADMA System Address register is the error descriptor address ST_FDS: Current location set in the ADMA System Address register is the error descriptor address ST_CADR: This sate is never set because do not generate ADMA error in this state. ST_TFR: Previous location set in the ADMA System Address register is the error descriptor address In case of write operation the Host Driver should use ACMD22 to get the number of written block rather than using this information since unwritten data may exist in the Host Controller. The Host Controller generates the ADMA Error Interrupt when it detects invalid descriptor data (Valid=0) at the ST_FDS state. In this case ADMA Error State indicates that an error occurs at ST_FDS state. The Host Driver may find that the Valid bit is not set in the error descriptor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admaes](admaes) module"]
pub type ADMAES = crate::Reg<u32, _ADMAES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMAES;
#[doc = "`read()` method returns [admaes::R](admaes::R) reader structure"]
impl crate::Readable for ADMAES {}
#[doc = "`write(|w| ..)` method takes [admaes::W](admaes::W) writer structure"]
impl crate::Writable for ADMAES {}
#[doc = "ADMA Error Status Register When ADMA Error Interrupt is occurred the ADMA Error States field in this register holds the ADMA state and the ADMA System Address Register holds the address around the error descriptor. For recovering the error the Host Driver requires the ADMA state to identify the error descriptor address as follows: ST_STOP: Previous location set in the ADMA System Address register is the error descriptor address ST_FDS: Current location set in the ADMA System Address register is the error descriptor address ST_CADR: This sate is never set because do not generate ADMA error in this state. ST_TFR: Previous location set in the ADMA System Address register is the error descriptor address In case of write operation the Host Driver should use ACMD22 to get the number of written block rather than using this information since unwritten data may exist in the Host Controller. The Host Controller generates the ADMA Error Interrupt when it detects invalid descriptor data (Valid=0) at the ST_FDS state. In this case ADMA Error State indicates that an error occurs at ST_FDS state. The Host Driver may find that the Valid bit is not set in the error descriptor."]
pub mod admaes;
#[doc = "ADMA System address Low bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admasal](admasal) module"]
pub type ADMASAL = crate::Reg<u32, _ADMASAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMASAL;
#[doc = "`read()` method returns [admasal::R](admasal::R) reader structure"]
impl crate::Readable for ADMASAL {}
#[doc = "`write(|w| ..)` method takes [admasal::W](admasal::W) writer structure"]
impl crate::Writable for ADMASAL {}
#[doc = "ADMA System address Low bits"]
pub mod admasal;
#[doc = "Versions Register This register contains the hard coded RTL vendor revision number the version number of SD specification compliancy and a slot status bit. MMCHS_REV\\[31:16\\]
= Host controller version MMCHS_REV\\[15:0\\]
= Slot Interrupt Status ****************************************************************************\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rev](rev) module"]
pub type REV = crate::Reg<u32, _REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REV;
#[doc = "`read()` method returns [rev::R](rev::R) reader structure"]
impl crate::Readable for REV {}
#[doc = "`write(|w| ..)` method takes [rev::W](rev::W) writer structure"]
impl crate::Writable for REV {}
#[doc = "Versions Register This register contains the hard coded RTL vendor revision number the version number of SD specification compliancy and a slot status bit. MMCHS_REV\\[31:16\\]
= Host controller version MMCHS_REV\\[15:0\\]
= Slot Interrupt Status ****************************************************************************"]
pub mod rev;
