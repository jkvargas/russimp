use russimp_sys::{aiScene, aiImportFile, aiReleaseImport, aiPostProcessSteps_aiProcess_CalcTangentSpace, aiPostProcessSteps_aiProcess_Triangulate, aiPostProcessSteps_aiProcess_JoinIdenticalVertices, aiPostProcessSteps_aiProcess_SortByPType, aiPostProcessSteps_aiProcess_MakeLeftHanded, aiPostProcessSteps_aiProcess_RemoveComponent, aiPostProcessSteps_aiProcess_GenNormals, aiPostProcessSteps_aiProcess_GenSmoothNormals, aiPostProcessSteps_aiProcess_SplitLargeMeshes, aiPostProcessSteps_aiProcess_PreTransformVertices, aiPostProcessSteps_aiProcess_LimitBoneWeights, aiPostProcessSteps_aiProcess_ValidateDataStructure, aiPostProcessSteps_aiProcess_ImproveCacheLocality, aiPostProcessSteps_aiProcess_RemoveRedundantMaterials, aiPostProcessSteps_aiProcess_FixInfacingNormals, aiPostProcessSteps_aiProcess_FindDegenerates, aiPostProcessSteps_aiProcess_FindInvalidData, aiPostProcessSteps_aiProcess_GenUVCoords, aiPostProcessSteps_aiProcess_TransformUVCoords, aiPostProcessSteps_aiProcess_FindInstances, aiPostProcessSteps_aiProcess_OptimizeMeshes, aiPostProcessSteps_aiProcess_OptimizeGraph, aiPostProcessSteps_aiProcess_FlipUVs, aiPostProcessSteps_aiProcess_FlipWindingOrder, aiPostProcessSteps_aiProcess_SplitByBoneCount, aiPostProcessSteps_aiProcess_Debone, aiPostProcessSteps_aiProcess_GlobalScale, aiPostProcessSteps_aiProcess_EmbedTextures, aiPostProcessSteps_aiProcess_ForceGenNormals, aiPostProcessSteps_aiProcess_DropNormals, aiPostProcessSteps_aiProcess_GenBoundingBoxes, aiGetErrorString};

use std::ffi::{CString, CStr};
use std::ops::BitOr;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct Scene {
    scene: *const aiScene
}

#[repr(u32)]
pub enum PostProcessSteps {
    CalcTangentSpace = aiPostProcessSteps_aiProcess_CalcTangentSpace,
    JoinIdenticalVertices = aiPostProcessSteps_aiProcess_JoinIdenticalVertices,
    MakeLeftHanded = aiPostProcessSteps_aiProcess_MakeLeftHanded,
    Triangulate = aiPostProcessSteps_aiProcess_Triangulate,
    RemoveComponent = aiPostProcessSteps_aiProcess_RemoveComponent,
    GenNormals = aiPostProcessSteps_aiProcess_GenNormals,
    GenSmoothNormals = aiPostProcessSteps_aiProcess_GenSmoothNormals,
    SplitLargeMeshes = aiPostProcessSteps_aiProcess_SplitLargeMeshes,
    PreTransformVertices = aiPostProcessSteps_aiProcess_PreTransformVertices,
    LimitBoneWeights = aiPostProcessSteps_aiProcess_LimitBoneWeights,
    ValidateDataStructure = aiPostProcessSteps_aiProcess_ValidateDataStructure,
    ImproveCacheLocality = aiPostProcessSteps_aiProcess_ImproveCacheLocality,
    RemoveRedundantMaterials = aiPostProcessSteps_aiProcess_RemoveRedundantMaterials,
    FixInfacingNormals = aiPostProcessSteps_aiProcess_FixInfacingNormals,
    SortByPType = aiPostProcessSteps_aiProcess_SortByPType,
    FindDegenerates = aiPostProcessSteps_aiProcess_FindDegenerates,
    FindInvalidData = aiPostProcessSteps_aiProcess_FindInvalidData,
    GenUVCoords = aiPostProcessSteps_aiProcess_GenUVCoords,
    TransformUVCoords = aiPostProcessSteps_aiProcess_TransformUVCoords,
    FindInstances = aiPostProcessSteps_aiProcess_FindInstances,
    OptimizeMeshes = aiPostProcessSteps_aiProcess_OptimizeMeshes,
    OptimizeGraph = aiPostProcessSteps_aiProcess_OptimizeGraph,
    FlipUVs = aiPostProcessSteps_aiProcess_FlipUVs,
    FlipWindingOrder = aiPostProcessSteps_aiProcess_FlipWindingOrder,
    SplitByBoneCount = aiPostProcessSteps_aiProcess_SplitByBoneCount,
    Debone = aiPostProcessSteps_aiProcess_Debone,
    GlobalScale = aiPostProcessSteps_aiProcess_GlobalScale,
    EmbedTextures = aiPostProcessSteps_aiProcess_EmbedTextures,
    ForceGenNormals = aiPostProcessSteps_aiProcess_ForceGenNormals,
    DropNormals = aiPostProcessSteps_aiProcess_DropNormals,
    GenBoundingBoxes = aiPostProcessSteps_aiProcess_GenBoundingBoxes,
}

impl BitOr<PostProcessSteps> for PostProcessSteps {
    type Output = u32;

    fn bitor(self, rhs: PostProcessSteps) -> Self::Output {
        self | rhs
    }
}

impl BitOr<u32> for PostProcessSteps {
    type Output = ();

    fn bitor(self, rhs: u32) -> Self::Output {
        self | rhs
    }
}

impl BitOr<PostProcessSteps> for u32 {
    type Output = u32;

    fn bitor(self, rhs: PostProcessSteps) -> Self::Output {
        self | rhs
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            aiReleaseImport(self.scene);
        }
    }
}

#[derive(Debug)]
pub enum RussimpError {
    Import(String)
}

impl Display for RussimpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RussimpError::Import(content) => {
                return write!(f, "{}", content);
            },
            _ => {
                return write!(f, "unknown error");
            }
        }
    }
}

impl Error for RussimpError {}

impl Scene {
    pub fn from(file_path: &str, flags: Vec<PostProcessSteps>) -> Result<Self, RussimpError>  {
        let c_str = CString::new(file_path).unwrap();
        let scene_import: *const aiScene = unsafe { aiImportFile(c_str.as_ptr(), flags.into_iter().fold(0, |x, y| x.bitor(y))) };

        if scene_import.is_null() {
            let error_buf  = unsafe { aiGetErrorString() };
            let error = unsafe { CStr::from_ptr(error_buf).to_string_lossy().into_owned() };
            return Err(RussimpError::Import(error));
        }

        Ok(Self {
            scene: scene_import
        })
    }
}