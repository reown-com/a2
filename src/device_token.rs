pub struct DeviceToken {
    pub token: String,
}

impl DeviceToken {
    pub fn new<S>(token: S) -> DeviceToken where S: Into<String> {
        DeviceToken {token: token.into()}
    }
}
