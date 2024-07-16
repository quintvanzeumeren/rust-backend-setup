
pub type ResourceName = &'static str;

pub trait ResourceType {
    fn name() -> ResourceName;
}
