use crate::Scheduler;

pub trait Plugin: Send + 'static {
    fn new(builder: &mut PluginBuilder);
}

pub type PluginBuilder = Scheduler;
