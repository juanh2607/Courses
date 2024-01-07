// Set canvas size (where the road will be shown)
const canvas = document.getElementById("myCanvas");
canvas.width = 200;
// ctx = context
const ctx = canvas.getContext("2d");

const road = new Road(canvas.width/2, canvas.width*0.9);
const car = new Car(road.getLaneCenter(0), 100, 30, 50);

animate();

function animate() {
  car.update(road.borders);
  // Solving to issues at once: resizing if the window changes and resizing 
  // forces to clear the canvas.
  canvas.height = window.innerHeight;

  ctx.save();
  // Camera above the car view. The translation affects how the objects are being drawn
  ctx.translate(0, -car.y + canvas.height*0.7);
  road.draw(ctx);
  car.draw(ctx);
  
  ctx.restore(); // The canvas is still, the drawings move
  requestAnimationFrame(animate);
}

