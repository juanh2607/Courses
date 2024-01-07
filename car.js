/**
 * Object used to render car using CanvasRenderingContext2D
 */
class Car {
  /**
   * @param {number} x - Center of car in X axis
   * @param {number} y - Center of car in Y axis
   * @param {number} width 
   * @param {number} height 
   */
  constructor(x, y, width, height) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;

    this.speed = 0;
    this.acceleration = 0.2;
    this.maxSpeed = 3;
    this.friction = 0.05;
    this.angle = 0;

    this.controls = new Controls();
    this.sensor = new Sensor(this);
  }

  /**
   * Update the position of the car
   */
  update(roadBorders) {
    this.#move();
    this.sensor.update(roadBorders);
  }

  /**
   * @param {CanvasRenderingContext2D} ctx 
   */
  draw(ctx) {
    
    // Save current state because we will translate and rotate the canvas for 
    // this particular draw.
    ctx.save(); 
    // Rotate car by rotating first the canvas and then drawing the car
    ctx.translate(this.x, this.y); // Center the context where we have the car
    ctx.rotate(-this.angle);

    ctx.beginPath();
    ctx.rect( // Already at center of car thanks to ctx.translate
      -this.width/2,
      -this.height/2,
      this.width,
      this.height
    );
    ctx.fill();
    ctx.restore(); // Otherwise we translate and rotate on each frame
    
    this.sensor.draw(ctx);
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
}