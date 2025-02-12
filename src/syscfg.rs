#[doc = "Register block for various chip control signals"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg(pub *mut u8);
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[doc = "Processor core 0 NMI source mask Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    pub fn proc0_nmi_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Processor core 1 NMI source mask Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    pub fn proc1_nmi_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Configuration for processors"]
    #[inline(always)]
    pub fn proc_config(self) -> crate::common::Reg<regs::ProcConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
    #[inline(always)]
    pub fn proc_in_sync_bypass(
        self,
    ) -> crate::common::Reg<regs::ProcInSyncBypass, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    #[inline(always)]
    pub fn proc_in_sync_bypass_hi(
        self,
    ) -> crate::common::Reg<regs::ProcInSyncBypassHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Directly control the SWD debug port of either processor"]
    #[inline(always)]
    pub fn dbgforce(self) -> crate::common::Reg<regs::Dbgforce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution"]
    #[inline(always)]
    pub fn mempowerdown(self) -> crate::common::Reg<regs::Mempowerdown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
pub mod regs;
