use crate::lexer::{self, Token, TypeOfToken};

#[derive(Clone)]
pub enum ASTNode
{
    Number(NumberNode),
    String(StringNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    Variable(VariableNode),
    VariableCall(VariableCallNode),
    Print(PrintNode)
}

impl ASTNode 
{
    pub fn display(&self) -> String 
    {
        match self 
        {
            ASTNode::Number(node) => node.display().to_string(),
            ASTNode::BinaryOp(node) => node.display(),
            ASTNode::UnaryOp(node) => node.display(),
            ASTNode::String(node) => node.display().to_string(),
            ASTNode::Variable(node) => node.display(),
            ASTNode::VariableCall(node) => node.display(),
            ASTNode::Print(node) => node.display()
        }
    }
}

#[derive(Clone)]
pub struct NumberNode
{
    pub token: Token,
    pub is_neg: bool,
    pub value: String,
}

impl NumberNode
{
    pub fn new(token: Token, is_neg: bool) -> NumberNode
    {
        NumberNode 
        { 
            token: token.clone(), 
            is_neg: is_neg, 
            value: token.value 
        }
    }

    pub fn display(&self) -> &str
    {
        &self.value
    }
}

#[derive(Clone)]
pub struct StringNode
{
    pub token: Token,
    pub value: String,
}

impl StringNode
{
    pub fn new(token: Token) -> StringNode
    {
        StringNode 
        { 
            token: token.clone(), 
            value: token.value 
        }
    }

    pub fn display(&self) -> &str
    {
        &self.value
    }
}

#[derive(Clone)]
pub struct BinaryOpNode
{
    pub left: Box<ASTNode>,
    pub right: Box<ASTNode>,
    pub operator: Token,
}

impl BinaryOpNode
{
    pub fn new(left: ASTNode, right: ASTNode, operator: Token) -> BinaryOpNode
    {
        BinaryOpNode 
        { 
            left: Box::new(left), 
            right: Box::new(right), 
            operator: operator.clone() 
        }
    }

    pub fn display(&self) -> String
    {
        format!("({} {} {})", self.left.display(), self.operator.value, self.right.display())
    }
}

#[derive(Clone)]
pub struct UnaryOpNode
{
    pub node: Box<ASTNode>,
    pub operator: Token,
}

impl UnaryOpNode
{
    pub fn new(node: ASTNode, operator: Token) -> UnaryOpNode
    {
        UnaryOpNode 
        { 
            node: Box::new(node), 
            operator: operator.clone() 
        }
    }

    pub fn display(&self) -> String
    {
        format!("{}{}", self.operator.value, self.node.display())
    }
}

#[derive(Clone)]
pub struct VariableNode
{
    pub name: String,
    pub value: Box<ASTNode>,
}

impl VariableNode
{
    pub fn new(name: String, value: Box<ASTNode>) -> VariableNode
    {
        VariableNode { name, value }
    }

    pub fn display(&self) -> String
    {
        format!("{} = {}", self.name, self.value.display())
    }
}

#[derive(Clone)]
pub struct VariableCallNode
{
    pub name: String,
}

impl VariableCallNode
{
    pub fn new(name: String) -> VariableCallNode
    {
        VariableCallNode { name }
    }

    pub fn display(&self) -> String
    {
        self.name.clone()
    }
}

#[derive(Clone)]
pub struct PrintNode
{
    pub node: Box<ASTNode>,
}

impl PrintNode
{
    pub fn new(node: ASTNode) -> PrintNode
    {
        PrintNode { node: Box::new(node) }
    }

    pub fn display(&self) -> String
    {
        format!("print({})", self.node.display())
    }
}

pub struct Parser 
{
    pub tokens: Option<Vec<Token>>,
    pub index: usize,
    pub column: usize,
    pub line: usize,
}

impl Parser 
{
    pub fn new(tokens: Option<Vec<Token>>) -> Self 
    {
        Parser 
        {
            tokens,
            index: 0,
            column: 1,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Token 
    {
        if let Some(tokens) = self.tokens.clone() 
        {
            if self.index < tokens.len() 
            {
                let token = tokens[self.index].clone();
                self.index += 1;

                if token.tot == lexer::TypeOfToken::NEWLINE 
                {
                    self.column = 1;
                    self.line += 1;
                }
                else 
                {
                    self.column += token.value.len();
                }

                return token;
            }
            Token::new(lexer::TypeOfToken::EOF, "".to_string())
        }
        else 
        {
            panic!("No tokens available");
        }
    }

    pub fn peek_next_token(&self) -> &Token 
    {
        if let Some(tokens) = &self.tokens 
        {
            if self.index < tokens.len() 
            {
                &tokens[self.index]
            }
            else if !tokens.is_empty() 
            {
                &tokens[tokens.len() - 1]
            }
            else 
            {
                panic!("No tokens available")
            }
        }
        else 
        {
            panic!("No tokens available")
        }
    }

    pub fn is_at_end(&self) -> bool 
    {
        if let Some(tokens) = &self.tokens 
        {
            self.index >= tokens.len()
        }
        else 
        {
            true
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> 
    {
        let mut nodes = Vec::new();

        while !self.is_at_end() 
        {
            let node = self.parse_expr();
            nodes.push(node);
        }

        nodes
    }

    pub fn parse_expr(&mut self) -> ASTNode 
    {
        let mut node: ASTNode = self.parse_term();
        while self.peek_next_token().tot == TypeOfToken::OPERATOR
            && (self.peek_next_token().value == "+" || self.peek_next_token().value == "-")
        {
            let token: Token = self.next_token();
            node = ASTNode::BinaryOp(BinaryOpNode::new(node, self.parse_term(), token))
        }
        node
    }

    pub fn parse_term(&mut self) -> ASTNode 
    {
        let mut node = self.parse_factor();
        while self.peek_next_token().tot == TypeOfToken::OPERATOR
            && (self.peek_next_token().value == "*" || self.peek_next_token().value == "/")
        {
            let token: Token = self.next_token();
            node = ASTNode::BinaryOp(BinaryOpNode::new(node, self.parse_factor(), token))
        }
        node
    }

    pub fn parse_factor(&mut self) -> ASTNode 
    { 
        let token: Token = self.next_token();
        match token.tot 
        {
            TypeOfToken::NUMBER => 
            {
                ASTNode::Number(NumberNode::new(token, false))
            }
            TypeOfToken::OPERATOR if token.value == "-" => 
            {
                let node = self.parse_factor();
                ASTNode::UnaryOp(UnaryOpNode::new(node, token))
            }
            TypeOfToken::BLOCKDELIMITERS if token.value == "(" => 
            {
                let node = self.parse_expr();
                self.expect_token(TypeOfToken::BLOCKDELIMITERS, ")");
                node
            }
            TypeOfToken::IDENTIFIER =>
            {
                ASTNode::VariableCall(VariableCallNode::new(token.value))
            }
            TypeOfToken::KEYWORD => 
            {
                let keyword = token.value.clone();
                match keyword.as_str() 
                {
                    "print" => 
                    {
                        let node = self.parse_expr();
                        ASTNode::Print(PrintNode::new(node))
                    }
                    "var" => 
                    {
                        let name = self.next_token();
                        self.expect_token(TypeOfToken::OPERATOR, "=");
                        let value = self.parse_expr();
                        ASTNode::Variable(VariableNode::new(name.value, Box::new(value)))
                    }
                    _ => panic!("Unexpected keyword: {}", keyword),
                }
            }
            TypeOfToken::STRING => 
            {
                ASTNode::String(StringNode::new(token))
            }
            _ => panic!("Unexpected token: {}", token.to_string()),
        }
    }

    fn expect_token(&mut self, expected_type: TypeOfToken, expected_value: &str) 
    {
        let token = self.next_token();
        if token.tot != expected_type || token.value != expected_value 
        {
            panic!("Expected token: {} {}, but got {} {}", expected_type.to_string(), expected_value, token.tot.to_string(), token.value);
        }
    }
}
