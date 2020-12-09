# russimp

Bindings for Assimp in Rust

# Overview

Russimp is a library for talking to the assimp library which enables you to read 3d models in different formats to a common structure.

This library is based on the assimp version v5.0.1.
In order to compile this project you can clone it with git clone --recursive https://github.com/jkvargas/russimp.git

## Helping

You are very welcome to help with development, being compiling it on windows or a mac.
Adding a feature, fixing a problem or just refactoring.
Try to do it with tests =)

# Requirements

## Rust

You will need rust stable, cmake a C and C++ compiler as well.

# How to use it?

Just call Scene::from with the filename and the flags you want. From the scene you will have access to the underlying structs.

```rust
let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/box.blend");

let scene = Scene::from(current_directory_buf.to_str().unwrap(),
vec![PostProcessSteps::CalcTangentSpace,
     PostProcessSteps::Triangulate,
     PostProcessSteps::JoinIdenticalVertices,
     PostProcessSteps::SortByPType]).unwrap();
```
