/*! Power management unit. Controls entering sleep modes */

/// Wakeup source for shallower sleep modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeupSource {
    /// Only interrupts (WFI)
    Interrupt,
    /// Interrupts and events (WFE)
    ///
    /// FIXME: unimplemented; same as Interrupt.
    Event,
}

/// Selects sleep mode.
///
/// Information from section 3.3.4. Power saving modes of GD32VF103 User Manual v1.4 and section 4.3 Power consumption of GD32VF103 Datasheet Rev 1.7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepMode {
    /// Shallowest sleep mode.
    ///
    /// Any interrupt or event wakes up the processor. Wakeup time is 4.5 µs. SoC power draw is about 70% of running power.
    Sleep(WakeupSource),
    /// Middle-deep sleep mode.
    ///
    /// Any interrupt or event **from EXTI** wakes up the processor. Wake up time is 6 µs. SoC power draw is about 0.45 mA, which is about 70% of the lowest-power configuration of Sleep mode in the datasheet at the same voltage.
    // TODO: test if DeepSleep disables all peripherals
    DeepSleep(WakeupSource),
    /// Deeper sleep mode.
    ///
    /// Puts all pins in high impedance mode. Does not preserve SRAM or register contents, except some in the Backup Domain. On wakeup, executes the poweron sequence.
    ///
    /// Wakes up from:
    /// - NRST pin
    /// - WKUP pin
    /// - FWDGT reset
    /// - RTC
    ///
    /// Wake up time is 119ms. Power draw is about 7µA.
    // TODO: test this mode.
    Standby,
}

pub fn sleep(pmu: &mut gd32vf103_pac::pmu::RegisterBlock, mode: SleepMode) {
    let (standby_on, deepsleep_on) = match mode {
        SleepMode::Sleep(_) => (false, false),
        SleepMode::DeepSleep(_) => (false, true),
        SleepMode::Standby => {
            // The User Manual calls for clearing PMU_CS:WUF ("wake up flag"?), but that bit is not present in the register description. Using "Wake up reset" instead, hoping that it works.
            // Untested.
            pmu.ctl.modify(|_, w| w.wurst().set_bit());
            (true, true)
        }
    };
    pmu.ctl.modify(|_, w| w.stbmod().bit(standby_on.into()));

    unsafe { nuclei_n205::sleepvalue::write(
        nuclei_n205::sleepvalue::Sleepvalue::from_bits(deepsleep_on.into())
    )};

    unsafe { riscv::asm::wfi() };
}
