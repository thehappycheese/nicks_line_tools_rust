use crate::vector2::Vector2;

#[derive(Clone, Debug, PartialEq)]
pub struct LineSegmentMeasured {
	pub a: Vector2,
	pub b: Vector2,
	pub mag: f64,
}

// pub trait LineSegmenty {
// 	fn intersect(&self, other: &LineSegmentMeasured) -> Option<(Vector2, f64, f64)>;
// }

impl LineSegmentMeasured {
	pub fn intersect(&self, other: &LineSegmentMeasured) -> Option<(Vector2, f64, f64)> {
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