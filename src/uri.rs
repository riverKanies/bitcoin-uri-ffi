use bitcoin_uri;

use std::str::FromStr;

extern crate alloc;


use alloc::borrow::ToOwned;
use alloc::borrow::Cow;
#[cfg(feature = "non-compliant-bytes")]
use alloc::vec::Vec;
use alloc::string::String;
#[cfg(feature = "non-compliant-bytes")]
use either::Either;
use core::convert::{TryFrom, TryInto};
use bitcoin::address::NetworkValidation;


#[derive(Clone)]
pub struct Uri(bitcoin_uri::Uri<'static>);

impl From<Uri> for bitcoin_uri::Uri<'static> {
    fn from(value: Uri) -> Self {
        value.0
    }
}

impl From<bitcoin_uri::Uri<'static>> for Uri {
    fn from(value: bitcoin_uri::Uri<'static>) -> Self {
        Uri(value)
    }
}

impl Uri {
    pub fn parse(uri: String) -> Result<Self, String> {
        match bitcoin_uri::Uri::from_str(uri.as_str()) {
            Ok(uri) => {
                // Validate the network before converting to our Uri type
                match uri.require_network(bitcoin_ffi::Network::Bitcoin) {
                    Ok(checked_uri) => Ok(checked_uri.into()),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn address(&self) -> String {
        self.0.address.to_string()
    }

    pub fn amount_sats(&self) -> Option<u64> {
        self.0.amount.map(|x| x.to_sat())
    }

    pub fn label(&self) -> Option<String> {
        self.0.label.clone().and_then(|l| l.try_into().ok())
    }

    pub fn message(&self) -> Option<String> {
        self.0.message.clone().and_then(|m| m.try_into().ok())
    }

    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

// Add builder pattern if needed for constructing URIs
pub struct UriBuilder(bitcoin_uri::Uri<'static>);

impl UriBuilder {
    pub fn new(address: bitcoin_ffi::Address) -> Self {
        Self(bitcoin_uri::Uri::new(address.into()))
    }

    pub fn amount_sats(mut self, amount: u64) -> Self {
        self.0.amount = Some(bitcoin::Amount::from_sat(amount));
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.0.label = Some(label.into());
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.0.message = Some(message.into());
        self
    }

    pub fn build(self) -> Uri {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn just_address() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert!(uri.amount_sats().is_none());
        assert!(uri.label().is_none());
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn address_with_name() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=Luke-Jr".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "Luke-Jr");
        assert!(uri.amount_sats().is_none());
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn request_20_point_30_btc_to_luke_dash_jr() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=20.3&label=Luke-Jr".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "Luke-Jr");
        assert_eq!(uri.amount_sats().unwrap(), 20_30_000_000);
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn request_50_btc_with_message() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?amount=50&label=Luke-Jr&message=Donation%20for%20project%20xyz".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.amount_sats().unwrap(), 50_00_000_000);
        assert_eq!(uri.label().unwrap(), "Luke-Jr");
        assert_eq!(uri.message().unwrap(), "Donation for project xyz");

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn required_not_understood() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?req-somethingyoudontunderstand=50&req-somethingelseyoudontget=999".to_string();
        let uri = Uri::parse(input);
        assert!(uri.is_err());
    }

    #[test]
    fn required_understood() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?somethingyoudontunderstand=50&somethingelseyoudontget=999".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert!(uri.amount_sats().is_none());
        assert!(uri.label().is_none());
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd");
    }

    #[test]
    fn label_with_rfc3986_param_separator() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo%26bar%20%3D%20baz/blah?;:@".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "foo&bar = baz/blah?;:@");
        assert!(uri.amount_sats().is_none());
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn label_with_rfc3986_fragment_separator() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo%23bar".to_string();
        let uri = Uri::parse(input.clone()).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "foo#bar");
        assert!(uri.amount_sats().is_none());
        assert!(uri.message().is_none());

        assert_eq!(uri.as_string(), input);
    }

    #[test]
    fn rfc3986_empty_fragment_not_defined_in_bip21() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo#".to_string();
        let uri = Uri::parse(input).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "foo");
        assert!(uri.amount_sats().is_none());
        assert!(uri.message().is_none());
        assert_eq!(uri.as_string(), "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo");
    }

    #[test]
    fn rfc3986_non_empty_fragment_not_defined_in_bip21() {
        let input = "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo#&message=not%20part%20of%20a%20message".to_string();
        let uri = Uri::parse(input).unwrap();
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.label().unwrap(), "foo");
        assert!(uri.amount_sats().is_none());
        assert!(uri.message().is_none());
        assert_eq!(uri.as_string(), "bitcoin:1andreas3batLhQa2FawWjeyjCqyBzypd?label=foo");
    }

    #[test]
    fn bad_unicode_scheme() {
        let input = "bitcoinö:1andreas3batLhQa2FawWjeyjCqyBzypd".to_string();
        let uri = Uri::parse(input);
        assert!(uri.is_err());
    }

    // Add new test for the builder pattern
    #[test]
    fn builder_pattern() {
        let address = bitcoin_ffi::Address::new("1andreas3batLhQa2FawWjeyjCqyBzypd".to_string(), bitcoin_ffi::Network::Bitcoin).unwrap();
        let uri = UriBuilder::new(address)
            .amount_sats(50_00_000_000)
            .label("Luke-Jr".to_string())
            .message("Donation for project xyz".to_string())
            .build();
        
        assert_eq!(uri.address(), "1andreas3batLhQa2FawWjeyjCqyBzypd");
        assert_eq!(uri.amount_sats().unwrap(), 50_00_000_000);
        assert_eq!(uri.label().unwrap(), "Luke-Jr");
        assert_eq!(uri.message().unwrap(), "Donation for project xyz");
    }
}