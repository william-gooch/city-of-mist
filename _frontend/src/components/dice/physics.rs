use rapier3d::{na::*, prelude::*};
use std::cell::Ref;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use web_sys::console::log_1;

pub struct PhysicsState {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    gravity: nalgebra::Vector3<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

#[derive(Clone)]
pub struct Physics {
    state: Rc<RefCell<PhysicsState>>,
}

impl Physics {
    pub fn new() -> Self {
        let scale = 0.3;

        let rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let joint_set = JointSet::new();
        let ccd_solver = CCDSolver::new();

        let container = ColliderBuilder::compound(vec![
            (
                Isometry3::translation(0.0, 0.0, 0.0),
                SharedShape::halfspace(Vector3::y_axis()),
            ),
            (
                Isometry3::translation(-scale, 0.0, 0.0),
                SharedShape::halfspace(Vector3::x_axis()),
            ),
            (
                Isometry3::translation(scale, 0.0, 0.0),
                SharedShape::halfspace(-Vector3::x_axis()),
            ),
            (
                Isometry3::translation(0.0, 0.0, -scale),
                SharedShape::halfspace(Vector3::z_axis()),
            ),
            (
                Isometry3::translation(0.0, 0.0, scale),
                SharedShape::halfspace(-Vector3::z_axis()),
            ),
        ])
        .build();
        collider_set.insert(container);

        let physics_hooks = ();
        let event_handler = ();

        let state = PhysicsState {
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
        };
        let state = Rc::new(RefCell::new(state));

        Self { state }
    }

    pub fn add_rigid_body(&self, rigid_body: RigidBody) -> RigidBodyHandle {
        let mut state = self.state.borrow_mut();
        state.rigid_body_set.insert(rigid_body)
    }

    pub fn remove_rigid_body(&self, rigid_body_handle: RigidBodyHandle) {
        let state = &mut *self.state.borrow_mut();
        state.rigid_body_set.remove(
            rigid_body_handle,
            &mut state.island_manager,
            &mut state.collider_set,
            &mut state.joint_set,
        );
    }

    pub fn get_rigid_body(
        &self,
        rigid_body_handle: RigidBodyHandle,
    ) -> impl Deref<Target = RigidBody> + '_ {
        Ref::map(self.state.borrow(), |state| {
            &state.rigid_body_set[rigid_body_handle]
        })
    }

    pub fn add_collider_with_parent(
        &self,
        collider: Collider,
        rigid_body_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        let state = &mut *self.state.borrow_mut();
        let cset = &mut state.collider_set;
        let rset = &mut state.rigid_body_set;
        cset.insert_with_parent(collider, rigid_body_handle, rset)
    }

    pub fn tick(&self) {
        let mut state = &mut *self.state.borrow_mut();
        state.physics_pipeline.step(
            &state.gravity,
            &state.integration_parameters,
            &mut state.island_manager,
            &mut state.broad_phase,
            &mut state.narrow_phase,
            &mut state.rigid_body_set,
            &mut state.collider_set,
            &mut state.joint_set,
            &mut state.ccd_solver,
            &state.physics_hooks,
            &state.event_handler,
        );
    }

    pub fn value(&self, rigid_body_handle: RigidBodyHandle) -> Option<i8> {
        let state = &*self.state.borrow();
        let rb = &state.rigid_body_set[rigid_body_handle];
        if rb.linvel().magnitude() > 0.01 || rb.angvel().magnitude() > 0.01 {
            None
        } else {
            let up_dir = rb.rotation() * Vector3::y();
            let front_dir = rb.rotation() * Vector3::x();
            let left_dir = rb.rotation() * Vector3::z();

            if up_dir.y > 0.99 {
                Some(4)
            } else if up_dir.y < -0.99 {
                Some(3)
            } else if front_dir.y > 0.99 {
                Some(5)
            } else if front_dir.y < -0.99 {
                Some(2)
            } else if left_dir.y > 0.99 {
                Some(6)
            } else if left_dir.y < -0.99 {
                Some(1)
            } else {
                None
            }
        }
    }
}
