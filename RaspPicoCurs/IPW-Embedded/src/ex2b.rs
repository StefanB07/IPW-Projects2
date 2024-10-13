#![no_std]
#![no_main]

use cortex_m::peripheral;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{self, Either};
use embassy_rp::{config, gpio::{Input, Level, Output, Pull}, pwm::Pwm};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_futures::select::select;

// #[embassy_executor::task]
// async fn button_pressed1(led: Output<'static>, mut button: Input<'static>) {
//     loop {
//         info!("waiting for button press");
//         button.wait_for_falling_edge().await;
//     }
// }

// #[embassy_executor::task]
// async fn button_pressed2(mut led: Output<'static, PIN_X>, mut button: Input<'static, PIN_13>) {
//     loop {
//         info!("waiting for button press");
//         button.wait_for_falling_edge().await;
//     }
// }

#[embassy_executor::main]
async fn main(spawner: Spawner){
    let peripherals= embassy_rp::init(Default::default());

    let mut config1: ConfigPwm = Default::default();
    let mut config2: ConfigPwm = Default::default();

    let mut button1 = Input::new(peripherals.PIN_12, Pull::Up);
    let mut button2 = Input::new(peripherals.PIN_13, Pull::Up);

    config1.top = 255;
    config1.compare_a = 250;
    config1.compare_b = 250;

    config2.top = 255;
    config2.compare_a = 250;

    let mut pwm_red_green = Pwm::new_output_ab(
        peripherals.PWM_SLICE0,
        peripherals.PIN_0,
        peripherals.PIN_1,
        config1.clone()
    );
    let mut pwm_blue = Pwm::new_output_a(
        peripherals.PWM_SLICE1,
        peripherals.PIN_2,
        config2.clone()
    );

    // spawner.spawn(button_pressed(config1, button1, button2)).unwrap();
    loop {
        pwm_red_green.set_config(&config1);
        pwm_blue.set_config(&config2);
        let select = select(button1.wait_for_falling_edge(), button2.wait_for_falling_edge());
        match select.await {
            Either::First(_) => {
                if config1.compare_a >=25 {
                    config1.compare_a -= 25;
                    config1.compare_b -= 25;
                    config2.compare_a -= 25;
                }
            },
        
            Either::Second(_) => {
                if config1.compare_a <=225 {
                    config1.compare_a += 25;
                    config1.compare_b += 25;
                    config2.compare_a += 25;
                }
            }

        }
    }
}
