# TODO

## sooner

- Get Win/loss conditions working
  - [ ] game over: no active player orbs
    - [ ] player health? Or just number of remaining orbs
  - [ ] game win: push N purple balls into the goal area(s)
- Get basic behaviors
  - [ ] construction goo makes a defense layer
    - [ ] attaches to either: player OR goo that's attached to player
  - [ ] destruction goo damages the hero
- [ ] spawn goo into the level from edges
- [..] get a WASM build working and ship to itch. MVP!
  - tried but `trunk serve` is showing a blank screen.. start again from template and verify, then build up?
- [ ] improve destruction - remove joints too
- [..] disappear over time
- [ ] add player rotation, so circles accrete all over
- [ ] allow easily tweaking some gameplay params (fall speed, etc) via Bevy EGUI

## later

### Mechanics-related

- accrete circles to each other?! not just player
  - experimented a lil here
- add 2p mode
- make a great character controller https://github.com/PROMETHIA-27/bevy_mod_wanderlust

### Game (non-Mechanics)

- Run WASM build
- add gamepad support

### Learning Bevy

https://bevy-cheatbook.github.io/tutorial/guide.html -- work through the concepts here.. what's unfamiliar? learn a thing

### Learning Rust

- get a nice automated "organize imports" commands in VSCode (cleanup all unused) ... maybe via Rust clippyr
