use std::thread;
use std::time::Duration;

// `Servo` is an abstract contract (a trait), not a concrete driver.
trait Servo {
    fn set_degrees(&self, degrees: u16);
}

// `ServoPlayer` is also abstract. It extends `Servo` (supertrait), so any
// `ServoPlayer` can do everything in `Servo` plus animation.
trait ServoPlayer: Servo {
    // (degrees, milliseconds to hold at that angle)
    fn animate(&self, steps: &[(u16, u64)]);
}

// Concrete servo driver (similar naming to the real example): `ServoEsp`.
struct ServoEsp;

impl Servo for ServoEsp {
    fn set_degrees(&self, degrees: u16) {
        println!("[ServoEsp] set angle -> {degrees}°");
    }
}

// Concrete servo player driver that can animate.
struct ServoPlayerEsp;

impl Servo for ServoPlayerEsp {
    fn set_degrees(&self, degrees: u16) {
        println!("[ServoPlayerEsp] set angle -> {degrees}°");
    }
}

impl ServoPlayer for ServoPlayerEsp {
    fn animate(&self, steps: &[(u16, u64)]) {
        for (degrees, ms) in steps {
            self.set_degrees(*degrees);
            println!("[ServoPlayerEsp] hold for {ms}ms");
            thread::sleep(Duration::from_millis(*ms));
        }
    }
}

// Generic program that only needs a `Servo`.
fn center_servo(servo: &impl Servo) {
    servo.set_degrees(90);
}

// Generic program that needs a `ServoPlayer`.
fn run_wave(player: &impl ServoPlayer) {
    player.animate(&[
        (0, 120),
        (45, 100),
        (90, 100),
        (135, 100),
        (180, 120),
        (135, 100),
        (90, 100),
        (45, 100),
        (0, 120),
    ]);
}

fn main() {
    let servo = ServoEsp;
    let servo_player = ServoPlayerEsp;

    center_servo(&servo);
    // `ServoPlayer` can do everything `Servo` can!
    center_servo(&servo_player);
    // and more.
    run_wave(&servo_player);
}
