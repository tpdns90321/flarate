#[derive(Debug)]
pub enum ChannelError {
    InvalidChannel,
    UnsupportedChannel,
}

#[derive(Debug)]
pub enum MelodyError {
    OddLength,
    NotNumber,
}

#[derive(Debug)]
pub enum ChartError {
    EmptyChart,
}
