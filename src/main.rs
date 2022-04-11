mod traffic_light;
mod shape;
mod sum;
use sum::sum_int_array::sum_int_array;
fn main() {
    // traffic lights
    let tl_green = traffic_light::traffic_light::TrafficLight::Green;
    let tl_red = traffic_light::traffic_light::TrafficLight::Red;
    let tl_yellow = traffic_light::traffic_light::TrafficLight::Yellow;
    println!("green light time: {}", tl_green.time());
    println!("red light time: {}", tl_red.time());
    println!("yellow light time: {}", tl_yellow.time());

    // sum integer array
    let a = vec!(1567,2,8);
    println!("sum of array: {}", sum_int_array(&a).unwrap_or(0));

    // area
    let square = shape::square::Square{length: 20.0};
    println!("area of square: {}", shape::area::calculate_ares(&square));
    let circle = shape::circle::Circle{radius: 5.0};
    println!("area of circle: {}", shape::area::calculate_ares(&circle));

}
