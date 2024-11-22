use logos::{Lexer, Logos};

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n]+")]
pub enum Token<'src> {
    #[token("func")]
    Func,
    #[token("type")]
    Type,
    #[token("file")]
    File,
    #[token("copy")]
    Copy,
    #[token("push")]
    Push,
    #[token("pop")]
    Pop,
    #[token("print")]
    Print,
    #[token("panic")]
    Panic,
    #[token("construct")]
    Construct,
    #[token("call")]
    Call,
    #[token("return")]
    Return,
    #[token("new")]
    NewLocal,
    #[token("get")]
    GetLocal,
    #[token("set")]
    SetLocal,
    #[token("goto")]
    Goto,
    #[token("param")]
    Param,
    #[token("mul")]
    Mul,
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("eq")]
    Eq,
    #[token("neq")]
    Neq,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("match")]
    Match,
    #[token("jmp")]
    Jmp,
    #[token("je")]
    Je,
    #[token("jne")]
    Jne,
    #[token("index")]
    Index,
    #[token("set_index")]
    SetIndex,
    #[token("gt")]
    Gt,
    #[token("lt")]
    Lt,
    #[token("gte")]
    Gte,
    #[token("lte")]
    Lte,
    #[token("clone")]
    Clone,
    #[token("vec_get")]
    VecGet,
    #[token("vec_set")]
    VecSet,
    #[token("vec_push")]
    VecPush,
    #[token("vec_pop")]
    VecPop,
    #[token("vec_len")]
    VecLen,
    #[token("vec_insert")]
    VecInsert,
    #[token("vec_remove")]
    VecRemove,
    #[token("dyn")]
    Dyn,
    #[token("dyn_call")]
    DynCall,
    #[token("vec_peak")]
    VecPeak,
    #[token("mark")]
    Mark,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("\\d+\\.\\d+")]
    Float(&'src str),
    #[regex("\\d+")]
    Int(&'src str),
    #[regex("\"[^\"]*\"", |lex| strip_quotes(lex))]
    String(&'src str),
    #[regex("'.'", |lex| let mut iter = lex.slice().chars(); iter.next(); iter.next())]
    Char(char),
}

impl<'src> Token<'src> {
    pub fn is_decl(&self) -> bool {
        matches!(self, Token::Func | Token::Type | Token::File)
    }
}

fn strip_quotes<'s>(lex: &mut Lexer<'s, Token<'s>>) -> &'s str {
    let slice = lex.slice();
    &slice[1..slice.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use logos::Logos as _;

    #[test]
    fn test_lex_keywords() {
        let text = r#"func type file copy push pop print"#;
        let mut lex = super::Token::lexer(text);
        assert_eq!(lex.next(), Some(Ok(Func)));
        assert_eq!(lex.next(), Some(Ok(Type)));
        assert_eq!(lex.next(), Some(Ok(File)));
        assert_eq!(lex.next(), Some(Ok(Copy)));
        assert_eq!(lex.next(), Some(Ok(Push)));
        assert_eq!(lex.next(), Some(Ok(Pop)));
        assert_eq!(lex.next(), Some(Ok(Print)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_lex_string() {
        let text = r#" "Hello" "World" "#;
        let mut lex = super::Token::lexer(text);
        assert_eq!(lex.next(), Some(Ok(String("Hello"))));
        assert_eq!(lex.next(), Some(Ok(String("World"))));
        assert_eq!(lex.next(), None);
    }
}
