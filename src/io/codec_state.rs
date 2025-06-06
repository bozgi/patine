pub enum CodecState {
    Regular {
        buffer: Option<Vec<String>>,
    },
    Data,
}