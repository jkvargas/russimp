# russimp ![russimp](https://github.com/jkvargas/russimp/workflows/russimp/badge.svg?branch=master) [![Crates.io](https://img.shields.io/crates/v/russimp.svg)](https://crates.io/crates/russimp)

Rust bindings for Assimp (https://github.com/assimp/assimp)

# Overview

Russimp is a library for talking to the assimp library which enables you to read 3d models in different formats to a common structure.

By default, russimp looks for the `assimp` library on your computer.  To install it:

* OSX: You will need to update brew and install assimp with it.
* Linux: You will need to install assimp through your package manager of choice.
* Windows: You can use [vcpkg](https://vcpkg.io/en/index.html), check russimp.yml on how to use it on windows, or use a prebuilt version of assimp.

Alternately, you may prefer to use prebuilt assimp binaries or compile it yourself; in either case russimp will statically link assimp into your binary.  Russimp exposes the following Cargo features to manage the assimp dependency (this documentation is reproduced from [russimp-sys](https://github.com/jkvargas/russimp-sys)):

## `prebuilt`

Download prebuilt `Assimp` static library binaries from github and skip building from source.

Because `Assimp` build is slow and have build environment requirements. We provide prebuilt binaries for common platforms and features.

When a new version is released, github action automatically runs pre-build tasks, and all prebuilt binaries are saved in [github releases](https://github.com/jkvargas/russimp-sys/releases).

The `russimp-sys` build script will try to download the prebuilt binaries from github first, and skip the full source build.

## `static-link`

Enabling `static-link` feature without `prebuilt` feature will build `assimp` from source.

Builds from source need the following dependencies:

* cmake
* clang
* Ninja for Linux and MacOS, Visual Studio 2019 for Windows

### `nozlib`

By default `russimp-sys` will statically link zlibstatic, you can disable this feature if it conflicts with other dependencies.

### `nolibcxx`

By default `russimp-sys` links to `libstdc++` in linux and `libc++` in macos, turning this on `russimp-sys` won't link to the c++ standard library.

# Helping


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

### 1.0.4
* Builds based on assimp v5.2.5

### 1.0.3
* colors vector inside the mesh turned into Vec<Option<Vec<Color4d>>>

### 1.0.2
* Expose `prebuilt` and other new Cargo features from [russimp-sys](https://github.com/jkvargas/russimp-sys)

### 1.0.1
* PostProcessing typo, GenenerateUVCoords was changed to GenerateUVCoords.

### 1.0.0
* Builds based on 5.1.0 release
