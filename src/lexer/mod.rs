pub static KEYWORDS: &[&str] =
&[
    "var", "for", "in", "foreach", "if", "else",
    "elseif", "return", "void", "while", "once",
    "import", "define", "and", "not", "or", "true",
    "false", "break", "print"
];

pub static BLOCKDELIMITERS: [char; 6] =
[
    '{', '}', '(', ')', '[', ']'
];

pub static PUNCTUATION:  &[char] = 
&[
    '.', ','
];

pub static OPERATORS: &[&str] =
&[
    "+" , "-" , "/" , "*" , "=" ,
    "+=", "-=", "/=", "*=", "==",
    "<", ">", "<=", ">=", "!=",
    "++", "--"
];

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum TypeOfToken 
{
    NUMBER,
    STRING,
    IDENTIFIER,
    KEYWORD,
    OPERATOR,
    PUNCTUATION,
    BLOCKDELIMITERS,
    NEWLINE,
    WHITESPACE,
    EOF
}

impl TypeOfToken
{
    pub fn to_string(&self) -> String 
    {
        match self 
        {
            TypeOfToken::NUMBER => "Number".to_string(),
            TypeOfToken::STRING => "String".to_string(),
            TypeOfToken::IDENTIFIER => "Identifier".to_string(),
            TypeOfToken::KEYWORD => "Keyword".to_string(),
            TypeOfToken::OPERATOR => "Operator".to_string(),
            TypeOfToken::PUNCTUATION => "Punctuation".to_string(),
            TypeOfToken::BLOCKDELIMITERS => "Block Delimiters".to_string(),
            TypeOfToken::NEWLINE => "Newline".to_string(),
            TypeOfToken::WHITESPACE => "Whitespace".to_string(),
            TypeOfToken::EOF => "EOF".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Token 
{
    pub tot: TypeOfToken,
    pub value: String,
}

impl Token 
{
    pub fn new(tot: TypeOfToken, value: String) -> Token 
    {
        Token { tot, value }
    }

    pub fn to_string(&self) -> String 
    {
        format!("Token: {:?}, Value: {}", self.tot, self.value)
    }
}

pub struct Lexer 
{
    src: String,
    index: usize,
    line: usize,
    column: usize
}

impl Lexer
{
    pub fn new(src: String) -> Lexer
    {
        Lexer { src, index: 0, line: 1, column: 1 }
    }

    pub fn next_char(&mut self) -> char
    {
        if self.src[self.index..].is_empty() 
        {
            return '\0';
        }
        let c = self.src[self.index..].chars().next().unwrap();
        self.index += 1;
        if c == '\n' 
        {
            self.line += 1;
            self.column = 1;
        } 
        else 
        {
            self.column += 1;
        }
        c
    }

    pub fn peek_next_char(&self) -> char
    {
        if self.src[self.index..].is_empty() 
        {
            return '\0';
        }
        self.src[self.index..].chars().next().unwrap()
    }

    pub fn tokenize(&mut self) -> Vec<Token>
    {
        let mut tokens = Vec::new();
        while self.index < self.src.len() 
        {
            let c = self.next_char();
            if c.is_whitespace() 
            {
                continue;
            } 
            else if c == '\n'
            {
                tokens.push(Token::new(TypeOfToken::NEWLINE, "newline".to_string()));
            }
            else if c.is_alphabetic() || c == '_' 
            {
                let mut identifier = String::new();
                identifier.push(c);
                while self.peek_next_char().is_alphanumeric() || self.peek_next_char() == '_' 
                {
                    identifier.push(self.next_char());
                }
                if KEYWORDS.contains(&identifier.as_str()) 
                {
                    tokens.push(Token::new(TypeOfToken::KEYWORD, identifier));
                } 
                else 
                {
                    tokens.push(Token::new(TypeOfToken::IDENTIFIER, identifier));
                }
            } 
            else if c.is_digit(10) 
            {
                let mut number = String::new();
                number.push(c);
                let mut has_decimal_point = false;
                while self.peek_next_char().is_digit(10) || (self.peek_next_char() == '.' && !has_decimal_point) 
                {
                    let next_char = self.next_char();
                    if next_char == '.' 
                    {
                        has_decimal_point = true;
                    }
                    number.push(next_char);
                }
                tokens.push(Token::new(TypeOfToken::NUMBER, number));
            }
            else if c == '"' 
            {
                let mut string = String::new();
                while self.peek_next_char() != '"' && self.peek_next_char() != '\0' 
                {
                    string.push(self.next_char());
                }
                if self.peek_next_char() == '"' 
                {
                    self.next_char(); 
                    tokens.push(Token::new(TypeOfToken::STRING, string));
                }
            } 
            else if PUNCTUATION.contains(&c) 
            {
                tokens.push(Token::new(TypeOfToken::PUNCTUATION, c.to_string()));
            } 
            else if OPERATORS.contains(&c.to_string().as_str()) 
            {
                tokens.push(Token::new(TypeOfToken::OPERATOR, c.to_string()));
            } 
            else if BLOCKDELIMITERS.contains(&c) 
            {
                tokens.push(Token::new(TypeOfToken::BLOCKDELIMITERS, c.to_string()));
            } 
            else if c == '#' 
            {
                let mut comment = String::new();
                comment.push(c);
                while self.peek_next_char() != '\n' && self.peek_next_char() != '\0' 
                {
                    comment.push(self.next_char());
                }
                continue; 
            }
        }
        tokens
    }
}
