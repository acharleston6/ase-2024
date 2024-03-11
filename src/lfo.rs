pub struct LFO {
    // revise later
    pub sample_rate_hz: f32, 
    pub freq: f32,
    pub phase: f32,

    og_sample_rate_hz: f32,
    og_freq: f32,
    og_phase: f32,
}

pub enum Params {
    SampleRate, 
    Freq,
    Phase,
}

pub enum Error {
    InvalidValue { param: Params, value: f32 }
}

impl LFO {
    pub fn new(sr: f32, hz: f32, theta: f32) -> Self {
        LFO {
            sample_rate_hz: sr,
            freq: hz,
            phase: theta,

            og_sample_rate_hz: sr,
            og_freq: hz,
            og_phase: theta,
        }
    }

    pub fn reset(&mut self) {
        self.sample_rate_hz = self.og_sample_rate_hz;
        self.freq = self.og_freq;
        self.phase = self.og_phase;
    }

    pub fn process() {

    }

    pub fn set_param(&mut self, param: Params, value: f32) -> Result<(), Error> { 
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
            } Params::Phase => {
                if true {
                    self.phase = value;
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
            } Params::Phase => {
                return self.phase;
            }
        }
    }
}