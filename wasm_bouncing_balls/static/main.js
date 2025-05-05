import init, { World } from "../pkg/wasm_bouncing_balls.js";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
let world;

init().then(() => {
  world = new World(canvas.width, canvas.height, 30);

  function animate() {
    world.update();
    world.draw(ctx);
    requestAnimationFrame(animate);
  }

  animate();
});