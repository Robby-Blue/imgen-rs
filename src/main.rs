use crate::{
    layout_plan::LayoutPlan,
    types::{
        attributes::AttributeType,
        expressions::{self},
        mults::AttributeDirection,
        values::{self, SizePx, Value},
    },
};

mod component;
mod components;
mod parser;
mod primitives;
mod renderer;
mod types;

mod layout_plan;

fn main() {
    let ast = parser::ast::parse_file_to_ast("sample.ign");
    let ast = match ast {
        Ok(ast) => ast,
        Err(err) => {
            err.print();
            return;
        }
    };
    let components = parser::component_builder::build_components(ast);

    let plan = LayoutPlan::new();
    let attrs = plan.plan(&components);

    let primitives = components
        .iter()
        .map(|c| c.component.to_primitives(c.id.clone(), &attrs))
        .flatten()
        .collect();

    let width =
        attrs.get_computed_value("root", AttributeType::Size, AttributeDirection::Horizontal);
    let height =
        attrs.get_computed_value("root", AttributeType::Size, AttributeDirection::Vertical);

    let width = expect_unit!(Value::Px, width).unwrap();
    let height = expect_unit!(Value::Px, height).unwrap();
    let size = SizePx::new(width as u32, height as u32);

    let ctx = renderer::render(size, primitives);
    let image = ctx.image;

    let _ = image.save("output.png");
}
