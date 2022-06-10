use bevy::prelude::*;

pub struct MathDisplayer<'a, T> {
    pub value: &'a T
}

pub trait DisplayerExt : Sized {
    fn display(&self) -> MathDisplayer<'_, Self>;
}

impl DisplayerExt for Transform {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl DisplayerExt for GlobalTransform {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl DisplayerExt for Vec2 {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl DisplayerExt for Vec3 {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl DisplayerExt for Vec4 {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl DisplayerExt for Quat {
    fn display(&self) -> MathDisplayer<Self> {
        MathDisplayer { value: self }
    }
}

impl std::fmt::Display for MathDisplayer<'_, Transform> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Transform {
            translation,
            rotation,
            scale,
        } = self.value;
        let (t, r, s) = (
            translation.as_ref(),
            rotation.as_ref(),
            scale.as_ref()
        );
        if let Some(p) = f.precision() {
            write!(f, "{{ {:.p$?}, {:.p$?}, {:.p$?} }}", t, r, s)
        } else {
            write!(f, "{{ {:?}, {:?}, {:?} }}", t, r, s)
        }
    }
}

impl std::fmt::Display for MathDisplayer<'_, GlobalTransform> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GlobalTransform {
            translation,
            rotation,
            scale,
        } = self.value;
        write!(f, "{{ ")?;
        translation.display().fmt(f)?;
        write!(f, ", ")?;
        rotation.display().fmt(f)?;
        write!(f, ", ")?;
        scale.display().fmt(f)?;
        write!(f, " }}")
    }
}

use std::fmt::Debug;

impl std::fmt::Display for MathDisplayer<'_, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().fmt(f)
    }
}

impl std::fmt::Display for MathDisplayer<'_, Vec3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().fmt(f)
    }
}

impl std::fmt::Display for MathDisplayer<'_, Vec4> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().fmt(f)
    }
}

impl std::fmt::Display for MathDisplayer<'_, Quat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().fmt(f)
    }
}
