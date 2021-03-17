/// GeometricSeries-Type struct.
///
/// #Variants
///
/// first_number - Type i32.
/// second_number - Type i32.
/// ratio - Type i32.

pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

/// Definition of trait 'Iterator'.
pub trait Iterator {
    fn geometric_series_generator(&mut self) -> Vec<i32>;
}
/// Implementing trait 'Iterator' for GeometricSeries.
impl Iterator for GeometricSeries {
    /// geometric_series_generator produces GP of given first value with given ratio.
    ///
    /// #Argument
    ///
    /// &mut self - Referenced mutable variable of type Self.
    ///
    /// #Return
    ///
    /// Return a vector of type i32 having all the geometric progression.
    fn geometric_series_generator(&mut self) -> Vec<i32> {
        let mut store_value: Vec<i32> = Vec::new();
        let start = self.first_number;
        let end = self.first_number + 12;
        for _term in start..end {
            self.current_number = self.first_number + self.ratio.pow(_term as u32);
            store_value.push(self.current_number)
        }
        store_value
    }
}
