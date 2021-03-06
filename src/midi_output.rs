use crate::{
    prelude::*,
    MIDIClient,
    MIDIEndpoint,
};

pub struct MIDIOutput {
    inner: coremidi::Destination,
    port: Option<coremidi::OutputPort>,
    client: MIDIClient,
    state_change: Option<Box<dyn MIDIPortStateChangeObserver>>,
}

impl MIDIOutput {
    pub(crate) fn new(inner: coremidi::Destination, client: MIDIClient) -> Self {
        Self {
            inner,
            port: None,
            client,
            state_change: None,
        }
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
        write!(
            f,
            "MIDIOutput {{{:?} {:?}}}",
            self.manufacturer(),
            self.name()
        )
    }
}

impl MIDIOutput {
    pub fn clear(&mut self) {
        self.endpoint().flush();
    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> MIDIPortID {
        self.endpoint().id()
    }

    fn manufacturer(&self) -> String {
        self.endpoint().manufacturer()
    }

    fn name(&self) -> String {
        self.endpoint().name()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Output
    }

    fn version(&self) -> u32 {
        self.endpoint().version()
    }

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState {
        self.endpoint().state()
    }

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState {
        if self.port.is_none() {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        todo!()
    }

    /// closes the port
    fn close(&mut self) {
        if self.connection() == MIDIPortConnectionState::Open {
            return;
        }
        self.endpoint().flush();
        self.port = None;

        let mut state_change = None;
        std::mem::swap(&mut self.state_change, &mut state_change);

        if let Some(f) = state_change.as_mut() {
            f.output_state_changed(&self);
        }
        std::mem::swap(&mut self.state_change, &mut state_change);
    }

    fn set_on_state_change(
        &mut self,
        on_state_change: Option<Box<dyn MIDIPortStateChangeObserver>>,
    ) {
        self.state_change = on_state_change;
    }
}
