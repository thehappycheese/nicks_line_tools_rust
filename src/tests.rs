
// use crate::LineString::{LineString, LineStringMeasured, LineStringy};
use crate::line_string_measured::{LineStringMeasured};
use crate::line_segment_measured::{LineSegmentMeasured};
use crate::vector2::Vector2;

#[test]
fn test_linestring_length() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
	].into();
	assert_eq!(ls.magnitude(), 2f64);
}

#[test]
fn test_offset_segments() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
	].into();
	assert_eq!(
		ls.offset_segments(1f64),
		vec![
			LineSegmentMeasured {
				a: Vector2::new(0.0, 1.0),
				b: Vector2::new(1.0, 1.0),
				mag: 1.0f64
			},
			LineSegmentMeasured {
				a: Vector2::new(0.0, 0.0),
				b: Vector2::new(0.0, 1.0),
				mag: 1.0f64
			},
		]
	);
}
#[test]
fn test_measured() {
	let ls_m:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
	].into();
	//println!("{:?}", ls);
	// let ls_m = ls.measured_segments();
	//println!("{:?}", ls_m);
	assert_eq!(
		ls_m,
		LineStringMeasured {
			segments: vec![
				LineSegmentMeasured {
					a: Vector2::new(0.0, 0.0),
					b: Vector2::new(1.0, 0.0),
					mag: 1.0f64
				},
				LineSegmentMeasured {
					a: Vector2::new(1.0, 0.0),
					b: Vector2::new(1.0, 1.0),
					mag: 1.0f64
				}
			],
			mag: 2f64
		}
	);
}
#[test]
fn test_cut() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(0.0, 1.0),
	].into();
	//println!("{:?}", ls);
	let ls_c = ls.cut(0.5f64);
	//println!("{:?}", ls_m);
	assert_eq!(
		ls_c,
		(
			Some(LineStringMeasured {
				segments: vec![
					LineSegmentMeasured {
						a: Vector2::new(0.0, 0.0),
						b: Vector2::new(1.0, 0.0),
						mag: 1.0f64
					},
					LineSegmentMeasured {
						a: Vector2::new(1.0, 0.0),
						b: Vector2::new(1.0, 0.5),
						mag: 0.5f64
					}
				],
				mag: 1.5f64
			}),
			Some(LineStringMeasured {
				segments: vec![
					LineSegmentMeasured {
						a: Vector2::new(1.0, 0.5),
						b: Vector2::new(1.0, 1.0),
						mag: 0.5f64
					},
					LineSegmentMeasured {
						a: Vector2::new(1.0, 1.0),
						b: Vector2::new(0.0, 1.0),
						mag: 1.0f64
					}
				],
				mag: 1.5f64
			})
		)
	);
	let ls_c = ls.cut(0f64);
	//println!("{:?}", ls_m);
	assert_eq!(ls_c, (None, Some(ls.clone())));

	let ls_c = ls.cut(1f64);
	//println!("{:?}", ls_m);
	assert_eq!(ls_c, (Some(ls), None,));

	let ls:LineStringMeasured = vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(0.0, 2.0),
		].into();
	let (a, b) = ls.cut(0.5f64);
	assert_eq!(
		b,
		Some(LineStringMeasured {
			segments: vec![
				LineSegmentMeasured {
					a: Vector2::new(1.0, 1.0),
					b: Vector2::new(0.0, 1.0),
					mag: 1.0f64
				},
				LineSegmentMeasured {
					a: Vector2::new(0.0, 1.0),
					b: Vector2::new(0.0, 2.0),
					mag: 1.0f64
				}
			],
			mag: 2f64
		})
	);
	assert_eq!(
		a,
		Some(LineStringMeasured {
			segments: vec![
				LineSegmentMeasured {
					a: Vector2::new(0.0, 0.0),
					b: Vector2::new(1.0, 0.0),
					mag: 1.0f64
				},
				LineSegmentMeasured {
					a: Vector2::new(1.0, 0.0),
					b: Vector2::new(1.0, 1.0),
					mag: 1.0f64
				}
			],
			mag: 2f64
		})
	);
}

#[test]
fn test_double_cut() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(0.0, 1.0),
		Vector2::new(0.0, 2.0),
	].into();
	//println!("{:?}", ls);
	let (a, b, c) = ls.cut_twice(0.5f64, 0.75f64);
	assert_eq!(
		a,
		Some(LineStringMeasured {
			segments: vec![
				LineSegmentMeasured {
					a: Vector2::new(0.0, 0.0),
					b: Vector2::new(1.0, 0.0),
					mag: 1.0f64
				},
				LineSegmentMeasured {
					a: Vector2::new(1.0, 0.0),
					b: Vector2::new(1.0, 1.0),
					mag: 1.0f64
				}
			],
			mag: 2f64
		})
	);
	assert_eq!(
		b,
		Some(LineStringMeasured {
			segments: vec![LineSegmentMeasured {
				a: Vector2::new(1.0, 1.0),
				b: Vector2::new(0.0, 1.0),
				mag: 1.0f64
			},],
			mag: 1f64
		})
	);
	assert_eq!(
		c,
		Some(LineStringMeasured {
			segments: vec![LineSegmentMeasured {
				a: Vector2::new(0.0, 1.0),
				b: Vector2::new(0.0, 2.0),
				mag: 1.0f64
			},],
			mag: 1f64
		})
	);
}




#[test]
fn test_interpolate() {
	let ls = LineStringMeasured::from(vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(0.0, 1.0),
		Vector2::new(0.0, 2.0),
	]);
	// let ls:LineStringMeasured = vec![
	// 	Vector2::new(0.0, 0.0),
	// 	Vector2::new(1.0, 0.0),
	// 	Vector2::new(1.0, 1.0),
	// 	Vector2::new(0.0, 1.0),
	// 	Vector2::new(0.0, 2.0),
	// ].into();
	assert_eq!(Some(Vector2::new(1.0, 1.0)), ls.interpolate( 0.50));
	assert_eq!(Some(Vector2::new(1.0, 0.0)), ls.interpolate( 0.25));
	assert_eq!(Some(Vector2::new(0.5, 0.0)), ls.interpolate( 0.125));
	assert_eq!(Some(Vector2::new(0.0, 0.0)), ls.interpolate(-1.00));
	assert_eq!(Some(Vector2::new(0.0, 0.0)), ls.interpolate( 0.00));
	assert_eq!(Some(Vector2::new(0.0, 2.0)), ls.interpolate( 2.00));

	let ls2:LineStringMeasured = vec![].into();
	assert_eq!(None, ls2.interpolate(0.5));
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

	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(1.5, 2.0),
		Vector2::new(1.0, 3.0),
	].into();
	let lsbo = ls.offset_basic(-0.5f64);
	println!("{:?}", lsbo);
	assert_eq!(
		lsbo,
		Some(vec![
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
			].into()
		)
	);
}

#[test]
fn test_linestring_conversion_to_vec() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(1.5, 2.0),
		Vector2::new(1.0, 3.0),
	].into();
	assert_eq!(
		ls.into_vector2(),
		vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(1.5, 2.0),
			Vector2::new(1.0, 3.0),
		]
	);
}


#[test]
fn test_multilinestring_conversion_to_tuples() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(1.0, 1.0),
		Vector2::new(1.5, 2.0),
		Vector2::new(1.0, 3.0),
	].into();
	assert_eq!(
		ls.into_tuples(),
		vec![
			(0.0, 0.0),
			(1.0, 1.0),
			(1.5, 2.0),
			(1.0, 3.0),
		]
	);
}

#[test]
fn test_multilinestring_conversion_to_measured_tuples() {
	let ls:LineStringMeasured = vec![
		Vector2::new(0.0, 0.0),
		Vector2::new(0.0, 2.0),//2
		Vector2::new(4.0, 2.0),//4
		Vector2::new(4.0, 10.0),//8
	].into();
	assert_eq!(
		ls.into_tuples_measured(0.0, 14.0),
		vec![
			(0.0, 0.0, 0.0),
			(0.0, 2.0, 2.0),
			(4.0, 2.0, 6.0),
			(4.0, 10.0, 14.0),
		]
	);

	assert_eq!(
		ls.into_tuples_measured(2.0, 30.0),
		vec![
			(0.0, 0.0, 2.0),
			(0.0, 2.0, 6.0),
			(4.0, 2.0, 14.0),
			(4.0, 10.0, 30.0),
		]
	);

}
