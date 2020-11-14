use russimp_sys::aiFace;

pub struct Face {
    face: aiFace
}

impl<'a> Into<Face> for aiFace {
    fn into(self) -> Face {
        Face {
            face: self
        }
    }
}
