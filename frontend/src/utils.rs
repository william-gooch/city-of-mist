use rand::Rng;
use rapier3d::na::*;
use rapier3d::prelude::*;

pub trait Random<T> {
    fn get(&self, rng: &mut impl Rng) -> T;
}

pub struct RandomVelocity(pub Real, pub Real);
pub struct RandomAngVel(pub Real);
pub struct RandomPosition(pub Real, pub Real, pub Real, pub Real, pub Real, pub Real);
pub struct RandomRotation;
pub struct RandomIsometry(pub RandomPosition);

impl Random<Vector3<Real>> for RandomVelocity {
    fn get(&self, rng: &mut impl Rng) -> Vector3<Real> {
        let magnitude = rng.gen_range(self.0..=self.1);
        magnitude
            * (Vector3::new(rng.gen_range(-1.0..=1.0), 0.0, rng.gen_range(-1.0..=1.0)).normalize())
    }
}

impl Random<Vector3<Real>> for RandomAngVel {
    fn get(&self, rng: &mut impl Rng) -> Vector3<Real> {
        Vector3::new(
            rng.gen_range(-self.0..=self.0),
            rng.gen_range(-self.0..=self.0),
            rng.gen_range(-self.0..=self.0),
        )
    }
}

impl Random<Vector3<Real>> for RandomPosition {
    fn get(&self, rng: &mut impl Rng) -> Vector3<Real> {
        Vector3::new(
            rng.gen_range(-self.0..=self.1),
            rng.gen_range(-self.2..=self.3),
            rng.gen_range(-self.4..=self.5),
        )
    }
}

impl Random<UnitQuaternion<Real>> for RandomRotation {
    fn get(&self, rng: &mut impl Rng) -> UnitQuaternion<Real> {
        UnitQuaternion::from_quaternion(Quaternion::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        ))
    }
}

impl Random<Isometry3<Real>> for RandomIsometry {
    fn get(&self, rng: &mut impl Rng) -> Isometry3<Real> {
        let position: Translation3<Real> = self.0.get(rng).into();
        let rotation = RandomRotation.get(rng);
        Isometry3::from_parts(position, rotation)
    }
}
