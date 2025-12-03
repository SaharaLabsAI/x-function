use attest::types::RawReport;

pub fn generate_raw_report(data: &[impl AsRef<[u8]>]) -> RawReport {
    let mut hasher = blake3::Hasher::new();

    for d in data {
        hasher.update(d.as_ref());
    }

    let h: [u8; 32] = hasher.finalize().into();

    let mut report = [0u8; 64];
    report[..32].copy_from_slice(&h);

    RawReport::new(report)
}

pub fn generate_raw_report_from_hash(h: [u8; 32]) -> RawReport {
    let mut report = [0u8; 64];
    report[..32].copy_from_slice(&h);

    RawReport::new(report)
}
