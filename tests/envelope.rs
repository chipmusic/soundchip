use soundchip::{Envelope, Knot, EnvelopeState};

#[test]
fn envelope_test() {
    let mut env = Envelope {
        attack: Knot {
            time: 1.0,
            value: 1.0,
        },
        decay: Knot {
            time: 2.0,
            value: 0.5,
        },
        sustain: Knot {
            time: 3.0,
            value: 1.0,
        },
        release: Knot {
            time: 4.0,
            value: 0.0,
        },
        state: EnvelopeState::Attack,
    };

    let mut time = 0.0;
    let delta = 0.25;
    while time <= 5.0 {
        let volume = env.process(time);
        // println!("t:{:.2}, {:.2?}", time, volume);
        if time == 1.0 {
            assert_eq!(volume, 1.0);
        } else if time == 2.0 {
            assert_eq!(volume, 0.5);
        } else if time == 3.0 {
            assert_eq!(volume, 1.0);
        } else if time > 4.0 {
            assert_eq!(volume, 0.0);
        }
        time += delta;
    }
}
