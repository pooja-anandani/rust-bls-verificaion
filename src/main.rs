// // mod lib;
// use blst::min_pk::{
//     PublicKey as BlstPublicKey,Signature as BlstSignature,
// };
//
// use datadog_apm::{Client, Config};
// fn main() {
//     pub const DST: &[u8; 43] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
//     lib::run_no_messages_sent();
//     let msg = "divyaraj";
//
//
//     let str_sign = "a74305066d74300ceb4845ab0b0267ceefc9875e0a32326aeadc6422682dd883535713e4a4f01302ac899968768f4f6a0d77cbaf24def2842d240fd1910f1e3ce274f8cacd8e4edefdf612cdca7ab815e75cacdbfeb5a686f0540802121ec84e";
//     let str_pk = "8cc32c16e8060f4c39b99a64e38be884dcb92009c3ddbb1e4b4d305e35674b58ba25b95b9264780babb1e66462f723ba";
//
//     let b_sign = hex::decode(str_sign.clone());
//     let b_pk = hex::decode(str_pk.clone());
//     let blst_sig = BlstSignature::from_bytes(&b_sign.unwrap()).unwrap();
//     println!("{:?}", blst_sig);
//     let blst_pk = BlstPublicKey::from_bytes(&b_pk.unwrap()).unwrap();
//     println!("{:?}", blst_sig.verify(false, msg.as_ref(), DST, &[], &blst_pk, false))
// }
use datadog_apm::{Client, Config};
use datadog_apm::{ErrorInfo, HttpInfo, Span, SqlInfo, Trace};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[tokio::main]
async fn main() {
    let client = Client::new(Config {
        env: Some("production".to_string()),
        service: "watchdog".to_string(),
        ..Default::default()
    });

    let trace = Trace {
        id: 123,
        priority: 1,
        spans: vec![
            Span {
                id: 1,
                parent_id: None,
                name: "request".to_string(),
                resource: "GET /path".to_string(),
                r#type: "web".to_string(),
                start: SystemTime::now(),
                duration: Duration::from_millis(50),
                http: Some(HttpInfo {
                    url: String::from("/path/2?param=true"),
                    method: String::from("GET"),
                    status_code: String::from("500"),
                }),
                error: Some(ErrorInfo {
                    r#type: "unknown".to_string(),
                    msg: "Internal error".to_string(),
                    stack: "stack here".to_string(),
                }),
                sql: None,
                tags: HashMap::new(),
            },
            Span {
                id: 2,
                parent_id: Some(1),
                name: "database".to_string(),
                resource: "select".to_string(),
                r#type: "db".to_string(),
                start: SystemTime::now(),
                duration: Duration::from_millis(20),
                http: None,
                error: None,
                sql: Some(SqlInfo {
                    query: "select 1".to_string(),
                    rows: "1".to_string(),
                    db: "test".to_string(),
                }),
                tags: HashMap::new(),
            },
        ],
    };

    client.send_trace(trace);

    // wait for buffer flush
//     tokio::time::interval(Duration::new(0,2)).await;
    println!("trace sent");
}