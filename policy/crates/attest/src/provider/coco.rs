use tdx_attestation_sdk::{device::DeviceOptions, Tdx};

use crate::{errors::AttestationError, types::RawReport};

pub fn get_raw_quote(report: RawReport) -> Result<Vec<u8>, AttestationError> {
    let tdx = Tdx::new();

    let (quote, _azure_only_extra_data) =
        tdx.get_attestation_report_raw_with_options(DeviceOptions {
            report_data: Some(report.to_bytes()),
        })?;

    Ok(quote)
}
