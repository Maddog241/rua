use std::fmt;

pub struct Token {
    tok_type: TokenType,
    line: usize,
}

impl Token {
    pub fn new(line: usize, tok_type: TokenType) -> Self {
        Self {
            line,
            tok_type,
        }
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

    // arith
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
    DIVNOREMAIN,
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

    LINEFEED,

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

            PLUS => write!(f, "+"),
            MINUS => write!(f, "-"),
            MUL => write!(f, "*"),
            DIV => write!(f, "/"),
            MOD => write!(f, "%"),
            POW => write!(f, "^"),
            DIVNOREMAIN => write!(f, "//"),
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

            LINEFEED => write!(f, "LINEFEED"),
            EOF => write!(f, "EOF"),
        }
    }
}
