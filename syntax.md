### Lua Subset Grammar

```lua
block ::= {stmt} 
stmt ::= ';' |
        varlist '=' explist |
        functioncall |
        break| 
        do block end | 
        while exp do block end | 
        if exp then block {elseif exp then block} [else block] end | 
        for Name '=' exp ',' exp [',' exp] do block end | 
        for namelist in explist do block end | 
        ['local'] function Name funcbody | 
        local namelist ['=' explist]
        return [explist] [';']
varlist ::= var {',' var}
var  ::= Name | prefixexp '[' exp ']' 
namelist ::= Name {',' Name}
explist ::= exp {',' exp}
exp ::= nil | false | true | Number | String | functiondef | prefixexp
        | tableconstructor | exp binop exp | unop exp
prefixexp ::= var | functioncall | '(' exp ')'
functioncall ::= prefixexp args
args ::= '(' [explist] ')' | tableconstructor | String
functiondef ::= function funcbody
funcbody ::= '(' [namelist] ')' block end
tableconstructor ::= '{' [fieldlist] '}'
fieldlist ::= field {fieldsep field} [fieldsep]
field ::= Name '=' exp | exp | '[' exp ']'
fieldsep ::= ',' | ';'
binop ::= '+' | '-' | '*' | '/' | '//' | '^' | '%' | '..' | 
    '<' | '>' | '>=' | '<=' | '==' | '~=' | and | or
unop ::= '-' | not 
```

Eliminate left recursions and get the following production rules

### Productions

```lua
block -> stmt*
stmt -> ';' |
        varlist '=' explist | 
        functioncall |
        break |
        do block end |
        while exp do block end |
        if exp then block (else if exp then block)* (else block)? end |
        for Name '=' exp ',' exp (',' exp)? do block end |
        for namelist in explist do block end | 
        function Name funcbody |
        local function Name funcbody | 
        local namelist ('=' explist)?
        return (explist)? (';')?
exp     -> logic_or
logic_or -> logic_and ('or' logic_and)*
logic_and -> equality ('and' equality)*
comparison -> term ( ('>' | '<' | '<=' | '>=') term )*
term    -> factor ( ('-' | '+') factor)*
factor  -> unary ( ('/' | '*') unary )*
unary   -> ('!' | '-' ) unary | primary
primary -> functiondef | 
           tableconstructor | 
           prefixexp |
           literal

functiondef -> function funcbody 
tableconstructor -> '{' (fieldlist)? '}'
prefixexp -> Name (('[' exp ']') | args )* | 
            '(' Exp ')' (('[' exp ']') | args )*

literal -> nil |
           false | 
           true |
           Number | 
           String
```

### Utility Rules

```lua
functioncall -> prefixexp args
varlist -> var (',' var)*
var     -> Name |
           prefixexp '[' exp ']'
namelist -> Name (',' Name)*
explist -> exp (',' exp)*
funcbody   -> '(' (namelist)? ')' block end
fieldlist -> field (fieldsep field)* (fieldsep)?
field -> '[' exp ']' | Name '=' exp | exp 
fieldsep -> ',' | 
            ';'
args    -> '(' (explist)? ')' | 
            tableconstructor | 
            String
```