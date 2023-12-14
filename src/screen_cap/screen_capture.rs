use lodepng::{encode_file, Encoder};
use scrap::{Capturer, Display};
use std::{fs::File, io::ErrorKind::WouldBlock, io::Write, thread, time};

pub fn snap() {
    let mut attempt:i32 = 0; 
    let display = Display::primary().expect("Failed to get the primary display.");

    println!(
        "Display \n Width = {}, height = {} ",
        display.width(),
        display.height()
    );

    let mut capture: Capturer = Capturer::new(display).expect("Failed to create capturer.");
    let (w, h) = (capture.width(), capture.height());
    let mut encoder: Encoder = Encoder::new();
    let mut img:Vec<u8>;
    loop {
        attempt += 1;
        println!("Attempt #{attempt}");
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
      
         img = match encoder.encode(&frame, w, h) {
            Ok(img) => img,
            Err(error) => {
                panic!("Error:{}", error);
            }
        };
        //Ouput is blue
        //let mut output_file = File::create("./output.png").expect("Failed to create output file.");
        //output_file.write_all(&img).expect("Failed to outputfile");

        // More accurate for some reason
        encode_file("./encode.png", &frame, w, h, lodepng::ColorType::BGRA, 8)
            .expect("Failed to output file");

        break;
    }
}

pub fn rec() {
    todo!();
}
