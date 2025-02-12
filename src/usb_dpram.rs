#[doc = "DPRAM layout for USB device."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbDpram(pub *mut u8);
unsafe impl Send for UsbDpram {}
unsafe impl Sync for UsbDpram {}
impl UsbDpram {
    #[doc = "Bytes 0-3 of the SETUP packet from the host."]
    #[inline(always)]
    pub fn setup_packet_low(self) -> crate::common::Reg<regs::SetupPacketLow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Bytes 4-7 of the setup packet from the host."]
    #[inline(always)]
    pub fn setup_packet_high(self) -> crate::common::Reg<regs::SetupPacketHigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[inline(always)]
    pub fn ep_in_control(self, n: usize) -> crate::common::Reg<regs::EpControl, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize + n * 8usize)) }
    }
    #[inline(always)]
    pub fn ep_out_control(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EpControl, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize + n * 8usize)) }
    }
    #[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub fn ep_in_buffer_control(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EpBufferControl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize + n * 8usize)) }
    }
    #[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub fn ep_out_buffer_control(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EpBufferControl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize + n * 8usize)) }
    }
}
pub mod regs;
pub mod vals;
