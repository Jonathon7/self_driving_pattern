use rust_gpiozero::output_devices::Motor;
// use rust_gpiozero::*;

// impl Motor {
//     pub fn new(forward_pin: u8, backward_pin: u8) -> Motor {
//         Motor
//     }
// }

fn main() {
    let mut test1 = Motor::new(7, 8);
    let mut test2 = Motor::new(9, 11);

    test1.forward();
    test2.forward();
}
