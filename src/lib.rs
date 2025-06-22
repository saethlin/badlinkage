#![allow(non_camel_case_types)]

use aws_lc_rs::hmac::Algorithm as HmacAlgorithm;

pub fn init() -> Vec<OpDecl> {
    vec![generate::DECL, sign::DECL]
}

pub struct OpDecl {
    #[allow(dead_code)]
    slow_fn: fn(),
}

pub struct sign;
impl sign {
    const DECL: OpDecl = OpDecl {
        slow_fn: Self::v8_fn_ptr as fn(),
    };
    fn v8_fn_ptr<'s>() {
        Self::call(std::hint::black_box(None));
    }
    #[inline(never)]
    pub fn call(args: Option<CryptoHash>) {
        let hash: HmacAlgorithm = args.unwrap().into();
        std::hint::black_box(hash);
    }
}

pub struct generate;
impl generate {
    const DECL: OpDecl = OpDecl {
        slow_fn: Self::v8_fn_ptr as fn(),
    };
    fn v8_fn_ptr() {
        Self::call(std::hint::black_box(CryptoHash::Sha1))
    }
    #[inline(never)]
    pub fn call(hash: CryptoHash) {
        let hash = match hash {
            CryptoHash::Sha1 => &aws_lc_rs::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            CryptoHash::Sha256 => &aws_lc_rs::hmac::HMAC_SHA256,
            CryptoHash::Sha384 => &aws_lc_rs::hmac::HMAC_SHA384,
            CryptoHash::Sha512 => &aws_lc_rs::hmac::HMAC_SHA512,
        };
        let length = hash.digest_algorithm().block_len();
        std::hint::black_box(length);
    }
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
