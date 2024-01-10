// Set canvas size (where the road will be shown)
const canvas = document.getElementById("myCanvas");
canvas.width = 200;

const ctx = canvas.getContext("2d");

const road = new Road(canvas.width/2, canvas.width*0.9);
const car = new Car(road.getLaneCenter(0), 100, 30, 50, "KEYS");

const traffic = [
  new Car(road.getLaneCenter(1), -100, 30, 50, "DUMMY", 2)
];

animate();

function animate() {
  for (var i = 0; i < traffic.length; i++) {
    // TODO: por ahora esto puede ir en el segundo loop (no se xq lo hace asi)
    // For now traffic is not able to be damaged by other cars
    traffic[i].update(road.borders, []);
  }

  car.update(road.borders, traffic);
  // Solving to issues at once: resizing if the window changes and resizing 
  // forces to clear the canvas.
  canvas.height = window.innerHeight;

  ctx.save();
  // Camera above the car view. The translation affects how the objects are being drawn
  ctx.translate(0, -car.y + canvas.height*0.7);
  road.draw(ctx);

  for (var i = 0; i < traffic.length; i++) {
    traffic[i].draw(ctx, "blue");
  }

  car.draw(ctx, "black");
  
  ctx.restore(); // The canvas is still, the drawings move
  requestAnimationFrame(animate);
}

