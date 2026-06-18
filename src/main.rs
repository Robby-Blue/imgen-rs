use crate::{
    layout_plan::{LayoutComponent, LayoutPlan},
    types::{
        attributes::{AttributeType, Dependency},
        expressions::{self, Operation},
        mults::{self, AttributeDirection},
        units,
    },
    units::{SizePx, SizeUnit, UnitValue},
};

mod component;
mod components;
mod primitives;
mod renderer;
mod types;

mod layout_plan;

fn main() {
    let mut root_res = create_root();
    let components = root_res.get_all();

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

    let size = SizePx::new(width as u32, height as u32);

    let ctx = renderer::render(size, primitives);
    let image = ctx.image;

    let _ = image.save("output.png");
}

struct CreateComponentResult {
    children: Vec<LayoutComponent>,
    others: Vec<LayoutComponent>,
}

impl CreateComponentResult {
    pub fn new_leaf(children: Vec<LayoutComponent>) -> Self {
        CreateComponentResult {
            children,
            others: vec![],
        }
    }
    pub fn new(children: Vec<LayoutComponent>, mut old: CreateComponentResult) -> Self {
        let others = old.get_all();
        CreateComponentResult { children, others }
    }
    pub fn get_all(&mut self) -> Vec<LayoutComponent> {
        let mut children = std::mem::take(&mut self.children);
        children.append(&mut self.others);
        children
    }
    pub fn get_children_ids(&self) -> Vec<String> {
        self.children.iter().map(|c| c.id.clone()).collect()
    }
}

fn create_root() -> CreateComponentResult {
    let id = "root".to_string();

    let bg_res = create_bg(id.clone());

    let root_component = components::root_component::Root::new(mults::Directions::new(
        units::Size::Set(expressions::Expression::Value(
            UnitValue::<SizeUnit>::new_px(100),
        )),
        units::Size::Set(expressions::Expression::Value(
            UnitValue::<SizeUnit>::new_px(100),
        )),
    ));

    let root = LayoutComponent::new(
        Box::new(root_component),
        id,
        None,
        bg_res.get_children_ids(),
    );

    CreateComponentResult::new(vec![root], bg_res)
}

fn create_bg(parent_id: String) -> CreateComponentResult {
    let id = "bg".to_string();

    let rect_res = create_rect(id.clone());

    let bg_component = components::rect_component::RectComponent::new(
        mults::Directions::new(units::Position::Auto, units::Position::Auto),
        mults::Directions::new(units::Size::Max, units::Size::Max),
        image::Rgba([0, 200, 0, 255]),
        0,
        0,
        0,
    );

    let bg = LayoutComponent::new(
        Box::new(bg_component),
        id,
        Some(parent_id),
        rect_res.get_children_ids(),
    );

    CreateComponentResult::new(vec![bg], rect_res)
}

fn create_rect(parent_id: String) -> CreateComponentResult {
    let id = "rect".to_string();

    let inner_rect_res = create_inner_rect(id.clone());

    let rect_component = components::rect_component::RectComponent::new(
        mults::Directions::new(
            units::Position::Auto,
            units::Position::Float(expressions::Expression::Operation(Operation::new(
                Box::new(expressions::Expression::Variable(Dependency::new(
                    id.clone(),
                    AttributeType::Size,
                    AttributeDirection::Vertical,
                ))),
                expressions::Operator::Division,
                Box::new(expressions::Expression::Value(UnitValue::new_unitless(2))),
            ))),
        ),
        mults::Directions::new(
            units::Size::Max,
            units::Size::Set(expressions::Expression::Operation(Operation::new(
                Box::new(expressions::Expression::Variable(Dependency::new(
                    parent_id.clone(),
                    AttributeType::Size,
                    AttributeDirection::Vertical,
                ))),
                expressions::Operator::Division,
                Box::new(expressions::Expression::Value(UnitValue::new_unitless(2))),
            ))),
        ),
        image::Rgba([255, 0, 0, 255]),
        20,
        10,
        0,
    );

    let rect = LayoutComponent::new(
        Box::new(rect_component),
        id,
        Some(parent_id),
        inner_rect_res.get_children_ids(),
    );

    CreateComponentResult::new(vec![rect], inner_rect_res)
}

fn create_inner_rect(parent_id: String) -> CreateComponentResult {
    let id = "inner_rect".to_string();

    let inner_rect_component = components::rect_component::RectComponent::new(
        mults::Directions::new(units::Position::Auto, units::Position::Auto),
        mults::Directions::new(
            units::Size::Set(expressions::Expression::Operation(Operation::new(
                Box::new(expressions::Expression::Variable(Dependency::new(
                    parent_id.clone(),
                    AttributeType::Size,
                    AttributeDirection::Horizontal,
                ))),
                expressions::Operator::Division,
                Box::new(expressions::Expression::Value(UnitValue::new_unitless(2))),
            ))),
            units::Size::Max,
        ),
        image::Rgba([0, 0, 0, 255]),
        0,
        0,
        0,
    );

    let inner_rect =
        LayoutComponent::new(Box::new(inner_rect_component), id, Some(parent_id), vec![]);

    CreateComponentResult::new_leaf(vec![inner_rect])
}
