use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, ItemFn, ReturnType, Visibility};

/// SyterKit async runtime function entry.
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "#[entry] attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 0 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function should include no parameters",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_some()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Default);

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[entry]` function must have signature `[unsafe] async fn()`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    // TODO origin metadata of function, like comments

    quote!(
        #[export_name = "main"]
        pub extern "C" fn main() -> ! {
            #[allow(non_snake_case)]
            #[inline(always)]
            #(#attrs)*
            #unsafety async fn __syterkit__main() {
                #(#stmts)*
            }
            use core::future::Future;
            let mut fut = core::pin::pin!(unsafe { __syterkit__main() });
            let mut cx = core::task::Context::from_waker(core::task::Waker::noop());
            loop {
                match fut.as_mut().poll(&mut cx) {
                    core::task::Poll::Ready(_) => break,
                    core::task::Poll::Pending => {
                        unsafe { core::arch::asm!("wfi") }
                    },
                }
            }
            loop {} // TODO perform 'shutdown' current environment
        }
    )
    .into()
}

// todo: #[test] macro. Ref:
// https://github.com/compio-rs/compio/blob/d75b76e7ffe0359378ea2b42a8c6c7d6f3ff15a0/compio-macros/src/lib.rs#L22
