# TODO

## sooner

- [x] add egui inspector
- [x] draw player - from existing
- [x] draw some circles
- [x] move circles down over time ("falling" aka player moving upward)
- [ ] collisions
  - [x] try [xpbd](https://github.com/Jondolf/bevy_xpbd)
  - [ ] handle collisions
- [ ] accrete circles to player
- [ ] remove [gravity](https://docs.rs/bevy_xpbd_2d/latest/bevy_xpbd_2d/resources/struct.Gravity.html) in physics simulation (top down)
- [ ] player movement
- [ ] add player rotation, so circles accrete all over
- [x] spawn Goo randomly, on timer
- [ ] allow easily tweaking some gameplay params (fall speed, etc) via Bevy EGUI

## later

### Gameplay Ideas

- accrete circles to each other?! not just player
- add 2p mode
- make a great character controller https://github.com/PROMETHIA-27/bevy_mod_wanderlust

### Other

- Run WASM build
- add gamepad support
