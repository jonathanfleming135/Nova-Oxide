use nova_oxide::matrix_controller::MatrixController;
use nova_oxide::patterns;
use nova_oxide::{NUM_COLS, NUM_LEDS, NUM_ROWS};

fn main() {
    println!("Program start");
    let mut matrix_controller = MatrixController::new();
    nova_oxide::patterns::rainbow(&mut matrix_controller, 1.0);
    matrix_controller.clear();
}
