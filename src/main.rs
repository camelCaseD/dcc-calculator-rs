use std::num::Float;
use std::io;

struct Velocity {
    x: f32,
    y: f32,
    v: f32
}

fn main() {
    println!("DCC Tennis Ball Launcher Calculator");

    loop {
        let time: f32 = calculate_time();
        let velocity: Velocity = calculate_velocity(time);
        let angle: f32 = calculate_angle(&velocity);
        let force: f32 = calculate_force(&velocity);

        println!("The time the ball will travel is {}s", time);
        println!("The velocity of the ball will be {} m/s", velocity.v);
        println!("The angle of the launcher should be {}º", angle);
        println!("The force exerted by the launcher on the ball should be {} N", force);
    }
}

fn calculate_time() -> f32 {
    println!("Enter the height of the appeture");
    let input = io::stdin().read_line().ok().expect("Failed to read line");
    let input_num: Option<f32> = input.trim().parse();

    let distance = match input_num {
        Some(distance) => distance,
        None => 0.0
    };

    return Float::sqrt(2.0 * distance / 9.8) * 2.0;
}

fn calculate_velocity(time: f32) -> Velocity {
    println!("Enter the range of the appeture");
    let input = io::stdin().read_line().ok().expect("Failed to read line");
    let input_num: Option<f32> = input.trim().parse();

    let range = match input_num {
        Some(range) => range,
        None => 0.0
    };

    let velocity_x: f32 = range / time;
    let velocity_y: f32 = 9.8 * time;

    let velocity = Velocity { x: velocity_x, y: velocity_y, v: Float::sqrt(Float::powf(velocity_x, 2.0) + Float::powf(velocity_y, 2.0))};

    return velocity;
}

fn calculate_angle(velocity: &Velocity) -> f32 {
    return Float::atan(velocity.y / velocity.x).to_degrees();
}

fn calculate_force(velocity: &Velocity) -> f32 {
    println!("Enter the mass of the ball");
    let input_m = io::stdin().read_line().ok().expect("Failed to read line");
    let input_num_m: Option<f32> = input_m.trim().parse();

    let mass = match input_num_m {
        Some(mass) => mass,
        None => 0.0
    };

    println!("Enter the length of the pipe");
    let input_l = io::stdin().read_line().ok().expect("Failed to read line");
    let input_num_l: Option<f32> = input_l.trim().parse();

    let length = match input_num_l {
        Some(length) => length,
        None => 0.0
    };

    let acceleration: f32 = Float::powf(velocity.v, 2.0) / 2.0 * length;

    return mass * acceleration;
}
