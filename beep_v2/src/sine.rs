use std::time::Duration;
use std::vec::IntoIter as VecIntoIter;

use rodio::source::Source;

use rodio::Sample;

/// A buffer of samples treated as a source.
pub struct SineWave {
    start:u64,
    samples: u64,
    frequency: f32,
    duration: Duration
}

const SAMPLES_RATE : u32 = 8000;

impl SineWave {
    /// Builds a new `SineWave`.
    ///
    /// # Panic
    ///
    /// - Panics if the number of channels is zero.
    /// - Panics if the samples rate is zero.
    /// - Panics if the length of the buffer is superior to approximatively 16 billion elements.
    ///   This is because the calculation of the duration would overflow.
    ///
    pub fn new(frequency: f32, duration: Duration) -> SineWave
    {
        SineWave {
            start: 0,
            samples: (duration.as_secs() as u32 * SAMPLES_RATE) as u64,
            frequency : frequency,
            duration: duration
        }
    }
}

impl Source for SineWave {
    #[inline]
    fn get_current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn get_channels(&self) -> u16 {
        1
    }

    #[inline]
    fn get_samples_rate(&self) -> u32 {
        SAMPLES_RATE
    }

    #[inline]
    fn get_total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.start > self.samples {
            None
        } else {
            let sin_value = (self.start as f32 * 900.0 * 2.0 * 3.141592 / SAMPLES_RATE as f32).sin();
            self.start += 1;
            Some(sin_value)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
