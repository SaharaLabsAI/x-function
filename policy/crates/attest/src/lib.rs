pub mod errors;
pub mod provider;
pub mod types;

use std::path::Path;

use errors::AttestationError;
use types::{K256PkReport, Quote, RawReport};

#[derive(Debug)]
pub enum Provider {
    Ioctl,
    Coco,
}

const IOCTL_DEVICE_PATH: &str = "/dev/tdx_guest";

/*
pub fn get_quote(report: RawReport) -> Result<Quote, AttestationError> {
    let provider = match tdx_attestation_sdk::device::Device::default() {
        Ok(_) => Provider::Coco,
        // Fallback to legacy /dev/tdx_guest, which is available on
        // patched kernel 5.x. For example, alinux3 from aliyun
        Err(_) if Path::new(IOCTL_DEVICE_PATH).exists() => Provider::Ioctl,
        Err(_) => return Err(AttestationError::NoProviderAvailable),
    };

    get_quote_with_provider(report, provider)
}
*/

pub fn get_quote(report: RawReport) -> Result<Quote, AttestationError> {
    let provider = match tdx_attestation_sdk::device::Device::default() {
        Ok(_) => Provider::Coco,
        Err(e) => {
            tracing::warn!("Coco provider failed: {:?}, falling back", e);
            if Path::new(IOCTL_DEVICE_PATH).exists() {
                Provider::Ioctl
            } else {
                return Err(AttestationError::NoProviderAvailable);
            }
        }
    };
    get_quote_with_provider(report, provider)
}

pub fn get_quote_for_k256_pk(report: K256PkReport) -> Result<Quote, AttestationError> {
    tracing::info!("quote report {}", report);

    get_quote(report.to_raw())
}

fn get_quote_with_provider(
    report: RawReport,
    provider: Provider,
) -> Result<Quote, AttestationError> {
    let raw_quote = match provider {
        Provider::Ioctl => {
            #[cfg(feature = "ioctl")]
            {
                provider::ioctl::get_raw_quote(report)?
            }
            #[cfg(not(feature = "ioctl"))]
            {
                return Err(AttestationError::Ioctl("feature isn't enabled".to_string()));
            }
        }
        Provider::Coco => provider::coco::get_raw_quote(report)?,
    };

    let quote = Quote::from_bytes(&raw_quote)?;

    Ok(quote)
}
