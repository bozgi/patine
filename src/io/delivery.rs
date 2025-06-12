// use std::sync::mpsc::Receiver;
//
// struct DeliveryJob {
//     pub from: String,
//     pub to: Vec<String>,
//     pub data: Vec<u8>,
// }
//
// pub async fn delivery_worker(mut rx: Receiver<DeliveryJob>)
// {
//     while let Ok(job) = rx.recv() {
//
//     }
// }

// fn try_delivery(delivery_job: DeliveryJob) -> Result<(), Error> {
//
// }