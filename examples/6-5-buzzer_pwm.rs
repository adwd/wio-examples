//! 6-5 ブザー/PWMのサンプルコードです。
//! ドレミファソラシドと1秒ずつ鳴ります。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-5-buzzer_pwm
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::pwm::Channel;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};
use wio_examples::Led;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化する
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    // ブザー（PWM）ドライバオブジェクトを初期化する
    let mut sets = Pins::new(peripherals.PORT).split();
    let mut buzzer = sets.buzzer.init(
        &mut clocks,
        peripherals.TCC0,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    let mut led = Led::new(sets.user_led, &mut sets.port);

    //           ド   レ    ミ   ファ  ソ   ラ   シ   ド
    let freqs = [261, 294, 329, 349, 392, 440, 494, 523];
    let c4 = Channel::_4;
    loop {
        for freq in freqs.iter() {
            // 周期（周波数）を設定する
            buzzer.set_period(freq.hz());

            // デューティ比を50%に設定する
            let max_duty = buzzer.get_max_duty();
            buzzer.set_duty(c4, max_duty / 2);

            // 1秒鳴らして止める
            buzzer.enable(c4);
            delay.delay_ms(1000u16);
            led.toggle();
            buzzer.disable(c4);
        }
    }
}
