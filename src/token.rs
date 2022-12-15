use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(line: usize, tok_type: TokenType) -> Self {
        Self { line, tok_type }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.tok_type)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum TokenType {
    // keywords
    AND,
    OR,
    IF,
    THEN,
    ELSE,
    ELSEIF,
    WHILE,
    FOR,
    DO,
    END,
    BREAK,
    LOCAL,
    TRUE,
    FALSE,
    IN,
    NOT,
    FUNCTION,
    NIL,
    RETURN,

    // punctuations
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACKET,
    RIGHTBRACKET,
    DOUBLELEFTBRACKET,
    DOUBLERIGHTBRACKET,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    SEMICOLON,

    // arith
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
    FLOORDIV,
    DOTDOT,

    EQUAL,
    EQUALEQUAL,
    NOTEQUAL, // ~=
    GREATER,
    LESS,
    GREATEREQUAL,
    LESSEQUAL,

    // types
    NUMBER { value: f64 },
    NAME { value: String },
    STRING { value: String },

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;
        match self {
            AND => write!(f, "and"),
            OR => write!(f, "or"),
            IF => write!(f, "if"),
            THEN => write!(f, "then"),
            ELSE => write!(f, "else"),
            ELSEIF => write!(f, "elseif"),
            WHILE => write!(f, "while"),
            FOR => write!(f, "for"),
            DO => write!(f, "do"),
            END => write!(f, "end"),
            BREAK => write!(f, "break"),
            LOCAL => write!(f, "local"),
            TRUE => write!(f, "true"),
            FALSE => write!(f, "false"),
            IN => write!(f, "in"),
            NOT => write!(f, "not"),
            FUNCTION => write!(f, "function"),
            NIL => write!(f, "nil"),
            RETURN => write!(f, "return"),

            LEFTPAREN => write!(f, "("),
            RIGHTPAREN => write!(f, ")"),
            LEFTBRACKET => write!(f, "["),
            RIGHTBRACKET => write!(f, "]"),
            DOUBLELEFTBRACKET => write!(f, "[["),
            DOUBLERIGHTBRACKET => write!(f, "]]"),
            LEFTBRACE => write!(f, "{{"),
            RIGHTBRACE => write!(f, "}}"),
            COMMA => write!(f, ","),
            SEMICOLON => write!(f, ";"),

            PLUS => write!(f, "+"),
            MINUS => write!(f, "-"),
            MUL => write!(f, "*"),
            DIV => write!(f, "/"),
            MOD => write!(f, "%"),
            POW => write!(f, "^"),
            FLOORDIV => write!(f, "//"),
            DOTDOT => write!(f, ".."),

            EQUAL => write!(f, "="),
            EQUALEQUAL => write!(f, "=="),
            NOTEQUAL => write!(f, "~="),
            GREATER => write!(f, ">"),
            LESS => write!(f, "<"),
            GREATEREQUAL => write!(f, ">="),
            LESSEQUAL => write!(f, "<="),

            NUMBER { value } => write!(f, "{}", *value),
            NAME { value } => write!(f, "{}", value),
            STRING { value } => write!(f, "\"{}\"", value),

            EOF => write!(f, "EOF"),
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        use TokenType::*;
        match (self, other) {
            (AND, AND)
            | (OR, OR)
            | (IF, IF)
            | (THEN, THEN)
            | (ELSE, ELSE)
            | (ELSEIF, ELSEIF)
            | (WHILE, WHILE)
            | (FOR, FOR)
            | (DO, DO)
            | (END, END)
            | (BREAK, BREAK)
            | (LOCAL, LOCAL)
            | (TRUE, TRUE)
            | (FALSE, FALSE)
            | (IN, IN)
            | (NOT, NOT)
            | (FUNCTION, FUNCTION)
            | (NIL, NIL)
            | (RETURN, RETURN)
            | (LEFTPAREN, LEFTPAREN)
            | (RIGHTPAREN, RIGHTPAREN)
            | (LEFTBRACKET, LEFTBRACKET)
            | (RIGHTBRACKET, RIGHTBRACKET)
            | (DOUBLELEFTBRACKET, DOUBLELEFTBRACKET)
            | (DOUBLERIGHTBRACKET, DOUBLERIGHTBRACKET)
            | (LEFTBRACE, LEFTBRACE)
            | (RIGHTBRACE, RIGHTBRACE)
            | (COMMA, COMMA)
            | (SEMICOLON, SEMICOLON)
            | (PLUS, PLUS)
            | (MINUS, MINUS)
            | (MUL, MUL)
            | (DIV, DIV)
            | (MOD, MOD)
            | (POW, POW)
            | (FLOORDIV, FLOORDIV)
            | (DOTDOT, DOTDOT)
            | (EQUAL, EQUAL)
            | (EQUALEQUAL, EQUALEQUAL)
            | (NOTEQUAL, NOTEQUAL)
            | (GREATER, GREATER)
            | (LESS, LESS)
            | (GREATEREQUAL, GREATEREQUAL)
            | (LESSEQUAL, LESSEQUAL) => true,

            (NUMBER { value: value1 }, NUMBER { value: value2 }) => value1 == value2,
            (NAME { value: value1 }, NAME { value: value2 }) => value1 == value2,
            (STRING { value: value1 }, STRING { value: value2 }) => value1 == value2,

            (EOF, EOF) => true,

            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenType;

    #[test]
    fn equal1() {
        let type1 = TokenType::AND;
        let type2 = TokenType::OR;
        let type3 = TokenType::AND;

        assert!(type1 != type2);
        assert!(type1 == type3)
    }
}
