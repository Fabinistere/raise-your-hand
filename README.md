# Raise your hand!

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf#license)

## Preview

## The game

### Game Pitch

You must reach your friend in a public area.
But the crowd is not making things easy for you...

### More details

First you have to spot your friend, second you have to run towards them.
Problem, there is some people / map terrain (a city environment for example) blocking you.
In the early stages there is few crowd and obstacle but the further you reach level the more you get glued and stuck by the "hostile" environment.

### Ideas

1. **Crowd Dynamics:**
   - Implement realistic crowd behavior. The crowd should have varying densities, and people should react dynamically to the player's movements. This can add an extra layer of challenge, requiring players to navigate through shifting crowds.

2. **Obstacle Variety:**
   - Introduce a variety of obstacles in the environment. These could include street vendors, street performers, construction sites, and other city elements. Each obstacle type could have a unique effect on the player, such as slowing them down or temporarily blocking their path.

3. **Power-Ups:**
   - Include power-ups that help the player navigate the crowd more efficiently. For example, a speed boost to dash through tight spaces, a temporary shield to avoid obstacles, or a crowd-clearing ability to create a path.

4. **Map Variety:**
   - Create diverse maps with different themes and challenges. A city environment could be just one setting; consider adding levels in crowded markets, busy train stations, or music festivals, each with its own set of obstacles and crowd dynamics.

5. **Stealth Mechanics:**
   - Introduce stealth elements where the player can blend into the crowd to avoid attention. However, staying in the crowd for too long might slow down progress, so there's a balance between blending in and making a direct approach.

6. **Dynamic Difficulty Scaling:**
   - Implement a system that adjusts the difficulty based on the player's performance. If they're progressing well, increase the crowd density and obstacle complexity. If they're struggling, provide temporary aids or reduce the challenge slightly.

7. **Multiplayer Mode:**
   - Consider adding a multiplayer mode where players can compete against each other to reach the target entity first. This could introduce a competitive element, encouraging players to strategize and outmaneuver their opponents.

8. **Storyline Integration:**
   - Weave a storyline into the game. Perhaps the friend the player is trying to reach is in a hurry, and the urgency of the situation adds pressure to the gameplay. The narrative can drive the player's motivation and create a sense of connection to the objective.

### Todos

#### Must-Have

- [ ] Player
  - [x] Basic Player movement

## Contribute

Release's format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This project also follows to [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/).

## Assets are excluded from git storage

Due to the inefficiency to store image in git,
all asset can be download here (from the latest version):
<!-- [Latest Assets - Google Drive](???) -->

or in the correct release note (if from other version):
<!-- [Releases - Github](???) -->

## License

This project is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

See the very good reasons for including both [here](https://github.com/bevyengine/bevy/issues/2373).
