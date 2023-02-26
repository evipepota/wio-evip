#![no_std]
#![no_main]

use embedded_graphics::image::ImageRawLE;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::PrimitiveStyleBuilder;
use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use panic_halt as _;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};
use wio_terminal as wio;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets = Pins::new(peripherals.PORT).split();
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();
    let button1 = sets.buttons.button1.into_floating_input(&mut sets.port);
    let button2 = sets.buttons.button2.into_floating_input(&mut sets.port);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let background = Rectangle::new(Point::new(0, 0), Point::new(319, 239)).into_styled(style);
    background.draw(&mut display).unwrap();
    let raw = ImageRawLE::new(include_bytes!("./assets/evip.raw"), 200, 200);
    let image = Image::new(&raw, Point { x: (60), y: (20) });
    image.draw(&mut display).unwrap();

    loop {
        if button1.is_low().unwrap() {
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let background =
                Rectangle::new(Point::new(0, 0), Point::new(319, 239)).into_styled(style);
            background.draw(&mut display).unwrap();
            let raw = ImageRawLE::new(include_bytes!("./assets/evip.raw"), 200, 200);
            let image = Image::new(&raw, Point { x: (60), y: (20) });
            image.draw(&mut display).unwrap();
        } else if button2.is_low().unwrap() {
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let background =
                Rectangle::new(Point::new(0, 0), Point::new(319, 239)).into_styled(style);
            background.draw(&mut display).unwrap();
            let raw = ImageRawLE::new(include_bytes!("./assets/qr.raw"), 200, 200);
            let image = Image::new(&raw, Point { x: (60), y: (20) });
            image.draw(&mut display).unwrap();
        }
    }
}
