use crate::{
    MIDIPortID,
    StateChangeObserver,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortKind {
    Input,
    Output,
}

// impl From<coremidi::
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortDeviceState {
    Disconnected,
    Connected,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MIDIPortConnectionState {
    Open,
    Closed,
}

pub trait MIDIPort: Eq + std::hash::Hash + std::fmt::Debug {
    fn id(&self) -> MIDIPortID;
    fn manufacturer(&self) -> String;

    fn name(&self) -> String;

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind;

    fn version(&self) -> u32;

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState;

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState;

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self);

    /// closes the port
    fn close(&mut self);

    fn set_on_state_change(&mut self, on_state_change: Option<Box<dyn StateChangeObserver>>);
}
