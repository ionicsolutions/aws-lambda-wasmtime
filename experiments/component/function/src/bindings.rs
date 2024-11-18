#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod function {
            #[allow(dead_code, clippy::all)]
            pub mod lambda {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(C)]
                #[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
                pub struct Event {
                    pub number: i64,
                }
                impl ::core::fmt::Debug for Event {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Event").field("number", &self.number).finish()
                    }
                }
                #[derive(Clone, serde::Deserialize, serde::Serialize)]
                pub struct Response {
                    pub factors: _rt::Vec<i64>,
                }
                impl ::core::fmt::Debug for Response {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Response")
                            .field("factors", &self.factors)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_handler_cabi<T: Guest>(arg0: i64) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::handler(Event { number: arg0 });
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Response { factors: factors2 } = result0;
                    let vec3 = (factors2).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_handler<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 8, 8);
                }
                pub trait Guest {
                    fn handler(event: Event) -> Response;
                }
                #[doc(hidden)]
                macro_rules! __export_component_function_lambda_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:function/lambda#handler"] unsafe extern "C" fn
                        export_handler(arg0 : i64,) -> * mut u8 { $($path_to_types)*::
                        _export_handler_cabi::<$ty > (arg0) } #[export_name =
                        "cabi_post_component:function/lambda#handler"] unsafe extern "C"
                        fn _post_return_handler(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_handler::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_function_lambda_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::function::lambda::__export_component_function_lambda_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::function::lambda);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:component:function:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 271] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x91\x01\x01A\x02\x01\
A\x02\x01B\x07\x01r\x01\x06numberx\x04\0\x05event\x03\0\0\x01px\x01r\x01\x07fact\
ors\x02\x04\0\x08response\x03\0\x03\x01@\x01\x05event\x01\0\x04\x04\0\x07handler\
\x01\x05\x04\0\x19component:function/lambda\x05\0\x04\0\x1acomponent:function/ex\
ample\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\
\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
