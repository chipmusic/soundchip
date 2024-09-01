pub struct SoundChip<const CHANNEL_COUNT:usize> {
    channels:[Channel; CHANNEL_COUNT],
    pub is_playing:bool,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
