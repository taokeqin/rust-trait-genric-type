pub struct Square{
    pub length: f64
}

impl super::area::CanCalculateArea for Square{
    fn calculate_area(&self) -> f64{
        return self.length * self.length
    }
}