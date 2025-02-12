#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Watchdog(pub *mut u8);
unsafe impl Send for Watchdog {}
unsafe impl Sync for Watchdog {}
impl Watchdog {
    #[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
    #[inline(always)]
    pub fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    #[inline(always)]
    pub fn load(self) -> crate::common::Reg<regs::Load, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    #[inline(always)]
    pub fn reason(self) -> crate::common::Reg<regs::Reason, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub fn scratch7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub fn tick(self) -> crate::common::Reg<regs::Tick, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
}
pub mod regs;
