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
	- [2.7. Converting to `Vec<(f64,f64)>`](#27-converting-to-vecf64f64)
	- [2.8. Converting to `Vec<(f64,f64,f64)>`](#28-converting-to-vecf64f64f64)
- [3. Struct `Vector2`](#3-struct-vector2)

## 1. Introduction

This is a library for basic operations on LineStrings.

This library provides the following structs:

1. `Vector`,
1. `LineStringMeasured`
1. `LineSegmentMeasured` (meant for internal use, read source if curious. It is small),

Most core functionality is implemented on `LineStringMeasured`.

This is a partial port of my previous python library
[nicks_line_tools](https://github.com/thehappycheese/nicks_line_tools) 
which in turn is a partial implementation of the method described by [Xu-Zheng Liu, Jun-Hai Yong, Guo-Qin Zheng, Jia-Guang Sun. An offset algorithm for polyline curves. Computers in Industry, Elsevier, 2007, 15p. inria-00518005](https://hal.inria.fr/inria-00518005/document). I wrote up a decent-ish summary of the psudocode on the python version.

My plan for this repo is to either abandon it and start again, or totally re-write it using the rust geo-types/geo ecosystem as a dependancy.
My main use for this repo is as a dependancy for my other project [nicklinref_rust](https://github.com/thehappycheese/nicklinref_rust)
which I am also considering abandoning.

The new hotness is [megalinref](https://github.com/thehappycheese/megalinref) which is a python library built with a rust binary which does the same thing but without the overhead of hosting a rest service on localhost. I need an offsetting algorithim for that project, but I am using `geo-types` / `geo` in that project.

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

Returns `Some(LineString)` at some offset distance from the original. Returns `None` if the linestring has fewer than two points.

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
);
```

### 2.7. Converting to `Vec<(f64,f64)>`

Use the `into_tuples()` function to convert a multi line string into `Vec<(f64,f64)>` objects.

```rust
let into_tuples = ls.into_tuples();

assert_eq!(
	into_tuples,
	vec![
		(0f64, 0f64),
		(1f64, 0f64),
		(1f64, 1f64),
	]
);
```

### 2.8. Converting to `Vec<(f64,f64,f64)>`

Use the `into_tuples_measured()` function to convert a multi line string into `Vec<(f64,f64,f64)>` objects.

```rust
let into_tuples_measured = ls.into_tuples_measured(2.0, 10.0);

assert_eq!(
	into_tuples_measured,
	vec![
		(0f64, 0f64, 2f64),
		(1f64, 0f64, 6f64),
		(1f64, 1f64, 10f64),
	]
);
```

## 3. Struct `Vector2`

A simple vector manipulation class implementing PartialEq, Clone, Copy, Deserialize and Serialise:

```rust
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}
```
