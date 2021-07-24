# Nicks LineString Tools<br>(Rust Version) <!-- omit in toc -->

## Contents <!-- omit in toc -->

- [1. Introduction](#1-introduction)
- [2. Struct `LineStringMeasured`](#2-struct-linestringmeasured)
	- [2.1. Constructing](#21-constructing)
	- [2.2. `.cut()`](#22-cut)
	- [2.3. `.cut_twice()`](#23-cut_twice)
	- [2.4. `.interpolate()`](#24-interpolate)
	- [2.5. `.offset_basic()`](#25-offset_basic)
	- [2.6. Converting to `Vec<Vector2>`](#26-converting-to-vecvector2)
- [3. Struct `Vector2`](#3-struct-vector2)

## 1. Introduction

This is a library for basic operations on LineStrings.

This library provides the following structs:

1. `Vector`,
1. `LineStringMeasured`
1. `LineSegmentMeasured` (meant for internal use, read source if curious. It is small),


Most core functionality is implemented on `LineStringMeasured`.

## 2. Struct `LineStringMeasured`

### 2.1. Constructing

No constructor is provided currently. Instead the `From<Vec<Vector2>>` trait can be used to create an instance as follows:

```rust
use line_string_measured::LineStringMeasured;
use vector2::Vector2;

let ls:LineStringMeasured = vec![
	Vector2::new(0f64, 0f64),
	Vector2::new(1f64, 0f64),
	Vector2::new(1f64, 1f64),
].into();
```

OR

```rust
let ls = LineStringMeasured::from(vec![
	Vector2::new(0f64, 0f64),
	Vector2::new(1f64, 0f64),
	Vector2::new(1f64, 1f64),
]);
```

### 2.2. `.cut()`

Splits a linestring at a fraction of its length and returns two new linestrings

```rust
let (a:Option<LineStringMeasured>, b:Option<LineStringMeasured>) = ls.cut(0.25f64);
```

### 2.3. `.cut_twice()`

Splits a linestring twice at a fractions of its length and returns three new linestrings

```rust
let (a:Option<LineStringMeasured>, b:Option<LineStringMeasured>, c:Option<LineStringMeasured>) = ls.cut(0.25f64, 0.66f64);
```

### 2.4. `.interpolate()`

Gets a point at some fraction of the distance along a linestring. Fails with zero length linestrings.

```rust
let a:Option<Vector2> = ls.interpolate(0.25f64);
```

### 2.5. `.offset_basic()`

Returns Some(LineString) at some offset distance from the original. Returns None

```rust
let c:Option<LineString> = ls.offset_basic(0.5f64);
```

### 2.6. Converting to `Vec<Vector2>`

User code may find it easier to handle `Vec<Vector2>` objects.
Rust does not provide an elegant way to call the Into trait, therefore the following helper function is provided:

```rust
let back_to_vec = ls.into_vector2();

assert!(
	back_to_vec,
	vec![
		Vector2::new(0f64, 0f64),
		Vector2::new(1f64, 0f64),
		Vector2::new(1f64, 1f64),
	]
)
```

## 3. Struct `Vector2`

A simple vector manipulation class implementing PartialEq, Clone, Copy, Deserialize and Serialise:

```rust
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}
```
