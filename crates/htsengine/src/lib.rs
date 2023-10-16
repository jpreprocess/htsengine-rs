use std::mem::MaybeUninit;

pub struct HTSEngine {
    inner: htsengine_sys::HTS_Engine,
}

impl HTSEngine {
    pub fn new() -> Self {
        unsafe {
            let mut htsengine = MaybeUninit::<htsengine_sys::HTS_Engine>::uninit();
            htsengine_sys::HTS_Engine_initialize(htsengine.as_mut_ptr());
            Self {
                inner: htsengine.assume_init(),
            }
        }
    }
}

impl Drop for HTSEngine {
    fn drop(&mut self) {
        unsafe { htsengine_sys::HTS_Engine_clear(&mut self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use crate::HTSEngine;

    #[test]
    fn load() {
        let _engine = HTSEngine::new();
    }
}
