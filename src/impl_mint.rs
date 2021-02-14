use crate::{Matrix4x4, Vector2D, Vector3D};

macro_rules! from_vec2s {
    ($($minttype:ty => $russtype:ty),+) => {
        $(impl From<$minttype> for $russtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self {
                    x: v.x as _,
                    y: v.y as _
                }
            }
        }

        impl From<$russtype> for $minttype {
            #[inline]
            fn from(v: $russtype) -> Self {
                Self {
                    x: v.x.into(),
                    y: v.y.into()
                }
            }
        })+
    }
}

macro_rules! from_vec3s {
    ($($minttype:ty => $russtype:ty),+) => {
        $(impl From<$minttype> for $russtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self{
                    x: v.x as _,
                    y: v.y as _,
                    z: v.z as _,
                }
            }
        }

        impl From<$russtype> for $minttype {
            #[inline]
            fn from(v: $russtype) -> Self {
                Self {
                    x: v.x.into(),
                    y: v.y.into(),
                    z: v.z.into()
                }
            }
        })+
    }
}

from_vec2s!(
    mint::Vector2<f32> => Vector2D,
    mint::Point2<f32> => Vector2D
);
from_vec2s!(
    mint::Vector2<f64> => Vector2D,
    mint::Point2<f64> => Vector2D
);

from_vec3s!(
    mint::Vector3<f32> => Vector3D,
    mint::Point3<f32> => Vector3D
);
from_vec3s!(
    mint::Vector3<f64> => Vector3D,
    mint::Point3<f64> => Vector3D
);

macro_rules! from_mat4s {
    ($($minttype:ty => $russtype:ty),+) => {
        $(impl From<$minttype> for $russtype {
            #[inline]
            fn from(v: $minttype) -> Self {
                Self{
                    a1: v.x.x as _,
                    a2: v.x.y as _,
                    a3: v.x.z as _,
                    a4: v.x.w as _,
                    b1: v.y.x as _,
                    b2: v.y.y as _,
                    b3: v.y.z as _,
                    b4: v.y.w as _,
                    c1: v.z.x as _,
                    c2: v.z.y as _,
                    c3: v.z.z as _,
                    c4: v.z.w as _,
                    d1: v.w.x as _,
                    d2: v.w.y as _,
                    d3: v.w.z as _,
                    d4: v.w.w as _,
                }
            }
        }

        impl From<$russtype> for $minttype {
            #[inline]
            fn from(v: $russtype) -> Self {
                Self {
                    x: mint::Vector4 { x: v.a1.into(), y: v.a2.into(), z: v.a3.into(), w: v.a4.into() },
                    y: mint::Vector4 { x: v.b1.into(), y: v.b2.into(), z: v.b3.into(), w: v.b4.into() },
                    z: mint::Vector4 { x: v.c1.into(), y: v.c2.into(), z: v.c3.into(), w: v.c4.into() },
                    w: mint::Vector4 { x: v.d1.into(), y: v.d2.into(), z: v.d3.into(), w: v.d4.into() },
                }
            }
        })+
    }
}

from_mat4s!(mint::ColumnMatrix4<f32> => Matrix4x4);
from_mat4s!(mint::ColumnMatrix4<f64> => Matrix4x4);
