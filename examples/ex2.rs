use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let access = MIDIAccess::new("name", move |notification| {
        tx.send(10);
    });

    let outputs: Vec<_> = access.outputs().iter().collect();
    for (_, p) in outputs.iter() {
        println!("{:?}", p);
    }

    let inputs: Vec<_> = access.inputs().iter().collect();
    for (_, p) in inputs.iter() {
        println!("{:?}", p);
    }

    let outputs = outputs
        .iter()
        .find(|x| x.1.display_name().contains("Driver"));
    let mut output = outputs.unwrap().1.clone();

    let mut input = access.input_for(&output).unwrap();

    input.set_on_midi_message(std::rc::Rc::new(|x| {
        println!("msg: {:?}", x);
    }));
    // output.open();
    // input.open();
    println!("input {:?}", input);
    output.send(&[0x90, 100, 100], None);

    // for (i, e) in outputs.iter() {
    //     println!("{:?}", e);
    // }
    // std::thread::sleep(std::time::Duration::from_secs(2));
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
