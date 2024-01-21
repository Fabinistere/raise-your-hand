# Dodge Measure

As the *slight dodge right* wasn't enough, see code below.

```rust
let (mut vel_x, mut vel_y, potential_new_direction) = move_to_target();

// ...

// Force dodge over goal shortest path
// The walker will dodge by *their* right
if dodging_measure.is_dodging() {
    let running_vel = 1. * **speed * (PI / 4.).cos();

    (vel_x, vel_y, *direction) = match *direction {
        Direction::Top => (running_vel, vel_y, Direction::TopRight),
        Direction::Bot => (-running_vel, vel_y, Direction::BotLeft),
        Direction::Left => (vel_x, running_vel, Direction::TopLeft),
        Direction::Right => (vel_x, -running_vel, Direction::BotRight),
        Direction::TopLeft => (vel_x, vel_y + running_vel, Direction::Top),
        Direction::TopRight => (vel_x, vel_y + -running_vel, Direction::Right), // = (vel_x, 0.)
        Direction::BotLeft => (vel_x, vel_y + running_vel, Direction::Left), // = (vel_x, 0.)
        Direction::BotRight => (vel_x, vel_y + -running_vel, Direction::Bot),
    };
}
```

I had to force walkers to *hard dodge right*, see code below.
To simplify/optimize, I just reversed `vel_x` or `vel_y` when possible and understandable.

```rust
let (mut vel_x, mut vel_y, potential_new_direction) = move_to_target();

// ...

// Force dodge over goal shortest path
// The walker will dodge by *their* right
if dodging_measure.is_dodging() {
    let running_vel = 1. * **speed * (PI / 4.).cos();

    (vel_x, vel_y, *direction) = match *direction {
        Direction::Top => (running_vel, 0., Direction::Right),
        Direction::Bot => (-running_vel, 0., Direction::Left),
        Direction::Left => (0., running_vel, Direction::Top),
        Direction::Right => (0., -running_vel, Direction::Bot),
        Direction::TopLeft => (-vel_x, vel_y, Direction::TopRight),
        Direction::TopRight => (vel_x, -vel_y, Direction::BotRight),
        Direction::BotLeft => (vel_x, -vel_y, Direction::TopLeft),
        Direction::BotRight => (-vel_x, vel_y, Direction::Bot),
    };
}
```
