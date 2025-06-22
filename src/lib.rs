#![allow(non_camel_case_types)]

use aws_lc_rs::hmac::Algorithm as HmacAlgorithm;

pub fn init() -> Vec<deno_core::OpDecl> {
    vec![op_crypto_generate_key(), op_crypto_sign_key()]
}

pub const fn op_crypto_sign_key() -> ::deno_core::_ops::OpDecl {
    pub struct op_crypto_sign_key {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_crypto_sign_key {
        const NAME: &'static str = "op_crypto_sign_key";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_crypto_sign_key".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                ("op_crypto_sign_key", ::deno_core::FastStaticString::new(s))
            },
            true,
            false,
            false,
            2usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            ::deno_core::AccessorType::None,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_crypto_sign_key {
        fn slow_function_impl<'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let result = {
                let arg0 = args.get(1usize as i32);
                let arg0 = match deno_core::_ops::serde_v8_to_rust(&mut scope, arg0) {
                    Ok(t) => t,
                    Err(arg0_err) => {
                        deno_core::_ops::throw_error_js_error_class(&mut scope, &arg0_err);
                        return 1;
                    }
                };
                Self::call(arg0)
            };
            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| {
                    deno_core::_ops::RustToV8Fallible::to_v8_fallible(
                        deno_core::_ops::RustToV8Marker::<deno_core::_ops::SerdeMarker, _>::from(
                            result,
                        ),
                        scope,
                    )
                },
            ) {
                match result {
                    Ok(result) => {
                        match deno_core::_ops::RustToV8Fallible::to_v8_fallible(
                            deno_core::_ops::RustToV8Marker::<
                                deno_core::_ops::SerdeMarker,
                                _,
                            >::from(result),
                            &mut scope,
                        ) {
                            Ok(v) => rv.set(v),
                            Err(rv_err) => {
                                deno_core::_ops::throw_error_js_error_class(
                                    &mut scope,
                                    &rv_err,
                                );
                                return 1;
                            }
                        }
                    }
                    Err(err) => {
                        let exception = deno_core::error::to_v8_error(&mut scope, &err);
                        scope.throw_exception(exception);
                        return 1;
                    }
                };
                return 0;
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(_info: *const deno_core::v8::FunctionCallbackInfo) {}
    }
    impl op_crypto_sign_key {
        #[inline(never)]
        pub async fn call(args: Option<CryptoHash>) -> Result<(), core::convert::Infallible> {
            let hash: HmacAlgorithm = args.unwrap().into();
            std::hint::black_box(hash);
            Ok(())
        }
    }
    <op_crypto_sign_key as ::deno_core::_ops::Op>::DECL
}
pub const fn op_crypto_generate_key() -> ::deno_core::_ops::OpDecl {
    pub struct op_crypto_generate_key {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_crypto_generate_key {
        const NAME: &'static str = "op_crypto_generate_key";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_crypto_generate_key".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                (
                    "op_crypto_generate_key",
                    ::deno_core::FastStaticString::new(s),
                )
            },
            true,
            false,
            false,
            2usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            ::deno_core::AccessorType::None,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_crypto_generate_key {
        fn slow_function_impl<'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let arg0 = args.get(1usize as i32);
            let arg0 = deno_core::_ops::serde_v8_to_rust(&mut scope, arg0).unwrap();
            let result = Self::call(arg0);

            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| {
                    deno_core::_ops::RustToV8Fallible::to_v8_fallible(
                        deno_core::_ops::RustToV8Marker::<deno_core::_ops::SerdeMarker, _>::from(
                            result,
                        ),
                        scope,
                    )
                },
            ) {
                let result = result.unwrap();
                let v = deno_core::_ops::RustToV8Fallible::to_v8_fallible(
                    deno_core::_ops::RustToV8Marker::<deno_core::_ops::SerdeMarker, _>::from(
                        result,
                    ),
                    &mut scope,
                )
                .unwrap();
                rv.set(v);
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(_info: *const deno_core::v8::FunctionCallbackInfo) {}
    }
    impl op_crypto_generate_key {
        #[inline(never)]
        pub async fn call(hash: ShaHash) -> Result<(), core::convert::Infallible> {
            let hash = match hash {
                ShaHash::Sha1 => &aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
                ShaHash::Sha256 => &aws_lc_rs::hmac::HMAC_SHA256,
                ShaHash::Sha384 => &aws_lc_rs::hmac::HMAC_SHA384,
                ShaHash::Sha512 => &aws_lc_rs::hmac::HMAC_SHA512,
            };
            let length = hash.digest_algorithm().block_len();
            std::hint::black_box(length);
            Ok(())
        }
    }
    <op_crypto_generate_key as ::deno_core::_ops::Op>::DECL
}
use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize)]
pub enum ShaHash {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}
#[derive(Serialize, Deserialize)]
pub enum CryptoHash {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}
impl From<CryptoHash> for HmacAlgorithm {
    fn from(hash: CryptoHash) -> HmacAlgorithm {
        match hash {
            CryptoHash::Sha1 => aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            CryptoHash::Sha256 => aws_lc_rs::hmac::HMAC_SHA256,
            CryptoHash::Sha384 => aws_lc_rs::hmac::HMAC_SHA384,
            CryptoHash::Sha512 => aws_lc_rs::hmac::HMAC_SHA512,
        }
    }
}
