use std::fmt::Display;

use dcap_rs::{
    constants::HEADER_LEN,
    types::quotes::{
        body::QuoteBody, version_3::QuoteV3, version_4::QuoteV4, version_5::QuoteV5, QuoteHeader,
    },
};
use k256::ecdsa::VerifyingKey;

use crate::errors::QuoteError;

#[derive(Clone, Debug)]
pub struct Quote {
    raw: Vec<u8>,
    report: QuoteReport,
}

#[derive(Clone, Debug)]
pub enum QuoteReport {
    V3(QuoteV3),
    V4(QuoteV4),
    V5(QuoteV5),
}

impl Quote {
    pub fn from_bytes(bytes: &[u8]) -> Result<Quote, QuoteError> {
        if bytes.len() < HEADER_LEN {
            return Err(QuoteError::InvalidHeaderSize(bytes.len()));
        }

        let header = QuoteHeader::from_bytes(&bytes[0..HEADER_LEN]);
        let report = match header.version {
            3 => QuoteReport::V3(QuoteV3::from_bytes(bytes)),
            4 => QuoteReport::V4(QuoteV4::from_bytes(bytes)),
            5 => QuoteReport::V5(QuoteV5::from_bytes(bytes)),
            _ => return Err(QuoteError::UnknownQuote(header.version)),
        };

        Ok(Quote {
            raw: bytes.to_vec(),
            report,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }

    pub fn k256_pk_report(&self) -> Result<K256PkReport, QuoteError> {
        let report_data = self.report_data();

        let point = k256::EncodedPoint::from_bytes(&report_data[0..33])
            .map_err(|e| QuoteError::ReportData(format!("invalid secp pk {e}")))?;
        let pk = k256::ecdsa::VerifyingKey::from_encoded_point(&point)
            .map_err(|e| QuoteError::ReportData(format!("invalid secp pk {e}")))?;

        Ok(K256PkReport { pk })
    }

    pub fn quote_report(&self) -> &QuoteReport {
        &self.report
    }

    pub fn report_data(&self) -> [u8; 64] {
        if let QuoteReport::V3(ref quote) = self.report {
            return quote.isv_enclave_report.report_data;
        }

        let body = match &self.report {
            QuoteReport::V4(quote) => quote.quote_body,
            QuoteReport::V5(quote) => quote.quote_body,
            _ => unreachable!(),
        };

        match body {
            QuoteBody::SGXQuoteBody(report) => report.report_data,
            QuoteBody::TD10QuoteBody(report) => report.report_data,
            QuoteBody::TD15QuoteBody(report) => report.report_data,
        }
    }
}

impl QuoteReport {
    pub fn rtmr3(&self) -> [u8; 48] {
        if let QuoteReport::V3(_) = self {
            return [0u8; 48];
        }

        let body = match self {
            QuoteReport::V4(quote) => quote.quote_body,
            QuoteReport::V5(quote) => quote.quote_body,
            _ => unreachable!(),
        };

        match body {
            QuoteBody::SGXQuoteBody(_) => [0u8; 48],
            QuoteBody::TD10QuoteBody(report) => report.rtmr3,
            QuoteBody::TD15QuoteBody(report) => report.rtmr3,
        }
    }
}

#[derive(Debug)]
pub struct RawReport([u8; 64]);

impl RawReport {
    pub fn new(raw: [u8; 64]) -> Self {
        RawReport(raw)
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0
    }
}

#[derive(Debug)]
pub struct K256PkReport {
    pk: VerifyingKey,
}

impl K256PkReport {
    pub fn new(pk: VerifyingKey) -> Self {
        K256PkReport { pk }
    }

    pub fn pubkey(&self) -> &VerifyingKey {
        &self.pk
    }

    pub fn to_raw(&self) -> RawReport {
        let point = self.pk.to_encoded_point(true);

        let mut buf = [0u8; 64];
        buf[0..33].copy_from_slice(&point.to_bytes());

        RawReport(buf)
    }
}

impl Display for K256PkReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let point = self.pk.to_encoded_point(true);

        write!(f, "report: pk {point}")
    }
}
