use crate::{component::Component, parser::component_builder::ArgumentsMap};

pub mod container_component;
pub mod rect_component;
pub mod root_component;

pub fn create_component(name: &str, args: ArgumentsMap) -> Box<dyn Component> {
    match name {
        "root" => Box::new(root_component::Root::new_from_args(args)),
        "rect" => Box::new(rect_component::RectComponent::new_from_args(args)),
        _ => panic!("bad comp name {name}"),
    }
}
