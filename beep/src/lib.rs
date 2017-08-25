extern crate cpal;
extern crate futures;

use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

/* need a beep object.

Simple beep object:
- frequency
- duration
- loop
- complete callback

optional:
read a structure with a beep description. ( like json ). It would allow more complex beep sequences
*/

struct Beep {
    pub frequency: f32,
    pub duration: i32,
    voice: cpal::Voice,
    stream: cpal::SamplesStream,
    format: cpal::Format
}

impl Beep {

    pub fn start(&self, event_loop: cpal::EventLoop) {
        let event_loop = Arc::new(cpal::EventLoop::new());

        // Produce a sinusoid of maximum amplitude.
        let samples_rate = self.format.samples_rate.0 as f32;
        let mut data_source = (0u64..).map(move |t| t as f32 * 900.0 * 2.0 * 3.141592 / samples_rate)     // 440 Hz
                                    .map(move |t| t.sin());

        let future_to_exec = self.stream.for_each(move |buffer| -> Result<_, ()> {
            match buffer {
                cpal::UnknownTypeBuffer::U16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(self.format.channels.len()).zip(&mut data_source) {
                        let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() { *out = value; }
                    }
                },
                cpal::UnknownTypeBuffer::I16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(self.format.channels.len()).zip(&mut data_source) {
                        let value = (value * std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() { *out = value; }
                    }
                },
                cpal::UnknownTypeBuffer::F32(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(self.format.channels.len()).zip(&mut data_source) {
                        for out in sample.iter_mut() { *out = value; }
                    }
                },
            };

            Ok(())
        });
        self.voice.play();
        {
            struct MyExecutor;
            impl Executor for MyExecutor {
                fn execute(&self, r: Run) {
                    r.run();
                }
            }
            task::spawn(future_to_exec).execute(Arc::new(MyExecutor));
        }

/*
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));
                self.voice.pause();
                thread::sleep(Duration::from_millis(500));
                self.voice.play();
            }
        });*/

        thread::Builder::new()
            .name("beep thread".to_string())
            .spawn({
                let event_loop = event_loop.clone();
                move || event_loop.run()
            })
            .ok().map(|jg| jg.thread().clone());
    }

}

pub struct Beeper {
    endpoint: cpal::Endpoint,
    format: cpal::Format,
}

impl Beeper {
    pub fn new() -> Beeper {
        let endpoint =  cpal::get_default_endpoint().expect("Failed to get default endpoint");
        Beeper {
            format : endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format"),
            endpoint : endpoint,
        }
    }

    pub fn build_beep(&self) -> BeepBuilder {
        BeepBuilder::new(&self.endpoint, &self.format)
    }
}

pub struct BeepBuilder {
    pub frequency: f32,
    pub duration: i32,
    endpoint: cpal::Endpoint,
    format: cpal::Format
}

impl BeepBuilder {
    fn new(endpoint: &cpal::Endpoint, format: &cpal::Format) -> BeepBuilder {
        BeepBuilder { frequency: 440.0, duration: 1000, endpoint: endpoint.clone(), format: format.clone() }
    }

    fn frequency(&mut self, frequency:f32) -> &mut BeepBuilder {
        self.frequency = frequency;
        self
    }

    fn duration(&mut self, duration:i32)  -> &mut BeepBuilder  {
        self.duration = duration;
        self
    }

    fn finalize(&self) ->  Beep {
        let event_loop = cpal::EventLoop::new();
        let (mut voice, stream) = cpal::Voice::new(&self.endpoint, &self.format, &event_loop).expect("Failed to create a voice");
        let beep = Beep { frequency: self.frequency, duration: self.duration, voice: voice, stream: stream, format: self.format };
        beep.start();
        beep

    }
}