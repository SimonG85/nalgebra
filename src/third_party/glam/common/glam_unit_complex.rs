use super::glam::{DMat2, DQuat, DVec3, Mat2, Quat, Vec3};
use crate::{Complex, UnitComplex};

impl From<UnitComplex<f32>> for Mat2 {
    #[inline]
    fn from(e: UnitComplex<f32>) -> Mat2 {
        e.to_rotation_matrix().into_inner().into()
    }
}

impl From<UnitComplex<f64>> for DMat2 {
    #[inline]
    fn from(e: UnitComplex<f64>) -> DMat2 {
        e.to_rotation_matrix().into_inner().into()
    }
}

impl From<Mat2> for UnitComplex<f32> {
    #[inline]
    fn from(e: Mat2) -> UnitComplex<f32> {
        UnitComplex::new_normalize(Complex::new(e.x_axis.x, e.x_axis.y))
    }
}

impl From<DMat2> for UnitComplex<f64> {
    #[inline]
    fn from(e: DMat2) -> UnitComplex<f64> {
        UnitComplex::new_normalize(Complex::new(e.x_axis.x, e.x_axis.y))
    }
}

impl From<UnitComplex<f32>> for Quat {
    fn from(rot: UnitComplex<f32>) -> Quat {
        Quat::from_axis_angle(Vec3::Z, rot.angle())
    }
}

impl From<UnitComplex<f64>> for DQuat {
    fn from(rot: UnitComplex<f64>) -> DQuat {
        DQuat::from_axis_angle(DVec3::Z, rot.angle())
    }
}
