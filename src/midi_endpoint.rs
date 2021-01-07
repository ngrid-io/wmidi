use crate::prelude::*;

pub struct MIDIEndpoint {
    inner: coremidi::Endpoint,
}

impl MIDIEndpoint {
    fn id(&self) -> u32 {
        // self.inner.set_property_integer(coremidi::kMIDIPropertyUniqueID, value);
        // self.inner.get_property_integer(coremidi::kMIDIPropertyUniqueID)
        self.inner.unique_id().unwrap()
    }

    fn manufacturer(&self) -> String {
        // self.inner.as_ref().set_property_string(name, value)
        self.inner.manufacturer().unwrap()
    }

    fn name(&self) -> String {
        self.inner.name().unwrap()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        todo!()
    }

    fn version(&self) -> u32 {
        todo!()
        // self.inner.ve
    }

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState {
        todo!()
    }

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState {
        todo!()
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        todo!()
    }

    /// closes the port
    fn close(&mut self) {
        todo!()
    }
}
