use std::str::FromStr;
use bitcoin_uri;

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

    pub fn amount_sats(&mut self, amount: u64) -> &mut Self {
        self.0.amount = Some(bitcoin::Amount::from_sat(amount));
        self
    }

    pub fn label(&mut self, label: String) -> &mut Self {
        self.0.label = Some(label.into());
        self
    }

    pub fn message(&mut self, message: String) -> &mut Self {
        self.0.message = Some(message.into());
        self
    }

    pub fn build(self) -> Uri {
        self.0.into()
    }
}

