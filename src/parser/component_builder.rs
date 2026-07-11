use std::collections::HashMap;

use glp::parser::nodes::{self, Definition, FuncCall};
use glp::tokenizer;

use crate::components;
use crate::expect_unit;
use crate::layout_plan::LayoutComponent;
use crate::parser::ast::CustomValue;
use crate::types::attributes::Attributes;
use crate::types::expressions::Operator;
use crate::types::values::Value;
use crate::{parser::ast::ImgCustomParser, types::expressions};

use crate::expressions::Expression;
use crate::expressions::Operation;

pub fn build_components(root_def: Definition<ImgCustomParser>) -> Vec<LayoutComponent> {
    func_to_component(*root_def.func, None).1
}

pub fn func_to_component(
    call: FuncCall<ImgCustomParser>,
    parent_id: Option<String>,
) -> (String, Vec<LayoutComponent>) {
    let name = call.name.text;

    let mut arg_map = HashMap::new();
    for arg in call.args {
        let key = arg.key.text;
        let val = transform_expression(*arg.val);

        arg_map.insert(key, val);
    }

    // eval the id based on nothing
    let attrs = Attributes::new();

    let id = arg_map.get("id").unwrap().evaluate(&attrs);
    let id = expect_unit!(Value::Text, id).unwrap();

    let mut child_comps = vec![];
    let mut child_ids = vec![];

    for func in call.funcs {
        let (child_id, mut children) = func_to_component(*func, Some(id.clone()));

        child_comps.append(&mut children);
        child_ids.push(child_id);
    }

    let comp = components::create_component(&name, ArgumentsMap::new(arg_map));

    let comp = LayoutComponent::new(comp, id.clone(), parent_id, child_ids);

    let mut comps = vec![comp];
    comps.append(&mut child_comps);

    (id, comps)
}

fn transform_expression(expr: nodes::Expression<ImgCustomParser>) -> Expression {
    match expr {
        nodes::Expression::Value(value) => transform_value_to_expr(value),
        nodes::Expression::Operation {
            val1,
            operator,
            val2,
        } => Expression::Operation(Operation::new(
            Box::new(transform_expression(*val2)),
            transform_operator(operator),
            Box::new(transform_expression(*val1)),
        )),
    }
}

fn transform_value_to_expr(value: nodes::Value<ImgCustomParser>) -> expressions::Expression {
    match value {
        nodes::Value::Number(v, u) => match u {
            Some(_) => Expression::Value(Value::Px(v)),
            None => Expression::Value(Value::Scalar(v)),
        },
        nodes::Value::String(s) => Expression::Value(Value::Text(s)),
        nodes::Value::Expression(expression) => transform_expression(*expression),
        nodes::Value::Custom(c) => transform_custom_value_to_expr(c),
    }
}

fn transform_custom_value_to_expr(val: CustomValue) -> expressions::Expression {
    match val {
        CustomValue::Hex(c) => Expression::Value(Value::HexColor(c)),
    }
}

fn transform_operator(expr: tokenizer::Operator) -> Operator {
    match expr {
        tokenizer::Operator::Addition => Operator::Addition,
        tokenizer::Operator::Subtraction => Operator::Subtraction,
        tokenizer::Operator::Multiplication => Operator::Multiplication,
        tokenizer::Operator::Division => Operator::Division,
    }
}

pub struct ArgumentsMap {
    map: HashMap<String, Expression>,
}

impl ArgumentsMap {
    pub fn new(map: HashMap<String, Expression>) -> Self {
        ArgumentsMap { map }
    }
    pub fn get(&self, key: &str, other: Expression) -> Expression {
        self.map.get(key).unwrap_or(&other).clone()
    }
}
