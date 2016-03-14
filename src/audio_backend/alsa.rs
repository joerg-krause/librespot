use super::{Open, Sink};
use std::io;
use alsa::{PCM, Stream, Mode, Format, Access};

pub struct AlsaSink(PCM);

impl Open for AlsaSink {
   fn open() -> AlsaSink {
        println!("Using AlsaSink");

        let pcm = PCM::open("default", Stream::Playback, Mode::Blocking,
                            Format::Signed16, Access::Interleaved, 2, 44100).ok().unwrap();
        /*
        let mut buf = [0f32; 44100];
        for (idx, sample) in buf.iter_mut().enumerate() {
            let phase = (idx as f32) / 100.0 * PI * 2.0;
            *sample = phase.sin();
        }
        pcm.write_interleaved(&buf).unwrap();
        */
        AlsaSink(pcm)
    }
}

impl Sink for AlsaSink {
    fn start(&self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&self) -> io::Result<()> {
        Ok(())
    }

    fn write(&self, data: &[i16]) -> io::Result<()> {
		
        self.0.write_interleaved(&data).unwrap();

        Ok(())
    }
}
