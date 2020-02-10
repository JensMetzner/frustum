# Frustum &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![frustum: rustc 1.40+]][Rust 1.40]

[Build Status]: https://github.com/jensmetzner/frustum/workflows/CI/badge.svg
[actions]: https://github.com/jensmetzner/frustum/actions?workflow=CI

[Latest Version]: https://img.shields.io/crates/v/frustum.svg
[crates.io]: https://crates.io/crates/frustum

[frustum: rustc 1.40+]: https://img.shields.io/badge/frustum-rustc_1.40+-lightgray.svg
[Rust 1.40]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html

This crate provides a simple library to set up a frustum, such as a camera.

```
Frustum {
    origin: Point3::<WorldSpace>::new(0.0, 0.0, 10.0),
    target: Point3::<WorldSpace>::new(0.0, 0.0, 0.0),
    fovy: 45.0,
    ncp: 1.0,
    fcp: 20.0,
    width: 500,
    height: 500,
}
```