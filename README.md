# russimp ![russimp](https://github.com/jkvargas/russimp/workflows/russimp/badge.svg?branch=master) [![Crates.io](https://img.shields.io/crates/v/russimp.svg)](https://crates.io/crates/russimp)

Rust bindings for Assimp (https://github.com/assimp/assimp)

# Overview

Russimp is a library for talking to the assimp library which enables you to read 3d models in different formats to a common structure.

* OSX: You will need to update Brew and install assimp with it.
* Linux: You will need to install assimp 5.1.0. I guess that ubuntu still has 5.0.1 on their repos. If that is the case then you can take a look at install_assimp.bash on how to install it manually.
* Windows: Still not supported since vcpkg still only offers assimp 5.0.1.

If you need bindings for version 5.0.1 just pickup a release before 1.0.0.

## Helping

If you want to help maintaining this package on windows or macos, please let me know.
For windows support you can check the last PR related to it, https://github.com/jkvargas/russimp/pull/16.

You are very welcome to help with development, adding a feature, fixing a problem or just refactoring.
Try to do it with tests =)

Make sure to run cargo fmt before creating a pull request.

# How to use it?

Just call Scene::from_file with the filename and the flags you want. From the scene you will have access to the underlying structs.

```rust
let scene = Scene::from_file("myfile.blend",
vec![PostProcess::CalcTangentSpace,
     PostProcess::Triangulate,
     PostProcess::JoinIdenticalVertices,
     PostProcess::SortByPType]).unwrap();
```

## Changelog

### 1.0.1
* PostProcessing typo, GenenerateUVCoords was changed to GenerateUVCoords.

### 1.0.0
* Builds based on 5.1.0 release