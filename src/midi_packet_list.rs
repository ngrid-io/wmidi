pub struct MIDIPacketListIterator<'a> {
    a: &'a u32,
}
pub struct MIDIPacketList<'a> {
    pub(crate) inner: &'a coremidi::PacketList,
}

// impl<'a> Iterator for MIDIPacketList<'a> {
//     type Item = &'a Packet;
// }
