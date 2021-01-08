use crate::{
    prelude::*,
    MIDIClient,
    MIDIEndpoint,
};

pub struct MIDIInput {
    inner: coremidi::Source,
    port: Option<coremidi::InputPort>,
    client: MIDIClient,
    state_change: Option<Box<dyn MIDIPortStateChangeObserver>>,
    input_observer: Option<std::sync::Arc<std::sync::Mutex<Box<dyn MIDIInputObserver>>>>,
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
    pub(crate) fn new(inner: coremidi::Source, client: MIDIClient) -> Self {
        Self {
            inner,
            port: None,
            client,
            state_change: None,
            input_observer: None,
        }
    }

    fn endpoint<'a>(&'a self) -> MIDIEndpoint<'a> {
        self.inner.endpoint().into()
    }

    pub fn set_input_observer(&mut self, input_observer: Box<dyn MIDIInputObserver>) {
        self.input_observer = Some(std::sync::Arc::new(std::sync::Mutex::new(input_observer)));
    }
}

impl MIDIPort for MIDIInput {
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
        if self.port.is_none() {
            MIDIPortConnectionState::Closed
        } else {
            MIDIPortConnectionState::Open
        }
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        if self.connection() == MIDIPortConnectionState::Open {
            return;
        }
        // switch type {
        //     case .input:
        //         let `self` = self as! MIDIInput
        //         ref = MIDIInputPortCreate(ref: client.ref) {
        //             `self`.onMIDIMessage?($0)
        //         }

        //         OSAssert(MIDIPortConnectSource(ref, endpoint.ref, nil))

        //     case .output:
        //         ref = MIDIOutputPortCreate(ref: client.ref)

        // let mut input_observer = None;
        // std::mem::swap(&mut self.input_observer, &mut input_observer);
        let mut input_observer = self.input_observer.clone();

        self.port = Some(self.client.open_input(&self.inner, move |p| {
            if let Some(input_observer) = input_observer.as_mut() {
                if let Ok(mut input_observer) = input_observer.lock() {
                    // i.receive(&MIDIPacketList { inner: p })
                    input_observer.receive(p);
                } else {
                    panic!("failed to lock input")
                };
            }
        }));
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

        let mut state_change = None;
        std::mem::swap(&mut self.state_change, &mut state_change);

        if let Some(f) = state_change.as_mut() {
            f.input_state_changed(&self);
        }
        std::mem::swap(&mut self.state_change, &mut state_change);
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

    fn set_on_state_change(
        &mut self,
        on_state_change: Option<Box<dyn MIDIPortStateChangeObserver>>,
    ) {
        self.state_change = on_state_change;
    }
}
