use std::{thread, time};

// use smart_leds::{SmartLedsWrite, RGB8};
// use ws281x_rpi::Ws2812Rpi;
use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType, RawColor};

const PIN: i32 = 18;
const NUM_ROWS: i32 = 28;
const NUM_COLS: i32 = 11;
const NUM_LEDS: i32 = NUM_ROWS * NUM_COLS;
const DELAY: time::Duration = time::Duration::from_millis(1000);
const SMALL_DELAY: time::Duration = time::Duration::from_millis(250);


fn main() {
    println!("Program start");

    // let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();

    let mut controller = ControllerBuilder::new()
    .freq(800_000)
    .dma(10)
    .channel(
        0, // Channel Index
        ChannelBuilder::new()
            .pin(PIN) // GPIO 10 = SPI0 MOSI
            .count(NUM_LEDS) // Number of LEDs
            .strip_type(StripType::Ws2812)
            .brightness(20) // default: 255
            .build(),
    )
    .build()
    .unwrap();

    // let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    // let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let mut data = controller.leds_mut(0);

    // for i in 0..NUM_ROWS {
    //     if i % 4 == 0 {
    //         set_row(i, 32, 0, 0, &mut data);
    //     } else if i % 4 == 1 {
    //         set_row(i, 0, 32, 0, &mut data);
    //     } else if i % 4 == 2 {
    //         set_row(i, 0, 0, 32, &mut data);
    //     } else {
    //         set_row(i, 32, 32, 32, &mut data);
    //     };
    // }

    // for i in 0..NUM_COLS {
    //     if i % 4 == 0 {
    //         set_col(i, 32, 0, 0, &mut data);
    //     } else if i % 4 == 1 {
    //         set_col(i, 0, 32, 0, &mut data);
    //     } else if i % 4 == 2 {
    //         set_col(i, 0, 0, 32, &mut data);
    //     } else {
    //         set_col(i, 32, 32, 32, &mut data);
    //     };
    // }

    // // Off
    // println!("LEDS off");
    // ws.write(empty.iter().cloned()).unwrap();
    // thread::sleep(DELAY);

    for i in 0..NUM_LEDS {
        set_led_by_num(i, 0, 255, 0, &mut data);
        // ws.write(data.iter().cloned()).unwrap();
    }

    controller.render().unwrap();
    thread::sleep(DELAY);

    // loop {
    //     // On
    //     println!("LEDS on");
    //     ws.write(data.iter().cloned()).unwrap();
    //     thread::sleep(DELAY);

    //     // Off
    //     println!("LEDS off");
    //     ws.write(empty.iter().cloned()).unwrap();
    //     thread::sleep(DELAY);
    // }
}

// fn set_led_by_num(led_num: usize, r: u8, g: u8, b: u8, data: &mut [RGB8; NUM_LEDS]) {
    fn set_led_by_num(led_num: i32, r: u8, g: u8, b: u8, data: &mut [RawColor]) {
    let row_offset = led_num % NUM_COLS;
    // let num_rows = ((led_num as f64 / NUM_COLS as f64).floor() as usize);
    let num_rows_f: f64 = (led_num as f64 / NUM_COLS as f64).floor();
    let num_rows_i: i32 = num_rows_f as i32;
    let offset = if led_num < (8*NUM_COLS) {
        if num_rows_i % 2 == 0 {
            row_offset
        } else {
            NUM_COLS - (row_offset+1)
        }
    } else {
        if num_rows_i % 2 == 0 {
            NUM_COLS - (row_offset+1)
        } else {
            row_offset
        }
    };
    let new_led_num = (num_rows_i*NUM_COLS) + offset;
    println!("led_num: {}", led_num);
    println!("new_led_num: {}", new_led_num);
    let mut led = data[new_led_num as usize];
    led[0] = r;
    led[1] = g;
    led[2] = b;
    data[new_led_num as usize] = led;
}

// fn set_led_by_matrix(row: usize, col: usize, r: u8, g: u8, b: u8, data: &mut [RGB8; NUM_LEDS]) {
    fn set_led_by_matrix(row: i32, col: i32, r: u8, g: u8, b: u8, data: &mut [RawColor]) {
    let led_num = row * NUM_COLS + col;
    // let offset = if row % 2 == 0 {
    //     col
    // } else {
    //     NUM_COLS - (col+1)
    // };
    // led_num += offset;
    set_led_by_num(led_num, r, g, b, data);
    // let mut led = data[led_num];
    // led.r = r;
    // led.g = g;
    // led.b = b;
    // println!("led_num: {}", led_num);
    // println!("row: {}", row);
    // println!("col: {}", col);
    // println!("r: {}", r);
    // println!("g: {}", g);
    // println!("b: {}", b);
    // println!("\n\n");
    // data[led_num] = led;
}

// fn set_row(row: usize, r: u8, g: u8, b: u8, data: &mut [RGB8; NUM_LEDS]) {
    fn set_row(row: i32, r: u8, g: u8, b: u8, data: &mut [RawColor]) {
    for i in 0..NUM_COLS {
        set_led_by_matrix(row, i, r, g, b, data);
    }
}

// fn set_col(col: usize, r: u8, g: u8, b: u8, data: &mut [RGB8; NUM_LEDS]) {
fn set_col(col: i32, r: u8, g: u8, b: u8, data: &mut [RawColor]) {
    for i in 0..NUM_ROWS {
        set_led_by_matrix(i, col, r, g, b, data);
    }
}




    // GPIO Pin 10 is SPI
    // Other modes and PINs are available depending on the Raspberry Pi revision
    // Additional OS configuration might be needed for any mode.
    // Check https://github.com/jgarff/rpi_ws281x for more information.
    // const PIN: i32 = 18;
    // const NUM_LEDS: usize = 308;
    // const DELAY: time::Duration = time::Duration::from_millis(1000);

    // let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();

    // let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    // let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    // // Blink the LED's in a blue-green-red-white pattern.
    // for led in data.iter_mut().step_by(4) {
    //     led.b = 32;
    // }

    // if NUM_LEDS > 1 {
    //     for led in data.iter_mut().skip(1).step_by(4) {
    //         led.g = 32;
    //     }
    // }

    // if NUM_LEDS > 2 {
    //     for led in data.iter_mut().skip(2).step_by(4) {
    //         led.r = 32;
    //     }
    // }

    // if NUM_LEDS > 3 {
    //     for led in data.iter_mut().skip(3).step_by(4) {
    //         led.r = 32;
    //         led.g = 32;
    //         led.b = 32;
    //     }
    // }
