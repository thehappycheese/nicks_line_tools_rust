use crate::pairable::Pairable;
use crate::vector2::Vector2;

#[derive(Clone, Debug, PartialEq)]
pub struct LineString {
	pub points: Vec<Vector2>,
}

#[derive(Clone, Debug,PartialEq)]
pub struct LineSegmentMeasured {
	a: Vector2,
	b: Vector2,
	mag: f64,
}

#[derive(Clone, Debug,PartialEq)]
pub struct LineStringMeasured {
	segments: Vec<LineSegmentMeasured>,
	mag: f64,
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
	fn double_cut_linestring(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	);
	fn cut_linestring(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>);
	fn direction(&self, fraction_of_length: f64) -> f64;

	/// Offset algorithim that performs no cleanup.
	/// Each segment is offset, then rejoined in the most naieve way.
	fn basic_offset(&self, distance: f64) -> Option<LineString>;
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

	fn cut_linestring(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>) {
		let mls = self.clone().measured_segments();
		mls.cut_linestring(fraction_of_length)
	}
	fn double_cut_linestring(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	) {
		let mls = self.clone().measured_segments();
		mls.double_cut_linestring(fraction_of_length_start, fraction_of_length_end)
	}

	fn direction(&self, fraction_of_length: f64) -> f64 {
		self.measured_segments().direction(fraction_of_length)
	}

	fn basic_offset(&self, distance: f64) -> Option<LineString> {
		self.measured_segments().basic_offset(distance)
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

	fn cut_linestring(
		&self,
		fraction_of_length: f64,
	) -> (Option<LineStringMeasured>, Option<LineStringMeasured>) {
		//let (segments, mag) = self;
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
					part_2.append(&mut self.segments[index + 1..].to_vec());

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

	fn double_cut_linestring(
		&self,
		fraction_of_length_start: f64,
		fraction_of_length_end: f64,
	) -> (
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
		Option<LineStringMeasured>,
	) {
		let (a, bc) = self.cut_linestring(fraction_of_length_start);
		match bc {
			Some(bc) => {
				let a_fraction_of_length = f64::max(fraction_of_length_start, 0f64);
				let bc_fraction_of_length = 1f64 - a_fraction_of_length;
				// if bc_fraction_of_length <= 0f64 {
				// 	return (a, None, None)
				// }
				let (b, c) = bc.cut_linestring(
					(fraction_of_length_end - a_fraction_of_length) / bc_fraction_of_length,
				);
				(a, b, c)
			}
			None => (a, None, None),
		}
	}

	fn direction(&self, fraction_of_length: f64) -> f64 {
		let de_normalised_distance_along = self.mag * fraction_of_length;
		let mut len_so_far = 0f64;
		for LineSegmentMeasured{a, b, mag:segment_length} in &self.segments {
			len_so_far += segment_length;
			if len_so_far >= de_normalised_distance_along {
				return (b - a).direction();
			}
		}
		return 0f64;
	}

	fn basic_offset(&self, distance: f64) -> Option<LineString> {
		
		if self.segments.len() == 0 {
			return None;
		}

		let offset_segments = self.offset_segments(distance);

		let mut points = Vec::with_capacity(offset_segments.len() + 5);
		
		points.push(offset_segments[0].a);

		for (mseg1, mseg2) in offset_segments.pairwise() {
			let LineSegmentMeasured{a, b, mag:_} = mseg1;
			let LineSegmentMeasured{a:c, b:d, mag:_} = mseg2;
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
		Some(LineString{points})
	}
}

#[cfg(test)]
mod tests {
	use crate::linestring::{LineStringy, LineString, LineStringMeasured, LineSegmentMeasured};
	use crate::vector2::Vector2;

	#[test]
	fn test_linestring_length() {
		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		]};
		assert_eq!(ls.magnitude(), 2f64);
	}

	#[test]
	fn test_offset_segments() {
		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		]};
		assert_eq!(
			ls.offset_segments(1f64),
			vec![
				LineSegmentMeasured{a:Vector2::new(0.0, 1.0), b:Vector2::new(1.0, 1.0), mag:1.0f64},
				LineSegmentMeasured{a:Vector2::new(0.0, 0.0), b:Vector2::new(0.0, 1.0), mag:1.0f64},
			]
		);
	}
	#[test]
	fn test_measured() {
		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		]};
		//println!("{:?}", ls);
		let ls_m = ls.measured_segments();
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_m,
			LineStringMeasured{
				segments:vec![
					LineSegmentMeasured{a:Vector2::new(0.0, 0.0), b:Vector2::new(1.0, 0.0), mag:1.0f64},
					LineSegmentMeasured{a:Vector2::new(1.0, 0.0), b:Vector2::new(1.0, 1.0), mag:1.0f64}
				],
				mag:2f64
			}
		);
	}
	#[test]
	fn test_cut() {
		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
		]};
		//println!("{:?}", ls);
		let ls_c = ls.cut_linestring(0.5f64);
		//println!("{:?}", ls_m);
		assert_eq!(
			ls_c,
			(
				Some(LineStringMeasured{
					segments:vec![
						LineSegmentMeasured{a:Vector2::new(0.0, 0.0), b:Vector2::new(1.0, 0.0), mag:1.0f64},
						LineSegmentMeasured{a:Vector2::new(1.0, 0.0), b:Vector2::new(1.0, 0.5), mag:0.5f64}
					],
					mag:1.5f64
				}),
				Some(LineStringMeasured{
					segments:vec![
						LineSegmentMeasured{a:Vector2::new(1.0, 0.5), b:Vector2::new(1.0, 1.0), mag:0.5f64},
						LineSegmentMeasured{a:Vector2::new(1.0, 1.0), b:Vector2::new(0.0, 1.0), mag:1.0f64}
					],
					mag:1.5f64
				})
			)
		);
		let ls_c = ls.cut_linestring(0f64);
		//println!("{:?}", ls_m);
		assert_eq!(ls_c, (None, Some(ls.measured_segments())));

		let ls_c = ls.cut_linestring(1f64);
		//println!("{:?}", ls_m);
		assert_eq!(ls_c, (Some(ls.measured_segments()), None,));

		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(0.0, 2.0),
		]};
		let (a, b) = ls.cut_linestring(0.5f64);
		assert_eq!(
			b,
			Some(LineStringMeasured{
				segments:vec![
					LineSegmentMeasured{a:Vector2::new(1.0, 1.0), b:Vector2::new(0.0, 1.0), mag:1.0f64},
					LineSegmentMeasured{a:Vector2::new(0.0, 1.0), b:Vector2::new(0.0, 2.0), mag:1.0f64}
				],
				mag:2f64
			})
		);
		assert_eq!(
			a,
			Some(LineStringMeasured{
				segments:vec![
					LineSegmentMeasured{a:Vector2::new(0.0, 0.0), b:Vector2::new(1.0, 0.0), mag:1.0f64},
					LineSegmentMeasured{a:Vector2::new(1.0, 0.0), b:Vector2::new(1.0, 1.0), mag:1.0f64}
				],
				mag:2f64
			})
		);
	}

	#[test]
	fn test_double_cut() {
		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(0.0, 2.0),
		]};
		//println!("{:?}", ls);
		let (a, b, c) = ls.double_cut_linestring(0.5f64, 0.75f64);
		assert_eq!(
			a,
			Some(LineStringMeasured{
				segments:vec![
					LineSegmentMeasured{a:Vector2::new(0.0, 0.0), b:Vector2::new(1.0, 0.0), mag:1.0f64},
					LineSegmentMeasured{a:Vector2::new(1.0, 0.0), b:Vector2::new(1.0, 1.0), mag:1.0f64}
				],
				mag:2f64
			})
		);
		assert_eq!(
			b,
			Some(LineStringMeasured{
				segments:vec![LineSegmentMeasured{a:Vector2::new(1.0, 1.0), b:Vector2::new(0.0, 1.0), mag:1.0f64},],
				mag:1f64
			})
		);
		assert_eq!(
			c,
			Some(LineStringMeasured{
				segments:vec![LineSegmentMeasured{a:Vector2::new(0.0, 1.0), b:Vector2::new(0.0, 2.0), mag:1.0f64},],
				mag:1f64
			})
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

		let ls = LineString{points:vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(1.5, 2.0),
			Vector2::new(1.0, 3.0),
		]};
		let lsbo = ls.basic_offset(-0.5f64);
		println!("{:?}", lsbo);
		assert_eq!(
			lsbo,
			Some(LineString{points:vec![
				Vector2 {
					x: 0.35355339059327373,
					y: -0.35355339059327373
				},
				Vector2 {
					x: 1.4109272075633472,
					y: 0.7038204263767998
				},
				Vector2 {
					x: 2.0590169943749475,
					y: 2.0
				},
				Vector2 {
					x: 1.4472135954999579,
					y: 3.223606797749979
				}
			]})
		);
	}
}
