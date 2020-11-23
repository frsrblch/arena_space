pub const M: Length = Length::in_m(1.0);
pub const KM: Length = Length::in_m(1e3);
pub const AU: Length = Length::in_m(1.495978707e11);
pub const LY: Length = Length::in_m(9.4607e15);

vector_and_scalar! {
    struct Distance([struct Length(f64); 2]) {
        fn in_m(meters) -> Self;
    }
}

impl Distance {
    /// Returns the position vector given an angle and a radius
    ///
    ///  # Arguments
    ///
    /// * `angle` - as measured clockwise from the positive y-axis
    /// * `magnitude` - length of the resulting vector
    pub fn from_angle_and_radius(angle: super::Angle, magnitude: Length) -> Self {
        let x = magnitude * angle.sin();
        let y = magnitude * angle.cos();
        Self { x, y }
    }
}

#[test]
fn vector_and_scalar() {
    let len = 4.0 * M;
    let dist = (2.0, 3.0) * M;

    assert_eq!(Length::in_m(4.0), len);
    assert_eq!(Distance::in_m(2.0, 3.0), dist)
}
