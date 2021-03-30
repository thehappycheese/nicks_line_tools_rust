use crate::vector2::Vector2;
use crate::pairable::Pairable;


pub type LineStringPlain = Vec<Vector2>;
pub type LineSegmentMeasured = (Vector2, Vector2, f64);
pub type LineStringMeasured = (Vec<LineSegmentMeasured>, f64);


pub trait LineSegmenty{
	fn intersect(self, other:LineSegmentMeasured) -> Option<(Vector2, f64, f64)>;
}

impl LineSegmenty for LineSegmentMeasured{
	
	fn intersect(self, other:LineSegmentMeasured) -> Option<(Vector2, f64, f64)>{
		let (a, b, _ab_len) = self;
		let ab =  b - a;
		let (c, d, _cd_len) = other;
		let cd =  d - c;

		let ab_cross_cd = ab.cross(cd);
		if ab_cross_cd == 0f64{
			return None;
		}
		let ac = c - a;
		let time_ab =  ac.cross(cd) / ab_cross_cd;
		let time_cd = -ab.cross(ac) / ab_cross_cd;

		Some((a+ab*time_ab, time_ab, time_cd))
	}
}


pub trait LineStringy {
	//fn iter_segments(&self) -> std::iter::Zip<std::slice::Iter<Vector2>, std::slice::Iter<Vector2>>;
	fn magnitude(&self) -> f64;
	fn measured_segments(&self) -> LineStringMeasured;
	fn offset_segments(&self, distance: f64) -> Vec<LineSegmentMeasured>;
	fn cut_linestring(&self, fraction_of_length: f64) -> (Option<LineStringMeasured>, Option<LineStringMeasured>);
	fn direction(&self, fraction_of_length:f64) -> f64;

	/// Offset algorithim that performs no cleanup. 
	/// Each segment is offset, then rejoined in the most naieve way.
	fn basic_offset(&self, distance:f64) -> Option<LineStringPlain>;
}


impl LineStringy for LineStringPlain {

	fn magnitude(&self) -> f64 {
		self.pairwise().map(|(a, b)| (b - a).magnitude()).sum()
	}

	fn offset_segments<'a>(self: &'a Self, distance: f64) -> Vec<LineSegmentMeasured> {
		self.pairwise()
			.map(move |(a, b)| {
				let ab = b - a;
				let mag = ab.magnitude();
				let offset_vector = ab.left() / mag * distance;
				(a + &offset_vector, b + &offset_vector, mag)
			})
			.collect()
	}

	fn measured_segments(&self) -> LineStringMeasured {
		let mut sum_mag = 0f64;
		let mut vec_part: Vec<LineSegmentMeasured> = Vec::with_capacity(self.len()-1);
		for (&a, &b) in self.pairwise(){
			let ab_mag = a.distance_to(b);
			sum_mag+=ab_mag;
			vec_part.push((a, b, ab_mag));
		}
		(vec_part, sum_mag)
	}
	
	fn cut_linestring(&self,fraction_of_length: f64) -> (Option<LineStringMeasured>, Option<LineStringMeasured>) {
		let mls = self.clone().measured_segments();
		mls.cut_linestring(fraction_of_length)
	}

	fn direction(&self, fraction_of_length: f64) -> f64{
		self.measured_segments().direction(fraction_of_length)
	}

	fn basic_offset(&self, distance: f64)->Option<LineStringPlain>{
		self.measured_segments().basic_offset(distance)
	}
}


impl LineStringy for LineStringMeasured {

	fn magnitude(&self) -> f64 {
		let (_measured_segments, self_magnitude) = self;
		return *self_magnitude;
	}

	fn offset_segments(&self, distance: f64) -> Vec<LineSegmentMeasured> {
		let (measured_segments, _total_length) = self;
		measured_segments.iter()
			.map(move |(a, b, c)| {
				let offset_vector = (b - a).left().unit() * distance;
				(a + &offset_vector, b + &offset_vector, *c)
			})
			.collect()
	}

	fn measured_segments(&self) -> LineStringMeasured {
		self.clone()
	}

	fn cut_linestring(&self, fraction_of_length: f64) -> (Option<LineStringMeasured>, Option<LineStringMeasured>){
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


	fn basic_offset(&self, distance: f64) -> Option<LineStringPlain>{
		let (self_segments, _self_magnitude) = self;
		if self_segments.len() == 0{
			return None
		}
		
		let offset_segments = self.offset_segments(distance);
		// this is not required since the pairwise loop will not run if there is only 1 element
		// if measured_segments.len()==1 {
		// 	let (a, b, _) = offset_segments[0];
		// 	return Some(vec![a,b])
		// }
		
		
		let mut result:LineStringPlain = Vec::with_capacity(offset_segments.len() + 5);
		
		let (first_point, _, _) = offset_segments[0];
		result.push(first_point);

		for (mseg1, mseg2) in offset_segments.pairwise(){
			let (a, b, _) = mseg1;
			let (c, d, _) = mseg2;
			let ab = b-a;
			let cd = d-c;
			if ab.cross(cd).abs()<0.00000001{
				result.push(*b);
			}else if let Some((intersection_point, time_ab, time_cd)) = mseg1.intersect(*mseg2){

				let tip_ab = 0f64 <= time_ab && time_ab <= 1f64;
				let fip_ab = !tip_ab;
				let pfip_ab = fip_ab && time_ab > 0f64;
				
				let tip_cd = 0f64 <= time_cd && time_cd <= 1f64;
				let fip_cd = !tip_cd;

				if tip_ab && tip_cd{
					// Case 2a
					// TODO: test for mitre limit
					result.push(intersection_point);
				}else if fip_ab && fip_cd{
					// Case 2b.
					if pfip_ab{
						// TODO: test for mitre limit
						result.push(intersection_point);
					}else{
						result.push(*b);
						result.push(*c);
					}
				}else{
					// Case 2c. (either ab or cd
					result.push(*b);
					result.push(*c);
				}
			}

		}
		let (_, last_point, _) = offset_segments[offset_segments.len()-1];
		result.push(last_point);
		Some(result)
	}

}






#[cfg(test)]
mod tests {
	use crate::linestring::LineStringy;
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

	#[test]
	fn test_linestring_basic_offset() {
		// let ls = vec![
		// 	Vector2::new(0.0, 0.0),
		// 	Vector2::new(1.0, 0.0),
		// 	Vector2::new(1.0, 1.0),
		// ];
		// let lsos = ls.offset_segments(5f64);
		// println!("{:?}",lsos);
		// let lsbo = ls.basic_offset(5f64);
		// println!("{:?}",lsbo);

		let ls = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(1.5, 2.0),
			Vector2::new(1.0, 3.0),
		];
		let lsbo = ls.basic_offset(-0.5f64);
		println!("{:?}",lsbo);
		assert_eq!(
			lsbo,
			Some(vec![Vector2 { x: 0.35355339059327373, y: -0.35355339059327373 }, Vector2 { x: 1.4109272075633472, y: 0.7038204263767998 }, Vector2 { x: 2.0590169943749475, y: 2.0 }, Vector2 { x: 1.4472135954999579, y: 3.223606797749979 }])
		);
	}
}
