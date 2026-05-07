// Mini version of device-envoy audio_player's dyn Playable pattern.
//
// Trait-object idea:
// - Different concrete clip types can be mixed behind one interface:
//   `&dyn Playable`.
// - Runtime dispatch chooses the right implementation per clip.

trait Playable {
    fn play(&self);
}

struct PcmClip {
    samples: &'static [i16],
}

impl Playable for PcmClip {
    fn play(&self) {
        println!("PCM clip with {} samples", self.samples.len());
    }
}

struct ToneClip {
    hz: u16,
    ms: u16,
}

impl Playable for ToneClip {
    fn play(&self) {
        println!("Tone {} Hz for {} ms", self.hz, self.ms);
    }
}

fn play_sequence(clips: &[&dyn Playable]) {
    for clip in clips {
        clip.play();
    }
}

fn main() {
    static PCM_SAMPLES: [i16; 6] = [0, 1200, -900, 700, -400, 0];

    let pcm = PcmClip {
        samples: &PCM_SAMPLES,
    };
    let tone = ToneClip { hz: 440, ms: 250 };

    let sequence: [&dyn Playable; 2] = [&pcm, &tone];
    play_sequence(&sequence);
}
