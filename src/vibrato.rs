//! This is my Vibrato.rs Rustdocs Page! It's literally the bestest html page ever
/// This is me explaining my Vibrato struct
/// So for this struct, I have a sample rate, frequency, depth, and time listings since we need
/// those. I set this up very similarly to the comb filter struct from Assignment 1, since I
/// really liked the structure of that one. I have new, reset, getter, setter, and process.

use std::f32::consts::PI;

use crate::lfo;

pub struct Vibrato {
    /// The current sample rate of the vibrato
    sample_rate_hz: f32, 
    /// The current freq (in hz) of the vibrato
    freq: f32,
    /// The current depth of the vibrato    
    depth: f32,
    /// The current time (in sec) of the vibrato   
    time: f32,

    /// The original sample rate of the vibrato
    og_sample_rate_hz: f32,
    /// The original freq of the vibrato
    og_freq: f32,
    /// The original depth of the vibrato
    og_depth: f32,
    /// The original time of the vibrato
    og_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Params {
    /// Used for selecting/peeking at the Sample Rate of the Vibrato effect
    SampleRate, 
    /// Used for selecting/peeking at the Frequency of the Vibrato effect (in Hz)
    Freq,
    /// Used for selecting/peeking at the Depth of the Vibrato effect
    Depth,
    /// Used for selecting/peeking at the duration of the Vibrato effect (in seconds)
    Time,
}

#[derive(Debug, Clone)]
pub enum Error {
    /// Helps with handling errors
    InvalidValue { param: Params, value: f32 }
}

impl Vibrato {

    /// Constructs a new Vibrato struct.
    /// * `sr` - The Sample Rate of the Vibrato
    /// * `hz` - The frequency of the Vibrato
    /// * `depth` - The depth of the Vibrato
    /// * `sec` - The time of the Vibrato
    /// # Examples
    ///
    /// ```
    /// let mut VibratoDemo = Vibrato::new(44100.0, 55.0, 1.0, 10.0);
    /// ```

    pub fn new(sr: f32, hz: f32, depth: f32, sec: f32) -> Self {
        Vibrato {
            sample_rate_hz: sr,
            freq: hz,
            depth: depth,
            time: 0.0,

            og_sample_rate_hz: sr,
            og_freq: hz,
            og_depth: depth,
            og_time: sec,
        }
    }

    /// Resets the Vibrato struct to original values
    ///
    /// # Examples
    ///
    /// ```
    /// let mut VibratoDemo = Vibrato::new(44100.0, 55.0, 1.0, 10.0);
    /// VibratoDemo.reset();
    /// ```

    pub fn reset(&mut self) {
        self.sample_rate_hz = self.og_sample_rate_hz;
        self.freq = self.og_freq;
        self.depth = self.og_depth;
        self.time = self.og_time;
    }

    /// Applies the current Vibrato Effect to the given wav file
    /// * `_lfo` - The lfo setting that we are using
    /// * `buffer` - A buffer to do operations with
    /// # Examples
    ///
    /// ```
    /// let mut VibratoDemo = Vibrato::new(44100.0, 55.0, 1.0, 0.0);
    /// let mut lfoDemo = LFO::new(44100.0, 24.0, 0.0)
    /// // This is where I will eventually update this with how to use process
    /// ```

    pub fn process(&mut self, _lfo: lfo::LFO, buffer: &mut [f32]) {
        let vibratoRate = (PI * 2.0 * self.freq) / self.sample_rate_hz;
        let lfoRate     = (PI * 2.0 * _lfo.freq) / _lfo.sample_rate_hz;
        for sample in buffer.iter_mut() {
            
            // Update the LFO Phase
            
            let lfoVal = _lfo.phase.sin();
            let vibrato = (self.time * vibratoRate).sin();
            let currentSample = self.depth * lfoVal * vibrato;

            *sample *= (1.0 + currentSample);

            self.time = self.time + (1.0 / self.sample_rate_hz);
        }
    }

    /// Sets a new param value for the Vibrato struct
    /// 
    /// * `param` - The selected element of the Vibrato to change
    /// * `value` - The value to replace the old one with
    ///
    /// # Examples
    ///
    /// ```
    /// let mut VibratoDemo = Vibrato::new(44100.0, 55.0, 1.0, 10.0);
    /// VibratoSetterGetter.set_param(Params::SampleRate, 22050.0).expect("Unable to set Param");
    /// VibratoSetterGetter.set_param(Params::Freq, 80.0).expect("Unable to set Param");    
    /// VibratoSetterGetter.set_param(Params::Depth, 2.0).expect("Unable to set Param");
    /// VibratoSetterGetter.set_param(Params::Time, 20.0).expect("Unable to set Param");
    /// ```
    pub fn set_param(&mut self, param: Params, value: f32) -> Result<(), Error> { 
        match param {
            Params::SampleRate => {
                if value > 0.0 {
                    self.sample_rate_hz = value;
                    return Ok(());              
                } else {
                    Err(Error::InvalidValue { param: param, value: value })
                }

            } Params::Freq => {
                if value > 0.0 {
                    self.freq = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            } Params::Depth => {
                if value > 0.0 {
                    self.depth = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            } Params::Time => {
                if value > 0.0 {
                    self.time = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            }
        }
    }

    /// Returns the current param value for the Vibrato struct
    ///
    /// * `param` - The selected element of the Vibrato to see
    /// # Examples
    ///
    /// ```
    /// let mut VibratoDemo = Vibrato::new(44100.0, 55.0, 1.0, 10.0);
    /// VibratoSetterGetter.get_param(Params::SampleRate);
    /// VibratoSetterGetter.get_param(Params::Freq);
    /// VibratoSetterGetter.get_param(Params::Depth);
    /// VibratoSetterGetter.get_param(Params::Time);
    /// 
    /// ```    

    pub fn get_param(&self, param: Params) -> f32 {
        match param {
            Params::SampleRate => {
                return self.sample_rate_hz;
            } Params::Freq => {
                return self.freq;
            } Params::Depth => {
                return self.depth;
            } Params::Time => {
                return self.time;
            }
        }
    }


}

/// This is my Vibrato.rs Test Explaination Page! It's literally the second bestest html page ever
mod tests {
    use super::*;

    /// Tests out the setters and getters and returns true if values are to be as expected
    #[test]
    fn test_setter_getter() {
        let mut VibratoSetterGetter = Vibrato::new(44100.0, 70.0, 1.0, 10.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::SampleRate), 44100.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Freq), 70.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Depth), 1.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Time), 10.0);

        VibratoSetterGetter.set_param(Params::SampleRate, 22050.0).expect("Unable to set Param");
        VibratoSetterGetter.set_param(Params::Freq, 80.0).expect("Unable to set Param");    
        VibratoSetterGetter.set_param(Params::Depth, 2.0).expect("Unable to set Param");
        VibratoSetterGetter.set_param(Params::Time, 20.0).expect("Unable to set Param");

        assert_eq!(VibratoSetterGetter.get_param(Params::SampleRate), 22050.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Freq), 80.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Depth), 2.0);
        assert_eq!(VibratoSetterGetter.get_param(Params::Time), 20.0);
    }
}



