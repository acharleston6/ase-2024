//! This is my Vibrato.rs Rustdocs Page! It's literally the bestest html page ever
/// This is me explaining my Vibrato struct
pub struct Vibrato {
    // Put Code Heres``
    sample_rate_hz: f32, 
    freq: f32,
    depth: f32,
    time: f32,

    og_sample_rate_hz: f32,
    og_freq: f32,
    og_depth: f32,
    og_time: f32,
}


pub enum Params {
    SampleRate, 
    Freq,
    Depth,
    Time,
}

pub enum Error {
    // Placeholder for later
    InvalidValue { param: Params, value: f32 }
}

impl Vibrato {
    pub fn new(sr: f32, hz: f32, depth: f32, sec: f32) -> Self {
        Vibrato {
            sample_rate_hz: sr,
            freq: hz,
            depth: depth,
            time: sec,

            og_sample_rate_hz: sr,
            og_freq: hz,
            og_depth: depth,
            og_time: sec,
        }
    }

    pub fn reset(&mut self) {
        self.sample_rate_hz = self.og_sample_rate_hz;
        self.freq = self.og_freq;
        self.depth = self.og_depth;
        self.time = self.og_time;
    }

    pub fn process() {

    }

    pub fn set_param(&mut self, param: Params, value: f32) -> Result<(), Error> { 
        // fix gain stuff
        match param {
            Params::SampleRate => {
                if true {
                    self.sample_rate_hz = value;
                    return Ok(());              
                } else {
                    Err(Error::InvalidValue { param: param, value: value })
                }

            } Params::Freq => {
                if true {
                    self.freq = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            } Params::Depth => {
                if true {
                    self.depth = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            } Params::Time => {
                if true {
                    self.time = value;
                    return Ok(());
                } else {
                    Err(Error::InvalidValue { param: param, value: value })                    
                }
            }
        }
    }

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