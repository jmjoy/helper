// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#[cfg(test)]
use proc_macro2::TokenStream;
use syn::{
    fold::{self, Fold},
    parse::ParseStream,
    Block, Expr, ExprParen, Result, Stmt, TraitBound, Type,
};

pub(crate) fn parse_expr(input: ParseStream<'_>) -> Result<Expr> {
    input.parse().map(|expr| ParenthesizeGroups.fold_expr(expr))
}

pub(crate) fn parse_type(input: ParseStream<'_>) -> Result<Type> {
    input.parse().map(|ty| ParenthesizeGroups.fold_type(ty))
}

pub(crate) fn parse_trait_bound(input: ParseStream<'_>) -> Result<TraitBound> {
    input
        .parse()
        .map(|bound| ParenthesizeGroups.fold_trait_bound(bound))
}

pub(crate) fn parse_statements(input: ParseStream<'_>) -> Result<Vec<Stmt>> {
    Block::parse_within(input).map(|statements| {
        statements
            .into_iter()
            .map(|stmt| ParenthesizeGroups.fold_stmt(stmt))
            .collect()
    })
}

pub(crate) fn unparenthesized_expr(mut expr: &Expr) -> &Expr {
    loop {
        expr = match expr {
            Expr::Paren(paren) => &paren.expr,
            Expr::Group(group) => &group.expr,
            _ => return expr,
        };
    }
}

struct ParenthesizeGroups;

impl Fold for ParenthesizeGroups {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        match fold::fold_expr(self, expr) {
            // rustc can ignore invisible groups re-emitted by a proc macro
            // (rust-lang/rust#67062). Preserve their precedence with parentheses.
            Expr::Group(group) => Expr::Paren(ExprParen {
                attrs: group.attrs,
                paren_token: syn::token::Paren(group.group_token.span),
                expr: group.expr,
            }),
            expr => expr,
        }
    }
}

#[cfg(test)]
pub(crate) fn pm_test_left(f: impl FnOnce(TokenStream) -> TokenStream, s: &str) -> String {
    f(s.parse().unwrap()).to_string()
}

#[cfg(test)]
pub(crate) fn pm_test_right(s: &str) -> String {
    s.parse::<TokenStream>().unwrap().to_string()
}
