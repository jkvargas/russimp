use russimp_sys::{
    aiLight,
    aiVector3D,
    aiColor3D,
    aiVector2D,
    aiLightSourceType_aiLightSource_UNDEFINED,
    aiLightSourceType_aiLightSource_AMBIENT,
    aiLightSourceType_aiLightSource_AREA,
    aiLightSourceType_aiLightSource_POINT,
    aiLightSourceType_aiLightSource_SPOT,
    aiLightSourceType_aiLightSource_DIRECTIONAL,
    aiLightSourceType,
};
use num_traits::ToPrimitive;
use crate::scene::{Scene, PostProcessSteps};

pub struct Light<'a> {
    light: &'a aiLight,
    up: aiVector3D,
    pos: aiVector3D,
    name: String,
    angle_inner_cone: f32,
    angle_outer_cone: f32,
    attenuation_linear: f32,
    attenuation_quadratic: f32,
    attenuation_constant: f32,
    color_ambient: aiColor3D,
    color_specular: aiColor3D,
    color_diffuse: aiColor3D,
    direction: aiVector3D,
    size: aiVector2D,
    light_source_type: LightSourceType,
}

impl<'a> Light<'a> {
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
}

#[derive(ToPrimitive, Debug, PartialEq)]
#[repr(u32)]
pub enum LightSourceType {
    Undefined = aiLightSourceType_aiLightSource_UNDEFINED,
    Ambient = aiLightSourceType_aiLightSource_AMBIENT,
    Area = aiLightSourceType_aiLightSource_AREA,
    Point = aiLightSourceType_aiLightSource_POINT,
    Spot = aiLightSourceType_aiLightSource_SPOT,
    Directional = aiLightSourceType_aiLightSource_DIRECTIONAL,
}

impl<'a> Into<Light<'a>> for &'a aiLight {
    fn into(self) -> Light<'a> {
        Light {
            light: self,
            up: self.mUp,
            pos: self.mPosition,
            name: self.mName.into(),
            angle_inner_cone: self.mAngleInnerCone,
            angle_outer_cone: self.mAngleOuterCone,
            attenuation_linear: self.mAttenuationLinear,
            attenuation_quadratic: self.mAttenuationQuadratic,
            attenuation_constant: self.mAttenuationConstant,
            color_ambient: self.mColorAmbient,
            color_specular: self.mColorSpecular,
            color_diffuse: self.mColorDiffuse,
            direction: self.mDirection,
            size: self.mSize,
            light_source_type: Light::get_light_source_type_from(&self.mType),
        }
    }
}

#[test]
fn light_available() {
    let current_directory_buf = std::env::current_dir().unwrap().join("russimp-sys/assimp/test/models/BLEND/AreaLight_269.blend");

    let scene = Scene::from(current_directory_buf.to_str().unwrap(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

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