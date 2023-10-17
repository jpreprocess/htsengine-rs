use std::{ffi::CString, mem::MaybeUninit};

pub struct HTSEngine {
    inner: htsengine_sys::HTS_Engine,
    model: Option<Vec<i8>>,
}

#[derive(PartialEq, Debug, thiserror::Error)]
pub enum HTSEngineError {
    #[error("model loading failed")]
    LoadError,
    #[error("model type is not HTS_TTS_JPN")]
    ModelTypeError,
    #[error("the string contains \\0")]
    CStringError,
    #[error("synthesis failed")]
    SynthesisError,
}

type HTSEngineResult<T> = Result<T, HTSEngineError>;

impl HTSEngine {
    pub fn new() -> Self {
        unsafe {
            let mut htsengine = MaybeUninit::<htsengine_sys::HTS_Engine>::uninit();
            htsengine_sys::HTS_Engine_initialize(htsengine.as_mut_ptr());
            Self {
                inner: htsengine.assume_init(),
                model: None,
            }
        }
    }
    pub fn load(&mut self, mut model: Vec<i8>) -> HTSEngineResult<()> {
        let result =
            unsafe { htsengine_sys::HTS_Engine_load(&mut self.inner, &mut model.as_mut_ptr(), 1) };
        if result != 1 {
            return Err(HTSEngineError::LoadError);
        }

        let format = unsafe {
            let ptr = htsengine_sys::HTS_Engine_get_fullcontext_label_format(&mut self.inner);
            CString::from_raw(ptr as *mut i8)
        };
        if !matches!(format.to_str(), Ok("HTS_TTS_JPN")) {
            return Err(HTSEngineError::ModelTypeError);
        }

        self.model = Some(model);
        Ok(())
    }

    pub fn set_sampling_frequency(&mut self, i: usize) {
        unsafe { htsengine_sys::HTS_Engine_set_sampling_frequency(&mut self.inner, i) }
    }
    pub fn set_fperiod(&mut self, i: usize) {
        unsafe { htsengine_sys::HTS_Engine_set_fperiod(&mut self.inner, i) }
    }
    pub fn set_alpha(&mut self, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_alpha(&mut self.inner, f) }
    }
    pub fn set_beta(&mut self, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_beta(&mut self.inner, f) }
    }
    pub fn set_speed(&mut self, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_speed(&mut self.inner, f) }
    }
    pub fn add_half_tone(&mut self, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_add_half_tone(&mut self.inner, f) }
    }
    pub fn set_msd_threshold(&mut self, i: usize, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_msd_threshold(&mut self.inner, i, f) }
    }
    pub fn set_gv_weight(&mut self, i: usize, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_gv_weight(&mut self.inner, i, f) }
    }
    pub fn set_volume(&mut self, f: f64) {
        unsafe { htsengine_sys::HTS_Engine_set_volume(&mut self.inner, f) }
    }
    pub fn set_audio_buff_size(&mut self, i: usize) {
        unsafe { htsengine_sys::HTS_Engine_set_audio_buff_size(&mut self.inner, i) }
    }

    pub fn synthesize(&mut self, fullcontext_labels: Vec<String>) -> HTSEngineResult<Vec<f64>> {
        let mut labels: Vec<*mut i8> = fullcontext_labels
            .into_iter()
            .map(|l| CString::new(l).map(|l| l.into_raw()))
            .collect::<Result<_, _>>()
            .map_err(|_| HTSEngineError::CStringError)?;
        let result = unsafe {
            htsengine_sys::HTS_Engine_synthesize_from_strings(
                &mut self.inner,
                labels.as_mut_ptr(),
                labels.len(),
            )
        };
        if result != 1 {
            return Err(HTSEngineError::SynthesisError);
        }

        let sample_count = unsafe { htsengine_sys::HTS_Engine_get_nsamples(&mut self.inner) };
        let mut buffer = Vec::with_capacity(sample_count);
        for index in 0..sample_count {
            let value =
                unsafe { htsengine_sys::HTS_Engine_get_generated_speech(&mut self.inner, index) };
            buffer.push(value);
        }

        unsafe {
            htsengine_sys::HTS_Engine_refresh(&mut self.inner);
        }

        Ok(buffer)
    }
}

impl Drop for HTSEngine {
    fn drop(&mut self) {
        unsafe { htsengine_sys::HTS_Engine_clear(&mut self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HTSEngine, HTSEngineError};

    #[test]
    fn new() {
        let _engine = HTSEngine::new();
    }

    #[test]
    fn synthesize_too_early() {
        let mut engine = HTSEngine::new();
        let result = engine.synthesize(vec!["a".to_string()]);
        assert_eq!(result, Err(HTSEngineError::SynthesisError));
    }
}
