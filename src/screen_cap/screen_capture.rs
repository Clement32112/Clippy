use lodepng::{encode_file, Encoder};
use scrap::{Capturer, Display};
use std::{error, fs::File, io::ErrorKind::WouldBlock, io::Write, thread, time};

pub fn snap() {
    let display = Display::primary().expect("Failed to get the primary display.");

    println!(
        "Display \n Width = {}, height = {} ",
        display.width(),
        display.height()
    );

    let mut capture: Capturer = Capturer::new(display).expect("Failed to create capturer.");
    let (w, h) = (capture.width(), capture.height());
    let mut output_file = File::create("./output.png").expect("Failed to create output file.");

    loop {
        let frame = match capture.frame() {
            Ok(frame) => frame,
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(time::Duration::from_millis(10));
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        // Frame has been gotten;

        encode_file("./encode.png", &frame, w, h, lodepng::ColorType::BGRA, 8)
            .expect("Failed to export image");
        break;
    }
}
