use crate::{sys::*, Color3D, Vector2D, Vector3D};
use derivative::Derivative;

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Light {
    pub up: Vector3D,
    pub pos: Vector3D,
    pub name: String,
    pub angle_inner_cone: f32,
    pub angle_outer_cone: f32,
    pub attenuation_linear: f32,
    pub attenuation_quadratic: f32,
    pub attenuation_constant: f32,
    pub color_ambient: Color3D,
    pub color_specular: Color3D,
    pub color_diffuse: Color3D,
    pub direction: Vector3D,
    pub size: Vector2D,
    pub light_source_type: LightSourceType,
}

impl From<&aiLight> for Light {
    fn from(light: &aiLight) -> Self {
        Self {
            up: (&light.mUp).into(),
            pos: (&light.mPosition).into(),
            name: light.mName.into(),
            angle_inner_cone: light.mAngleInnerCone,
            angle_outer_cone: light.mAngleOuterCone,
            attenuation_linear: light.mAttenuationLinear,
            attenuation_quadratic: light.mAttenuationQuadratic,
            attenuation_constant: light.mAttenuationConstant,
            color_ambient: (&light.mColorAmbient).into(),
            color_specular: (&light.mColorSpecular).into(),
            color_diffuse: (&light.mColorDiffuse).into(),
            direction: (&light.mDirection).into(),
            size: (&light.mSize).into(),
            light_source_type: (light.mType as u32).into(),
        }
    }
}

#[derive(Derivative, num_enum::IntoPrimitive, num_enum::FromPrimitive, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum LightSourceType {
    Ambient = aiLightSourceType_aiLightSource_AMBIENT as _,
    Area = aiLightSourceType_aiLightSource_AREA as _,
    Directional = aiLightSourceType_aiLightSource_DIRECTIONAL as _,
    Point = aiLightSourceType_aiLightSource_POINT as _,
    Spot = aiLightSourceType_aiLightSource_SPOT as _,
    #[num_enum(default)]
    Undefined = aiLightSourceType_aiLightSource_UNDEFINED as _,
}

impl Default for LightSourceType {
    fn default() -> Self {
        LightSourceType::Undefined
    }
}

#[test]
fn light_available() {
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

    let current_directory_buf = utils::get_model("models/BLEND/AreaLight_269.blend");

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

    assert_eq!(3, scene.lights.len());

    assert_eq!(0.60381645, scene.lights[0].color_diffuse.r);
    assert_eq!(0.60381645, scene.lights[0].color_diffuse.g);
    assert_eq!(0.60381645, scene.lights[0].color_diffuse.b);

    assert_eq!(0.60381645, scene.lights[0].color_specular.r);
    assert_eq!(0.60381645, scene.lights[0].color_specular.g);
    assert_eq!(0.60381645, scene.lights[0].color_specular.b);

    assert_eq!(0.60381645, scene.lights[0].color_ambient.b);
    assert_eq!(0.60381645, scene.lights[0].color_ambient.g);
    assert_eq!(0.60381645, scene.lights[0].color_ambient.r);

    assert_eq!(0.0, scene.lights[0].attenuation_constant);
    assert_eq!(0.0, scene.lights[0].attenuation_quadratic);
    assert_eq!(0.0, scene.lights[0].attenuation_linear);
    assert_eq!(6.2831855, scene.lights[0].angle_outer_cone);
    assert_eq!(6.2831855, scene.lights[0].angle_inner_cone);
    assert_eq!("Baz".to_string(), scene.lights[0].name);
    assert_eq!(0.0, scene.lights[0].up.x);
    assert_eq!(0.0, scene.lights[0].up.y);
    assert_eq!(0.0, scene.lights[0].up.z);
    assert_eq!(0.0, scene.lights[0].direction.x);
    assert_eq!(0.0, scene.lights[0].direction.y);
    assert_eq!(0.0, scene.lights[0].direction.z);

    assert_eq!(0.0, scene.lights[0].size.x);
    assert_eq!(0.0, scene.lights[0].size.x);

    assert_eq!(LightSourceType::Point, scene.lights[0].light_source_type);
    assert_eq!(LightSourceType::Area, scene.lights[1].light_source_type);
    assert_eq!(LightSourceType::Area, scene.lights[2].light_source_type);
}

#[test]
fn debug_light() {
    use crate::{
        scene::{PostProcess, Scene},
        utils,
    };

    let current_directory_buf = utils::get_model("models/BLEND/AreaLight_269.blend");

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

    dbg!(&scene.lights);
}
