use std::borrow::Borrow;

use esp_idf_hal::{
    gpio::OutputPin,
    ledc::{LedcChannel, LedcDriver, LedcTimerDriver},
    peripheral::Peripheral,
};
use esp_idf_sys as _;

pub struct Servo<'a> {
    ledc: LedcDriver<'a>,
    max_angle: u32,
}

impl<'a> Servo<'a> {
    pub fn new<C: LedcChannel, B: Borrow<LedcTimerDriver<'a>>>(
        _channel: impl Peripheral<P = C> + 'a,
        timer_driver: B,
        pin: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            ledc: LedcDriver::new(_channel, timer_driver, pin)?,
            max_angle: 90,
        })
    }

    pub fn set_angle(self: &mut Self, angle: u32) -> anyhow::Result<()> {
        let max_duty = self.ledc.get_max_duty();
        self.ledc
            .set_duty(max_duty * (self.max_angle + angle) / self.max_angle / 20)?;
        Ok(())
    }
}
