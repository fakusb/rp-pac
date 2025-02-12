#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio(pub *mut u8);
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[doc = "GPIO status"]
    #[inline(always)]
    pub fn status(self) -> crate::common::Reg<regs::GpioStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    #[inline(always)]
    pub fn ctrl(self) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub *mut u8);
unsafe impl Send for Int {}
unsafe impl Sync for Int {}
impl Int {
    #[doc = "Interrupt Enable for proc1"]
    #[inline(always)]
    pub fn inte(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    #[inline(always)]
    pub fn intf(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize + n * 4usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    #[inline(always)]
    pub fn ints(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io(pub *mut u8);
unsafe impl Send for Io {}
unsafe impl Sync for Io {}
impl Io {
    #[inline(always)]
    pub fn gpio(self, n: usize) -> Gpio {
        assert!(n < 30usize);
        unsafe { Gpio(self.0.add(0usize + n * 8usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize + n * 4usize)) }
    }
    #[inline(always)]
    pub fn int_proc(self, n: usize) -> Int {
        assert!(n < 2usize);
        unsafe { Int(self.0.add(256usize + n * 48usize)) }
    }
    #[inline(always)]
    pub fn int_dormant_wake(self) -> Int {
        unsafe { Int(self.0.add(352usize)) }
    }
}
pub mod regs;
pub mod vals;
