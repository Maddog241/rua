use std::fmt;

use crate::token::Token;

// chunk
pub struct Chunk {
    pub block: Block,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block)
    }
}

// block
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.statements
            .iter()
            .fold(Ok(()), |_result, stmt| write!(f, "{}", stmt))
    }
}

// statement
pub enum Stmt {
    Assignment {
        local: bool,
        left: NameList,
        right: ExpList,
    },
    Break,
    DoBlockEnd {
        block: Block,
    },
    WhileStmt {
        condition: Exp,
        body: Block,
    },
    IfStmt {
        condition: Exp,
        then_branch: Block,
        elseif_branches: Vec<(Exp, Block)>,
        option_else_branch: Option<Block>,
    },
    NumericFor {
        name: Name,
        start: Exp,
        end: Exp,
        step: Exp,
        body: Block,
    },
    GenericFor {
        namelist: NameList,
        explist: ExpList,
        body: Block,
    },
    FuncDecl {
        local: bool,
        name: Name,
        parlist: NameList,
        body: Block,
    },
    FunctionCall {
        func_call: Exp,
    },
    RetStmt {
        explist: ExpList,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assignment { local, left, right } => {
                if *local {
                    write!(f, "local {} = {}\n", left, right)
                } else {
                    write!(f, "{} = {}\n", left, right)
                }
            }

            Self::FunctionCall { func_call } => {
                write!(f, "{}\n", func_call)
            }

            Self::Break => {
                write!(f, "break\n")
            }

            Self::DoBlockEnd { block } => {
                write!(f, "{}", block)
            }

            Self::FuncDecl {
                local,
                name,
                parlist,
                body,
            } => {
                if *local {
                    write!(f, "\nFunctionDecl: local {}({}){{\n{}}}\n", name, parlist, body)
                } else {
                    write!(f, "\nFunctionDecl: {}({}){{\n{}}}\n", name, parlist, body)
                }
            }

            Self::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => {
                write!(f, "\nif({}) {{\n{}}} ", condition, then_branch)?;
                for (condition, elseif_branch) in elseif_branches.iter() {
                    write!(f, "elseif({}){{\n{}}}", condition, elseif_branch)?;
                }

                match option_else_branch {
                    Some(else_branch) => {
                        write!(f, "else{{\n{}}}\n", else_branch)
                    }
                    None => {
                        write!(f, "\n")
                    }
                }
            }

            Self::WhileStmt { condition, body } => {
                write!(f, "while({}) {{\n{}}}\n", condition, body)
            }

            Self::NumericFor {
                name,
                start,
                end,
                step,
                body,
            } => {
                write!(
                    f,
                    "NumericFor({}={},{},{}) do {{\n{}}}\n",
                    name, start, end, step, body
                )
            }

            Self::GenericFor {
                namelist,
                explist,
                body,
            } => {
                write!(f, "GenericFor({} = {}) do {{\n{}}}\n", namelist, explist, body)
            }

            Self::RetStmt { explist } => {
                write!(f, "return {}\n", explist)
            }
        }
    }
}


// name and namelist
pub struct Name(pub String);
pub struct NameList(pub Vec<Name>);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for NameList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "Namelist(")?;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}


// expression and explist
pub enum Exp {
    Literal {
        // nil, false, true, numeral, literal string
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Exp>,
    },
    Binary {
        left: Box<Exp>,
        operator: Token,
        right: Box<Exp>,
    },
    Grouping {
        expr: Box<Exp>,
    },
    FuncExp {
        funcbody: FuncBody,
    },
    FunctionCall {
        name: Name,
        arguments: ExpList,
    },
    TableConstructor {
        fieldlist: FieldList,
    },
}

pub struct ExpList(pub Vec<Exp>);

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal { value } => write!(f, "{}", value.tok_type),
            Self::Unary { operator, right } => write!(f, "({} {})", operator.tok_type, right),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.tok_type, right),
            Self::Grouping { expr } => write!(f, "({})", expr),
            Self::FuncExp { funcbody } => write!(f, "{}", funcbody),
            Self::FunctionCall { name, arguments } => write!(f, "FunctionCall: {}({})", name, arguments),
            Self::TableConstructor { fieldlist } => write!(f, "Table{{{}}}", fieldlist),
        }
    }
}

impl fmt::Display for ExpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "ExpList(")?;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}


// funcbody
pub struct FuncBody {
    pub parlist: NameList,
    pub block: Block,
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function({}){{{}}}", self.parlist, self.block)
    }
}

// field and fieldlist
pub struct Field {
    pub name: Option<Name>,
    pub exp: Exp,
}

pub struct FieldList {
    pub fields: Vec<Field>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => {
                write!(f, "{} = {}", name, self.exp)
            }
            None => {
                write!(f, "{}", self.exp)
            }
        }
    }
}

impl fmt::Display for FieldList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        self.fields.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.fields.len() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}

