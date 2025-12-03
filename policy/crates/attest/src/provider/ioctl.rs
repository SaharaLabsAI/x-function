use crate::{errors::AttestationError, types::RawReport};

pub fn get_raw_quote(report: RawReport) -> Result<Vec<u8>, AttestationError> {
    panic!("ioctl is deprecated, please upgrade to new kernel")
}
