use crate::generic::*;
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Divmode(u8);
impl Divmode {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Divmode {
        Divmode(val)
    }
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    pub const DIV: Self = Self(0);
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    pub const LEVEL: Self = Self(0x01);
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    pub const RISE: Self = Self(0x02);
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    pub const FALL: Self = Self(0x03);
}