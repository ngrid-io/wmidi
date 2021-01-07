use crate::{
    prelude::*,
    MIDIClient,
};

// pub struct MIDIAccess {
//     imp: std::cell::RefCell<std::rc::Rc<MIDIAccessImpl>>
// }

pub struct MIDIAccess {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
    // rx: std::sync::mpsc::Receiver<coremidi::Notification>,
}

impl MIDIAccess {
    pub fn new(name: &str) -> Self {
        let client = MIDIClient::new(name);
        // let (tx, rx) = std::sync::mpsc::channel();
        // // let client = MIDIClient::new_with_notification(name, move |x| {

        //     // tx.send(*x);
        // });
        Self {
            inputs: MIDIPortMap::<MIDIInput>::new(&client),
            outputs: MIDIPortMap::<MIDIOutput>::new(&client),
            // rx,
            client,
        }
    }

    pub fn inputs(&self) -> &MIDIPortMap<MIDIInput> {
        &self.inputs
    }

    pub fn outputs(&self) -> &MIDIPortMap<MIDIOutput> {
        &self.outputs
    }

    pub fn restart(&self) {
        coremidi::restart();
    }
    // fn insert(&self,)

    // fn notification(&mut self, notification: coremidi::Notification) {
    //     match notification {
    //         coremidi::Notification::ObjectAdded(info) => {
    //             // let object = info.child;
    //             // let end = coremidi::Endpoint::from(object);
    //             todo!()
    //         }
    //         coremidi::Notification::ObjectRemoved(info) => {
    //             todo!()
    //         }
    //         _ => todo!(),
    //     }
    // }
}
