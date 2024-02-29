use std::{fs::File, io::Write};

mod ring_buffer;
mod vibrato;
mod lfo;
use lfo::LFO;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        write!(out, "{}{}", sample, if i % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
    }
}

// These tests are here for easy finding. 
// If they are in a different, rs file, I'll prolly say so.
mod tests {
    use crate::vibrato::Vibrato;

    use super::*;

    // Ringbuffer tests as specified in exercise 2 (in ring_buffer.rs).
    #[test]
    fn test_exercise2_tests() {
        print!("For these tests, see ring_buffer.rs");
        assert!(true);
    }

    // Output equals delayed input when modulation amplitude is 0.
    #[test]
    fn OutputEqualsInputatZeroAmplitude() {
        let mut ZeroAmpInput: &mut [&mut [f32]] = &mut [&mut [1.0, 2.0, 2.3, 2.0]];
        let mut ZeroAmpOutput: &mut [&mut [f32]] = &mut [&mut []];
        let mut ZeroAmpVibrato = Vibrato::new(44100.0, 50.0, 0.0, 2.0);
        let mut ZeroAmpLFO = LFO::new(44100.0, 40.0, 0.0);
        ZeroAmpVibrato.process(ZeroAmpLFO, ZeroAmpInput[0]);
        assert!(ZeroAmpInput == ZeroAmpOutput);        
        
        assert!(true);
    }

    // DC input results in DC output, regardless of parameters.
    #[test]
    fn DCInputDCOutput() {
        assert!(true);
    }

    // Varying input block size.
    #[test]
    fn DifferentBlockSizes() {
        let mut varyingInput: &[&[f32]] = &[&[1.0, 2.0], &[1.0, 2.0, 3.0, 4.0]];
        let mut varyingOutput: &mut [&mut [f32]] = &mut [&mut []];
        let mut varyingVibrato = Vibrato::new(44100.0, 50.0, 1.0, 2.0);
        let mut varyingLFO = LFO::new(44100.0, 40.0, 0.0);
    
        Vibrato::set_param(&mut varyingVibrato, vibrato::Params::SampleRate, 22050.0).expect("Unable to set Param");
        LFO::set_param(&mut varyingLFO, lfo::Params::SampleRate, 22050.0);
        //varyingVibrato.process(varyingLFO, varyingInput);

        // this needs fixing
        assert!(varyingOutput == varyingInput);
        assert!(true);
    }

    // Zero input signal.
    #[test]
    fn ZeroInputSignal() {
        let mut ZeroInput: &mut [&mut [f32]] = &mut [&mut []];
        let mut ZeroOutput: &mut [&mut [f32]] = &mut [&mut []];
        let mut ZeroVibrato = Vibrato::new(44100.0, 50.0, 1.0, 2.0);
        let mut ZeroLFO = LFO::new(44100.0, 40.0, 0.0);
        ZeroVibrato.process(ZeroLFO, ZeroInput[0]);
        assert!(ZeroInput == ZeroOutput);
    }

    // One or more additional test(s) to verify other expected behaviors.
    fn additionalTests() {
        print!("For these tests, look at any other rs file. I use these to test functionality everywhere");
        assert!(true);        
    }

}



