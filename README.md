# russimp ![russimp](https://github.com/jkvargas/russimp/workflows/russimp/badge.svg?branch=master) [![Crates.io](https://img.shields.io/crates/v/russimp.svg)](https://crates.io/crates/russimp)

Rust bindings for Assimp (https://github.com/assimp/assimp)

# Overview

Russimp is a library for talking to the assimp library which enables you to read 3d models in different formats to a common structure.
These bindings are based on assimp v5.0.1.

## Helping

You are very welcome to help with development, adding a feature, fixing a problem or just refactoring.
Try to do it with tests =)

We need help to compile it on windows and on mac.

Make sure to run cargo fmt before creating a pull request.

# Requirements

## Rust

You will need rust stable, cmake, C and C++ compiler as well.

# How to use it?

Just call Scene::from_file with the filename and the flags you want. From the scene you will have access to the underlying structs.

```rust
let scene = Scene::from_file("myfile.blend",
vec![PostProcess::CalcTangentSpace,
     PostProcess::Triangulate,
     PostProcess::JoinIdenticalVertices,
     PostProcess::SortByPType]).unwrap();
```
