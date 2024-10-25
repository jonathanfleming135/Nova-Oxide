use std::{thread, time};
use rs_ws281x::{ControllerBuilder, ChannelBuilder, StripType, RawColor, Controller};

const PIN: i32 = 18;
const NUM_ROWS: i32 = 28;
const NUM_COLS: i32 = 11;
const NUM_LEDS: i32 = NUM_ROWS * NUM_COLS;
const DELAY: time::Duration = time::Duration::from_millis(3000);
const SMALL_DELAY: time::Duration = time::Duration::from_millis(250);


fn main() {
    println!("Program start");

    let mut controller = ControllerBuilder::new()
    .freq(800_000)
    .dma(10)
    .channel(
        0, // Channel Index
        ChannelBuilder::new()
            .pin(PIN) // GPIO 18 = PWM0
            .count(NUM_LEDS) // Number of LEDs
            .strip_type(StripType::Ws2812)
            .brightness(100) // default: 255
            .build(),
    )
    .build()
    .unwrap();

    test_rows(&mut controller);
    thread::sleep(DELAY);
    clear(&mut controller);

    test_cols(&mut controller);
    thread::sleep(DELAY);
    clear(&mut controller);

    test_all_leds(&mut controller);
    thread::sleep(DELAY);
    clear(&mut controller);
}

fn clear(controller: &mut Controller) {
    let mut data = controller.leds_mut(0);

    for i in 0..NUM_LEDS {
        set_led_by_num(i, [0, 0, 0, 0], &mut data);
    }

    controller.render().unwrap();
}

fn test_rows(controller: &mut Controller) {
    let mut data = controller.leds_mut(0);

    for i in 0..NUM_ROWS {
        if i % 4 == 0 {
            set_row(i, [32, 0, 0, 255], &mut data);
        } else if i % 4 == 1 {
            set_row(i, [0, 32, 0, 255], &mut data);
        } else if i % 4 == 2 {
            set_row(i, [0, 0, 32, 255], &mut data);
        } else {
            set_row(i, [32, 32, 32, 255], &mut data);
        };
    }

    controller.render().unwrap();
}

fn test_cols(controller: &mut Controller) {
    let mut data = controller.leds_mut(0);

    for i in 0..NUM_COLS {
        if i % 4 == 0 {
            set_col(i, [32, 0, 0, 255], &mut data);
        } else if i % 4 == 1 {
            set_col(i, [0, 32, 0, 255], &mut data);
        } else if i % 4 == 2 {
            set_col(i, [0, 0, 32, 0], &mut data);
        } else {
            set_col(i, [32, 32, 32, 0], &mut data);
        };
    }

    controller.render().unwrap();
}

fn test_all_leds(controller: &mut Controller) {
    for i in 0..NUM_LEDS {
        let mut data = controller.leds_mut(0);
        set_led_by_num(i, [0, 100, 0, 20], &mut data);
        controller.render().unwrap();
        thread::sleep(SMALL_DELAY);
    }
}

/// Sets led color by number of the led in the matrix.
///
/// # Arguments
///
/// * `led_num` - Number of the led in the matrix (see example below)
/// * `color`   - RawColor [u8: 4] to set the led to
/// * `data`    - Borrowed reference to array of RawColors for led's
///
/// # Example
///
/// ```
/// This function will set the led's by number in the ordering shown below,
/// regardless of actual wiring of the matrix:
///
/// [  8,  9, 10, 11]
/// [  4,  5,  6,  7]
/// [  0,  1,  2,  3]
///
/// As such, the numbering starts at the bottom-left corner of the matrix,
/// proceeds to the right, and upon reaching the edge of the matrix, proceeds at
/// the left-most led of the next row above.
///
/// For the example above, led_num=3 would correspond to the bottom-right most
/// led in the matrix.
/// ```
fn set_led_by_num(led_num: i32, color: RawColor, data: &mut [RawColor]) {
    let row_offset = led_num % NUM_COLS;
    let num_rows: i32 = (led_num as f64 / NUM_COLS as f64).floor() as i32;

    // My led matrix switches wiring after row 8, hence why this is required
    let wired_offset = if led_num < (8*NUM_COLS) {
        if num_rows % 2 == 0 {
            row_offset
        } else {
            NUM_COLS - (row_offset+1)
        }
    } else {
        if num_rows % 2 == 0 {
            NUM_COLS - (row_offset+1)
        } else {
            row_offset
        }
    };

    let new_led_num = (num_rows*NUM_COLS) + wired_offset;
    data[new_led_num as usize] = color;
}

/// Sets single led color by row and column specified
///
/// # Arguments
///
/// * `row`   - Number of the row of specified led
/// * `col`   - Number of the column of specified led
/// * `color` - RawColor [u8: 4] to set the led to
/// * `data`  - Borrowed reference to array of RawColors for led's
///
/// # Example
///
/// ```
/// [  8,  9, 10, 11]
/// [  4,  5,  6,  7]
/// [  0,  1,  2,  3]
///
/// For the led matrix above, row=1 and col=2 would set led 6 to the color
/// passed in.
/// ```
fn set_led_by_matrix(row: i32, col: i32, color: RawColor, data: &mut [RawColor]) {
    let led_num = row * NUM_COLS + col;
    set_led_by_num(led_num, color, data);
}

/// Sets a row of led's to the color specified
///
/// # Arguments
///
/// * `row`   - Number of the row of led's specified
/// * `color` - RawColor [u8: 4] to set the led's to
/// * `data`  - Borrowed reference to array of RawColors for led's
///
/// # Example
///
/// ```
/// [  8,  9, 10, 11]
/// [  4,  5,  6,  7]
/// [  0,  1,  2,  3]
///
/// For the led matrix above, row=1 would set led's 4, 5, 6, and 7 to the color
/// specified
/// ```
fn set_row(row: i32, color: RawColor, data: &mut [RawColor]) {
    for i in 0..NUM_COLS {
        set_led_by_matrix(row, i, color, data);
    }
}

/// Sets a column of led's to the color specified
///
/// # Arguments
///
/// * `col`   - Number of the column of led's specified
/// * `color` - RawColor [u8: 4] to set the led's to
/// * `data`  - Borrowed reference to array of RawColors for led's
///
/// # Example
///
/// ```
/// [  8,  9, 10, 11]
/// [  4,  5,  6,  7]
/// [  0,  1,  2,  3]
///
/// For the led matrix above, col=2 would set led's 2, 6, and 10 to the color
/// specified
/// ```
fn set_col(col: i32, color: RawColor, data: &mut [RawColor]) {
    for i in 0..NUM_ROWS {
        set_led_by_matrix(i, col, color, data);
    }
}
