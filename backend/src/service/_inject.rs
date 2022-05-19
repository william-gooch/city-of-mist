use std::ops::Deref;
use std::sync::Arc;
use crate::service::{
    database::Db,
    character::CharacterMgr,
    rooms::Rooms,
};

pub struct Inject<T>(T);
pub struct Param<T, const N: usize>(T);

pub trait InjectParam {}

impl<T> InjectParam for Inject<T> {}
impl<T, const N: usize> InjectParam for Param<T, N> {}

impl InjectParam for () {}
impl<P0>
    InjectParam for (P0,)
    where P0: InjectParam {}
impl<P0, P1>
    InjectParam for (P0, P1)
    where P0: InjectParam,
          P1: InjectParam {}
impl<P0, P1, P2>
    InjectParam for (P0, P1, P2)
    where P0: InjectParam,
          P1: InjectParam,
          P2: InjectParam {}
impl<P0, P1, P2, P3>
    InjectParam for (P0, P1, P2, P3)
    where P0: InjectParam,
          P1: InjectParam,
          P2: InjectParam,
          P3: InjectParam {}

pub trait InjectedParam {
    type Result: InjectedParam;
}
impl<T, const N: usize> InjectedParam for Param<T, N> {
    type Result = Param<T, N>;
}

impl InjectedParam for () {
    type Result = ();
}
impl<T0, const N0: usize>
    InjectedParam for (Param<T0, N0>,) {
    type Result = (Param<T0, N0>,);
}
impl<T0>
    InjectedParam for (Inject<T0>,) {
    type Result = ();
}
impl<T0, const N0: usize, T1, const N1: usize>
    InjectedParam for (Param<T0, N0>, Param<T1, N1>) {
    type Result = (Param<T0, N0>, Param<T1, N1>);
}
impl<T0, const N0: usize, T1>
    InjectedParam for (Param<T0, N0>, Inject<T1>) {
    type Result = (Param<T0, N0>,);
}
impl<T0, T1, const N1: usize>
    InjectedParam for (Inject<T0>, Param<T1, N1>) {
    type Result = (Param<T1, N1>,);
}
impl<T0, T1>
    InjectedParam for (Inject<T0>, Inject<T1>) {
    type Result = ();
}

pub trait InjectFunction<P: InjectParam, O> {
    fn run(&mut self, param: P) -> O;
}

pub trait InjectFunctionExt<P: InjectParam, O> {
    fn inject<I: Injector<P> + Send + 'static>(self, injector: I) -> Box<dyn InjectedFunction + Send>;
}

impl<T, P: InjectParam + 'static, O> InjectFunctionExt<P, O> for T
where T: InjectFunction<P, O> + Send + 'static {
    fn inject<I: Injector<P> + Send + 'static>(self, injector: I) -> Box<dyn InjectedFunction -> O + Send> {
        Box::new(move || {
            self.run(injector.get())
        })
    }
}

impl<F, O> InjectFunction<(), O> for F
where F: FnMut() -> O {
    fn run(&mut self, _param: ()) -> O {
        self()
    }
}

impl<F, O, P0: InjectParam> InjectFunction<(P0,), O> for F
where F: FnMut(P0) -> O {
    fn run(&mut self, param: (P0,)) -> O {
        self(param.0)
    }
}

impl<F, O, P0: InjectParam, P1: InjectParam> InjectFunction<(P0, P1), O> for F
where F: FnMut(P0, P1) -> O {
    fn run(&mut self, param: (P0, P1)) -> O {
        self(param.0, param.1)
    }
}

impl<F, O, P0: InjectParam, P1: InjectParam, P2: InjectParam> InjectFunction<(P0, P1, P2), O> for F
where F: FnMut(P0, P1, P2) -> O {
    fn run(&mut self, param: (P0, P1, P2)) -> O {
        self(param.0, param.1, param.2)
    }
}

impl<F, O, P0: InjectParam, P1: InjectParam, P2: InjectParam, P3: InjectParam> InjectFunction<(P0, P1, P2, P3), O> for F
where F: FnMut(P0, P1, P2, P3) -> O {
    fn run(&mut self, param: (P0, P1, P2, P3)) -> O {
        self(param.0, param.1, param.2, param.3)
    }
}

pub trait InjectedFunction<P: InjectedParam, O> {
    fn run(&mut self, param: P) -> O;
}

impl<F, O> InjectedFunction<(), O> for F
where F: FnMut() -> O {
    fn run(&mut self, _param: ()) -> O {
        self()
    }
}

impl<F, O, T0, const N0: usize> InjectedFunction<(Param<T0, N0>,), O> for F
where F: FnMut(Param<T0, N0>,) -> O {
    fn run(&mut self, param: (Param<T0, N0>,)) -> O {
        self(param.0)
    }
}

impl<F, O, T0, const N0: usize, T1, const N1: usize> InjectedFunction<(Param<T0, N0>, Param<T1, N1>), O> for F
where F: FnMut(Param<T0, N0>, Param<T1, N1>) -> O {
    fn run(&mut self, param: (Param<T0, N0>, Param<T1, N1>)) -> O {
        self(param.0, param.1)
    }
}

pub trait Injector<T, I>
where T: InjectParam {
    fn get<P: InjectedParam<Result = I>>(&self, param: P) -> T;
}

#[derive(Clone)]
pub struct AppInjector(Arc<AppInjectorInner>);
pub struct AppInjectorInner {
    db: Db,
    character_mgr: CharacterMgr,
    rooms: Rooms,
}

impl AppInjector {
    pub fn new(db: Db, character_mgr: CharacterMgr, rooms: Rooms) -> Self {
        Self(Arc::new(AppInjectorInner {
            db,
            character_mgr,
            rooms,
        }))
    }
}

impl<I> Injector<Inject<Db>, I> for AppInjector {
    fn get<P: InjectedParam<Result = I>>(&self, param: P) -> Inject<Db> {
        Inject(self.0.db.clone())
    }
}

impl<I> Injector<Inject<CharacterMgr>, I> for AppInjector {
    fn get<P: InjectedParam<Result = I>>(&self, param: P) -> Inject<CharacterMgr> {
        Inject(self.0.character_mgr.clone())
    }
}

impl<I> Injector<Inject<Rooms>, I> for AppInjector {
    fn get<P: InjectedParam<Result = I>>(&self, param: P) -> Inject<Rooms> {
        Inject(self.0.rooms.clone())
    }
}

impl<T, const N: usize, I> Injector<Param<T, N>, I> for AppInjector {
    fn get<P: InjectedParam<Result = I>>(&self, param: P) -> Param<T, N> {
        Param(param.get(N))
    }
}

impl<T> Deref for Inject<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Param<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
