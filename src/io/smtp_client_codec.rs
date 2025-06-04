use tokio_util::codec::Decoder;
use crate::io::codec_state::CodecState;

pub struct SmtpClientCodec {
    state: CodecState
}

// impl Decoder for SmtpClientCodec {
//
// }