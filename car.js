/**
 * Object used to render car using CanvasRenderingContext2D
 */
class Car {
  /**
   * @param {number} x - Center of car in X axis
   * @param {number} y - Center of car in Y axis
   * @param {number} width 
   * @param {number} height 
   * @param {string} controlType - KEYS or DUMMY
   */
  constructor(x, y, width, height, controlType, maxSpeed = 3) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;
    this.polygon = null;

    this.speed = 0;
    this.acceleration = 0.2;
    this.maxSpeed = maxSpeed;
    this.friction = 0.05;
    this.angle = 0;

    this.damaged = false;

    this.useBrain = controlType == "AI";

    if (controlType != "DUMMY") {
      this.sensor = new Sensor(this);
      const controlNeurons = 4;
      this.brain  = new NeuralNetwork(
        // Set as many layers as you want in between. 
        [this.sensor.rayCount, 6, controlNeurons]
      );
    }
    
    this.controls = new Controls(controlType);
  }

  /**
   * Update the position of the car
   * @param {Array<number>} roadBorders - [beginning, end] pairs
   * @param {Array<Car>} - traffic
   */
  update(roadBorders, traffic) {
    if (!this.damaged) {
      this.#move();
      this.polygon = this.#createPolygon();
      this.damaged = this.#assessDamage(roadBorders, traffic);
    }
    if (this.sensor) {
      this.sensor.update(roadBorders, traffic);
      const offsets = this.sensor.readings.map(s =>
        // If the sensor is null, it sees nothing. If it isn't null, we want to
        // have small numbers when it is far away and close to one when it is near
        s == null ? 0 : 1 - s.offset
      );
      // The outputs are [up, left, right, down] with values of 0 or 1.
      const outputs = NeuralNetwork.feedForward(offsets, this.brain);
      
      if (this.useBrain) {
        this.controls.forward = outputs[0];
        this.controls.left    = outputs[1];
        this.controls.right   = outputs[2];
        this.controls.reverse = outputs[3];
      }
    }
  }

  /**
   * @param {CanvasRenderingContext2D} ctx 
   * @param {string} color
   */
  draw(ctx, color) {
    if (this.damaged) {
      ctx.fillStyle = "gray";
    } else {
      ctx.fillStyle = color;
    }

    ctx.beginPath();
    ctx.moveTo(this.polygon[0].x, this.polygon[0].y);

    for (var i = 1; i < this.polygon.length; i++) {
      ctx.lineTo(this.polygon[i].x, this.polygon[i].y);
    }
    
    ctx.fill();
    
    if (this.sensor) {
      this.sensor.draw(ctx);
    }
  }

  #move() {
    // Up and down controls
    if (this.controls.forward)
      this.speed += this.acceleration;
    if (this.controls.reverse)
      this.speed -= this.acceleration;
    // Set speed limits
    if (this.speed > this.maxSpeed)
      this.speed = this.maxSpeed;
    if (this.speed < -this.maxSpeed/2)
      this.speed = -this.maxSpeed/2;
    // Apply friction
    if (this.speed > 0)
      this.speed -= this.friction;
    if (this.speed < 0)
      this.speed += this.friction;
    // To avoid perpetual moving
    if (Math.abs(this.speed) < this.friction)
      this.speed = 0;
    // Flip controls (forward vs reverse)
    if (this.speed != 0) {
      const flip = this.speed > 0 ? 1 : -1;
      // Left and right controls
      if (this.controls.left) 
        this.angle += 0.03 * flip;
      if (this.controls.right)
        this.angle -= 0.03 * flip;
    }
    
    this.x -= this.speed * Math.sin(this.angle);
    this.y -= this.speed * Math.cos(this.angle);
  }

  #createPolygon() {
    const points = []; // Coordinates of the car's corners
    const radius = Math.hypot(this.width/2, this.height/2);
    const alpha  = Math.atan2(this.width, this.height);
    
    points.push({ // Top right point
      x: this.x - Math.sin(this.angle - alpha) * radius,
      y: this.y - Math.cos(this.angle - alpha) * radius
    });
    
    points.push({ // Top left point
      x: this.x - Math.sin(this.angle + alpha) * radius,
      y: this.y - Math.cos(this.angle + alpha) * radius
    });
    
    points.push({ // Bottom right point
      x: this.x - Math.sin(Math.PI + this.angle - alpha) * radius,
      y: this.y - Math.cos(Math.PI + this.angle - alpha) * radius
    });

    points.push({ // Bottom left point
      x: this.x - Math.sin(Math.PI + this.angle + alpha) * radius,
      y: this.y - Math.cos(Math.PI + this.angle + alpha) * radius
    });
    
    return points;
  }

  #assessDamage(roadBorders, traffic) {
    for (var i = 0; i < roadBorders.length; i++) {
      if (polysIntersect(this.polygon, roadBorders[i])) {
        return true;
      }
    }

    for (var i = 0; i < traffic.length; i++) {
      if (polysIntersect(this.polygon, traffic[i].polygon)) {
        return true;
      }
    }
    return false;
  }
}