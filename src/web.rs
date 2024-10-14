use std::{
    sync::mpsc::{Receiver, Sender},
    io::Cursor
};
use rodio::{Decoder, OutputStream, Sink, Source};

pub fn web_entry(_ui_sender: Sender<i32>, ui_receiver: Receiver<(i32, String)>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink: Sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.05);

    let mut in_call = false;
    let mut code_correct = false;
    let mut block_events = false;

    loop {
        if let Ok(message) = ui_receiver.recv() {
            if message.0 == 0 {
                sink.clear();

                in_call = false;
                code_correct = false;
                block_events = false;
            } else if !block_events {
                if message.0 == 1 {
                    println!("Playing Dial Tone"); 
                    let source = Decoder::new(Cursor::new(include_bytes!("dialtone.flac"))).unwrap();
                    
                    sink.clear();
                    sink.append(source.convert_samples::<f32>());
                    sink.play();

                    in_call = false;
                    code_correct = false;
                    block_events = false;
                } else if message.0 == 2 {
                    println!("Playing Question");

                    let source = Decoder::new(Cursor::new(include_bytes!("./question.flac"))).unwrap();
                
                    sink.clear();
                    sink.append(source.convert_samples::<f32>());
                    sink.play();

                    in_call = true;
                    code_correct = false;
                    block_events = false;
                } else if in_call && !code_correct {
                    if message.1 == "53" {
                        println!("Playing Success");

                        let source = Decoder::new(Cursor::new(include_bytes!("./success.flac"))).unwrap();
                
                        sink.clear();
                        sink.append(source.convert_samples::<f32>());
                        sink.play();

                        in_call = true;
                        code_correct = true;
                        block_events = true;
                    } else if message.1.len() >= 2 {
                        println!("Playing Failure");

                        let source = Decoder::new(Cursor::new(include_bytes!("./failure.flac"))).unwrap();
            
                        sink.clear();
                        sink.append(source.convert_samples::<f32>());
                        sink.play();

                        in_call = true;
                        code_correct = false;
                        block_events = true;
                    }
                }
            }
        }
    }
}
