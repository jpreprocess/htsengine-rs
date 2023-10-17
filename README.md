# htsengine-rs

Rust binding of hts_engine_API ([SourceForge](https://hts-engine.sourceforge.net))

## Example

```rs
use htsengine::HTSEngine;

let fullcontext_label = vec![
    /* fullcontext labels (String) */
];

let mut engine = HTSEngine::new();
engine.load(vec!["nitech_jp_atr503_m001.htsvoice".to_string()])?;
engine.set_sampling_frequency(48000);

let result = engine
    .synthesize(fullcontext_label)?
    .into_iter()
    .map(|d| {
        if *d < (i16::MIN as f64) {
            i16::MIN
        } else if *d > (i16::MAX as f64) {
            i16::MAX
        } else {
            *d as i16
        }
    })
    .collect();

let mut out_file = File::create("out.wav")?;
wav::write(
    wav::Header::new(wav::WAV_FORMAT_PCM, 1, 48000, 16),
    &wav::BitDepth::Sixteen(result),
    &mut out_file,
)?;
```

## Copyright Notice

See NOTICE file for license text.

- crates/htsengine-sys/src/bindings.rs
  - hts_engine_API
    - Copyright (c) 2001-2014 Nagoya Institute of Technology, Department of Computer Science
    - Copyright (c) 2001-2008 Tokyo Institute of Technology, Interdisciplinary Graduate School of Science and Engineering
- crates/htsengine-sys/build.rs
  - [VOICEVOX/open_jtalk-rs](https://github.com/VOICEVOX/open_jtalk-rs)
    - Copyright (c) 2022, qwerty2501

## License

MIT License
