use anyhow::{Context, Result};
use std::time::SystemTime;
use tracing::{debug, info};

use super::types::ComplianceQuote;

/// Generate a real TEE attestation quote for a compliance check result
/// 
/// This generates an actual attestation quote from the TEE (TDX/SGX) that includes:
/// - A hash of the compliance check data in the report_data field
/// - TEE measurements (RTMR values)
/// - Signature from the TEE's attestation key
/// - Certificate chain for verification
pub fn generate_compliance_quote(
    tool_name: &str,
    compliant: bool,
    policy_ids: &[String],
    user_query: &str,
    arguments: &str,
) -> Result<ComplianceQuote> {
    // Generate a deterministic hash of the compliance check inputs
    // This hash will be embedded in the TEE attestation quote's report_data
    let compliance_hash = hash_compliance_data(
        tool_name,
        compliant,
        policy_ids,
        user_query,
        arguments,
    );

    debug!(
        tool_name = %tool_name,
        compliant = compliant,
        compliance_hash = %const_hex::encode(compliance_hash),
        "Generating TEE attestation quote for compliance check"
    );

    // Generate the raw report with the compliance hash
    let raw_report = crate::utils::attest::generate_raw_report_from_hash(compliance_hash);

    // Get the actual TEE attestation quote (TDX/SGX)
    let quote = attest::get_quote(raw_report)
        .context("Failed to generate TEE attestation quote for compliance check")?;

    let quote_bytes = quote.to_bytes();

    info!(
        tool_name = %tool_name,
        compliant = compliant,
        quote_size = quote_bytes.len(),
        "Generated TEE attestation quote for compliance check"
    );

    Ok(ComplianceQuote {
        tool_name: tool_name.to_string(),
        compliant,
        quote_bytes,
        compliance_hash,
        timestamp: SystemTime::now(),
    })
}

/// Hash the compliance check data
/// 
/// Creates a deterministic hash that represents the compliance check decision.
/// This hash is embedded in the TEE attestation quote's report_data field.
fn hash_compliance_data(
    tool_name: &str,
    compliant: bool,
    policy_ids: &[String],
    user_query: &str,
    arguments: &str,
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    
    // Hash a version prefix
    hasher.update(b"COMPLIANCE_V1");
    
    // Hash tool name
    hasher.update(tool_name.as_bytes());
    
    // Hash compliance result
    hasher.update(&[if compliant { 1u8 } else { 0u8 }]);
    
    // Hash policy IDs (sorted for determinism)
    let mut sorted_policies = policy_ids.to_vec();
    sorted_policies.sort();
    for policy_id in sorted_policies {
        hasher.update(policy_id.as_bytes());
    }
    
    // Hash user query
    hasher.update(user_query.as_bytes());
    
    // Hash arguments
    hasher.update(arguments.as_bytes());
    
    hasher.finalize().into()
}

/// Verify a compliance quote (dummy implementation for tools)
/// 
/// In a real implementation, this would:
/// 1. Parse the quote and verify its signature using DCAP
/// 2. Check the certificate chain against Intel/AMD root certificates
/// 3. Verify RTMR/measurement values match the expected hypervisor
/// 4. Extract the compliance_hash from report_data and validate it
/// 5. Check the quote freshness/timestamp
/// 
/// For this simulation, we do basic validation and return success.
/// The actual verification would require DCAP libraries and root certificates.
pub fn verify_compliance_quote_dummy(
    quote: &ComplianceQuote,
    expected_tool_name: &str,
) -> Result<bool> {
    // Check tool name matches
    if quote.tool_name != expected_tool_name {
        debug!(
            expected = %expected_tool_name,
            actual = %quote.tool_name,
            "Quote verification failed: tool name mismatch"
        );
        return Ok(false);
    }

    // Check we have a quote
    if quote.quote_bytes.is_empty() {
        debug!("Quote verification failed: empty quote");
        return Ok(false);
    }

    // In a real implementation, we would:
    // 1. Parse the quote using dcap-rs or similar
    // 2. Verify the quote signature
    // 3. Check the certificate chain
    // 4. Validate RTMR values against known good measurements
    // 5. Extract report_data and verify it matches compliance_hash
    // 6. Check timestamp for freshness
    
    // For now, we just parse it to check basic validity
    let quote_parsed = attest::types::Quote::from_bytes(&quote.quote_bytes);
    
    match quote_parsed {
        Ok(parsed_quote) => {
            // Extract report_data from the quote
            let report_data = parsed_quote.report_data();
            let embedded_hash = &report_data[..32];
            
            // Verify the compliance hash matches what's in the quote
            if embedded_hash != quote.compliance_hash {
                debug!(
                    expected = %const_hex::encode(quote.compliance_hash),
                    actual = %const_hex::encode(embedded_hash),
                    "Quote verification failed: compliance hash mismatch"
                );
                return Ok(false);
            }
            
            info!(
                tool_name = %quote.tool_name,
                compliant = quote.compliant,
                "Quote verification PASSED (dummy verification - signature not checked)"
            );
            
            Ok(true)
        }
        Err(e) => {
            debug!(error = %e, "Quote verification failed: invalid quote format");
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_compliance_data() {
        let hash1 = hash_compliance_data(
            "PriceFeedTool",
            true,
            &["L1".to_string()],
            "What is the price of BTC?",
            r#"{"symbol": "BTC"}"#,
        );

        let hash2 = hash_compliance_data(
            "PriceFeedTool",
            true,
            &["L1".to_string()],
            "What is the price of BTC?",
            r#"{"symbol": "BTC"}"#,
        );

        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);

        // Different compliance result should produce different hash
        let hash3 = hash_compliance_data(
            "PriceFeedTool",
            false, // changed
            &["L1".to_string()],
            "What is the price of BTC?",
            r#"{"symbol": "BTC"}"#,
        );

        assert_ne!(hash1, hash3);
    }

    #[test]
    #[ignore] // Requires TEE environment
    fn test_generate_quote() {
        let quote = generate_compliance_quote(
            "PriceFeedTool",
            true,
            &["L1".to_string()],
            "What is the price of BTC?",
            r#"{"symbol": "BTC"}"#,
        )
        .unwrap();

        assert_eq!(quote.tool_name, "PriceFeedTool");
        assert_eq!(quote.compliant, true);
        assert!(!quote.quote_bytes.is_empty());
    }
}
