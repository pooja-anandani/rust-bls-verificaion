// mod lib;
use blst::min_pk::{
    PublicKey as BlstPublicKey,Signature as BlstSignature,
};

use datadog_apm::{Client, Config};
fn main() {
    pub const DST: &[u8; 43] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    lib::run_no_messages_sent();
    let msg = "divyaraj";


    let str_sign = "a74305066d74300ceb4845ab0b0267ceefc9875e0a32326aeadc6422682dd883535713e4a4f01302ac899968768f4f6a0d77cbaf24def2842d240fd1910f1e3ce274f8cacd8e4edefdf612cdca7ab815e75cacdbfeb5a686f0540802121ec84e";
    let str_pk = "8cc32c16e8060f4c39b99a64e38be884dcb92009c3ddbb1e4b4d305e35674b58ba25b95b9264780babb1e66462f723ba";

    let b_sign = hex::decode(str_sign.clone());
    let b_pk = hex::decode(str_pk.clone());
    let blst_sig = BlstSignature::from_bytes(&b_sign.unwrap()).unwrap();
    println!("{:?}", blst_sig);
    let blst_pk = BlstPublicKey::from_bytes(&b_pk.unwrap()).unwrap();
    println!("{:?}", blst_sig.verify(false, msg.as_ref(), DST, &[], &blst_pk, false))
}
