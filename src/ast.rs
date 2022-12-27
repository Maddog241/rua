use std::fmt;

use crate::token::Token;

// // chunk
// pub struct Chunk {
//     pub block: Block,
// }

// impl fmt::Display for Chunk {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.block)
//     }
// }

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
    Assign {
        left: VarList,
        right: ExpList,
    },
    LocalAssign {
        left: NameList,
        right: Option<ExpList>,
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
        parlist: Option<NameList>,
        body: Block,
    },
    FunctionCall {
        func_call: Exp,
    },
    RetStmt {
        explist: Option<ExpList>,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { left, right } => {
                write!(f, "{} = {}\n", left, right)
            }

            Self::LocalAssign { left, right } => {
                match right {
                    Some(explist) => {
                        write!(f, "local {} = {}\n", left, explist)
                    } 
                    None => write!(f, "local {}", left)
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
                    match parlist {
                        Some(namelist) => {
                            write!(
                                f,
                                "\nFunctionDecl: local {}({}){{\n{}}}\n",
                                name, namelist, body
                            )
                        }
                        None => write!(f, "\nFunctionDecl: local {}(){{\n{}}}\n", name, body),
                    }
                } else {
                    match parlist {
                        Some(namelist) => {
                            write!(f, "\nFunctionDecl: {}({}){{\n{}}}\n", name, namelist, body)
                        }
                        None => write!(f, "\nFunctionDecl: {}(){{\n{}}}\n", name, body),
                    }
                }
            }

            Self::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => {
                write!(f, "if({}) {{\n{}}} ", condition, then_branch)?;
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
                write!(
                    f,
                    "GenericFor({} = {}) do {{\n{}}}\n",
                    namelist, explist, body
                )
            }

            Self::RetStmt { explist } => {
                match explist {
                    Some(list) => {
                        write!(f, "return {}\n", list)
                    },
                    None => write!(f, "return \n")
                }
            }
        }
    }
}

pub enum Var {
    Name {
        name: Name,
    },
    TableIndex {
        prefixexp: Box<Exp>,
        exp: Box<Exp>,
    }
}

pub struct VarList {
    pub vars: Vec<Var>,
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name { name } => write!(f, "{}", name),
            Self::TableIndex { prefixexp, exp } => write!(f, "{}[{}]", prefixexp, exp),
        }
    }
}

impl fmt::Display for VarList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "Varlist(")?;
        self.vars.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.vars.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
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
    Function {
        funcbody: FuncBody,
    },
    // prefix exp
    Var {
        var: Var,
    },
    FunctionCall {
        prefixexp: Box<Exp>,
        arguments: Option<ExpList>,
    },
    TableConstructor {
        fieldlist: Option<FieldList>,
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
            Self::Function { funcbody } => write!(f, "{}", funcbody),
            Self::Var { var } => write!(f, "{}", var),
            Self::FunctionCall { prefixexp, arguments } => {
                match arguments {
                    Some(args) => {
                        write!(f, "{}({})", prefixexp, args)
                    },
                    None => write!(f, "{}()", prefixexp)
                }
            }
            Self::TableConstructor { fieldlist } => match fieldlist {
                Some(fieldlist) => {
                    write!(f, "Table{{{}}}", fieldlist)
                }
                None => write!(f, "Table{{}}"),
            },
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
    pub parlist: Option<NameList>,
    pub block: Block,
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.parlist {
            Some(parlist) => write!(f, "function({}){{{}}}", parlist, self.block),
            None => write!(f, "function(){{{}}}", self.block),
        }
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
