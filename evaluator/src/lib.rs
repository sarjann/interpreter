use ast::ast::{Expression, Statement};
use ast::{expressions, statements};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Env>,
}

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: Env) -> Env {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }
}

#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub parameters: Vec<String>,
    pub body: Rc<statements::BlockStatement>,
    pub env: Env,
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Function(FunctionObject),
    Error(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(value) => write!(f, "{}", value),
            Object::Function(function) => {
                let params = function.parameters.join(", ");
                write!(f, "fn({}) {{ ... }}", params)
            }
            Object::Error(message) => write!(f, "ERROR: {}", message),
        }
    }
}

pub fn eval(program: &statements::ProgramStatement, env: Env) -> Object {
    eval_program(program, env)
}

fn eval_program(program: &statements::ProgramStatement, env: Env) -> Object {
    let mut result = Object::Null;
    for statement in program.body.iter() {
        result = eval_statement(statement.as_ref(), env.clone());

        match result {
            Object::ReturnValue(value) => return *value,
            Object::Error(_) => return result,
            _ => {}
        }
    }
    result
}

fn eval_block_statement(block: &statements::BlockStatement, env: Env) -> Object {
    let mut result = Object::Null;

    for statement in block.statements.iter() {
        let statement = match statement {
            Some(statement) => statement,
            None => continue,
        };

        result = eval_statement(statement.as_ref(), env.clone());
        match result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }
    result
}

fn eval_statement(statement: &dyn Statement, env: Env) -> Object {
    if let Some(let_stmt) = statement
        .as_any()
        .downcast_ref::<statements::LetStatement>()
    {
        let value = match &let_stmt.value {
            Some(expr) => eval_expression(expr.as_ref(), env.clone()),
            None => Object::Null,
        };
        if is_error(&value) {
            return value;
        }
        env.borrow_mut()
            .set(let_stmt.name.value.clone(), value);
        return Object::Null;
    }

    if let Some(return_stmt) = statement
        .as_any()
        .downcast_ref::<statements::ReturnStatement>()
    {
        let value = match &return_stmt.return_value {
            Some(expr) => eval_expression(expr.as_ref(), env),
            None => Object::Null,
        };
        if is_error(&value) {
            return value;
        }
        return Object::ReturnValue(Box::new(value));
    }

    if let Some(expr_stmt) = statement
        .as_any()
        .downcast_ref::<statements::ExpressionStatement>()
    {
        return match &expr_stmt.expression {
            Some(expr) => eval_expression(expr.as_ref(), env),
            None => Object::Null,
        };
    }

    Object::Null
}

fn eval_expression(expression: &dyn Expression, env: Env) -> Object {
    if let Some(identifier) = expression
        .as_any()
        .downcast_ref::<expressions::Identifier>()
    {
        return eval_identifier(identifier, env);
    }

    if let Some(literal) = expression
        .as_any()
        .downcast_ref::<expressions::IntegerLiteral>()
    {
        return Object::Integer(literal.value);
    }

    if let Some(literal) = expression.as_any().downcast_ref::<expressions::Bool>() {
        return Object::Boolean(literal.value);
    }

    if let Some(prefix) = expression.as_any().downcast_ref::<expressions::Prefix>() {
        let right_expr = match &prefix.right {
            Some(expr) => expr,
            None => return new_error("missing right expression".to_string()),
        };
        let right = eval_expression(right_expr.as_ref(), env);
        if is_error(&right) {
            return right;
        }
        return eval_prefix_expression(&prefix.operator, right);
    }

    if let Some(infix) = expression.as_any().downcast_ref::<expressions::Infix>() {
        let left_expr = match &infix.left {
            Some(expr) => expr,
            None => return new_error("missing left expression".to_string()),
        };
        let right_expr = match &infix.right {
            Some(expr) => expr,
            None => return new_error("missing right expression".to_string()),
        };

        let left = eval_expression(left_expr.as_ref(), env.clone());
        if is_error(&left) {
            return left;
        }
        let right = eval_expression(right_expr.as_ref(), env);
        if is_error(&right) {
            return right;
        }
        return eval_infix_expression(&infix.operator, left, right);
    }

    if let Some(if_expr) = expression.as_any().downcast_ref::<expressions::If>() {
        return eval_if_expression(if_expr, env);
    }

    if let Some(func) = expression
        .as_any()
        .downcast_ref::<expressions::FunctionLiteral>()
    {
        let parameters = func
            .parameters
            .as_ref()
            .map(|params| params.iter().map(|param| param.value.clone()).collect())
            .unwrap_or_default();
        let body = match &func.body {
            Some(body) => Rc::clone(body),
            None => return new_error("missing function body".to_string()),
        };
        return Object::Function(FunctionObject {
            parameters,
            body,
            env,
        });
    }

    if let Some(call) = expression
        .as_any()
        .downcast_ref::<expressions::CallExpression>()
    {
        let function_expr = match &call.function {
            Some(expr) => expr,
            None => return new_error("missing function expression".to_string()),
        };
        let function = eval_expression(function_expr.as_ref(), env.clone());
        if is_error(&function) {
            return function;
        }

        let args = match &call.arguments {
            Some(args) => match eval_expressions(args, env) {
                Ok(args) => args,
                Err(err) => return err,
            },
            None => vec![],
        };

        return apply_function(function, args);
    }

    Object::Null
}

fn eval_expressions(
    expressions: &[Box<dyn Expression>],
    env: Env,
) -> Result<Vec<Object>, Object> {
    let mut result = Vec::new();
    for expression in expressions.iter() {
        let evaluated = eval_expression(expression.as_ref(), env.clone());
        if is_error(&evaluated) {
            return Err(evaluated);
        }
        result.push(evaluated);
    }
    Ok(result)
}

fn eval_identifier(identifier: &expressions::Identifier, env: Env) -> Object {
    match env.borrow().get(&identifier.value) {
        Some(value) => value,
        None => new_error(format!("identifier not found: {}", identifier.value)),
    }
}

fn eval_if_expression(if_expr: &expressions::If, env: Env) -> Object {
    let condition_expr = match &if_expr.condition {
        Some(expr) => expr,
        None => return new_error("missing if condition".to_string()),
    };
    let condition = eval_expression(condition_expr.as_ref(), env.clone());
    if is_error(&condition) {
        return condition;
    }

    if is_truthy(&condition) {
        match &if_expr.first {
            Some(block) => eval_block_statement(block, env),
            None => Object::Null,
        }
    } else {
        match &if_expr.second {
            Some(block) => eval_block_statement(block, env),
            None => Object::Null,
        }
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!("unknown operator: {}{}", operator, object_type(&right))),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(operator, *left, *right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(operator, *left, *right)
        }
        (Object::Null, Object::Null) => match operator {
            "==" => Object::Boolean(true),
            "!=" => Object::Boolean(false),
            _ => new_error(format!(
                "unknown operator: {} {} {}",
                object_type(&left),
                operator,
                object_type(&right)
            )),
        },
        _ => match operator {
            "==" => Object::Boolean(false),
            "!=" => Object::Boolean(true),
            _ => new_error(format!(
                "type mismatch: {} {} {}",
                object_type(&left),
                operator,
                object_type(&right)
            )),
        },
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => new_error(format!("unknown operator: INTEGER {} INTEGER", operator)),
    }
}

fn eval_boolean_infix_expression(operator: &str, left: bool, right: bool) -> Object {
    match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => new_error(format!("unknown operator: BOOLEAN {} BOOLEAN", operator)),
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(value) => Object::Integer(-value),
        _ => new_error(format!("unknown operator: -{}", object_type(&right))),
    }
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Function(function) => {
            let extended_env = extend_function_env(&function, args);
            let evaluated = eval_block_statement(function.body.as_ref(), extended_env);
            unwrap_return_value(evaluated)
        }
        _ => new_error(format!("not a function: {}", object_type(&function))),
    }
}

fn extend_function_env(function: &FunctionObject, args: Vec<Object>) -> Env {
    let env = Environment::new_enclosed(Rc::clone(&function.env));
    for (param, arg) in function.parameters.iter().zip(args.into_iter()) {
        env.borrow_mut().set(param.clone(), arg);
    }
    env
}

fn unwrap_return_value(obj: Object) -> Object {
    match obj {
        Object::ReturnValue(value) => *value,
        _ => obj,
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(value) => *value,
        Object::Null => false,
        _ => true,
    }
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}

fn object_type(obj: &Object) -> &'static str {
    match obj {
        Object::Integer(_) => "INTEGER",
        Object::Boolean(_) => "BOOLEAN",
        Object::Null => "NULL",
        Object::ReturnValue(_) => "RETURN_VALUE",
        Object::Function(_) => "FUNCTION",
        Object::Error(_) => "ERROR",
    }
}

fn new_error(message: String) -> Object {
    Object::Error(message)
}
