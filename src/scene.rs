use crate::{
    animation::Animation,
    camera::Camera,
    light::Light,
    material::{Material, MaterialFactory},
    mesh::Mesh,
    metadata::MetaData,
    node::Node,
    sys::*,
    *,
};
use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    rc::Rc,
};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub metadata: Option<MetaData>,
    pub animations: Vec<Animation>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    pub root: Option<Rc<RefCell<Node>>>,
    pub flags: u32,
}

#[derive(Derivative)]
#[derivative(Debug)]
#[repr(u32)]
pub enum PostProcess {
    /// Calculates the tangents and bitangents for the imported meshes.
    ///
    /// Does nothing if a mesh does not have normals. You might want this post
    /// processing step to be executed if you plan to use tangent space
    /// calculations such as normal mapping applied to the meshes. There’s
    /// a config setting, `AI_CONFIG_PP_CT_MAX_SMOOTHING_ANGLE`, which
    /// allows you to specify a maximum smoothing angle for the algorithm.
    /// However, usually you’ll want to leave it at the default value.
    CalculateTangentSpace = aiPostProcessSteps_aiProcess_CalcTangentSpace as _,
    /// Identifies and joins identical vertex data sets within all imported
    /// meshes.
    ///
    /// After this step is run, each mesh contains unique vertices, so a vertex
    /// may be used by multiple faces. You usually want to use this post
    /// processing step. If your application deals with indexed geometry, this
    /// step is compulsory or you’ll just waste rendering time.
    ///
    /// **If this flag is *not* specified, no vertices are referenced by more
    /// than one face and no index buffer is required for rendering.**
    JoinIdenticalVertices = aiPostProcessSteps_aiProcess_JoinIdenticalVertices as _,
    /// Converts all the imported data to a left-handed coordinate space.
    ///
    /// By default the data is returned in a right-handed coordinate space
    /// (which OpenGL prefers). In this space, +X points to the right, +Z points
    /// towards the viewer, and +Y points upwards. In the DirectX coordinate
    /// space +X points to the right, +Y points upwards, and +Z points away
    /// from the viewer.
    ///
    /// you’ll probably want to consider this flag if you use Direct3D for
    /// rendering. The
    /// [`ConvertToLeftHanded`](PostProcess::ConvertToLeftHanded) flag
    /// supersedes this setting and bundles all conversions typically
    /// required for D3D-based applications.
    MakeLeftHanded = aiPostProcessSteps_aiProcess_MakeLeftHanded as _,
    /// Triangulates all faces of all meshes.
    ///
    /// By default the imported mesh data might contain faces with more than
    /// three indices. For rendering you’ll usually want all faces to be
    /// triangles. This post processing step splits up faces with more than
    /// three indices into triangles. Line and point primitives are *not*
    /// modified! If you want ‘triangles only’ with no other kinds of
    /// primitives, try the following solution:
    ///
    /// * Specify both [`Triangulate`](PostProcess::Triangulate) and
    ///   [`SortByPrimitiveType`](PostProcess::SortByPrimitiveType)
    /// * Ignore all point and line meshes when you process assimp's output
    Triangulate = aiPostProcessSteps_aiProcess_Triangulate as _,
    /// Removes some parts of the data structure (animations, materials, light
    /// sources, cameras, textures, vertex components).
    ///
    /// The components to be removed are specified in a separate configuration
    /// option, `AI_CONFIG_PP_RVC_FLAGS`. This is quite useful if you don't need
    /// all parts of the output structure. Vertex colors are rarely used today
    /// for example... Calling this step to remove unneeded data from the
    /// pipeline as early as possible results in increased performance and a
    /// more optimized output data structure. This step is also useful if you
    /// want to force Assimp to recompute normals or tangents. The corresponding
    /// steps don't recompute them if they’re already there (loaded from the
    /// source asset). By using this step you can make sure they are NOT there.
    ///
    /// This flag is a poor one, mainly because its purpose is usually
    /// misunderstood. Consider the following case: a 3D model has been exported
    /// from a CAD app, and it has per-face vertex colors. Vertex positions
    /// can't be shared, thus the
    /// [`JoinIdenticalVertices`](PostProcess::JoinIdenticalVertices) step
    /// fails to optimize the data because of these nasty little vertex colors.
    /// Most apps don't even process them, so it’s all for nothing. By using
    /// this step, unneeded components are excluded as early as possible thus
    /// opening more room for internal optimizations.
    RemoveComponent = aiPostProcessSteps_aiProcess_RemoveComponent as _,
    /// Generates normals for all faces of all meshes.
    ///
    /// This is ignored if normals are already there at the time this flag is
    /// evaluated. Model importers try to load them from the source file, so
    /// they’re usually already there. Face normals are shared between all
    /// points of a single face, so a single point can have multiple
    /// normals, which forces the library to duplicate vertices in some cases.
    /// [`JoinIdenticalVertices`](PostProcess::JoinIdenticalVertices) is
    /// *senseless* then.
    ///
    /// This flag may *not* be specified together with
    /// [`GenerateSmoothNormals`](PostProcess::GenerateSmoothNormals).
    GenerateNormals = aiPostProcessSteps_aiProcess_GenNormals as _,
    /// Generates smooth normals for all vertices in the mesh.
    ///
    /// This is ignored if normals are already there at the time this flag is
    /// evaluated. Model importers try to load them from the source file, so
    /// they're usually already there.
    ///
    /// This flag may not be specified together with
    /// [`GenerateNormals`](PostProcess::GenerateNormals)
    ///
    /// There’s a configuration option, `AI_CONFIG_PP_GSN_MAX_SMOOTHING_ANGLE`,
    /// which allows you to specify an angle maximum for the normal smoothing
    /// algorithm. Normals exceeding this limit are not smoothed, resulting in a
    /// ’hard’ seam between two faces. Using a decent angle here (e.g. 80
    /// degrees) results in very good visual appearance.
    GenerateSmoothNormals = aiPostProcessSteps_aiProcess_GenSmoothNormals as _,
    /// Splits large meshes into smaller sub-meshes.
    ///
    /// This is quite useful for real-time rendering, where the number of
    /// triangles which can be maximally processed in a single draw-call is
    /// limited by the video driver/hardware. The maximum vertex buffer is
    /// usually limited too. Both requirements can be met with this step: you
    /// may specify both a triangle and vertex limit for a single mesh.
    ///
    /// The split limits can (and should!) be set through the
    /// `AI_CONFIG_PP_SLM_VERTEX_LIMIT` and `AI_CONFIG_PP_SLM_TRIANGLE_LIMIT`
    /// settings. The default values are <tt>#AI_SLM_DEFAULT_MAX_VERTICES</tt>
    /// and `AI_SLM_DEFAULT_MAX_TRIANGLES`.
    ///
    /// Note that splitting is generally a time-consuming task, but only if
    /// there’s something to split. The use of this step is recommended for most
    /// users.
    SplitLargeMeshes = aiPostProcessSteps_aiProcess_SplitLargeMeshes as _,
    /// Removes the node graph and pre-transforms all vertices with the local
    /// transformation matrices of their nodes.
    ///
    /// The output scene still contains nodes, however there is only a root node
    /// with children, each one referencing only one mesh, and each mesh
    /// referencing one material. For rendering, you can simply render all
    /// meshes in order - you don't need to pay attention to local
    /// transformations and the node hierarchy. Animations are removed during
    /// this step. This step is intended for applications without a scenegraph.
    /// The step CAN cause some problems: if e.g. a mesh of the asset contains
    /// normals and another, using the same material index, does not, they will
    /// be brought together, but the first meshes's part of the normal list is
    /// zeroed. However, these artifacts are rare.
    ///
    /// > The `AI_CONFIG_PP_PTV_NORMALIZE` configuration property can be
    /// set to normalize the scene’s spatial dimension to the -1...1 range.
    PreTransformVertices = aiPostProcessSteps_aiProcess_PreTransformVertices as _,
    /// Limits the number of bones simultaneously affecting a single vertex to a
    /// maximum value.
    ///
    /// If any vertex is affected by more than the maximum number of bones, the
    /// least important vertex weights are removed and the remaining vertex
    /// weights are renormalized so that the weights still sum up to 1. The
    /// default bone weight limit is 4 (defined as `AI_LMW_MAX_WEIGHTS` in
    /// config.h), but you can use the `AI_CONFIG_PP_LBW_MAX_WEIGHTS` setting to
    /// supply your own limit to the post processing step.
    ///
    /// If you intend to perform the skinning in hardware, this post processing
    /// step might be of interest to you.
    LimitBoneWeights = aiPostProcessSteps_aiProcess_LimitBoneWeights as _,
    /// Validates the imported scene data structure. This makes sure that all
    /// indices are valid, all animations and bones are linked correctly, all
    /// material references are correct, etc.
    ///
    /// It is recommended that you capture Assimp's log output if you use this
    /// flag, so you can easily find out what's wrong if a file fails the
    /// validation. The validator is quite strict and will find all
    /// inconsistencies in the data structure... It is recommended that plugin
    /// developers use it to debug their loaders. There are two types of
    /// validation failures:
    ///
    /// * Error: There’s something wrong with the imported data. Further
    ///   postprocessing is not possible and the data is not usable at all. The
    ///   import fails.
    /// * Warning: There are some minor issues (e.g. 1,000,000 animation
    ///   keyframes with the same time), but further postprocessing and use of
    ///   the data structure is still safe.
    ///
    /// This post-processing step is not time-consuming. Its use is not
    /// compulsory, but recommended.
    ValidateDataStructure = aiPostProcessSteps_aiProcess_ValidateDataStructure as _,
    /// Reorders triangles for better vertex cache locality.
    ///
    /// The step tries to improve the ACMR (average post-transform vertex cache
    /// miss ratio) for all meshes. The implementation runs in 𝖮(𝗇) and is
    /// roughly based on the
    /// [‘tipsify’ algorithm](http://www.cs.princeton.edu/gfx/pubs/Sander_2007_%3ETR/tipsy.pdf).
    ///
    /// If you intend to render huge models in hardware, this step might be of
    /// interest to you. The `AI_CONFIG_PP_ICL_PTCACHE_SIZE` config setting can
    /// be used to fine-tune the cache optimization.
    ImproveCacheLocality = aiPostProcessSteps_aiProcess_ImproveCacheLocality as _,
    /// Searches for redundant/unreferenced materials and removes them.
    ///
    /// This is especially useful in combination with the
    /// [`PreTransformVertices`](PostProcess::PreTransformVertices) and
    /// [`OptimizeMeshes`](PostProcess::OptimizeMeshes) flags. Both join
    /// small meshes with equal characteristics, but they can't do their
    /// work if two meshes have different materials. Because several
    /// material settings are lost during Assimp's import filters, (and
    /// because many exporters don't check for redundant materials), huge
    /// models often have materials which are are defined several times with
    /// exactly the same settings.
    ///
    /// Several material settings not contributing to the final appearance of a
    /// surface are ignored in all comparisons (e.g. the material name). So, if
    /// you’re passing additional information through the content pipeline
    /// (probably using *magic* material names), don’t specify this flag.
    /// Alternatively take a look at the `AI_CONFIG_PP_RRM_EXCLUDE_LIST`
    /// setting.
    RemoveRedundantMaterials = aiPostProcessSteps_aiProcess_RemoveRedundantMaterials as _,
    /// Tries to determine which meshes have normal vectors that are
    /// facing inwards and inverts them.
    ///
    /// The algorithm is simple but effective: the bounding box of all vertices
    /// and their normals is compared against the volume of the bounding box of
    /// all vertices without their normals. This works well for most objects,
    /// problems might occur with planar surfaces. However, the step tries to
    /// filter such cases. The step inverts all in-facing normals. Generally it
    /// is recommended to enable this step, although the result is not always
    /// correct.
    FixInfacingNormals = aiPostProcessSteps_aiProcess_FixInfacingNormals as _,
    /// Splits meshes with more than one primitive type in homogeneous
    /// sub-meshes.
    ///
    /// The step is executed after the triangulation step. After the step
    /// returns, just one bit is set in aiMesh::mPrimitiveTypes. This is
    /// especially useful for real-time rendering where point and line
    /// primitives are often ignored or rendered separately. You can use the
    /// AI_CONFIG_PP_SBP_REMOVE option to specify which primitive types you
    /// need. This can be used to easily exclude lines and points, which are
    /// rarely used, from the import.
    SortByPrimitiveType = aiPostProcessSteps_aiProcess_SortByPType as _,
    /// Searches all meshes for degenerate primitives and converts
    /// them to proper lines or points.
    ///
    /// A face is 'degenerate' if one or more of its points are identical. To
    /// have the degenerate stuff not only detected and collapsed but removed,
    /// try one of the following procedures: 1. (if you support lines and
    /// points for rendering but don’t want the degenerates)
    ///   * Specify the aiProcess_FindDegenerates flag.
    ///   * Set the `AI_CONFIG_PP_FD_REMOVE` option to 1. This will cause the
    ///     step to remove degenerate triangles from the import as soon as
    ///     they're detected. They won't pass any further pipeline steps.
    /// 2. (if you don't support lines and points at all)
    ///   * Specify the aiProcess_FindDegenerates flag.
    ///   * Specify the aiProcess_SortByPrimitiveType flag. This moves line and
    ///     point primitives to separate meshes.
    ///   * Set the `AI_CONFIG_PP_SBP_REMOVE` option to `aiPrimitiveType_POINTS
    ///     | aiPrimitiveType_LINES` to cause SortByPrimitiveType to reject
    ///     point and line meshes from the scene.
    ///
    /// > Degenerate polygons are not necessarily evil and that’s why they’re
    /// not removed by default. There are several file formats which don't
    /// support lines or points, and some exporters bypass the format
    /// specification and write them as degenerate triangles instead.
    FindDegenerates = aiPostProcessSteps_aiProcess_FindDegenerates as _,
    /// Searches all meshes for invalid data, such as zeroed normal
    /// vectors or invalid UV coords and removes/fixes them. This is intended to
    /// get rid of some common exporter errors.
    ///
    /// This is especially useful for normals. If they are invalid, and the step
    /// recognizes this, they will be removed and can later be recomputed, i.e.
    /// by the aiProcess_GenSmoothNormals flag. The step will also remove
    /// meshes that are infinitely small and reduce animation tracks consisting
    /// of hundreds if redundant keys to a single key. The
    /// `AI_CONFIG_PP_FID_ANIM_ACCURACY` config property decides the accuracy of
    /// the check for duplicate animation tracks.
    FixOrRemoveInvalidData = aiPostProcessSteps_aiProcess_FindInvalidData as _,
    /// Converts non-UV mappings (such as spherical or cylindrical
    /// mapping) to proper texture coordinate channels.
    ///
    /// Most applications will support UV mapping only, so you will probably
    /// want to specify this step in every case. Note that Assimp is not always
    /// able to match the original mapping implementation of the 3D app which
    /// produced a model perfectly. It's always better to let the modelling app
    /// compute the UV channels - 3ds max, Maya, Blender, LightWave, and Modo do
    /// this for example.
    ///
    /// > If this step is not requested, you’ll need to process the
    /// `AI_MATKEY_MAPPING` material property in order to display all assets
    /// properly.
    GenerateUVCoords = aiPostProcessSteps_aiProcess_GenUVCoords as _,
    /// Applies per-texture UV transformations and bakes them into
    /// stand-alone vtexture coordinate channels.
    ///
    /// UV transformations are specified per-texture — see the
    /// `AI_MATKEY_UVTRANSFORM` material key for more information. This step
    /// processes all textures with transformed input UV coordinates and
    /// generates a new (pre-transformed) UV channel which replaces the old
    /// channel. Most applications won't support UV transformations, so you will
    /// probably want to specify this step.
    ///
    /// > UV transformations are usually implemented in real-time apps by
    /// transforming texture coordinates at vertex shader stage with a 3x3
    /// (homogenous) transformation matrix.
    TransformUVCoords = aiPostProcessSteps_aiProcess_TransformUVCoords as _,
    /// This step searches for duplicate meshes and replaces them with
    /// references to the first mesh.
    ///
    /// This step takes a while, so don't use it if speed is a concern. Its main
    /// purpose is to workaround the fact that many export file formats don't
    /// support instanced meshes, so exporters need to duplicate meshes. This
    /// step removes the duplicates again. Please note that Assimp does not
    /// currently support per-node material assignment to meshes, which means
    /// that identical meshes with different materials are currently not joined,
    /// although this is planned for future versions.
    FindInstances = aiPostProcessSteps_aiProcess_FindInstances as _,
    /// Reduces the number of meshes.
    ///
    /// This will, in fact, reduce the number of draw calls.
    ///
    /// This is a very effective optimization and is recommended to be used
    /// together with [`OptimizeGraph`](PostProcess::OptimizeGraph), if
    /// possible. The flag is fully compatible with both
    /// [`SplitLargeMeshes`](PostProcess::SplitLargeMeshes) and
    /// [`SortByPrimitiveType`](PostProcess::SortByPrimitiveType).
    OptimizeMeshes = aiPostProcessSteps_aiProcess_OptimizeMeshes as _,
    /// Optimizes the scene hierarchy.
    ///
    /// Nodes without animations, bones, lights or cameras assigned are
    /// collapsed and joined.
    ///
    /// Node names can be lost during this step. If you use special ‘tag nodes’
    /// to pass additional information through your content pipeline, use the
    /// `AI_CONFIG_PP_OG_EXCLUDE_LIST` setting to specify a list of node names
    /// you want to be kept. Nodes matching one of the names in this list won’t
    /// touched or modified.
    ///
    /// Use this flag with caution. Most simple files will be collapsed to a
    /// single node, so complex hierarchies are usually completely lost. This is
    /// not useful for editor environments, but probably a very effective
    /// optimization if you just want to get the model data, convert it to your
    /// own format, and render it as fast as possible.
    ///
    /// This flag is designed to be used with
    /// [`OptimizeMeshes`](PostProcess::OptimizeMeshes) for best
    /// results.
    ///
    /// > ‘Crappy’ scenes with thousands of extremely small meshes packed in
    /// deeply nested nodes exist for almost all file formats.
    /// [`OptimizeMeshes`](PostProcess::OptimizeMeshes) in combination with
    /// [`OptimizeGraph`](PostProcess::OptimizeGraph) usually fixes
    /// them all and makes them renderable.
    OptimizeGraph = aiPostProcessSteps_aiProcess_OptimizeGraph as _,
    /// This step flips all UV coordinates along the y-axis and adjusts material
    /// settings and bitangents accordingly.
    ///
    /// You’ll probably want to consider this flag if you use Direct3D for
    /// rendering. The
    /// [`ConvertToLeftHanded`](PostProcess::ConvertToLeftHanded) flag
    /// supersedes this setting and bundles all conversions typically
    /// required for Direct3D-based applications.
    FlipUVs = aiPostProcessSteps_aiProcess_FlipUVs as _,
    /// Adjusts the output face winding order to be clockwise (CW).
    ///
    /// The default face winding order is counter clockwise (CCW).
    FlipWindingOrder = aiPostProcessSteps_aiProcess_FlipWindingOrder as _,
    /// Splits meshes with many bones into sub-meshes so that each su-bmesh has
    /// fewer or as many bones as a given limit.
    SplitByBoneCount = aiPostProcessSteps_aiProcess_SplitByBoneCount as _,
    /// This step removes bones losslessly or according to some threshold.
    ///
    /// In some cases (i.e. formats that require it) exporters are forced to
    /// assign dummy bone weights to otherwise static meshes assigned to
    /// animated meshes. Full, weight-based skinning is expensive while
    /// animating nodes is extremely cheap, so this step is offered to clean up
    /// the data in that regard.
    ///
    /// Use `AI_CONFIG_PP_DB_THRESHOLD` to control this.
    /// Use `AI_CONFIG_PP_DB_ALL_OR_NONE` if you want bones removed if and only
    /// if all bones within the scene qualify for removal.
    Debone = aiPostProcessSteps_aiProcess_Debone as _,
    GlobalScale = aiPostProcessSteps_aiProcess_GlobalScale as _,
    /// Force embedding of textures (using the `path = "*1"` convention).
    ///
    /// If a texture’s file does not exist at the specified path (due, for
    /// instance, to an absolute path generated on another system),  it will
    /// check if a file with the same name exists at the root folder of the
    /// imported model. And if so, it uses that.
    EmbedTextures = aiPostProcessSteps_aiProcess_EmbedTextures as _,
    ForceGenerateNormals = aiPostProcessSteps_aiProcess_ForceGenNormals as _,
    DropNormals = aiPostProcessSteps_aiProcess_DropNormals as _,
    /// Calculate [axis-aligned bounding boxes](crate::AABB) for all meshes in
    /// a scene.
    GenerateBoundingBoxes = aiPostProcessSteps_aiProcess_GenBoundingBoxes as _,
}

pub type PostProcessSteps = Vec<PostProcess>;

impl Scene {
    fn new(scene: &aiScene) -> Russult<Self> {
        let root = unsafe { scene.mRootNode.as_ref() };
        let materials = MaterialFactory::new(scene)?.create_materials();

        Ok(Self {
            materials,
            meshes: utils::get_vec_from_raw(scene.mMeshes, scene.mNumMeshes),
            metadata: utils::get_raw(scene.mMetaData),
            animations: utils::get_vec_from_raw(scene.mAnimations, scene.mNumAnimations),
            cameras: utils::get_vec_from_raw(scene.mCameras, scene.mNumCameras),
            lights: utils::get_vec_from_raw(scene.mLights, scene.mNumLights),
            root: root.map(|f| Node::new(f)),
            flags: scene.mFlags,
        })
    }

    pub fn from_file(file_path: &str, flags: PostProcessSteps) -> Russult<Scene> {
        let bitwise_flag = flags.into_iter().fold(0, |acc, x| acc | (x as u32));
        let file_path = CString::new(file_path).unwrap();

        let raw_scene = Scene::get_scene_from_file(file_path, bitwise_flag);
        if raw_scene.is_none() {
            return Err(Scene::get_error());
        }

        let result = Scene::new(raw_scene.unwrap());
        Scene::drop_scene(raw_scene);

        result
    }

    pub fn from_buffer(buffer: &[u8], flags: PostProcessSteps, hint: &str) -> Russult<Scene> {
        let bitwise_flag = flags.into_iter().fold(0, |acc, x| acc | (x as u32));
        let hint = CString::new(hint).unwrap();

        let raw_scene = Scene::get_scene_from_file_from_memory(buffer, bitwise_flag, hint);
        if raw_scene.is_none() {
            return Err(Scene::get_error());
        }

        let result = Scene::new(raw_scene.unwrap());
        Scene::drop_scene(raw_scene);

        result
    }

    #[inline]
    fn drop_scene(scene: Option<&aiScene>) {
        if let Some(content) = scene {
            unsafe {
                aiReleaseImport(content);
            }
        }
    }

    #[inline]
    fn get_scene_from_file<'a>(string: CString, flags: u32) -> Option<&'a aiScene> {
        unsafe { aiImportFile(string.as_ptr(), flags).as_ref() }
    }

    #[inline]
    fn get_scene_from_file_from_memory<'a>(
        buffer: &[u8],
        flags: u32,
        hint: CString,
    ) -> Option<&'a aiScene> {
        unsafe {
            aiImportFileFromMemory(
                buffer.as_ptr() as *const _,
                buffer.len() as _,
                flags,
                hint.as_ptr(),
            )
            .as_ref()
        }
    }

    fn get_error() -> RussimpError {
        let error_buf = unsafe { aiGetErrorString() };
        let error = unsafe { CStr::from_ptr(error_buf).to_string_lossy().into_owned() };
        RussimpError::Import(error)
    }
}

#[test]
fn importing_invalid_file_returns_error() {
    let current_directory_buf = utils::get_model("models/box.blend");

    let scene = Scene::from_file(
        current_directory_buf.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    );

    assert!(scene.is_err())
}

#[test]
fn importing_valid_file_returns_scene() {
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

    assert_eq!(8, scene.flags);
}

#[test]
fn debug_scene() {
    let box_file_path = utils::get_model("models/BLEND/box.blend");

    let scene = Scene::from_file(
        box_file_path.as_str(),
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();

    dbg!(&scene);
}

#[test]
fn debug_scene_from_memory() {
    let box_file_path = b"solid foo bar
    facet normal 0.1 0.2 0.3
        outer loop
            vertex 1 2 3
            vertex 4 5 6e-15
            vertex 7 8 9.87654321
        endloop
    endfacet
    endsolid foo bar";

    let scene = Scene::from_buffer(
        box_file_path,
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
        "stl",
    )
    .unwrap();

    dbg!(&scene);
}
