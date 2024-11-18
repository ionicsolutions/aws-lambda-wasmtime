#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod function {
        #[allow(dead_code, clippy::all)]
        pub mod lambda {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
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
                    f.debug_struct("Response").field("factors", &self.factors).finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn handler(event: Event) -> Response {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let Event { number: number0 } = event;
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:function/lambda")]
                    extern "C" {
                        #[link_name = "handler"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(number0), ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    Response {
                        factors: _rt::Vec::from_raw_parts(l2.cast(), len4, len4),
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    extern crate alloc as alloc_crate;
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:runtime:app:app:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 256] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x86\x01\x01A\x02\x01\
A\x02\x01B\x07\x01r\x01\x06numberx\x04\0\x05event\x03\0\0\x01px\x01r\x01\x07fact\
ors\x02\x04\0\x08response\x03\0\x03\x01@\x01\x05event\x01\0\x04\x04\0\x07handler\
\x01\x05\x03\0\x19component:function/lambda\x05\0\x04\0\x0fruntime:app/app\x04\0\
\x0b\x09\x01\0\x03app\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-com\
ponent\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
