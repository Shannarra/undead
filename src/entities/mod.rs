pub mod zombie;

/// A trait representing what
/// an Entity means in the Zombie
/// language.
pub trait Entity {
    fn name(&self) -> &str;
    fn is_active(&self) -> bool;
    fn tasks_count(&self) -> i32;
    fn perform_tasks(&self);
}