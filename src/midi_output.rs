use crate::{
    prelude::*,
    MIDIEndpoint,
};
pub struct MIDIOutput {
    // inner: MIDIEndpoint,
    inner: coremidi::Destination,
}

impl MIDIOutput {
    pub(crate) fn new(inner: coremidi::Destination) -> Self {
        Self { inner }
    }

    fn endpoint<'a>(&'a self) -> MIDIEndpoint<'a> {
        self.inner.endpoint().into()
    }
}

impl PartialEq for MIDIOutput {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for MIDIOutput {}

impl std::hash::Hash for MIDIOutput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl std::fmt::Debug for MIDIOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIDIOutput {{{:?}}}", self.name())
    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> u32 {
        self.endpoint().id()
    }

    fn manufacturer(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        self.endpoint().name()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        todo!()
    }

    fn version(&self) -> u32 {
        todo!()
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
