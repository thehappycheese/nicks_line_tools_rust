use crate::pairable::Pairable;
use crate::vector2::Vector2;

#[derive(Clone, Debug, PartialEq)]
pub struct LineString {
	pub points: Vec<Vector2>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineSegmentMeasured {
	pub a: Vector2,
	pub b: Vector2,
	pub mag: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineStringMeasured {
	pub segments: Vec<LineSegmentMeasured>,
	pub mag: f64,
}

pub trait LineSegmenty {
	fn intersect(&self, other: &LineSegmentMeasured) -> Option<(Vector2, f64, f64)>;
}

impl LineSegmenty for LineSegmentMeasured {
	fn intersect(&self, other: &LineSegmentMeasured) -> Option<(Vector2, f64, f64)> {
		let LineSegmentMeasured { a, b, mag: _ab_len } = self;
		let ab = b - a;
		let LineSegmentMeasured {
			a: c,
			b: d,
			mag: _cd_len,
		} = other;
		let cd = d - c;

		let ab_cross_cd = ab.cross(cd);
		if ab_cross_cd == 0f64 {
			return None;
		}
		let ac = c - &a;
		let time_ab = ac.cross(cd) / ab_cross_cd;
		let time_cd = -ab.cross(ac) / ab_cross_cd;

		Some((a + &(ab * time_ab), time_ab, time_cd))
	}
}

pub trait LineStringy {
	//fn iter_segments(&self) -> std::iter::Zip<std::slice::Iter<Vector2>, std::slice::Iter<Vector2>>;
	fn magnitude(&self) -> f64;

	fn measured_segments(&self) -> LineStringMeasured;

	fn offset_segments(&self, distance: f64) -> Vec<LineSegmentMeasured>;

	fn cut_twice(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	);

	fn cut(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>);

	fn direction(&self, fraction_of_length: f64) -> f64;

	/// Offset algorithim that performs no cleanup.
	/// Each segment is offset, then rejoined in the most naive way.
	fn offset_basic(&self, distance: f64) -> Option<LineString>;

	//fn offset(&self, distance: f64) -> Option<LineString>;
}

impl LineStringy for LineString {
	fn magnitude(&self) -> f64 {
		self.points
			.pairwise()
			.map(|(a, b)| (b - a).magnitude())
			.sum()
	}

	fn offset_segments<'a>(self: &'a Self, distance: f64) -> Vec<LineSegmentMeasured> {
		self.points
			.pairwise()
			.map(move |(a, b)| {
				let ab = b - a;
				let mag = ab.magnitude();
				let offset_vector = ab.left() / mag * distance;
				LineSegmentMeasured {
					a: a + &offset_vector,
					b: b + &offset_vector,
					mag,
				}
			})
			.collect()
	}

	fn measured_segments(&self) -> LineStringMeasured {
		let mut sum_mag = 0f64;
		let mut vec_part: Vec<LineSegmentMeasured> = Vec::with_capacity(self.points.len() - 1);
		for (&a, &b) in self.points.pairwise() {
			let ab_mag = a.distance_to(b);
			sum_mag += ab_mag;
			vec_part.push(LineSegmentMeasured { a, b, mag: ab_mag });
		}
		LineStringMeasured {
			segments: vec_part,
			mag: sum_mag,
		}
	}

	fn cut(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>) {
		let mls = self.clone().measured_segments();
		mls.cut(fraction_of_length)
	}

	fn cut_twice(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	) {
		let mls = self.clone().measured_segments();
		mls.cut_twice(fraction_of_length_start, fraction_of_length_end)
	}

	fn direction(&self, fraction_of_length: f64) -> f64 {
		self.measured_segments().direction(fraction_of_length)
	}

	fn offset_basic(&self, distance: f64) -> Option<LineString> {
		self.measured_segments().offset_basic(distance)
	}

	// fn offset(&self, distance: f64) -> Option<LineString> {
	// 	todo!();
	// }
}

impl From<&Vec<Vector2>> for LineStringMeasured {
	fn from(other: &Vec<Vector2>) -> Self {
		let mut sum_mag = 0f64;
		let mut vec_part: Vec<LineSegmentMeasured> = Vec::with_capacity(other.len() - 1);
		for (&a, &b) in other.pairwise() {
			let ab_mag = a.distance_to(b);
			sum_mag += ab_mag;
			vec_part.push(LineSegmentMeasured { a, b, mag: ab_mag });
		}
		LineStringMeasured {
			segments: vec_part,
			mag: sum_mag,
		}
	}
}

impl LineStringMeasured {
	pub fn from_vec(other: &Vec<Vector2>) -> LineStringMeasured {
		other.into()
	}
	pub fn to_line_string(&self) -> LineString {
		self.into()
	}
}

impl From<&LineStringMeasured> for LineString {
	fn from(lsm: &LineStringMeasured) -> LineString {
		let mut points = Vec::with_capacity(lsm.segments.len() + 1);
		points.extend(lsm.segments.iter().map(|item| item.a));
		points.push(lsm.segments[lsm.segments.len() - 1].b);
		LineString { points }
	}
}

impl LineStringy for LineStringMeasured {
	fn magnitude(&self) -> f64 {
		return self.mag;
	}

	fn offset_segments(&self, distance: f64) -> Vec<LineSegmentMeasured> {
		self.segments
			.iter()
			.map(move |LineSegmentMeasured { a, b, mag }| {
				let offset_vector = (b - a).left().unit() * distance;
				LineSegmentMeasured {
					a: a + &offset_vector,
					b: b + &offset_vector,
					mag: *mag,
				}
			})
			.collect()
	}

	fn measured_segments(&self) -> LineStringMeasured {
		self.clone()
	}

	fn cut(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>) {
		let distance_along = self.mag * fraction_of_length;

		if distance_along <= 0f64 {
			return (None, Some(self.clone()));
		} else if distance_along >= self.mag {
			return (Some(self.clone()), None);
		} else {
			let mut distance_so_far = 0f64;
			let mut distance_remaining = distance_along;
			for (
				index,
				LineSegmentMeasured {
					a,
					b,
					mag: segment_length,
				},
			) in self.segments.iter().enumerate()
			{
				if distance_remaining <= 0f64 {
					return (
						Some(LineStringMeasured {
							segments: self.segments[..index].to_vec(),
							mag: distance_along,
						}),
						Some(LineStringMeasured {
							segments: self.segments[index..].to_vec(),
							mag: self.mag - distance_along,
						}),
					);
				} else if distance_remaining < *segment_length {
					let ab_unit = (b - a) / *segment_length;
					let intermediate_point = *a + ab_unit * distance_remaining;

					let mut part_1 = self.segments[..index].to_vec();
					part_1.push(LineSegmentMeasured {
						a: *a,
						b: intermediate_point,
						mag: distance_remaining,
					});

					let mut part_2 = vec![LineSegmentMeasured {
						a: intermediate_point,
						b: *b,
						mag: segment_length - distance_remaining,
					}];
					part_2.append(&mut (self.segments[index + 1..].to_vec()));

					return (
						Some(LineStringMeasured {
							segments: part_1,
							mag: distance_so_far + distance_remaining,
						}),
						Some(LineStringMeasured {
							segments: part_2,
							mag: self.mag - distance_so_far - distance_remaining,
						}),
					);
				}
				distance_so_far += segment_length;
				distance_remaining -= segment_length
			}
		}
		return (None, None);
	}

	fn cut_twice(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	) {
		let (a, bc) = self.cut(fraction_of_length_start);
		match bc {
			Some(bc) => {
				let a_fraction_of_length = f64::max(fraction_of_length_start, 0f64);
				let bc_fraction_of_length = 1f64 - a_fraction_of_length;
				// if bc_fraction_of_length <= 0f64 {
				// 	return (a, None, None)
				// }
				let (b, c) =
					bc.cut((fraction_of_length_end - a_fraction_of_length) / bc_fraction_of_length);
				(a, b, c)
			}
			None => (a, None, None),
		}
	}

	fn direction(&self, fraction_of_length: f64) -> f64 {
		let de_normalised_distance_along = self.mag * fraction_of_length;
		let mut len_so_far = 0f64;
		for LineSegmentMeasured {
			a,
			b,
			mag: segment_length,
		} in &self.segments
		{
			len_so_far += segment_length;
			if len_so_far >= de_normalised_distance_along {
				return (b - a).direction();
			}
		}
		return 0f64;
	}

	fn offset_basic(&self, distance: f64) -> Option<LineString> {
		if self.segments.len() == 0 {
			return None;
		}

		let offset_segments = self.offset_segments(distance);

		let mut points = Vec::with_capacity(offset_segments.len() + 5);
		points.push(offset_segments[0].a);

		for (mseg1, mseg2) in offset_segments.pairwise() {
			let LineSegmentMeasured { a, b, mag: _ } = mseg1;
			let LineSegmentMeasured { a: c, b: d, mag: _ } = mseg2;
			let ab = b - a;
			let cd = d - c;
			if ab.cross(cd).abs() < 0.00000001 {
				points.push(*b);
			} else if let Some((intersection_point, time_ab, time_cd)) = mseg1.intersect(mseg2) {
				let tip_ab = 0f64 <= time_ab && time_ab <= 1f64;
				let fip_ab = !tip_ab;
				let pfip_ab = fip_ab && time_ab > 0f64;
				let tip_cd = 0f64 <= time_cd && time_cd <= 1f64;
				let fip_cd = !tip_cd;

				if tip_ab && tip_cd {
					// Case 2a
					// TODO: test for mitre limit
					points.push(intersection_point);
				} else if fip_ab && fip_cd {
					// Case 2b.
					if pfip_ab {
						// TODO: test for mitre limit
						points.push(intersection_point);
					} else {
						points.push(*b);
						points.push(*c);
					}
				} else {
					// Case 2c. (either ab or cd
					points.push(*b);
					points.push(*c);
				}
			}
		}
		points.push(offset_segments[offset_segments.len() - 1].b);
		Some(LineString { points })
	}

	// fn offset(&self, distance:f64) -> Option<LineString>{
	// 	todo!();
	// }
}
