use chumsky::{primitive::just, Parser};

use crate::{
    kw,
    parser::expr::qualified_name::{qualified_name_parser, SpannedQualifiedName},
    AstParser,
};

#[must_use]
pub fn use_parser<'tokens, 'src: 'tokens>() -> AstParser!(SpannedQualifiedName) {
    just(kw!(use)).ignore_then(qualified_name_parser())
}
