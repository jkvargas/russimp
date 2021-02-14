use num_traits::ToPrimitive;

use crate::{
    scene::{PostProcessSteps, Scene},
    sys::{
        aiColor3D, aiLight, aiLightSourceType, aiLightSourceType_aiLightSource_AMBIENT,
        aiLightSourceType_aiLightSource_AREA, aiLightSourceType_aiLightSource_DIRECTIONAL,
        aiLightSourceType_aiLightSource_POINT, aiLightSourceType_aiLightSource_SPOT,
        aiLightSourceType_aiLightSource_UNDEFINED, aiVector2D, aiVector3D,
    },
    Color3D, Utils, Vector2D, Vector3D,
};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Light {
    up: Vector3D,
    pos: Vector3D,
    name: String,
    angle_inner_cone: f32,
    angle_outer_cone: f32,
    attenuation_linear: f32,
    attenuation_quadratic: f32,
    attenuation_constant: f32,
    color_ambient: Color3D,
    color_specular: Color3D,
    color_diffuse: Color3D,
    direction: Vector3D,
    size: Vector2D,
    light_source_type: LightSourceType,
}

impl Light {
    fn get_light_source_type_from(m_type: &aiLightSourceType) -> LightSourceType {
        if (ToPrimitive::to_u32(&LightSourceType::Area).unwrap() & *m_type) != 0 {
            return LightSourceType::Area;
        }

        if (ToPrimitive::to_u32(&LightSourceType::Ambient).unwrap() & *m_type) != 0 {
            return LightSourceType::Ambient;
        }

        if (ToPrimitive::to_u32(&LightSourceType::Spot).unwrap() & *m_type) != 0 {
            return LightSourceType::Spot;
        }

        if (ToPrimitive::to_u32(&LightSourceType::Point).unwrap() & *m_type) != 0 {
            return LightSourceType::Point;
        }

        if (ToPrimitive::to_u32(&LightSourceType::Directional).unwrap() & *m_type) != 0 {
            return LightSourceType::Directional;
        }

        if (ToPrimitive::to_u32(&LightSourceType::Undefined).unwrap() & *m_type) != 0 {
            return LightSourceType::Undefined;
        }

        LightSourceType::Undefined
    }

    pub fn new(light: &aiLight) -> Light {
        Self {
            up: Vector3D::new(&light.mUp),
            pos: Vector3D::new(&light.mPosition),
            name: light.mName.into(),
            angle_inner_cone: light.mAngleInnerCone,
            angle_outer_cone: light.mAngleOuterCone,
            attenuation_linear: light.mAttenuationLinear,
            attenuation_quadratic: light.mAttenuationQuadratic,
            attenuation_constant: light.mAttenuationConstant,
            color_ambient: Color3D::new(&light.mColorAmbient),
            color_specular: Color3D::new(&light.mColorSpecular),
            color_diffuse: Color3D::new(&light.mColorDiffuse),
            direction: Vector3D::new(&light.mDirection),
            size: Vector2D::new(&light.mSize),
            light_source_type: Light::get_light_source_type_from(&light.mType),
        }
    }
}

#[derive(Derivative, ToPrimitive, PartialEq)]
#[derivative(Debug)]
#[repr(u32)]
pub enum LightSourceType {
    Undefined = aiLightSourceType_aiLightSource_UNDEFINED,
    Ambient = aiLightSourceType_aiLightSource_AMBIENT,
    Area = aiLightSourceType_aiLightSource_AREA,
    Point = aiLightSourceType_aiLightSource_POINT,
    Spot = aiLightSourceType_aiLightSource_SPOT,
    Directional = aiLightSourceType_aiLightSource_DIRECTIONAL,
}

#[test]
fn light_available() {
    let current_directory_buf = Utils::get_model("models/BLEND/AreaLight_269.blend");

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

    assert_eq!(LightSourceType::Spot, scene.lights[0].light_source_type);
    assert_eq!(LightSourceType::Area, scene.lights[1].light_source_type);
    assert_eq!(LightSourceType::Area, scene.lights[2].light_source_type);
}

#[test]
fn debug_light() {
    let current_directory_buf = Utils::get_model("models/BLEND/AreaLight_269.blend");

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

    dbg!(&scene.lights);
}
