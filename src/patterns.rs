use std::time::{Duration, Instant};
use std::{time, thread};
use rand::Rng;

use crate::matrix_controller::MatrixController;
use crate::{NUM_COLS, NUM_LEDS, NUM_ROWS};

pub fn rainbow(matrix_controller: &mut MatrixController, duration_secs: f64) {
    let duration_millis = duration_secs * 1000.0;
    let duration = Duration::from_millis(duration_millis as u64);
    let start = Instant::now();

    let mut rng = rand::thread_rng();
    let mut blu: u8 = rng.gen_range(10..=245);
    let mut grn: u8 = rng.gen_range(10..=245);
    let mut red: u8 = rng.gen_range(10..=245);
    let mut blu_add = true;
    let mut grn_add = true;
    let mut red_add = true;
    let mut blu_diff: u8 = rng.gen_range(0..=10);
    let mut grn_diff: u8 = rng.gen_range(0..=10);
    let mut red_diff: u8 = rng.gen_range(0..=10);
    let mut count = 0;
    while Instant::now() - start < duration {
        count += 1;
        // if count >= 250 {
        //     blu_diff = 0;
        //     grn_diff = 0;
        //     red_diff = 0;
        //     let color_select = rng.gen_range(0..=2);
        //     if color_select == 0 {
        //         blu_diff = 1;
        //     } else if color_select == 1 {
        //         grn_diff = 1;
        //     } else {
        //         red_diff = 1;
        //     };
        // };
        if count >= 10 {
            blu_diff = rng.gen_range(0..=10);
            grn_diff = rng.gen_range(0..=10);
            red_diff = rng.gen_range(0..=10);
        }

        if blu <= 245 && blu_add {
            blu += blu_diff;
            if blu >= 245 {
                blu_add = false;
            }
        } else if blu >= 10 && !blu_add {
            blu -= blu_diff;
            if blu <= 10 {
                blu_add = true;
            }
        }

        if grn <= 245 && grn_add {
            grn += grn_diff;
            if grn >= 245 {
                grn_add = false;
            }
        } else if grn >= 10 && !grn_add {
            grn -= grn_diff;
            if grn <= 10 {
                grn_add = true;
            }
        }

        if red <= 245 && red_add {
            red += red_diff;
            if red >= 245 {
                red_add = false;
            }
        } else if red >= 10 && !red_add {
            red -= red_diff;
            if red <= 10 {
                red_add = true;
            }
        }

        let pre_matrix_blu = blu;
        let pre_matrix_grn = grn;
        let pre_matrix_red = red;
        let mut blu_matrix_add = true;
        let mut grn_matrix_add = true;
        let mut red_matrix_add = true;
        for i in 0..NUM_ROWS {
            if i > (NUM_ROWS-3) {
                // println!("Row {}, blu: {}, grn: {}, red: {}", i, blu, grn, red);
                MatrixController::set_row(i, [blu, grn, red, 0], &mut matrix_controller.leds);
                continue;
            };

            if blu <= 200 && blu_matrix_add {
                blu += 10;
                if blu >= 200 {
                    blu_matrix_add = false;
                }
            } else if blu >= 50 && !blu_matrix_add {
                blu -= 10;
                if blu <= 50 {
                    blu_matrix_add = true;
                }
            } else if blu >= 200 {
                blu_matrix_add = false;
            }

            if grn <= 200 && grn_matrix_add {
                grn += 10;
                if grn >= 200 {
                    grn_matrix_add = false;
                }
            } else if grn >= 50 && !grn_matrix_add {
                grn -= 10;
                if grn <= 50 {
                    grn_matrix_add = true;
                }
            } else if grn >= 200 {
                grn_matrix_add = false;
            }

            if red <= 200 && red_matrix_add {
                red += 10;
                if red >= 200 {
                    red_matrix_add = false;
                }
            } else if red >= 50 && !red_matrix_add {
                red -= 10;
                if red <= 50 {
                    red_matrix_add = true;
                }
            } else if red >= 200 {
                red_matrix_add = false;
            }

            // println!("Row {}, blu: {}, grn: {}, red: {}", i, blu, grn, red);
            MatrixController::set_row(i, [blu, grn, red, 0], &mut matrix_controller.leds);
            let pre_render_timestamp = Instant::now().duration_since(start).as_millis();
            matrix_controller.render();
            let post_render_timestamp = Instant::now().duration_since(start).as_millis();
            if 20 - (post_render_timestamp - pre_render_timestamp) > 0 {
                let sleep_time = 20 - (post_render_timestamp - pre_render_timestamp);
                thread::sleep(time::Duration::from_millis(sleep_time as u64));
            }
            println!("{}", Instant::now().duration_since(start).as_millis());
        }
        blu = pre_matrix_blu;
        grn = pre_matrix_grn;
        red = pre_matrix_red;
        // println!("{}", Instant::now().duration_since(start).as_millis());
        // thread::sleep(time::Duration::from_millis(50));
    }

    println!("Finished Looping!");
}
