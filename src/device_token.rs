use std::fmt;
use std::borrow::Cow;

/// Specify the hexadecimal bytes of the device token for the target device.
pub struct DeviceToken<'a> {
    pub token: Cow<'a, str>,
}

impl<'a> DeviceToken<'a> {
    pub fn new<S>(token: S) -> DeviceToken<'a>
        where S: Into<Cow<'a, str>>
    {
        DeviceToken { token: token.into() }
    }
}

impl<'a> fmt::Display for DeviceToken<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
