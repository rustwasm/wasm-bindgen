use core::mem;

use convert::{FromWasmAbi, IntoWasmAbi, GlobalStack, Stack};
use throw;

macro_rules! stack_closures {
    ($( ($($var:ident)*) )*) => ($(
        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a (Fn($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: IntoWasmAbi
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* R: IntoWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as IntoWasmAbi>::Abi {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &Fn($($var),*) -> R = mem::transmute((a, b));
                    let mut _stack = GlobalStack::new();
                    $(
                        let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                    )*
                    f($($var),*).into_abi(&mut GlobalStack::new())
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }

        impl<'a, 'b, $($var,)*> IntoWasmAbi for &'a (Fn($($var),*) + 'b)
            where $($var: FromWasmAbi,)*
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* >(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &Fn($($var),*) = mem::transmute((a, b));
                    let mut _stack = GlobalStack::new();
                    $(
                        let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                    )*
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)*> as u32
                }
            }
        }

        impl<'a, 'b, $($var,)* R> IntoWasmAbi for &'a mut (FnMut($($var),*) -> R + 'b)
            where $($var: FromWasmAbi,)*
                  R: IntoWasmAbi
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* R: IntoWasmAbi>(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) -> <R as IntoWasmAbi>::Abi {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &mut FnMut($($var),*) -> R = mem::transmute((a, b));
                    let mut _stack = GlobalStack::new();
                    $(
                        let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                    )*
                    f($($var),*).into_abi(&mut GlobalStack::new())
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }

        impl<'a, 'b, $($var,)*> IntoWasmAbi for &'a mut (FnMut($($var),*) + 'b)
            where $($var: FromWasmAbi,)*
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var: FromWasmAbi,)* >(
                    a: usize,
                    b: usize,
                    $($var: <$var as FromWasmAbi>::Abi),*
                ) {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &mut FnMut($($var),*) = mem::transmute((a, b));
                    let mut _stack = GlobalStack::new();
                    $(
                        let $var = <$var as FromWasmAbi>::from_abi($var, &mut _stack);
                    )*
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)*> as u32
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

