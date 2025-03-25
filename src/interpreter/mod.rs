use crate::parser::{self, ASTNode, VariableNode, VariableCallNode, PrintNode};

pub struct Interpreter
{
    pub ast: Vec<ASTNode>,
    pub context: Vec<(String, String)>,
}

impl Interpreter
{
    pub fn new(ast: Vec<ASTNode>) -> Interpreter
    {
        Interpreter { ast, context: Vec::new() }
    }

    fn evaluate(&mut self, node: &ASTNode) -> String
    {
        match node
        {
            ASTNode::Number(num_node) =>
            {
                num_node.display().to_string()
            }
            ASTNode::String(str_node) =>
            {
                str_node.display().to_string()
            }
            ASTNode::UnaryOp(un_op_node) =>
            {
                let operator = un_op_node.operator.value.clone();
                let value = self.evaluate(&*un_op_node.node);

                if operator == "-"
                {
                    let num_value: f32 = value.parse().unwrap();
                    (-num_value).to_string()
                }
                else
                {
                    panic!("Unsupported unary operator");
                }
            }
            ASTNode::BinaryOp(bin_op_node) =>
            {
                let left_value = self.evaluate(&*bin_op_node.left);
                let right_value = self.evaluate(&*bin_op_node.right);
                let operator = bin_op_node.operator.value.clone();

                if operator == "+"
                {
                    if let (Ok(left_num), Ok(right_num)) = (left_value.parse::<f32>(), right_value.parse::<f32>())
                    {
                        (left_num + right_num).to_string()
                    }
                    else
                    {
                        format!("{}{}", left_value, right_value)
                    }
                }
                else if operator == "-"
                {
                    let left_num: f32 = left_value.parse().unwrap();
                    let right_num: f32 = right_value.parse().unwrap();
                    (left_num - right_num).to_string()
                }
                else if operator == "*"
                {
                    let left_num: f32 = left_value.parse().unwrap();
                    let right_num: f32 = right_value.parse().unwrap();
                    (left_num * right_num).to_string()
                }
                else if operator == "/"
                {
                    let left_num: f32 = left_value.parse().unwrap();
                    let right_num: f32 = right_value.parse().unwrap();
                    if right_num == 0.0
                    {
                        panic!("Error: Division by zero");
                    }
                    else
                    {
                        (left_num / right_num).to_string()
                    }
                }
                else
                {
                    panic!("Unsupported binary operator");
                }
            }
            ASTNode::Variable(var_node) =>
            {
                let var_name = var_node.name.clone();
                let value = self.evaluate(&*var_node.value);
                let mut variable_exists = false;
                for (name, _) in &mut self.context
                {
                    if *name == var_name
                    {
                        variable_exists = true;
                        break;
                    }
                }
                
                if !variable_exists
                {
                    self.context.push((var_name.clone(), value.clone()));
                }
                var_name
            }
            ASTNode::VariableCall(var_call_node) =>
            {
                let var_name = var_call_node.name.clone();
                let mut value = String::new();
                for (name, val) in &self.context
                {
                    if *name == var_name
                    {
                        value = val.clone();
                        break;
                    }
                }
                value
            }
            ASTNode::Print(print_node) =>
            {
                let value = self.evaluate(&*print_node.node);
                println!("{}", value);
                value
            }
            _ =>
            {
                panic!("Unsupported AST node");
            }
        }
    }

    pub fn interpret(&mut self) -> Vec<String>
    {
        let mut results = Vec::new();
        for node in self.ast.clone()
        {
            let result = self.evaluate(&node);
            results.push(result);
        }
        results
    }
}
