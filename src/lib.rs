// mod utils;
// use datadog_logs::{
//     client::HttpDataDogClient,
//     config::DataDogConfig,
//     logger::{DataDogLogLevel, DataDogLogger},
// };
//
//
// pub async fn run_no_messages_sent() {
//     let (logger, receiver) = create_default_logger();
//
//     std::mem::drop(logger);
//
//     let messages: Vec<DataDogLog> = receiver.stream().collect().await;
//     assert_eq!(0, messages.len());
// }
//
// #[test]
// fn test_logger_stops_http() {
//     let config = DataDogConfig::default();
//     let client = HttpDataDogClient::new(&config).unwrap();
//     let logger = DataDogLogger::blocking::<HttpDataDogClient>(client, config);
//
//     logger.log("message", DataDogLogLevel::Alert);
//
//     // it should hang forever if logging loop does not break
//     std::mem::drop(logger);
// }
//
// #[tokio::test]
// pub async fn test_async_logger_stops_http() {
//     let config = DataDogConfig::default();
//     let client = HttpDataDogClient::new(&config).unwrap();
//     let (logger, future) = DataDogLogger::non_blocking_cold::<HttpDataDogClient>(client, config);
//
//     tokio::spawn(future);
//
//     logger.log("message", DataDogLogLevel::Alert);
//
//     // it should hang forever if logging loop does not break
//     std::mem::drop(logger);
// }
//
// pub fn create_default_logger() -> (DataDogLogger, Receiver<DataDogLog>) {
//     let (sender, receiver) = unbounded();
//     let logger = DataDogLogger::non_blocking_with_tokio(
//         utils::DataDogClientStub::new(sender),
//         DataDogConfig {
//             enable_self_log: true,
//             ..Default::default()
//         },
//     );
//     (logger, receiver)
// }