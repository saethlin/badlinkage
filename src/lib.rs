#![allow(non_camel_case_types)]

use aws_lc_rs::hmac::Algorithm as HmacAlgorithm;
use deno_core::v8::FunctionCallbackInfo;
use std::mem::transmute;

pub fn init() -> Vec<deno_core::OpDecl> {
    vec![op_crypto_generate_key(), op_crypto_sign_key()]
}

struct OpDecl {
    slow_fn: fn(),
}

pub fn op_crypto_sign_key() -> ::deno_core::_ops::OpDecl {
    pub struct op_crypto_sign_key;
    impl op_crypto_sign_key {
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
            unsafe { transmute(Self::v8_fn_ptr as fn(*const FunctionCallbackInfo)) },
            unsafe { transmute(Self::v8_fn_ptr_metrics as fn(*const FunctionCallbackInfo)) },
            ::deno_core::AccessorType::None,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_crypto_sign_key {
        fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            Self::call(std::hint::black_box(None));
        }
        fn v8_fn_ptr_metrics<'s>(_info: *const deno_core::v8::FunctionCallbackInfo) {}
    }
    impl op_crypto_sign_key {
        #[inline(never)]
        pub fn call(args: Option<CryptoHash>) {
            let hash: HmacAlgorithm = args.unwrap().into();
            std::hint::black_box(hash);
        }
    }
    op_crypto_sign_key::DECL
}
pub fn op_crypto_generate_key() -> ::deno_core::_ops::OpDecl {
    pub struct op_crypto_generate_key;
    impl op_crypto_generate_key {
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
            unsafe { transmute(Self::v8_fn_ptr as fn(*const FunctionCallbackInfo)) },
            unsafe { transmute(Self::v8_fn_ptr_metrics as fn(*const FunctionCallbackInfo)) },
            ::deno_core::AccessorType::None,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_crypto_generate_key {
        fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            Self::call(std::hint::black_box(ShaHash::Sha1))
        }
        fn v8_fn_ptr_metrics<'s>(_info: *const deno_core::v8::FunctionCallbackInfo) {}
    }
    impl op_crypto_generate_key {
        #[inline(never)]
        pub fn call(hash: ShaHash) {
            let hash = match hash {
                ShaHash::Sha1 => &aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
                ShaHash::Sha256 => &aws_lc_rs::hmac::HMAC_SHA256,
                ShaHash::Sha384 => &aws_lc_rs::hmac::HMAC_SHA384,
                ShaHash::Sha512 => &aws_lc_rs::hmac::HMAC_SHA512,
            };
            let length = hash.digest_algorithm().block_len();
            std::hint::black_box(length);
        }
    }
    op_crypto_generate_key::DECL
}
pub enum ShaHash {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}
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
