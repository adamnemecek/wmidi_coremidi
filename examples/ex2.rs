use wmidi_coremidi::prelude::*;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let access = MIDIAccess::new("name", move |notification| {
        tx.send(10);
    });

    // let res = access.inputs().iter().find(|x| true);
    // let mut input;
    // for (i, e) in access.inputs().iter() {
    //     input = e;
    //     println!("{:?} {:?}", i, e);
    // }

    let outputs: Vec<_> = access.outputs().iter().collect();
    // let (_, output) = outputs.first().unwrap();
    // output.
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
        todo!("received msg");
    }));
    println!("input {:?}", input);
    output.send(&[0x90, 100, 100], None);

    // for (i, e) in outputs.iter() {
    //     println!("{:?}", e);
    // }
    std::thread::sleep(std::time::Duration::from_secs(2));
    // let input = res.unwrap().1;

    // let mut input = input.clone();
    // input.set_on_midi_message(|x| {
    //     tx.send(10);
    // });
}
