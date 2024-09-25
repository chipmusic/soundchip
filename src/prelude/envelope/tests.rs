#[allow(unused)]
use crate::{
    Vec,
    prelude::{get_loop_position_f32, Envelope, Knot, LoopKind},
    math::lerp,
};

#[test]
fn envelope_tests() {
    // These tests will only work if each knot's time increases by exactly 1.0, and the delta time
    // is always divided by multiples of 2 (0.5, 0.25, etc)!
    // The point is using a completely different method to obtain the interpolated values,
    // but OH BOY what a pain to get the looping behavior to match.
    // Callling it done for now after wasting a day...

    fn generate_index(time:f32, loop_in:f32, loop_out:f32, repeat:bool, round_up:bool) -> usize {
        if time <= loop_in { return 0 };
        if time == loop_in { return loop_in as usize };
        if time == loop_out { return loop_out as usize };
        if !repeat && time > loop_out {
            return loop_out as usize
        }
        let result = get_loop_position_f32(time, loop_in, loop_out) - loop_in;
        // println!("New index: {:.1}, {:.1}, {:.1} => {:.1}", time, loop_in, loop_out, result);
        if round_up {
            result.ceil() as usize
        } else {
            result.floor() as usize
        }
    }

    fn generate(min_time: f32, min_value: f32, max_value: f32, len: usize) -> Envelope<f32> {
        let mut time = min_time;
        let env_knots: Vec<Knot<f32>> = (0..len)
            .map(|i| {
                let x = i as f32 / (len - 1) as f32;
                let value = lerp(min_value, max_value, x);
                time += 1.0;
                Knot::new(time, value)
            })
            .collect();
        Envelope::from(env_knots.as_slice())
    }

    fn test_envelope(env: Envelope<f32>, start_time: f32, end_time: f32, delta: f32) {
        // println!("\nTesting...");
        // println!("\nTesting... {:#.2?}", env);
        let mut env = env;
        let mut time = start_time;
        let first_knot = env.knots[0];
        let last_knot = env.knots[env.len() - 1];
        let repeat = env.loop_kind == LoopKind::Repeat;
        while time <= end_time {
            let a = generate_index(time, first_knot.time, last_knot.time, repeat, false);
            let b = generate_index(time + delta, first_knot.time, last_knot.time, repeat, true);
            let b = if b < a {  // Fixes incorrect interpolation wrapping around. Argh.
                last_knot.time as usize
            } else {
                b
            };
            let local_time = get_loop_position_f32(time, first_knot.time, last_knot.time);
            if b > env.len()-1 { break }    // cop-out!
            let a_time = env.knots[a].time;
            let goal = lerp(
                env.knots[a].value,
                env.knots[b].value,
                local_time - a_time
            );
            let value = env.peek(time);
            // println!(
            //     "t:{:.3} -> {:.3},  a:{},  b:{},  knot_a:{:#.3?}, knot_b:{:#.3?}, v:{:.3},  goal:{:.3}",
            //     time, local_time, a, b, env.knots[a], env.knots[b], value, goal
            // );
            assert!((value - goal).abs() < (f32::EPSILON * 2.0));
            time += delta;
        }
    }

    // let mut test = generate(1.0, 1.0, 0.0, 4).loop_kind(LoopKind::Repeat);
    // let mut i = 0.0;
    // while i < 10.0 {
    //     println!("{} => {}", i, test.peek(i));
    //     i += 0.25;
    // }

    let envelope_with_times_above_zero = generate(1.0, 1.0, 0.0, 7);
    let envelope_with_times_starting_at_zero = generate(0.0, 0.5, 0.0, 8);

    test_envelope(envelope_with_times_starting_at_zero.clone(), 0.0, 35.0, 0.125);
    test_envelope(envelope_with_times_above_zero.clone(), 0.0, 30.0, 0.25);
    test_envelope(envelope_with_times_starting_at_zero.set_loop(LoopKind::Repeat), 0.0, 15.0, 0.25);
    test_envelope(
        envelope_with_times_above_zero.set_loop(LoopKind::Repeat),
        0.0,
        15.0,
        0.25,
    );


}
