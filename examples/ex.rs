// use wmidi_coremidi::prelude::*;

// fn main() {
//     // let i = wmidi_coremidi::MIDISourceIterator::new();

//     // for e in i {
//     //     println!("endpoint {:?}", e.manufacturer());
//     // }

//     // let a = "one";
//     // let b = "two";

//     // println!("{:?} {:?}", a, b);
//     if true {
//         let access = MIDIAccess::new("example");
//         // let v = vec![1,2,3];
//         // access.outputs().iter().first
//         // for (k, v) in access.outputs().iter() {
//         //     println!("{:?}", v.manufacturer());
//         // }
//         let outputs = access.outputs();

//         let port = outputs
//             .iter()
//             .find(|x| !x.1.manufacturer().contains("Snoize"))
//             .unwrap();

//         // println!("here");
//         println!("output manufacturer {:?}", port.1.manufacturer());

//         // .map(|x| x.1.sender())
//         let sender = port.1.sender();
//         println!("sender name {}", sender.display_name());
//         // .unwrap();
//         let mut recv = access
//             .inputs()
//             .iter()
//             .next()
//             .map(|x| x.1.receiver())
//             .unwrap();

//         println!("recv name {}", recv.display_name());
//         std::thread::spawn(move || {
//             println!("spawn ");
//             match recv.recv() {
//                 Ok(packet) => {
//                     println!("midipacket: {:?}", packet);
//                 }
//                 Err(e) => {
//                     println!("error {:?}", e);
//                 }
//             }
//         });

//         // println!("here");
//         // sender.send(&MIDIPacket::new(0, &[0x80, 0x80, 0x80]));
//         sender.send(0, &[0x80, 0x80, 0x00]);
//         std::thread::sleep(std::time::Duration::from_secs(2));
//         // let res = recv.try_recv();
//     }

//     // let nanos = wmidi_coremidi::current_host_time();
//     // println!("{}", wmidi_coremidi::convert_nanos_to_host_time(nanos));

//     // sender.send()
//     // for (k, v) in access.outputs().iter() {
//     //     sender = Some(v.sender());
//     //     break;
//     // }

//     // let a = access.inputs().iter().first();
//     // let mut recv = None;
//     // for (k, v) in access.inputs().iter() {
//     //     recv = Some(v.receiver());
//     //     break;
//     // }
// }
