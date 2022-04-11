pub struct Circle{
    pub radius: f64
}

const PI:f64 = 3.14159;
impl super::area::CanCalculateArea for Circle{
    fn calculate_area(&self) -> f64{
        return PI * self.radius * self.radius
    }
}