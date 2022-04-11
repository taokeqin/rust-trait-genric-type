pub trait CanCalculateArea{
    fn calculate_area(&self) -> f64;
}

pub fn calculate_ares<T: CanCalculateArea>(object: &T) -> f64{
    object.calculate_area()
}