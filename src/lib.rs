#![doc = "Peripheral access API for CC3220SF microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 8;
#[cfg(feature = "rt")]
extern "C" {
    fn INTERRUPT_GPIOA0();
    fn INTERRUPT_GPIOA1();
    fn INTERRUPT_GPIOA2();
    fn INTERRUPT_GPIOA3();
    fn INTERRUPT_UART0();
    fn INTERRUPT_UART1();
    fn INTERRUPT_I2C();
    fn INTERRUPT_ADC0();
    fn INTERRUPT_ADC1();
    fn INTERRUPT_ADC2();
    fn INTERRUPT_ADC3();
    fn INTERRUPT_WDT();
    fn INTERRUPT_TIMERA0A();
    fn INTERRUPT_TIMERA0B();
    fn INTERRUPT_TIMERA1A();
    fn INTERRUPT_TIMERA1B();
    fn INTERRUPT_TIMERA2A();
    fn INTERRUPT_TIMERA2B();
    fn INTERRUPT_TIMERA3A();
    fn INTERRUPT_TIMERA3B();
    fn INTERRUPT_UDMA();
    fn INTERRUPT_I2S();
    fn INTERRUPT_CAM();
    fn INTERRUPT_RAMWR();
    fn INTERRUPT_NET();
    fn INTERRUPT_SPI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 193] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_GPIOA0,
    },
    Vector {
        _handler: INTERRUPT_GPIOA1,
    },
    Vector {
        _handler: INTERRUPT_GPIOA2,
    },
    Vector {
        _handler: INTERRUPT_GPIOA3,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_UART0,
    },
    Vector {
        _handler: INTERRUPT_UART1,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_I2C,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_ADC0,
    },
    Vector {
        _handler: INTERRUPT_ADC1,
    },
    Vector {
        _handler: INTERRUPT_ADC2,
    },
    Vector {
        _handler: INTERRUPT_ADC3,
    },
    Vector {
        _handler: INTERRUPT_WDT,
    },
    Vector {
        _handler: INTERRUPT_TIMERA0A,
    },
    Vector {
        _handler: INTERRUPT_TIMERA0B,
    },
    Vector {
        _handler: INTERRUPT_TIMERA1A,
    },
    Vector {
        _handler: INTERRUPT_TIMERA1B,
    },
    Vector {
        _handler: INTERRUPT_TIMERA2A,
    },
    Vector {
        _handler: INTERRUPT_TIMERA2B,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_TIMERA3A,
    },
    Vector {
        _handler: INTERRUPT_TIMERA3B,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_UDMA,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_I2S,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_CAM,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_RAMWR,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_NET,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INTERRUPT_SPI,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "16 - GPIO port S0"]
    INTERRUPT_GPIOA0 = 16,
    #[doc = "17 - GPIO port S1"]
    INTERRUPT_GPIOA1 = 17,
    #[doc = "18 - GPIO port S2"]
    INTERRUPT_GPIOA2 = 18,
    #[doc = "19 - GPIO port S3"]
    INTERRUPT_GPIOA3 = 19,
    #[doc = "21 - UART0 Rx and Tx"]
    INTERRUPT_UART0 = 21,
    #[doc = "22 - UART1 Rx and Tx"]
    INTERRUPT_UART1 = 22,
    #[doc = "24 - I2C controller"]
    INTERRUPT_I2C = 24,
    #[doc = "30 - ADC Channel 0"]
    INTERRUPT_ADC0 = 30,
    #[doc = "31 - ADC Channel 1"]
    INTERRUPT_ADC1 = 31,
    #[doc = "32 - ADC Channel 2"]
    INTERRUPT_ADC2 = 32,
    #[doc = "33 - ADC Channel 3"]
    INTERRUPT_ADC3 = 33,
    #[doc = "34 - Watchdog Timer"]
    INTERRUPT_WDT = 34,
    #[doc = "35 - 16/32-Bit General Purpose Timer A0A"]
    INTERRUPT_TIMERA0A = 35,
    #[doc = "36 - 16/32-Bit General Purpose Timer A0B"]
    INTERRUPT_TIMERA0B = 36,
    #[doc = "37 - 16/32-Bit General Purpose Timer A1A"]
    INTERRUPT_TIMERA1A = 37,
    #[doc = "38 - 16/32-Bit General Purpose Timer A1B"]
    INTERRUPT_TIMERA1B = 38,
    #[doc = "39 - 16/32-Bit General Purpose Timer A2A"]
    INTERRUPT_TIMERA2A = 39,
    #[doc = "40 - 16/32-Bit General Purpose Timer A2B"]
    INTERRUPT_TIMERA2B = 40,
    #[doc = "51 - 16/32-Bit General Purpose Timer A3A"]
    INTERRUPT_TIMERA3A = 51,
    #[doc = "52 - 16/32-Bit General Purpose Timer A3B"]
    INTERRUPT_TIMERA3B = 52,
    #[doc = "62 - uDMA Software Interrupt"]
    INTERRUPT_UDMA = 62,
    #[doc = "177 - I2S controller"]
    INTERRUPT_I2S = 177,
    #[doc = "179 - Camera"]
    INTERRUPT_CAM = 179,
    #[doc = "184 - RAM WR Error"]
    INTERRUPT_RAMWR = 184,
    #[doc = "187 - Network Interrupt"]
    INTERRUPT_NET = 187,
    #[doc = "192 - SPI Interrupt"]
    INTERRUPT_SPI = 192,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "WDT"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "WDT"]
pub mod wdt;
#[doc = "GPIOA0"]
pub struct GPIOA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA0 {}
impl GPIOA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for GPIOA0 {
    type Target = gpioa0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA0::ptr() }
    }
}
#[doc = "GPIOA0"]
pub mod gpioa0;
#[doc = "GPIOA1"]
pub struct GPIOA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA1 {}
impl GPIOA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa1::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for GPIOA1 {
    type Target = gpioa1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA1::ptr() }
    }
}
#[doc = "GPIOA1"]
pub mod gpioa1;
#[doc = "GPIOA2"]
pub struct GPIOA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA2 {}
impl GPIOA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa2::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for GPIOA2 {
    type Target = gpioa2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA2::ptr() }
    }
}
#[doc = "GPIOA2"]
pub mod gpioa2;
#[doc = "GPIOA3"]
pub struct GPIOA3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA3 {}
impl GPIOA3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa3::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for GPIOA3 {
    type Target = gpioa3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA3::ptr() }
    }
}
#[doc = "GPIOA3"]
pub mod gpioa3;
#[doc = "UARTA0"]
pub struct UARTA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTA0 {}
impl UARTA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarta0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for UARTA0 {
    type Target = uarta0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTA0::ptr() }
    }
}
#[doc = "UARTA0"]
pub mod uarta0;
#[doc = "UARTA1"]
pub struct UARTA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTA1 {}
impl UARTA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarta1::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for UARTA1 {
    type Target = uarta1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTA1::ptr() }
    }
}
#[doc = "UARTA1"]
pub mod uarta1;
#[doc = "I2CA0"]
pub struct I2CA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CA0 {}
impl I2CA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2ca0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for I2CA0 {
    type Target = i2ca0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2CA0::ptr() }
    }
}
#[doc = "I2CA0"]
pub mod i2ca0;
#[doc = "GPIOA4"]
pub struct GPIOA4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA4 {}
impl GPIOA4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa4::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for GPIOA4 {
    type Target = gpioa4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA4::ptr() }
    }
}
#[doc = "GPIOA4"]
pub mod gpioa4;
#[doc = "TIMERA0"]
pub struct TIMERA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERA0 {}
impl TIMERA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timera0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for TIMERA0 {
    type Target = timera0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMERA0::ptr() }
    }
}
#[doc = "TIMERA0"]
pub mod timera0;
#[doc = "TIMERA1"]
pub struct TIMERA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERA1 {}
impl TIMERA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timera1::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for TIMERA1 {
    type Target = timera1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMERA1::ptr() }
    }
}
#[doc = "TIMERA1"]
pub mod timera1;
#[doc = "TIMERA2"]
pub struct TIMERA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERA2 {}
impl TIMERA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timera2::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for TIMERA2 {
    type Target = timera2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMERA2::ptr() }
    }
}
#[doc = "TIMERA2"]
pub mod timera2;
#[doc = "TIMERA3"]
pub struct TIMERA3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMERA3 {}
impl TIMERA3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timera3::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for TIMERA3 {
    type Target = timera3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMERA3::ptr() }
    }
}
#[doc = "TIMERA3"]
pub mod timera3;
#[doc = "STACKDIE_CTRL"]
pub struct STACKDIE_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STACKDIE_CTRL {}
impl STACKDIE_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stackdie_ctrl::RegisterBlock {
        0x400f_5000 as *const _
    }
}
impl Deref for STACKDIE_CTRL {
    type Target = stackdie_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STACKDIE_CTRL::ptr() }
    }
}
#[doc = "STACKDIE_CTRL"]
pub mod stackdie_ctrl;
#[doc = "COMMON_REG"]
pub struct COMMON_REG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMMON_REG {}
impl COMMON_REG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const common_reg::RegisterBlock {
        0x400f_7000 as *const _
    }
}
impl Deref for COMMON_REG {
    type Target = common_reg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMMON_REG::ptr() }
    }
}
#[doc = "COMMON_REG"]
pub mod common_reg;
#[doc = "FLASH_CONTROL"]
pub struct FLASH_CONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CONTROL {}
impl FLASH_CONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_control::RegisterBlock {
        0x400f_d000 as *const _
    }
}
impl Deref for FLASH_CONTROL {
    type Target = flash_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CONTROL::ptr() }
    }
}
#[doc = "FLASH_CONTROL"]
pub mod flash_control;
#[doc = "SYSTEM_CONTROL"]
pub struct SYSTEM_CONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM_CONTROL {}
impl SYSTEM_CONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0x400f_e000 as *const _
    }
}
impl Deref for SYSTEM_CONTROL {
    type Target = system_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEM_CONTROL::ptr() }
    }
}
#[doc = "SYSTEM_CONTROL"]
pub mod system_control;
#[doc = "UDMA"]
pub struct UDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA {}
impl UDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for UDMA {
    type Target = udma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA::ptr() }
    }
}
#[doc = "UDMA"]
pub mod udma;
#[doc = "CAMERA"]
pub struct CAMERA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAMERA {}
impl CAMERA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const camera::RegisterBlock {
        0x4401_8000 as *const _
    }
}
impl Deref for CAMERA {
    type Target = camera::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAMERA::ptr() }
    }
}
#[doc = "CAMERA"]
pub mod camera;
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x4401_c000 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "SSPI"]
pub struct SSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSPI {}
impl SSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sspi::RegisterBlock {
        0x4402_0000 as *const _
    }
}
impl Deref for SSPI {
    type Target = sspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSPI::ptr() }
    }
}
#[doc = "SSPI"]
pub mod sspi;
#[doc = "GSPI"]
pub struct GSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GSPI {}
impl GSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gspi::RegisterBlock {
        0x4402_1000 as *const _
    }
}
impl Deref for GSPI {
    type Target = gspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GSPI::ptr() }
    }
}
#[doc = "GSPI"]
pub mod gspi;
#[doc = "ARCM"]
pub struct ARCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ARCM {}
impl ARCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const arcm::RegisterBlock {
        0x4402_5000 as *const _
    }
}
impl Deref for ARCM {
    type Target = arcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ARCM::ptr() }
    }
}
#[doc = "ARCM"]
pub mod arcm;
#[doc = "APPS_CONFIG"]
pub struct APPS_CONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APPS_CONFIG {}
impl APPS_CONFIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apps_config::RegisterBlock {
        0x4402_6000 as *const _
    }
}
impl Deref for APPS_CONFIG {
    type Target = apps_config::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*APPS_CONFIG::ptr() }
    }
}
#[doc = "APPS_CONFIG"]
pub mod apps_config;
#[doc = "GPRCM"]
pub struct GPRCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPRCM {}
impl GPRCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gprcm::RegisterBlock {
        0x4402_d000 as *const _
    }
}
impl Deref for GPRCM {
    type Target = gprcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPRCM::ptr() }
    }
}
#[doc = "GPRCM"]
pub mod gprcm;
#[doc = "OCP_SHARED"]
pub struct OCP_SHARED {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCP_SHARED {}
impl OCP_SHARED {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocp_shared::RegisterBlock {
        0x4402_e000 as *const _
    }
}
impl Deref for OCP_SHARED {
    type Target = ocp_shared::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCP_SHARED::ptr() }
    }
}
#[doc = "OCP_SHARED"]
pub mod ocp_shared;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4402_e800 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "HIB1P2"]
pub struct HIB1P2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HIB1P2 {}
impl HIB1P2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hib1p2::RegisterBlock {
        0x4402_f000 as *const _
    }
}
impl Deref for HIB1P2 {
    type Target = hib1p2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HIB1P2::ptr() }
    }
}
#[doc = "HIB1P2"]
pub mod hib1p2;
#[doc = "HIB3P3"]
pub struct HIB3P3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HIB3P3 {}
impl HIB3P3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hib3p3::RegisterBlock {
        0x4402_f800 as *const _
    }
}
impl Deref for HIB3P3 {
    type Target = hib3p3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HIB3P3::ptr() }
    }
}
#[doc = "HIB3P3"]
pub mod hib3p3;
#[doc = "DTHE"]
pub struct DTHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DTHE {}
impl DTHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dthe::RegisterBlock {
        0x4403_0000 as *const _
    }
}
impl Deref for DTHE {
    type Target = dthe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DTHE::ptr() }
    }
}
#[doc = "DTHE"]
pub mod dthe;
#[doc = "SHAMD5"]
pub struct SHAMD5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHAMD5 {}
impl SHAMD5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const shamd5::RegisterBlock {
        0x4403_5000 as *const _
    }
}
impl Deref for SHAMD5 {
    type Target = shamd5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHAMD5::ptr() }
    }
}
#[doc = "SHAMD5"]
pub mod shamd5;
#[doc = "AES"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4403_7000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "AES"]
pub mod aes;
#[doc = "DES"]
pub struct DES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DES {}
impl DES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const des::RegisterBlock {
        0x4403_9000 as *const _
    }
}
impl Deref for DES {
    type Target = des::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DES::ptr() }
    }
}
#[doc = "DES"]
pub mod des;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "GPIOA0"]
    pub GPIOA0: GPIOA0,
    #[doc = "GPIOA1"]
    pub GPIOA1: GPIOA1,
    #[doc = "GPIOA2"]
    pub GPIOA2: GPIOA2,
    #[doc = "GPIOA3"]
    pub GPIOA3: GPIOA3,
    #[doc = "UARTA0"]
    pub UARTA0: UARTA0,
    #[doc = "UARTA1"]
    pub UARTA1: UARTA1,
    #[doc = "I2CA0"]
    pub I2CA0: I2CA0,
    #[doc = "GPIOA4"]
    pub GPIOA4: GPIOA4,
    #[doc = "TIMERA0"]
    pub TIMERA0: TIMERA0,
    #[doc = "TIMERA1"]
    pub TIMERA1: TIMERA1,
    #[doc = "TIMERA2"]
    pub TIMERA2: TIMERA2,
    #[doc = "TIMERA3"]
    pub TIMERA3: TIMERA3,
    #[doc = "STACKDIE_CTRL"]
    pub STACKDIE_CTRL: STACKDIE_CTRL,
    #[doc = "COMMON_REG"]
    pub COMMON_REG: COMMON_REG,
    #[doc = "FLASH_CONTROL"]
    pub FLASH_CONTROL: FLASH_CONTROL,
    #[doc = "SYSTEM_CONTROL"]
    pub SYSTEM_CONTROL: SYSTEM_CONTROL,
    #[doc = "UDMA"]
    pub UDMA: UDMA,
    #[doc = "CAMERA"]
    pub CAMERA: CAMERA,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "SSPI"]
    pub SSPI: SSPI,
    #[doc = "GSPI"]
    pub GSPI: GSPI,
    #[doc = "ARCM"]
    pub ARCM: ARCM,
    #[doc = "APPS_CONFIG"]
    pub APPS_CONFIG: APPS_CONFIG,
    #[doc = "GPRCM"]
    pub GPRCM: GPRCM,
    #[doc = "OCP_SHARED"]
    pub OCP_SHARED: OCP_SHARED,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "HIB1P2"]
    pub HIB1P2: HIB1P2,
    #[doc = "HIB3P3"]
    pub HIB3P3: HIB3P3,
    #[doc = "DTHE"]
    pub DTHE: DTHE,
    #[doc = "SHAMD5"]
    pub SHAMD5: SHAMD5,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "DES"]
    pub DES: DES,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WDT: WDT {
                _marker: PhantomData,
            },
            GPIOA0: GPIOA0 {
                _marker: PhantomData,
            },
            GPIOA1: GPIOA1 {
                _marker: PhantomData,
            },
            GPIOA2: GPIOA2 {
                _marker: PhantomData,
            },
            GPIOA3: GPIOA3 {
                _marker: PhantomData,
            },
            UARTA0: UARTA0 {
                _marker: PhantomData,
            },
            UARTA1: UARTA1 {
                _marker: PhantomData,
            },
            I2CA0: I2CA0 {
                _marker: PhantomData,
            },
            GPIOA4: GPIOA4 {
                _marker: PhantomData,
            },
            TIMERA0: TIMERA0 {
                _marker: PhantomData,
            },
            TIMERA1: TIMERA1 {
                _marker: PhantomData,
            },
            TIMERA2: TIMERA2 {
                _marker: PhantomData,
            },
            TIMERA3: TIMERA3 {
                _marker: PhantomData,
            },
            STACKDIE_CTRL: STACKDIE_CTRL {
                _marker: PhantomData,
            },
            COMMON_REG: COMMON_REG {
                _marker: PhantomData,
            },
            FLASH_CONTROL: FLASH_CONTROL {
                _marker: PhantomData,
            },
            SYSTEM_CONTROL: SYSTEM_CONTROL {
                _marker: PhantomData,
            },
            UDMA: UDMA {
                _marker: PhantomData,
            },
            CAMERA: CAMERA {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            SSPI: SSPI {
                _marker: PhantomData,
            },
            GSPI: GSPI {
                _marker: PhantomData,
            },
            ARCM: ARCM {
                _marker: PhantomData,
            },
            APPS_CONFIG: APPS_CONFIG {
                _marker: PhantomData,
            },
            GPRCM: GPRCM {
                _marker: PhantomData,
            },
            OCP_SHARED: OCP_SHARED {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            HIB1P2: HIB1P2 {
                _marker: PhantomData,
            },
            HIB3P3: HIB3P3 {
                _marker: PhantomData,
            },
            DTHE: DTHE {
                _marker: PhantomData,
            },
            SHAMD5: SHAMD5 {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            DES: DES {
                _marker: PhantomData,
            },
        }
    }
}
