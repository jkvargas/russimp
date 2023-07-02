## Changelog

### 2.0.6
- Moved to a Changelog File
- Some Dependency Updates
- Minior Code Improvements

### 2.0.5
- Adding Sheen, ClearCoat and Transmission texture types.

### 2.0.4
- Updating edition from 2018 to 2021.

### 2.0.3
- Fixed scene metadata parsing.
- Fixed error where material property was not read correctly.
- Fixed memory leak caused by Rc cycles in the node graph.

- The scene structure has been modified.

### 2.0.0
- Fixed issue to load embedded textures.

- Both material and texture structures have been modified.

### 1.0.6
- Updating documentation

### 1.0.5
**Added missing texture types:**
-  Sheen
-  Clearcoat
-  Transmission

- Material, MaterialProperty, Texture and PropertyTypeInfo are now cloneable.

### 1.0.4
-  Builds based on assimp v5.2.5

### 1.0.3
-  colors vector inside the mesh turned into Vec<Option<Vec\<Color4d>>>

### 1.0.2
-  Expose `prebuilt` and other new Cargo features from [russimp-sys](https://github.com/jkvargas/russimp-sys)

### 1.0.1
-  PostProcessing typo, GenenerateUVCoords was changed to GenerateUVCoords.

### 1.0.0
-  Builds based on 5.1.0 release