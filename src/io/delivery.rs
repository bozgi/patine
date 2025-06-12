use crate::io::smtp_codec::SmtpCodec;
use std::sync::mpsc::Receiver;
use tokio_util::codec::Framed;

struct DeliveryJob {
    pub from: String,
    pub to: Vec<String>,
    pub data: Vec<u8>,
}

pub async fn delivery_worker(mut rx: Receiver<DeliveryJob>)
{
    while let Ok(job) = rx.recv() {

    }
}

// fn try_delivery(delivery_job: DeliveryJob) -> Result<(), Error> {
//
// }