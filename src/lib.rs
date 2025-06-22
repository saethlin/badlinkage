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

pub fn op_crypto_sign_key() -> OpDecl {
    pub struct op_crypto_sign_key;
    impl op_crypto_sign_key {
        const DECL: OpDecl = OpDecl {
            unsafe { transmute(Self::v8_fn_ptr as fn(*const FunctionCallbackInfo)) },
        }
        fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            Self::call(std::hint::black_box(None));
        }
        #[inline(never)]
        pub fn call(args: Option<CryptoHash>) {
            let hash: HmacAlgorithm = args.unwrap().into();
            std::hint::black_box(hash);
        }
    }
    op_crypto_sign_key::DECL
}
pub fn op_crypto_generate_key() -> OpDecl {
    pub struct op_crypto_generate_key;
    impl op_crypto_generate_key {
        const DECL: OpDecl = OpDecl {
            slow_fn: unsafe { transmute(Self::v8_fn_ptr as fn(*const FunctionCallbackInfo)) },
        };
        fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            Self::call(std::hint::black_box(ShaHash::Sha1))
        }
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
