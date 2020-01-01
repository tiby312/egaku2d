## Overview

A library that lets you draw various simple 2d geometry primitives fast using a single
shader program and a single vertex buffer object with a safe api (provided no other libray
is calling opengl functions).

## Screenshot

<img src="./assets/screenshot.gif" alt="screenshot">


## Example

# Example

```rust
use axgeom::*;
let events_loop = glutin::event_loop::EventLoop::new();
let mut glsys = very_simple_2d::WindowedSystem::newp(600, 480, &events_loop,"test window");

let mut canvas = glsys.canvas_mut();

//Make the background dark gray.
canvas.clear_color([0.2,0.2,0.2]);

//Push some squares to a static vertex buffer object on the gpu.
let rect_save = canvas.squares(5.0)
   .addp(40., 40.)
   .addp(40., 40.)
   .save();

//Draw the squares we saved.
rect_save.draw(&mut canvas,[0.0, 1.0, 0.1, 0.5]);

//Draw some arrows.
canvas.arrows(5.0)
  .add(vec2(40., 40.), vec2(40., 200.))
  .add(vec2(40., 40.), vec2(200., 40.))
  .send_and_draw([0.0, 1.0, 0.1, 0.5]);

//Draw some circles.
canvas.circles(4.0)
  .add(vec2(5.,6.))
  .add(vec2(7.,8.))
  .add(vec2(9.,5.))
  .send_and_draw([0., 1., 1., 0.1]);

//Draw some circles from f32 primitives.
canvas.circles(4.0)
  .addp(5.,6.)
  .addp(7.,8.)
  .addp(9.,5.)
  .send_and_draw([0., 1., 1., 0.1]);

//Swap buffers on the opengl context.
glsys.swap_buffers();
```