use core::mem;

use convert::{FromWasmAbi, IntoWasmAbi, GlobalStack, Stack, ReturnWasmAbi};
use throw_str;

macro_rules! stack_closures {
    ($( ($($var:ident)*) )*) => ($(
        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a (Fn($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as ReturnWasmAbi>::Abi {
                    if a == 0 {
                        throw_str("closure invoked recursively or destroyed already");
                    }
                    // Scope all local variables before we call `return_abi` to
                    // ensure they're all destroyed as `return_abi` may throw
                    let ret = {
                        let f: &Fn($($var),*) -> R = mem::transmute((a, b));
                        let mut _stack = GlobalStack::new();
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                        )*
                        f($($var),*)
                    };
                    ret.return_abi(&mut GlobalStack::new())
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }

        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a mut (FnMut($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: ReturnWasmAbi
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as ReturnWasmAbi>::Abi {
                    if a == 0 {
                        throw_str("closure invoked recursively or destroyed already");
                    }
                    // Scope all local variables before we call `return_abi` to
                    // ensure they're all destroyed as `return_abi` may throw
                    let ret = {
                        let f: &mut FnMut($($var),*) -> R = mem::transmute((a, b));
                        let mut _stack = GlobalStack::new();
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                        )*
                        f($($var),*)
                    };
                    ret.return_abi(&mut GlobalStack::new())
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }
    )*)
}

stack_closures! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
}

