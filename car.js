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

    this.controls = new Controls();
  }

  /**
   * Update the position of the car
   */
  update() {
    if(this.controls.forward)
      this.y -= 2;
    if (this.controls.reverse)
      this.y += 2;
  }

  /**
   * @param {CanvasRenderingContext2D} ctx 
   */
  draw(ctx) {
    ctx.beginPath();
    ctx.rect(
      this.x - this.width/2,
      this.y - this.height/2,
      this.width,
      this.height
    );
    ctx.fill();
  }
}