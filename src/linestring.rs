use crate::vector2::Vector2;

use crate::pairable::Pairable;


pub trait LineString {
	//fn iter_segments(&self) -> std::iter::Zip<std::slice::Iter<Vector2>, std::slice::Iter<Vector2>>;
	fn magnitude(&self) -> f64;
	fn measured_segments(&self) -> (Vec<(Vector2, Vector2, f64)>, f64);
	fn offset_segments(&self, distance: f64) -> Vec<(Vector2, Vector2, f64)>;
	fn cut_linestring(&self, fraction_of_length: f64) -> (Option<(Vec<(Vector2, Vector2, f64)>, f64)>,Option<(Vec<(Vector2, Vector2, f64)>, f64)>);
	fn direction(&self, fraction_of_length:f64) -> f64;
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
	fn measured_segments(&self) -> (Vec<(Vector2, Vector2, f64)>, f64) {
		// let mut result: Vec<(Vector2, Vector2, f64)> = Vec::with_capacity(self.len());
		// for i in 0..self.len() - 1 {
		// 	let a = self[i];
		// 	let b = self[i - 1];
		// 	result.push((a, b, (b - a).magnitude()));
		// }
		// result
		let mut sum_mag = 0f64;
		let mut vec_part: Vec<(Vector2, Vector2, f64)> = Vec::with_capacity(self.len()-1);
		for (&a, &b) in self.pairwise(){
			let ab_mag = a.distance_to(b);
			sum_mag+=ab_mag;
			vec_part.push((a, b, ab_mag));
		}
		(vec_part, sum_mag)
		
	}
	// fn joined_offset_segments(&self, distance:f64) -> Vec<Vector2>{
	//   	let segments = self.offset_segments(distance);
	// }
	fn cut_linestring(&self,fraction_of_length: f64) ->  (Option<(Vec<(Vector2, Vector2, f64)>, f64)>,Option<(Vec<(Vector2, Vector2, f64)>, f64)>) {
		let mls = self.clone().measured_segments();
		mls.cut_linestring(fraction_of_length)
	}

	fn direction(&self, fraction_of_length: f64) -> f64{
		self.measured_segments().direction(fraction_of_length)
	}
}

impl LineString for (Vec<(Vector2, Vector2, f64)>, f64) {
	fn magnitude(&self) -> f64 {
		let (_measured_segments, self_magnitude) = self;
		return *self_magnitude;
	}
	fn offset_segments(&self, distance: f64) -> Vec<(Vector2, Vector2, f64)> {
		let (measured_segments, _total_length) = self;
		measured_segments.iter()
			.map(move |(a, b, c)| {
				let offset_vector = (b - a).left().unit() * distance;
				(a + &offset_vector, b + &offset_vector, *c)
			})
			.collect()
	}
	fn measured_segments(&self) -> (Vec<(Vector2, Vector2, f64)>, f64) {
		self.clone()
	}
	fn cut_linestring(&self, fraction_of_length: f64) -> (Option<(Vec<(Vector2, Vector2, f64)>, f64)>,Option<(Vec<(Vector2, Vector2, f64)>, f64)>) {
		let (measured_segments, self_magnitude) = self;
		
		let distance_along = self_magnitude * fraction_of_length;

		if distance_along <= 0f64 {
			return (None, Some(self.clone()));
		} else if distance_along >= *self_magnitude {
			return (Some(self.clone()), None);
		} else {
			let mut distance_so_far = 0f64;
			let mut distance_remaining = distance_along;
			for (index, (a, b, segment_length)) in measured_segments.iter().enumerate() {
				if distance_remaining <= 0f64 {
					return (
						Some((measured_segments[..index + 1].to_vec(), distance_along)),
						Some((measured_segments[index..].to_vec(), self_magnitude-distance_along)),
					);
				} else if distance_remaining < *segment_length {
					let ab_unit = (b - a) / *segment_length;
					let intermediate_point = *a + ab_unit * distance_remaining;

					let mut part_1 = measured_segments[..index].to_vec();
					part_1.push((*a, intermediate_point, distance_remaining));

					let mut part_2 = vec![(intermediate_point, *b, segment_length - distance_remaining)];
					part_2.append(&mut measured_segments[index+1..].to_vec());

					return (
						Some((
							part_1,
							distance_so_far+distance_remaining
						)), 
						Some((
							part_2,
							self_magnitude-distance_so_far-distance_remaining
						))
					);
				}
				distance_so_far += segment_length;
				distance_remaining -= segment_length
			}
		}
		return (None, None);
	}

	fn direction(&self, fraction_of_length: f64) -> f64{
		let (measured_segments, self_magnitude) = self;
		let de_normalised_distance_along = self_magnitude * fraction_of_length;
		let mut len_so_far = 0f64;
		for (a, b, segment_length) in measured_segments{
			len_so_far += segment_length;
			if len_so_far >= de_normalised_distance_along{
				return (b-a).direction();
			}
		}
		return 0f64;
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
			(
				vec![
					(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0f64),
					(Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0), 1.0f64)
				],
				2f64
			)
		);
	}
	#[test]
	fn test_cut() {
		let ls = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
		];
		//println!("{:?}", ls);
		let ls_c = ls.cut_linestring(0.5f64);
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_c,
			(
				Some((
					vec![
						(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0f64),
						(Vector2::new(1.0, 0.0), Vector2::new(1.0, 0.5), 0.5f64)
					],
					1.5f64
				)),
				Some((
					vec![
						(Vector2::new(1.0, 0.5), Vector2::new(1.0, 1.0), 0.5f64),
						(Vector2::new(1.0, 1.0), Vector2::new(0.0, 1.0), 1.0f64)
					],
					1.5f64
				))
			)
		);
		let ls_c = ls.cut_linestring(0f64);
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_c,
			(
				None,
				Some(ls.measured_segments())
			)
		);

		let ls_c = ls.cut_linestring(1f64);
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_c,
			(
				Some(ls.measured_segments()),
				None,
			)
		);
	}
}
