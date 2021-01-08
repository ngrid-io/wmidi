use crate::prelude::*;

pub(crate) struct MIDIEndpoint<'a> {
    inner: &'a coremidi::Endpoint,
}

impl<'a> From<&'a coremidi::Endpoint> for MIDIEndpoint<'a> {
    fn from(inner: &'a coremidi::Endpoint) -> Self {
        Self { inner }
    }
}

impl<'a> MIDIEndpoint<'a> {
    pub fn id(&self) -> u32 {
        // self.inner.set_property_integer(coremidi::kMIDIPropertyUniqueID, value);
        // self.inner.get_property_integer(coremidi::kMIDIPropertyUniqueID)
        self.inner.unique_id().unwrap()
    }

    pub fn manufacturer(&self) -> String {
        // self.inner.as_ref().set_property_string(name, value)
        self.inner.manufacturer().unwrap()
    }

    pub fn name(&self) -> String {
        self.inner.name().unwrap()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    // pub fn kind(&self) -> MIDIPortKind {
    //     todo!()
    // }

    pub fn version(&self) -> u32 {
        todo!()
        // self.inner.ve
    }

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    pub fn state(&self) -> MIDIPortDeviceState {
        todo!()
    }

    /// .open | .closed
    // pub fn connection(&self) -> MIDIPortConnectionState {
    //     todo!()
    // }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    // pub fn open(&mut self) {
    //     todo!()
    // }

    // /// closes the port
    // pub fn close(&mut self) {
    //     todo!()
    // }

    pub fn flush(&self) {
        let _ = self.inner.flush();
    }
}
