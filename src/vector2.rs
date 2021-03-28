use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}

impl Vector2 {
	pub fn new(x: f64, y: f64) -> Vector2 {
		Vector2 { x, y }
	}
	pub fn magnitude_squared(&self) -> f64 {
		self.x * self.x + self.y * self.y
	}
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	/// Returns direction in radians
	pub fn direction(&self) -> f64{
		(self.y).atan2(self.x)
	}
	pub fn distance_to(&self, other: Vector2) -> f64 {
		let dx = other.x - self.x;
		let dy = other.y - self.y;
		(dx * dx + dy * dy).sqrt()
	}
	pub fn dot(&self, other: Vector2) -> f64 {
		self.x * other.x + self.y * other.y
	}
	pub fn cross(&self, other: Vector2) -> f64 {
		self.x * other.y - self.y * other.x
	}
	pub fn left(&self) -> Self {
		Vector2::new(-self.y, self.x)
	}
	pub fn right(&self) -> Self {
		Vector2::new(self.y, -self.x)
	}
	pub fn unit(&self) -> Self {
		let mag = self.magnitude();
		Vector2::new(self.x / mag, self.y / mag)
	}
}

impl ops::Add for Vector2 {
	type Output = Vector2;
	fn add(self, other: Vector2) -> Vector2 {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}
impl<'a> ops::Add<&'a Vector2> for &Vector2 {
	type Output = Vector2;
	fn add(self, other: &'a Vector2) -> Vector2 {
		Vector2 {
			x: self.x + &other.x,
			y: self.y + &other.y,
		}
	}
}

impl ops::Sub for Vector2 {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}
impl<'a> ops::Sub<&'a Vector2> for &Vector2 {
	type Output = Vector2;
	fn sub(self, other: &'a Vector2) -> Vector2 {
		Vector2 {
			x: self.x - &other.x,
			y: self.y - &other.y,
		}
	}
}

impl ops::Neg for Vector2 {
	type Output = Self;
	fn neg(self) -> Self {
		Vector2 {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl ops::Div<f64> for Vector2 {
	type Output = Self;
	fn div(self, other: f64) -> Self {
		Self {
			x: self.x / other,
			y: self.y / other,
		}
	}
}
impl ops::Div<f64> for &Vector2 {
	type Output = Vector2;
	fn div(self, other: f64) -> Vector2 {
		Vector2 {
			x: self.x / other,
			y: self.y / other,
		}
	}
}

impl ops::Mul<f64> for Vector2 {
	type Output = Self;
	fn mul(self, other: f64) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
		}
	}
}




#[cfg(test)]
mod tests {
    use crate::vector2::{Vector2};

    #[test]
    fn create_vector() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn equality_derived() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0, 2.0);
        let v3 = Vector2::new(1.0001, 2.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
    #[test]
    fn clone_derived() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = v1.clone();
        assert_eq!(v1, v2);
        
    }
    #[test]
    fn magnitude_squared() {
        let v1 = Vector2::new(3.0, 4.0);
        assert_eq!(v1.magnitude_squared(), 25.0);
    }
    #[test]
    fn magnitude() {
        let v1 = Vector2::new(3.0, 4.0);
        assert_eq!(v1.magnitude(), 5.0);
    }
    #[test]
    fn sub() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(1.0, 1.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 2.0);
        assert_eq!(v3.y, 3.0);
    }
    #[test]
    fn add() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(1.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 5.0);
    }
    #[test]
    fn neg() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = -v1;
        assert_eq!(v2.x, -3.0);
        assert_eq!(v2.y, -4.0);
    }
    #[test]
    fn mul() {
        let v1 = Vector2::new(3.0, 4.0);
        let v3 = v1*2.0;
        assert_eq!(v3.x, 6.0);
        assert_eq!(v3.y, 8.0);
    }
    #[test]
    fn div() {
        let v1 = Vector2::new(3.0, 4.0);
        let v3 = v1/2.0;
        assert_eq!(v3.x, 1.5);
        assert_eq!(v3.y, 2.0);
    }
    #[test]
    fn dot() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(5.0, 6.0);
        assert_eq!(v1.dot(v2), 3.0*5.0 + 4.0*6.0);
    }
    #[test]
    fn cross() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = Vector2::new(5.0, 6.0);
        assert_eq!(v1.cross(v2), 3.0*6.0 - 4.0*5.0);
    }
    #[test]
    fn left() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.left();
        assert_eq!(v2.x, -4.0);
        assert_eq!(v2.y, 3.0);
    }
    #[test]
    fn right() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.right();
        assert_eq!(v2.x, 4.0);
        assert_eq!(v2.y, -3.0);
    }
    #[test]
    fn unit() {
        let v1 = Vector2::new(3.0, 4.0);
        let v2 = v1.unit();
        let v3 = v2 * v1.magnitude();
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 4.0);
    }
}