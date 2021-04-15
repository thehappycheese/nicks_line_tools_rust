
use crate::linestring::{LineSegmentMeasured, LineString, LineStringMeasured, LineStringy};
use crate::vector2::Vector2;

#[test]
fn test_linestring_length() {
	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		],
	};
	assert_eq!(ls.magnitude(), 2f64);
}

#[test]
fn test_offset_segments() {
	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		],
	};
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
	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
		],
	};
	//println!("{:?}", ls);
	let ls_m = ls.measured_segments();
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
	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
		],
	};
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
	assert_eq!(ls_c, (None, Some(ls.measured_segments())));

	let ls_c = ls.cut(1f64);
	//println!("{:?}", ls_m);
	assert_eq!(ls_c, (Some(ls.measured_segments()), None,));

	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(0.0, 2.0),
		],
	};
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
	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(0.0, 2.0),
		],
	};
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

	let ls = LineString {
		points: vec![
			Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(1.5, 2.0),
			Vector2::new(1.0, 3.0),
		],
	};
	let lsbo = ls.offset_basic(-0.5f64);
	println!("{:?}", lsbo);
	assert_eq!(
		lsbo,
		Some(LineString {
			points: vec![
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
			]
		})
	);
}