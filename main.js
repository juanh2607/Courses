// Set canvas size (where the road will be shown)
const carCanvas = document.getElementById("carCanvas");
carCanvas.width = 200;

const networkCanvas = document.getElementById("networkCanvas");
networkCanvas.width = 300;

const carCtx = carCanvas.getContext("2d");
const networkCtx = networkCanvas.getContext("2d");

const road = new Road(carCanvas.width/2, carCanvas.width*0.9);
const car = new Car(road.getLaneCenter(1), 100, 30, 50, "AI");

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
  carCanvas.height = window.innerHeight;
  networkCanvas.height = window.innerHeight;

  carCtx.save();
  // Camera above the car view. The translation affects how the objects are being drawn
  carCtx.translate(0, -car.y + carCanvas.height*0.7);
  road.draw(carCtx);

  for (var i = 0; i < traffic.length; i++) {
    traffic[i].draw(carCtx, "blue");
  }

  car.draw(carCtx, "black");
  
  carCtx.restore(); // The canvas is still, the drawings move
  
  Visualizer.drawNetwork(networkCtx, car.brain);
  
  requestAnimationFrame(animate);
}

