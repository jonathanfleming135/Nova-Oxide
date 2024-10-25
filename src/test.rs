use crate::matrix_controller::MatrixController;
use crate::{NUM_COLS, NUM_LEDS, NUM_ROWS};
use std::{thread, time};

const DELAY: time::Duration = time::Duration::from_millis(3000);
const SMALL_DELAY: time::Duration = time::Duration::from_millis(250);

pub fn test_rows(matrix_controller: &mut MatrixController) {
    for i in 0..NUM_ROWS {
        if i % 4 == 0 {
            MatrixController::set_row(i, [32, 0, 0, 255], &mut matrix_controller.leds);
        } else if i % 4 == 1 {
            MatrixController::set_row(i, [0, 32, 0, 255], &mut matrix_controller.leds);
        } else if i % 4 == 2 {
            MatrixController::set_row(i, [0, 0, 32, 255], &mut matrix_controller.leds);
        } else {
            MatrixController::set_row(i, [32, 32, 32, 255], &mut matrix_controller.leds);
        };
    }

    matrix_controller.render();
    thread::sleep(DELAY);
    MatrixController::clear(matrix_controller);
}

pub fn test_cols(matrix_controller: &mut MatrixController) {
    for i in 0..NUM_COLS {
        if i % 4 == 0 {
            MatrixController::set_col(i, [32, 0, 0, 255], &mut matrix_controller.leds);
        } else if i % 4 == 1 {
            MatrixController::set_col(i, [0, 32, 0, 255], &mut matrix_controller.leds);
        } else if i % 4 == 2 {
            MatrixController::set_col(i, [0, 0, 32, 0], &mut matrix_controller.leds);
        } else {
            MatrixController::set_col(i, [32, 32, 32, 0], &mut matrix_controller.leds);
        };
    }

    matrix_controller.render();
    thread::sleep(DELAY);
    MatrixController::clear(matrix_controller);
}

pub fn test_all_leds(matrix_controller: &mut MatrixController) {
    for i in 0..NUM_LEDS {
        MatrixController::set_led_by_num(i, [0, 100, 0, 20], &mut matrix_controller.leds);
        matrix_controller.render();
        thread::sleep(SMALL_DELAY);
    }
    MatrixController::clear(matrix_controller);
}
