#![no_std]

extern crate msp430;
extern crate msp430fr2433;

use msp430::asm;

fn delay(n: u16) {
    let mut i = 0;
    loop {
        asm::nop();

        i += 1;

        if i == n {
            break;
        }
    }
}

// P1.0 = LED1
// P1.1 = LED2
fn main() {
    let p = msp430fr2433::Peripherals::take().unwrap();
    // Disable watchdog
    let wdt = p.WATCHDOG_TIMER;
    wdt.wdtctl.write(|w| {
        unsafe { w.bits(0x5A00) } // password
        .wdthold().set_bit()
    });

    let port_1_2 = p.PORT_1_2;

    // set P1.0 high and P1.1 low
    port_1_2.p1out.modify(|_, w| w.p1out0().set_bit().p1out1().clear_bit());

    // Set P1.0 and P1.1 as outputs
    port_1_2.p1dir.modify(|_, w| w.p1dir0().set_bit().p1dir1().set_bit());

    // Borrowed from TI's examples:
    // Disable the GPIO power-on default high-impedance mode
    // to activate previously configured port settings
    let pmm = p.PMM;
    pmm.pm5ctl0.modify(|_, w| w.locklpm5().clear_bit());

    loop {
        delay(10_000);

        // toggle outputs
        port_1_2.p1out.modify(
            |r, w| w.p1out0().bit(!r.p1out0().bit())
                    .p1out1().bit(!r.p1out1().bit()),
        );
    }
}
