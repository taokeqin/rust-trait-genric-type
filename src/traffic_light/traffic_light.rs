pub enum TrafficLight {
    Green,
    Red,
    Yellow
}

impl TrafficLight {
    pub fn time(&self) -> u32 {
        match *self {
            TrafficLight::Green => 100,
            TrafficLight::Red => 200,
            TrafficLight::Yellow => 300
        }
    }
}