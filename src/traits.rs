use crate::prelude::*;
// pub type OnInputStateChange = dyn Fn(&MIDIInput) -> () + 'static ;
pub trait StateChangeObserver {
    fn input_state_changed(&mut self, input: &MIDIInput);
    fn output_state_changed(&mut self, output: &MIDIOutput);
}

// pub type InputCallback = std::sync::Arc<dyn FnMut() -> () + 'static + Sync + Send>;
pub trait InputReceiver: Send + Sync {
    fn receive(&mut self, packet_list: &coremidi::PacketList);
}
