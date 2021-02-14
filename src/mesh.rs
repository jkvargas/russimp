use crate::{bone::Bone, face::Face, sys::*, *};
use derivative::Derivative;
use num_traits::ToPrimitive;
use std::ops::BitAnd;

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Mesh {
    pub normals: Vec<Vector3D>,
    pub name: String,
    pub vertices: Vec<Vector3D>,
    pub texture_coords: Vec<Option<Vector3D>>,
    pub tangents: Vec<Vector3D>,
    pub bitangents: Vec<Vector3D>,
    pub uv_components: Vec<u32>,
    pub primitive_types: u32,
    pub bones: Vec<Bone>,
    pub material_index: u32,
    pub method: u32,
    pub anim_meshes: Vec<AnimMesh>,
    pub faces: Vec<Face>,
    pub colors: Vec<Option<Color4D>>,
    pub aabb: AABB,
}

#[derive(Derivative, FromPrimitive, PartialEq, ToPrimitive)]
#[derivative(Debug)]
#[repr(u32)]
pub enum PrimitiveType {
    Force32Bit = aiPrimitiveType__aiPrimitiveType_Force32Bit,
    Line = aiPrimitiveType_aiPrimitiveType_LINE,
    Point = aiPrimitiveType_aiPrimitiveType_POINT,
    Polygon = aiPrimitiveType_aiPrimitiveType_POLYGON,
    Triangle = aiPrimitiveType_aiPrimitiveType_TRIANGLE,
}

impl From<&aiMesh> for Mesh {
    fn from(mesh: &aiMesh) -> Self {
        Self {
            normals: utils::get_vec(mesh.mNormals, mesh.mNumVertices),
            name: mesh.mName.into(),
            vertices: utils::get_vec(mesh.mVertices, mesh.mNumVertices),
            texture_coords: utils::get_vec_from_slice(&mesh.mTextureCoords),
            tangents: utils::get_vec(mesh.mTangents, mesh.mNumVertices),
            bitangents: utils::get_vec(mesh.mBitangents, mesh.mNumVertices),
            uv_components: mesh.mNumUVComponents.to_vec(),
            primitive_types: mesh.mPrimitiveTypes as u32,
            bones: utils::get_vec_from_raw(mesh.mBones, mesh.mNumBones),
            material_index: mesh.mMaterialIndex,
            method: mesh.mMethod,
            anim_meshes: utils::get_vec_from_raw(mesh.mAnimMeshes, mesh.mNumAnimMeshes),
            faces: utils::get_vec(mesh.mFaces, mesh.mNumFaces),
            colors: utils::get_vec_from_slice(&mesh.mColors),
            aabb: (&mesh.mAABB).into(),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct AnimMesh(Vec<Vector3D>);

impl From<&aiAnimMesh> for AnimMesh {
    fn from(mesh: &aiAnimMesh) -> Self {
        Self(utils::get_vec(mesh.mBitangents, mesh.mNumVertices))
    }
}

impl BitAnd<PrimitiveType> for PrimitiveType {
    type Output = u32;

    fn bitand(self, rhs: PrimitiveType) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<PrimitiveType> for u32 {
    type Output = u32;

    fn bitand(self, rhs: PrimitiveType) -> Self::Output {
        self & ToPrimitive::to_u32(&rhs).unwrap()
    }
}

impl BitAnd<u32> for PrimitiveType {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        ToPrimitive::to_u32(&self).unwrap() & rhs
    }
}

#[test]
fn mesh_available() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    assert_eq!(1, scene.meshes.len());
    assert_eq!(8, scene.meshes[0].normals.len());
    assert_eq!(8, scene.meshes[0].vertices.len());
    assert!(scene.meshes[0].texture_coords.iter().all(|x| x.is_none()));
    assert!(scene.meshes[0].tangents.is_empty());
    assert!(scene.meshes[0].bitangents.is_empty());
    assert_eq!(8, scene.meshes[0].uv_components.len());
    assert_eq!(true, scene.meshes[0].uv_components.iter().all(|x| *x == 0));
    assert_eq!(4, scene.meshes[0].primitive_types);
    assert!(scene.meshes[0].bones.is_empty());
    assert!(scene.meshes[0].anim_meshes.is_empty());
    assert_eq!(12, scene.meshes[0].faces.len());
    assert!(&scene.meshes[0].anim_meshes.is_empty());
    assert_eq!(0, scene.meshes[0].method);
    assert_eq!(0, scene.meshes[0].material_index);
    assert_eq!(0.0, scene.meshes[0].aabb.min.x);
    assert_eq!(0.0, scene.meshes[0].aabb.min.y);
    assert_eq!(0.0, scene.meshes[0].aabb.min.z);
    assert_eq!(0.0, scene.meshes[0].aabb.max.x);
    assert_eq!(0.0, scene.meshes[0].aabb.max.y);
    assert_eq!(0.0, scene.meshes[0].aabb.max.z);
    assert!(scene.meshes[0].colors.iter().all(|x| x.is_none()));
}

#[test]
fn bitwise_primitive_types() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    assert_eq!(
        4,
        scene.meshes[0].primitive_types & PrimitiveType::Force32Bit
    );
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Line);
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Point);
    assert_eq!(4, scene.meshes[0].primitive_types & PrimitiveType::Triangle);
    assert_eq!(0, scene.meshes[0].primitive_types & PrimitiveType::Polygon);
}

#[test]
fn debug_mesh() {
    use crate::scene::{PostProcess, Scene};

    let current_directory_buf = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.meshes);
}
