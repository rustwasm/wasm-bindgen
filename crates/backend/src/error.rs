use proc_macro2::*;
use quote::{ToTokens, TokenStreamExt};

#[macro_export]
macro_rules! err_span {
    ($span:expr, $($msg:tt)*) => (
        $crate::Diagnostic::span_error(&$span, format!($($msg)*))
    )
}

#[macro_export]
macro_rules! bail_span {
    ($($t:tt)*) => (
        return Err(err_span!($($t)*).into())
    )
}

#[derive(Debug)]
pub struct Diagnostic {
    inner: Repr,
}

#[derive(Debug)]
enum Repr {
    Single {
        text: String,
        span: Option<(Span, Span)>,
    },
    Multi {
        diagnostics: Vec<Diagnostic>,
    }
}

impl Diagnostic {
    pub fn error<T: Into<String>>(text: T) -> Diagnostic {
        Diagnostic {
            inner: Repr::Single {
                text: text.into(),
                span: None,
            }
        }
    }

    pub fn span_error<T: Into<String>>(node: &ToTokens, text: T) -> Diagnostic {
        Diagnostic {
            inner: Repr::Single {
                text: text.into(),
                span: extract_spans(node),
            }
        }
    }

    pub fn from_vec(diagnostics: Vec<Diagnostic>) -> Result<(), Diagnostic> {
        if diagnostics.len() == 0 {
            Ok(())
        } else {
            Err(Diagnostic { inner: Repr::Multi { diagnostics }  })
        }
    }

    #[allow(unconditional_recursion)]
    pub fn panic(&self) -> ! {
        match &self.inner {
            Repr::Single { text, .. } => panic!("{}", text),
            Repr::Multi { diagnostics } => diagnostics[0].panic(),
        }
    }
}

fn extract_spans(node: &ToTokens) -> Option<(Span, Span)> {
    let mut t = TokenStream::new();
    node.to_tokens(&mut t);
    let mut tokens = t.into_iter();
    let start = tokens.next().map(|t| t.span());
    let end = tokens.last().map(|t| t.span());
    start.map(|start| (start, end.unwrap_or(start)))
}

impl ToTokens for Diagnostic {
    fn to_tokens(&self, dst: &mut TokenStream) {
        match &self.inner {
            Repr::Single { text, span } => {
                let cs2 = (Span::call_site(), Span::call_site());
                let (start, end) = span.unwrap_or(cs2);
                dst.append(Ident::new("compile_error", start));
                dst.append(Punct::new('!', Spacing::Alone));
                let mut message = TokenStream::new();
                message.append(Literal::string(text));
                let mut group = Group::new(Delimiter::Brace, message);
                group.set_span(end);
                dst.append(group);
            }
            Repr::Multi { diagnostics } => {
                for diagnostic in diagnostics {
                    diagnostic.to_tokens(dst);
                }
            }
        }
    }
}
