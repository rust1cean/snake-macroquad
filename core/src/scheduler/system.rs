use crate::Storage;
use std::{
    marker::PhantomData,
    time::{Duration, SystemTime},
};

// Transforming function into system

pub trait IntoSystem<Params: SystemParam>: Send + 'static {
    type System: System;

    fn into(self, call_interval: Option<Duration>) -> Self::System;
}

impl<F, Params: SystemParam> IntoSystem<Params> for F
where
    F: SystemParamFunction<Params>,
{
    type System = FunctionSystem<F, Params>;

    fn into(self, call_interval: Option<Duration>) -> Self::System {
        FunctionSystem {
            system: self,
            params: PhantomData,
            last_call: Duration::ZERO,
            call_interval,
        }
    }
}

pub struct FunctionSystem<F: 'static, Params: SystemParam> {
    system: F,
    params: PhantomData<Params>,
    call_interval: Option<Duration>,
    last_call: Duration,
}

// Run system

pub trait System: Send + 'static {
    fn run(&mut self, storage: &mut Storage);
}

impl<F, Params: SystemParam> System for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Params> + Send + 'static,
{
    fn run(&mut self, storage: &mut Storage) {
        // Run if there is no timer or the time has passed
        if let Some(call_interval) = self.call_interval {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(now) => {
                    if self.last_call <= now {
                        self.last_call = now + call_interval;
                    } else {
                        return ();
                    }
                }
                Err(err) => panic!("{}", err),
            }
        }

        SystemParamFunction::run(&mut self.system, storage);
    }
}

// System's params
pub trait SystemParam: Send + 'static {
    fn fetch(_: &mut Storage) -> Option<&mut Self> {
        None
    }
}

impl SystemParam for Storage {
    fn fetch(storage: &mut Storage) -> Option<&mut Storage> {
        Some(storage)
    }
}

// Tuples
// TODO: Add macro

impl SystemParam for () {}
impl<T1: SystemParam> SystemParam for (T1,) {}
impl<T1: SystemParam, T2: SystemParam> SystemParam for (T1, T2) {}

// Calling functions in systems
// TODO: Add macro

pub trait SystemParamFunction<Params: SystemParam>: Send + 'static {
    fn run(&mut self, storage: &mut Storage);
}

impl<F> SystemParamFunction<()> for F
where
    F: Fn() -> () + Send + 'static,
{
    fn run(&mut self, _: &mut Storage) {
        self();
    }
}

impl<F, P1: SystemParam> SystemParamFunction<(P1,)> for F
where
    F: Fn(&mut P1) -> () + Send + 'static,
{
    fn run(&mut self, storage: &mut Storage) {
        self(P1::fetch(storage).unwrap());
    }
}
