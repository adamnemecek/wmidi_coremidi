use wmidi_coremidi::prelude::*;

fn main() {
    // let i = wmidi_coremidi::MIDISourceIterator::new();

    // for e in i {
    //     println!("endpoint {:?}", e.manufacturer());
    // }

    // let a = "one";
    // let b = "two";

    // println!("{:?} {:?}", a, b);
    if true {
        let access = MIDIAccess::new("example");
        // let v = vec![1,2,3];
        // access.outputs().iter().first
        let mut sender = access
            .outputs()
            .iter()
            .next()
            .map(|x| x.1.sender())
            .unwrap();
        let mut recv = access
            .inputs()
            .iter()
            .next()
            .map(|x| x.1.receiver())
            .unwrap();
        std::thread::spawn(move || {
            let packet = recv.try_recv().unwrap();
            println!("midipacket: {:?}", packet);
        });

        // println!("here");
        // sender.send(&MIDIPacket::new(0, &[0x80, 0x80, 0x80]));
        sender.send(0, &[0x80, 0x80, 0x00]);
        std::thread::sleep(std::time::Duration::from_secs(10));
        // let res = recv.try_recv();
    }

    // let nanos = wmidi_coremidi::current_host_time();
    // println!("{}", wmidi_coremidi::convert_nanos_to_host_time(nanos));

    // sender.send()
    // for (k, v) in access.outputs().iter() {
    //     sender = Some(v.sender());
    //     break;
    // }

    // let a = access.inputs().iter().first();
    // let mut recv = None;
    // for (k, v) in access.inputs().iter() {
    //     recv = Some(v.receiver());
    //     break;
    // }
}
