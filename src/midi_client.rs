pub struct MIDIClient {
    inner: coremidi::Client,
}

impl MIDIClient {
    pub fn new() -> Self {
        Self {
            inner: coremidi::Client::new("fdsa").unwrap(),
        }
    }

    pub fn new_with_notification() -> Self {
        todo!()
    }
}
