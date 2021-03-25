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

impl ops::Mul<f64> for Vector2 {
	type Output = Self;
	fn mul(self, other: f64) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
		}
	}
}
