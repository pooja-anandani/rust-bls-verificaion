// mod lib;
use blst::min_pk::{
    PublicKey as BlstPublicKey,Signature as BlstSignature,
};

use datadog_apm::{Client, Config};
fn main() {
    pub const DST: &[u8; 43] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    lib::run_no_messages_sent();
    let msg = "";


    let str_sign = "";
    let str_pk = "";

    let b_sign = hex::decode(str_sign.clone());
    let b_pk = hex::decode(str_pk.clone());
    let blst_sig = BlstSignature::from_bytes(&b_sign.unwrap()).unwrap();
    println!("{:?}", blst_sig);
    let blst_pk = BlstPublicKey::from_bytes(&b_pk.unwrap()).unwrap();
    println!("{:?}", blst_sig.verify(false, msg.as_ref(), DST, &[], &blst_pk, false))
}
