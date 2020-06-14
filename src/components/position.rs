use super::*;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Position {
        pub x: Length,
        pub y: Length,
    }

    impl Position {
        pub const fn in_m(x: f64, y: f64) -> Self {
            Self { 
                x: Length::in_m(x),
                y: Length::in_m(y)
            }
        }

        pub fn in_ly(x: f64, y: f64) -> Self {
            Self::in_m(x * Self::M_PER_LY, y * Self::M_PER_LY)
        }

        const M_PER_LY: f64 = 9.460_730_472_580_800e15;
    }

    impl From<Distance> for Position {
        fn from(value: Distance) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl Add<Distance> for Position {
        type Output = Position;
        fn add(self, rhs: Distance) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }    
    }

    impl Add<&Distance> for Position {
        type Output = Position;
        fn add(self, rhs: &Distance) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }    
    }

    impl AddAssign<Distance> for Position {
        fn add_assign(&mut self, rhs: Distance) {
            self.x += rhs.x;
            self.y += rhs.y;
        }    
    }

    impl AddAssign<&Distance> for Position {
        fn add_assign(&mut self, rhs: &Distance) {
            self.x += rhs.x;
            self.y += rhs.y;
        }    
    }

    impl Sub<Distance> for Position {
        type Output = Position;
        fn sub(self, rhs: Distance) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y
            }
        }    
    }

    impl Sub<&Distance> for Position {
        type Output = Position;
        fn sub(self, rhs: &Distance) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y
            }
        }    
    }

    impl Sub<Distance> for &Position {
        type Output = Position;
        fn sub(self, rhs: Distance) -> Self::Output {
            Self::Output {
                x: self.x - rhs.x,
                y: self.y - rhs.y
            }
        }    
    }

    impl Sub<&Distance> for &Position {
        type Output = Position;
        fn sub(self, rhs: &Distance) -> Self::Output {
            Self::Output {
                x: self.x - rhs.x,
                y: self.y - rhs.y
            }
        }    
    }

    impl SubAssign<Distance> for Position {
        fn sub_assign(&mut self, rhs: Distance) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }    
    }

    impl SubAssign<&Distance> for Position {
        fn sub_assign(&mut self, rhs: &Distance) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }    
    }

    impl Sub for Position {
        type Output = Distance;
        fn sub(self, rhs: Position) -> Distance {
            Distance {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Sub<&Position> for Position {
        type Output = Distance;
        fn sub(self, rhs: &Position) -> Distance {
            Distance {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Sub<Position> for &Position {
        type Output = Distance;
        fn sub(self, rhs: Position) -> Distance {
            Distance {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Sub<&Position> for &Position {
        type Output = Distance;
        fn sub(self, rhs: &Position) -> Distance {
            Distance {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }