#[derive(Clone)]
pub(crate) struct MIDIClient {
    inner: std::rc::Rc<coremidi::Client>,
}

impl MIDIClient {
    pub fn new(name: &str) -> Self {
        Self {
            inner: std::rc::Rc::new(coremidi::Client::new(name).unwrap()),
        }
    }

    pub fn new_with_notification<F>(name: &str, callback: F) -> Self
    where
        F: FnMut(&coremidi::Notification) + Send + 'static,
    {
        Self {
            inner: std::rc::Rc::new(coremidi::Client::new_with_notifications(name, callback).unwrap()),
        }
    }

    pub(crate) fn open_input<F>(&mut self, source: coremidi::Source, callback: F)
    where
        F: FnMut(&coremidi::PacketList) + Send + 'static,
    {
        let z = self.inner.input_port("", callback);
        todo!()
    }
}
