use std::sync::Arc;
use lazy_static::lazy_static;
use crate::modifiers::JokerEdition;
pub trait Joker: Sync + Send {
    fn get_edition(&self) -> JokerEdition;
    fn get_description(&self) -> String;
    fn get_name(&self) -> String;
    fn get_base_cost(&self) -> usize;
    fn get_cost(&self) -> i32 {
        return self.get_base_cost() as i32
    }

    fn get_shop_description(&self) -> String {
        return format!("{} - {} (Cost: {})", self.get_name(), self.get_description(), self.get_cost());
    }
    fn apply_mult(&self, current_mult: usize) -> usize {
        return current_mult;
    }
    fn apply_chips(&self, current_chips: usize) -> usize {
        return current_chips;
    }
}

macro_rules! create_joker_struct {
    ($struct_name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $struct_name { edition: JokerEdition }
    }
}
macro_rules! joker_base {
    ($name:expr, $description:expr, $cost:expr) => {

    fn get_edition(&self) -> JokerEdition {
        return self.edition;
    }
    fn get_name(&self) -> String {
        return $name
    }
    fn get_description(&self) -> String {
        return $description
    }
    fn get_base_cost(&self) -> usize {
        return $cost;
    }
    }
}
create_joker_struct!(JokerMult4);

impl Joker for JokerMult4 {
    joker_base!("Joker".to_string(), "This joker adds 4 to mult".to_string(), 4);
    fn apply_mult(&self, current_mult: usize) -> usize{
        return current_mult + 4;
    }
}
lazy_static! {
    pub static ref ALL_JOKERS: Vec<Arc<dyn Joker>> = {
        let mut v = Vec::new();
        v.push(Arc::new(JokerMult4::default()) as Arc<dyn Joker>);
        v
    };
}