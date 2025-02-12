#[doc = "Controls time and alarms time is a 64 bit value indicating the time in usec since power-on timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr An alarm is set by setting alarm_enable and writing to the corresponding alarm register When an alarm is pending, the corresponding alarm_running signal will be high An alarm can be cancelled before it has finished by clearing the alarm_enable When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared To clear the interrupt write a 1 to the corresponding alarm_irq"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub *mut u8);
unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}
impl Timer {
    #[doc = "Write to bits 63:32 of time always write timelw before timehw"]
    #[inline(always)]
    pub fn timehw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Write to bits 31:0 of time writes do not get copied to time until timehw is written"]
    #[inline(always)]
    pub fn timelw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read from bits 63:32 of time always read timelr before timehr"]
    #[inline(always)]
    pub fn timehr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Read from bits 31:0 of time"]
    #[inline(always)]
    pub fn timelr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    #[inline(always)]
    pub fn alarm(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize + n * 4usize)) }
    }
    #[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
    #[inline(always)]
    pub fn armed(self) -> crate::common::Reg<regs::Armed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Raw read from bits 63:32 of time (no side effects)"]
    #[inline(always)]
    pub fn timerawh(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Raw read from bits 31:0 of time (no side effects)"]
    #[inline(always)]
    pub fn timerawl(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
    #[inline(always)]
    pub fn dbgpause(self) -> crate::common::Reg<regs::Dbgpause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Set high to pause the timer"]
    #[inline(always)]
    pub fn pause(self) -> crate::common::Reg<regs::Pause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
}
pub mod regs;
