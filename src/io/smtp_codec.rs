use std::marker::PhantomData;
use crate::io::codec_state::CodecState;

pub struct SmtpCodec<I, O> {
    pub(crate) codec_state: CodecState,
    marker: PhantomData<(I, O)>,
}

impl<I, O> SmtpCodec<I, O> {
    pub(crate) fn new() -> Self {
        Self {
            codec_state: CodecState::Regular,
            marker: PhantomData,
        }
    }
}

