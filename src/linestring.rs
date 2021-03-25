use crate::vector2::Vector2;

pub trait Pairable<P>: IntoIterator {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<P>, std::slice::Iter<P>>;
}
impl<T> Pairable<T> for Vec<T> {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<T>, std::slice::Iter<T>> {
		self.iter().zip(self[1..].iter())
	}
}

pub trait LineString {
	//fn iter_segments(&self) -> std::iter::Zip<std::slice::Iter<Vector2>, std::slice::Iter<Vector2>>;
	fn magnitude(&self) -> f64;
	fn measured_segments(self) -> Vec<(Vector2, Vector2, f64)>;
	fn offset_segments(&self, distance: f64) -> Vec<(Vector2, Vector2, f64)>;
	//fn joined_offset_segments(&self, distance:f64) -> Vec<Vector2>;
}

impl LineString for Vec<Vector2> {
	// fn iter_segments(&self) -> std::iter::Zip<std::slice::Iter<Vector2>, std::slice::Iter<Vector2>>{
	// 	self.iter().zip(self[1..].iter())
	// }
	fn magnitude(&self) -> f64 {
		self.pairwise().map(|(a, b)| (b - a).magnitude()).sum()
	}
	fn offset_segments<'a>(self: &'a Self, distance: f64) -> Vec<(Vector2, Vector2, f64)> {
		self.pairwise()
			.map(move |(a, b)| {
				let ab = b - a;
				let mag = ab.magnitude();
				let offset_vector = ab.left() / mag * distance;
				(a + &offset_vector, b + &offset_vector, mag)
			})
			.collect()
	}
	fn measured_segments(self) -> Vec<(Vector2, Vector2, f64)> {
		// let mut result: Vec<(Vector2, Vector2, f64)> = Vec::with_capacity(self.len());
		// for i in 0..self.len() - 1 {
		// 	let a = self[i];
		// 	let b = self[i - 1];
		// 	result.push((a, b, (b - a).magnitude()));
		// }
		// result
		self.pairwise()
			.map(|(&a, &b)| (a, b, (b - a).magnitude()))
			.collect()
	}
	// fn joined_offset_segments(&self, distance:f64) -> Vec<Vector2>{
	//   	let segments = self.offset_segments(distance);
	// }
}

impl LineString for Vec<(Vector2, Vector2, f64)> {
	fn magnitude(&self) -> f64 {
		self.iter().map(|(_a, _b, c)| c).sum()
	}
	fn offset_segments(&self, distance: f64) -> Vec<(Vector2, Vector2, f64)> {
		self.iter()
			.map(move |(a, b, c)| {
				let offset_vector = (b - a).left().unit() * distance;
				(a + &offset_vector, b + &offset_vector, *c)
			})
			.collect()
	}
	fn measured_segments(self) -> Vec<(Vector2, Vector2, f64)> {
		self
	}
}

#[cfg(test)]
mod tests {
	use crate::linestring::LineString;
	use crate::vector2::Vector2;

	#[test]
	fn test_linestring_length() {
		let ls = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		];
		assert_eq!(ls.magnitude(), 2f64);
	}

	#[test]
	fn test_offset_segments() {
		let ls = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		];
		assert_eq!(
			ls.offset_segments(1f64),
			vec![
				(Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0), 1.0f64),
				(Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0), 1.0f64),
			]
		);
	}
	#[test]
	fn test_measured() {
		let ls = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		];
		//println!("{:?}", ls);
		let ls_m = ls.measured_segments();
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_m,
			vec![
				(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0f64),
				(Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0), 1.0f64)
			]
		);
	}
}
