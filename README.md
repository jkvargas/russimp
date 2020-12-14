# russimp

Bindings for Assimp in Rust

# Overview

Russimp is a library for talking to the assimp library which enables you to read 3d models in different formats to a common structure.
These bindings are based on assimp v5.0.1.

## Helping

You are very welcome to help with development, being compiling it on windows or a mac.
Adding a feature, fixing a problem or just refactoring.
Try to do it with tests =)

# Requirements

## Rust

You will need rust stable, cmake, C and C++ compiler as well.

# How to use it?

Just call Scene::from with the filename and the flags you want. From the scene you will have access to the underlying structs.

```rust
let scene = Scene::from("myfile.blend",
vec![PostProcessSteps::CalcTangentSpace,
     PostProcessSteps::Triangulate,
     PostProcessSteps::JoinIdenticalVertices,
     PostProcessSteps::SortByPType]).unwrap();
```
