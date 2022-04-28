use rapier3d::prelude::*;
use rapier3d::na::*;

fn randrange(min: Real, max: Real) -> Real
{
    ((js_sys::Math::random() as Real) * (max - min)) + min
}

pub trait Random<T> {
    fn get(&self) -> T;
}

pub struct RandomVelocity(pub Real, pub Real);
pub struct RandomAngVel(pub Real);
pub struct RandomPosition(pub Real, pub Real, pub Real, pub Real, pub Real, pub Real);

impl Random<Vector3<Real>> for RandomVelocity {
    fn get(&self) -> Vector3<Real> {
        let magnitude = randrange(self.0, self.1);
        magnitude * (Vector3::new(
            randrange(-1.0, 1.0),
            0.0,
            randrange(-1.0, 1.0)
        ).normalize())
    }
}

impl Random<Vector3<Real>> for RandomAngVel {
    fn get(&self) -> Vector3<Real> {
        Vector3::new(
            randrange(-self.0, self.0),
            randrange(-self.0, self.0),
            randrange(-self.0, self.0)
        )
    }
}
