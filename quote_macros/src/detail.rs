extern crate proc_macro;

use proc_macro::{
    token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};
use std::iter::{FromIterator, IntoIterator, Peekable};

fn tt_ident(span: Span, s: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(s, span))
}

fn tt_punct(span: Span, c: char) -> TokenTree {
    let mut p = Punct::new(c, Spacing::Alone);
    p.set_span(span);
    TokenTree::Punct(p)
}

fn tt_group(span: Span, delim: Delimiter, tts: impl IntoIterator<Item = TokenTree>) -> TokenTree {
    let mut g = Group::new(delim, TokenStream::from_iter(tts));
    g.set_span(span);
    TokenTree::Group(g)
}

fn helper(span: Span, args: impl IntoIterator<Item = TokenTree>) -> TokenStream {
    TokenStream::from_iter(vec![
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        tt_ident(span, "quote"),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        tt_ident(span, "quote_proc_macro_rt"),
        tt_punct(span, '!'),
        tt_group(span, Delimiter::Parenthesis, args),
    ])
}


// Call the `quote_proc_macro_rt` helper.
fn helper_stmt(span: Span, args: impl IntoIterator<Item = TokenTree>) -> TokenStream {
    let mut ts = helper(span, args);
    ts.extend(Some(tt_punct(span, ';')));
    ts
}

type TokenIter = Peekable<token_stream::IntoIter>;

fn quote_stream(tokvar: &Ident, spvar: &Ident, mut iterator: TokenIter) -> TokenStream {
    let mut stream = TokenStream::new();
    loop {
        let tt = match iterator.next() {
            Some(tt) => tt,
            None => break,
        };

        match tt {
            TokenTree::Literal(literal) => {
                stream.extend(helper_stmt(
                    literal.span(),
                    vec![
                        tt_ident(literal.span(), "parse"),
                        tokvar.clone().into(),
                        spvar.clone().into(),
                        TokenTree::Literal(Literal::string(&literal.to_string())),
                    ],
                ));
            }

            // It's possible to directly create a punctuation token without
            // parsing.
            TokenTree::Punct(punct) => {
                stream.extend(helper_stmt(
                    punct.span(),
                    vec![
                        tt_ident(punct.span(), "Punct"),
                        tokvar.clone().into(),
                        spvar.clone().into(),
                        tt_ident(
                            punct.span(),
                            match punct.spacing() {
                                Spacing::Alone => "Alone",
                                Spacing::Joint => "Joint",
                            },
                        ),
                        TokenTree::Literal(Literal::character(punct.as_char())),
                    ],
                ));
            }

            TokenTree::Ident(ident) => {
                // If we have a raw string, we cannot create it directly, as
                // new_raw isn't stable yet. We instead have to just ParseUnwrap
                // directly.
                let idstr = ident.to_string();
                let lit = TokenTree::Literal(Literal::string(&idstr));
                if idstr.starts_with("r#") {
                    stream.extend(helper_stmt(
                        ident.span(),
                        vec![
                            tt_ident(ident.span(), "parse"),
                            tokvar.clone().into(),
                            spvar.clone().into(),
                            lit,
                        ],
                    ));
                } else {
                    stream.extend(helper_stmt(
                        ident.span(),
                        vec![
                            tt_ident(ident.span(), "Ident"),
                            tokvar.clone().into(),
                            spvar.clone().into(),
                            lit,
                        ],
                    ));
                }
            }

            TokenTree::Group(group) => {
                stream.extend(helper_stmt(
                    group.span(),
                    vec![
                        tt_ident(group.span(), "Group"),
                        tokvar.clone().into(),
                        spvar.clone().into(),
                        tt_ident(
                            group.span(),
                            match group.delimiter() {
                                Delimiter::Parenthesis => "Parenthesis",
                                Delimiter::Brace => "Brace",
                                Delimiter::Bracket => "Bracket",
                                Delimiter::None => "None",
                            },
                        ),
                    ]
                    .into_iter()
                    .chain(group.stream().into_iter()),
                ));
            }
        }
    }
    stream
}

pub fn quote_one_token_func(item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let tokvar = Ident::new("tokens", span);
    let spvar = Ident::new("span", span);

    let mut iterator = item.into_iter().peekable();
    match iterator.next() {
        Some(TokenTree::Ident(name)) => TokenStream::from_iter(vec![
            tt_ident(span, "pub"),
            tt_ident(span, "fn"),
            name.into(),
            tt_group(
                span,
                Delimiter::Parenthesis,
                vec![
                    tokvar.clone().into(),
                    tt_punct(span, ':'),
                    tt_group(
                        span,
                        Delimiter::None,
                        helper(span, vec![tt_ident(span, "TokenStream")]),
                    ),
                    tt_punct(span, ','),
                    spvar.clone().into(),
                    tt_punct(span, ':'),
                    tt_group(
                        span,
                        Delimiter::None,
                        helper(span, vec![tt_ident(span, "Span")]),
                    ),
                ],
            ),
            tt_group(
                span,
                Delimiter::Brace,
                quote_stream(&tokvar, &spvar, iterator),
            ),
        ]),

        _ => panic!("Unexpected macro input"),
    }
}
