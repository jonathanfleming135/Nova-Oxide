pub mod matrix_controller;
pub mod test;
pub mod patterns;

pub const PIN: i32 = 18;
pub const NUM_ROWS: i32 = 28;
pub const NUM_COLS: i32 = 11;
pub const NUM_LEDS: i32 = NUM_ROWS * NUM_COLS;

#[cfg(test)]
mod tests {
    use crate::matrix_controller;
    use crate::test;

    #[test]
    fn leds_test() {
        let mut matrix_controller = matrix_controller::MatrixController::new();
        test::test_rows(&mut matrix_controller);
        test::test_cols(&mut matrix_controller);
        test::test_all_leds(&mut matrix_controller);
        assert!(true);
    }
}
