use std::fmt;

pub struct DeviceToken {
    pub token: String,
}

impl DeviceToken {
    pub fn new<S>(token: S) -> DeviceToken
        where S: Into<String>
    {
        DeviceToken { token: token.into() }
    }
}

impl fmt::Display for DeviceToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
