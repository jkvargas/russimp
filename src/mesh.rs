use std::ops::BitAnd;

use crate::{
    bone::Bone,
    face::Face,
    scene::{PostProcessSteps, Scene},
    sys::{
        aiAABB, aiAnimMesh, aiColor4D, aiMesh, aiPrimitiveType__aiPrimitiveType_Force32Bit,
        aiPrimitiveType_aiPrimitiveType_LINE, aiPrimitiveType_aiPrimitiveType_POINT,
        aiPrimitiveType_aiPrimitiveType_POLYGON, aiPrimitiveType_aiPrimitiveType_TRIANGLE,
        aiVector3D,
    },
    Color4D, Utils, Vector3D, AABB,
};

use num_traits::ToPrimitive;

use crate::metadata::MetadataType::Vector3d;
use derivative::Derivative;

#[derive(Derivative)]
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

impl Mesh {
    pub fn new(mesh: &aiMesh) -> Mesh {
        Self {
            normals: Utils::get_vec(mesh.mNormals, mesh.mNumVertices, &Vector3D::new),
            name: mesh.mName.into(),
            vertices: Utils::get_vec(mesh.mVertices, mesh.mNumVertices, &Vector3D::new),
            texture_coords: Utils::get_vec_from_slice(&mesh.mTextureCoords, &Vector3D::new),
            tangents: Utils::get_vec(mesh.mTangents, mesh.mNumVertices, &Vector3D::new),
            bitangents: Utils::get_vec(mesh.mBitangents, mesh.mNumVertices, &Vector3D::new),
            uv_components: mesh.mNumUVComponents.to_vec(),
            primitive_types: mesh.mPrimitiveTypes as u32,
            bones: Utils::get_vec_from_raw(mesh.mBones, mesh.mNumBones, &Bone::new),
            material_index: mesh.mMaterialIndex,
            method: mesh.mMethod,
            anim_meshes: Utils::get_vec_from_raw(
                mesh.mAnimMeshes,
                mesh.mNumAnimMeshes,
                &AnimMesh::new,
            ),
            faces: Utils::get_vec(mesh.mFaces, mesh.mNumFaces, &Face::new),
            colors: Utils::get_vec_from_slice(&mesh.mColors, &Color4D::new),
            aabb: AABB::new(&mesh.mAABB),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct AnimMesh {
    bitangents: Vec<Vector3D>,
}

impl AnimMesh {
    pub fn new(mesh: &aiAnimMesh) -> AnimMesh {
        Self {
            bitangents: Utils::get_vec(mesh.mBitangents, mesh.mNumVertices, &Vector3D::new),
        }
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
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
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
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
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
    let current_directory_buf = Utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from(
        current_directory_buf.as_str(),
        vec![
            PostProcessSteps::CalculateTangentSpace,
            PostProcessSteps::Triangulate,
            PostProcessSteps::JoinIdenticalVertices,
            PostProcessSteps::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene.meshes);
}
