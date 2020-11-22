use russimp_sys::aiAnimation;

pub struct Animation {
    animation: *mut aiAnimation
}

impl Into<Animation> for *mut aiAnimation {
    fn into(self) -> Animation {
        Animation {
            animation: self
        }
    }
}

