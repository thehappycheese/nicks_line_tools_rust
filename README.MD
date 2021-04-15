# Nicks LineString Tools<br>(Rust Version)

This is a library for basic opperations on LineStrings.

```rust
use linestring::LineString;
use vector2::Vector2;

let ls = LineString {
	points:vec![
		Vector2::new(0f64, 0f64),
		Vector2::new(1f64, 0f64),
		Vector2::new(1f64, 1f64),
	]
}

// splits a linestring at a fraction of its length and returns two new linestrings
let (a:Option<LineString>, b:Option<LineString>) = ls.cut(0.25f64);

// creates and returns an offset a linestring at some distance from the original
let c:Option<LineString> = ls.offset_basic(0.5f64);

```

