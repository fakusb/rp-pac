#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio0ctrlFuncsel(pub u8);
impl Gpio0ctrlFuncsel {
    pub const JTAG_TCK: Self = Self(0);
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_0: Self = Self(0x04);
    pub const SIO_0: Self = Self(0x05);
    pub const PIO0_0: Self = Self(0x06);
    pub const PIO1_0: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio10ctrlFuncsel(pub u8);
impl Gpio10ctrlFuncsel {
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_5: Self = Self(0x04);
    pub const SIO_10: Self = Self(0x05);
    pub const PIO0_10: Self = Self(0x06);
    pub const PIO1_10: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VM: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio11ctrlFuncsel(pub u8);
impl Gpio11ctrlFuncsel {
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_5: Self = Self(0x04);
    pub const SIO_11: Self = Self(0x05);
    pub const PIO0_11: Self = Self(0x06);
    pub const PIO1_11: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SUSPND: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio12ctrlFuncsel(pub u8);
impl Gpio12ctrlFuncsel {
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_6: Self = Self(0x04);
    pub const SIO_12: Self = Self(0x05);
    pub const PIO0_12: Self = Self(0x06);
    pub const PIO1_12: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SPEED: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio13ctrlFuncsel(pub u8);
impl Gpio13ctrlFuncsel {
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_6: Self = Self(0x04);
    pub const SIO_13: Self = Self(0x05);
    pub const PIO0_13: Self = Self(0x06);
    pub const PIO1_13: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VPO: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio14ctrlFuncsel(pub u8);
impl Gpio14ctrlFuncsel {
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_7: Self = Self(0x04);
    pub const SIO_14: Self = Self(0x05);
    pub const PIO0_14: Self = Self(0x06);
    pub const PIO1_14: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VMO: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio15ctrlFuncsel(pub u8);
impl Gpio15ctrlFuncsel {
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_7: Self = Self(0x04);
    pub const SIO_15: Self = Self(0x05);
    pub const PIO0_15: Self = Self(0x06);
    pub const PIO1_15: Self = Self(0x07);
    pub const USB_MUXING_DIGITAL_DP: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio16ctrlFuncsel(pub u8);
impl Gpio16ctrlFuncsel {
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_0: Self = Self(0x04);
    pub const SIO_16: Self = Self(0x05);
    pub const PIO0_16: Self = Self(0x06);
    pub const PIO1_16: Self = Self(0x07);
    pub const USB_MUXING_DIGITAL_DM: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio17ctrlFuncsel(pub u8);
impl Gpio17ctrlFuncsel {
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_0: Self = Self(0x04);
    pub const SIO_17: Self = Self(0x05);
    pub const PIO0_17: Self = Self(0x06);
    pub const PIO1_17: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio18ctrlFuncsel(pub u8);
impl Gpio18ctrlFuncsel {
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_1: Self = Self(0x04);
    pub const SIO_18: Self = Self(0x05);
    pub const PIO0_18: Self = Self(0x06);
    pub const PIO1_18: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio19ctrlFuncsel(pub u8);
impl Gpio19ctrlFuncsel {
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_1: Self = Self(0x04);
    pub const SIO_19: Self = Self(0x05);
    pub const PIO0_19: Self = Self(0x06);
    pub const PIO1_19: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio1ctrlFuncsel(pub u8);
impl Gpio1ctrlFuncsel {
    pub const JTAG_TMS: Self = Self(0);
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_0: Self = Self(0x04);
    pub const SIO_1: Self = Self(0x05);
    pub const PIO0_1: Self = Self(0x06);
    pub const PIO1_1: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio20ctrlFuncsel(pub u8);
impl Gpio20ctrlFuncsel {
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_2: Self = Self(0x04);
    pub const SIO_20: Self = Self(0x05);
    pub const PIO0_20: Self = Self(0x06);
    pub const PIO1_20: Self = Self(0x07);
    pub const CLOCKS_GPIN_0: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio21ctrlFuncsel(pub u8);
impl Gpio21ctrlFuncsel {
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_2: Self = Self(0x04);
    pub const SIO_21: Self = Self(0x05);
    pub const PIO0_21: Self = Self(0x06);
    pub const PIO1_21: Self = Self(0x07);
    pub const CLOCKS_GPOUT_0: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio22ctrlFuncsel(pub u8);
impl Gpio22ctrlFuncsel {
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_3: Self = Self(0x04);
    pub const SIO_22: Self = Self(0x05);
    pub const PIO0_22: Self = Self(0x06);
    pub const PIO1_22: Self = Self(0x07);
    pub const CLOCKS_GPIN_1: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio23ctrlFuncsel(pub u8);
impl Gpio23ctrlFuncsel {
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_3: Self = Self(0x04);
    pub const SIO_23: Self = Self(0x05);
    pub const PIO0_23: Self = Self(0x06);
    pub const PIO1_23: Self = Self(0x07);
    pub const CLOCKS_GPOUT_1: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio24ctrlFuncsel(pub u8);
impl Gpio24ctrlFuncsel {
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_4: Self = Self(0x04);
    pub const SIO_24: Self = Self(0x05);
    pub const PIO0_24: Self = Self(0x06);
    pub const PIO1_24: Self = Self(0x07);
    pub const CLOCKS_GPOUT_2: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio25ctrlFuncsel(pub u8);
impl Gpio25ctrlFuncsel {
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_4: Self = Self(0x04);
    pub const SIO_25: Self = Self(0x05);
    pub const PIO0_25: Self = Self(0x06);
    pub const PIO1_25: Self = Self(0x07);
    pub const CLOCKS_GPOUT_3: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio26ctrlFuncsel(pub u8);
impl Gpio26ctrlFuncsel {
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_5: Self = Self(0x04);
    pub const SIO_26: Self = Self(0x05);
    pub const PIO0_26: Self = Self(0x06);
    pub const PIO1_26: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio27ctrlFuncsel(pub u8);
impl Gpio27ctrlFuncsel {
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_5: Self = Self(0x04);
    pub const SIO_27: Self = Self(0x05);
    pub const PIO0_27: Self = Self(0x06);
    pub const PIO1_27: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio28ctrlFuncsel(pub u8);
impl Gpio28ctrlFuncsel {
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_6: Self = Self(0x04);
    pub const SIO_28: Self = Self(0x05);
    pub const PIO0_28: Self = Self(0x06);
    pub const PIO1_28: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio29ctrlFuncsel(pub u8);
impl Gpio29ctrlFuncsel {
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_6: Self = Self(0x04);
    pub const SIO_29: Self = Self(0x05);
    pub const PIO0_29: Self = Self(0x06);
    pub const PIO1_29: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio2ctrlFuncsel(pub u8);
impl Gpio2ctrlFuncsel {
    pub const JTAG_TDI: Self = Self(0);
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_1: Self = Self(0x04);
    pub const SIO_2: Self = Self(0x05);
    pub const PIO0_2: Self = Self(0x06);
    pub const PIO1_2: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio3ctrlFuncsel(pub u8);
impl Gpio3ctrlFuncsel {
    pub const JTAG_TDO: Self = Self(0);
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_1: Self = Self(0x04);
    pub const SIO_3: Self = Self(0x05);
    pub const PIO0_3: Self = Self(0x06);
    pub const PIO1_3: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio4ctrlFuncsel(pub u8);
impl Gpio4ctrlFuncsel {
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_2: Self = Self(0x04);
    pub const SIO_4: Self = Self(0x05);
    pub const PIO0_4: Self = Self(0x06);
    pub const PIO1_4: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio5ctrlFuncsel(pub u8);
impl Gpio5ctrlFuncsel {
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_2: Self = Self(0x04);
    pub const SIO_5: Self = Self(0x05);
    pub const PIO0_5: Self = Self(0x06);
    pub const PIO1_5: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio6ctrlFuncsel(pub u8);
impl Gpio6ctrlFuncsel {
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_3: Self = Self(0x04);
    pub const SIO_6: Self = Self(0x05);
    pub const PIO0_6: Self = Self(0x06);
    pub const PIO1_6: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SOFTCON: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio7ctrlFuncsel(pub u8);
impl Gpio7ctrlFuncsel {
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_3: Self = Self(0x04);
    pub const SIO_7: Self = Self(0x05);
    pub const PIO0_7: Self = Self(0x06);
    pub const PIO1_7: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_OE_N: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio8ctrlFuncsel(pub u8);
impl Gpio8ctrlFuncsel {
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_4: Self = Self(0x04);
    pub const SIO_8: Self = Self(0x05);
    pub const PIO0_8: Self = Self(0x06);
    pub const PIO1_8: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_RCV: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gpio9ctrlFuncsel(pub u8);
impl Gpio9ctrlFuncsel {
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_4: Self = Self(0x04);
    pub const SIO_9: Self = Self(0x05);
    pub const PIO0_9: Self = Self(0x06);
    pub const PIO1_9: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VP: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inover(pub u8);
impl Inover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Irqover(pub u8);
impl Irqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Oeover(pub u8);
impl Oeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Outover(pub u8);
impl Outover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
