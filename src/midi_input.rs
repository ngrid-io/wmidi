use crate::{
    prelude::*,
    MIDIClient,
    MIDIEndpoint,
};

// pub type OnInputStateChange = dyn Fn(&MIDIInput) -> () + 'static ;
pub trait StateChangeObserver {
    fn input_state_changed(&self, input: &MIDIInput);
    fn output_state_changed(&self, output: &MIDIOutput);

    // fn port_state_changed(&self, port: impl MIDIPort);
}

pub struct MIDIInput {
    inner: coremidi::Source,
    port: Option<coremidi::InputPort>,
    client: MIDIClient,
    state_change: Box<dyn StateChangeObserver>,
}

impl PartialEq for MIDIInput {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for MIDIInput {}

impl std::hash::Hash for MIDIInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl std::fmt::Debug for MIDIInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MIDIInput {{{:?} {:?}}}",
            self.manufacturer(),
            self.name()
        )
    }
}

impl MIDIInput {
    pub(crate) fn new(inner: coremidi::Source) -> Self {
        // Self { inner }
        todo!()
    }

    fn endpoint<'a>(&'a self) -> MIDIEndpoint<'a> {
        self.inner.endpoint().into()
    }
}

impl MIDIPort for MIDIInput {
    fn id(&self) -> u32 {
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
        MIDIPortKind::Input
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
        todo!()
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        // self.client.
        todo!()
        // self.inner.
        // todo!()
    }

    /// closes the port
    fn close(&mut self) {
        if self.connection() == MIDIPortConnectionState::Closed {
            return;
        }

        if let Some(port) = &self.port {
            let _ = port.disconnect_source(&self.inner);
        }
        self.port = None;
        self.state_change.input_state_changed(self);
        // self.on_state_change(self);

        //     guard connection != .closed else { return }

        //     switch type {
        //     case .input:
        //         OSAssert(MIDIPortDisconnectSource(ref, endpoint.ref))
        //     case .output:
        //         endpoint.flush()
        //     }

        //     ref = 0
        //     onStateChange?(self)
        //     onStateChange = nil
        // }
    }
}
