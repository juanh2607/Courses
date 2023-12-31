// Set canvas size (where the road will be shown)
const canvas = document.getElementById("myCanvas");
canvas.width = 200;
// ctx = context
const ctx = canvas.getContext("2d");

const car = new Car(100, 100, 30, 50);
car.draw(ctx);

animate();

function animate() {
  car.update();
  // Solving to issues at once: resizing if the window changes and resizing 
  // forces to clear the canvas.
  canvas.height = window.innerHeight;
  car.draw(ctx);
  requestAnimationFrame(animate);
}

