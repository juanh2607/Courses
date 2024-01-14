class Road {
  /**
   * @param {number} x - Center of road's x axis 
   * @param {number} width - The width of the whole road
   * @param {number} laneCount 
   */
  constructor(x, width, laneCount=3) {
    this.x = x;
    this.width = width;
    this.laneCount = laneCount;
    // Some useful values
    this.left = x - width/2;
    this.right = x + width/2;
    // JavaScript provides and infinity value but says it works weird when drawing
    const infinity = 1000000;
    this.top = -infinity;
    this.bottom = infinity;
    // Borders
    const topLeft = {x: this.left, y: this.top};
    const topRight = {x: this.right, y: this.top};
    const bottomLeft = {x: this.left, y: this.bottom};
    const bottomRight = {x: this.right, y: this.bottom};
    this.borders = [
      // Add the line segments you want. You can create highways, curves, etc
      [topLeft, bottomLeft],
      [topRight, bottomRight]
    ];
  }

  /**
   * @param {CanvasRenderingContext2D} ctx 
   */
  draw(ctx) {
    ctx.lineWidth = 5;
    ctx.strokeStyle = "white";
    // Draw stripe lines
    for (var i = 1; i <= this.laneCount-1; i++) {
      const x = lerp(
        this.left,
        this.right, 
        i/this.laneCount
      );
      
      // 20 pixels of line and 20 of space
      ctx.setLineDash([20, 20]);

      // Draw right line of the lane
      ctx.beginPath();
      ctx.moveTo(x, this.top); // Move starting point of new path
      ctx.lineTo(x, this.bottom); // Draw from starting point to this one
      ctx.stroke(); // Actually draw the line
    }
    // Draw borders
    ctx.setLineDash([]);
    this.borders.forEach( border => {
      ctx.beginPath();
      ctx.moveTo(border[0].x, border[0].y);
      ctx.lineTo(border[1].x, border[1].y);
      ctx.stroke();
    });
  }

  /**
   * Get the x coordinate of the center of the specified lane
   * @param {number} laneIndex - Index that goes from left to right starting at 0
   */
  getLaneCenter(laneIndex) {
    const laneWidth = this.width / this.laneCount;
    return this.left + laneWidth/2 + 
      Math.min(laneIndex, this.laneCount-1) * laneWidth;
  }
}
