#[derive(Debug, thiserror::Error)]
pub enum QuoteError {
    #[error("invalid header size {0}")]
    InvalidHeaderSize(usize),

    #[error("unknown version {0}")]
    UnknownQuote(u16),

    #[error("report data {0}")]
    ReportData(String),
}

#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    #[error("ioctl {0}")]
    Ioctl(String),

    #[error("coco {0}")]
    Coco(#[from] tdx_attestation_sdk::error::TdxError),

    #[error("quote {0}")]
    Quote(#[from] QuoteError),

    #[error("no provider available, should run inside guest vm")]
    NoProviderAvailable,
}
