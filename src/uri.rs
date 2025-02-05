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

// impl Uri {
//     pub fn parse(uri: String) -> Result<Self, PayjoinError> {
//         match payjoin::Uri::from_str(uri.as_str()) {
//             Ok(e) => Ok(e.assume_checked().into()),
//             Err(e) => Err(PayjoinError::PjParseError { message: e.to_string() }),
//         }
//     }
//     pub fn address(&self) -> String {
//         self.clone().0.address.to_string()
//     }
//     /// Gets the amount in satoshis.
//     pub fn amount_sats(&self) -> Option<u64> {
//         self.0.amount.map(|x| x.to_sat())
//     }
//     #[cfg(not(feature = "uniffi"))]
//     pub fn check_pj_supported(&self) -> Result<PjUri, PayjoinError> {
//         match self.0.clone().check_pj_supported() {
//             Ok(e) => Ok(e.into()),
//             Err(_) => {
//                 Err(PayjoinError::PjNotSupported {
//                     message: "Uri doesn't support payjoin".to_string(),
//                 })
//             }
//         }
//     }
//     #[cfg(feature = "uniffi")]
//     pub fn check_pj_supported(&self) -> Result<Arc<PjUri>, PayjoinError> {
//         match self.0.clone().check_pj_supported() {
//             Ok(e) => Ok(Arc::new(e.into())),
//             Err(_) => {
//                 Err(PayjoinError::PjNotSupported {
//                     message: "Uri doesn't support payjoin".to_string(),
//                 })
//             }
//         }
//     }
//     pub fn as_string(&self) -> String {
//         self.0.clone().to_string()
//     }
// }
