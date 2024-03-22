/**
 * Object that knows where the car is (think of it as attached to the car) and
 * updates based on the properties of the car
 */
class Sensor {
  /**
   * @param {Car} car 
   */
  constructor(car) {
    this.car = car;
    
    this.rayCount = 5; // The sensor works by casting rays in different directions
    this.rayLength = 100; // The length determines how far can the ray "see"
    this.raySpread = Math.PI / 2; // The angle between the leftmost and rightmost ray
    
    this.rays = []; // Where the segment of the rays will be stored
    this.readings = []; // Stores for each ray if there is a border and how far 
  }

  /**
   * @param {Array<number>} roadBorders - [beginning, end] pairs
   * @param {Array<Car>} traffic
   */
  update(roadBorders, traffic) {
    this.#castRays();
    this.readings = [];
    for (var i = 0; i < this.rays.length; i++) { // Read Borders
      this.readings.push(
        this.#getReading(
          this.rays[i], 
          roadBorders,
          traffic
        )
      );
    }

  }

  /**
   * @param {CanvasRenderingContext2D} ctx 
   */
  draw(ctx) {
    for (var i = 0; i < this.rayCount; i++) {
      // Set ray end coordinate
      var end = this.rays[i][1]
      if (this.readings[i]) {
        end = this.readings[i];
      }
      
      ctx.beginPath();
      ctx.lineWidth = 2;
      ctx.strokeStyle = "yellow";
      ctx.moveTo(
        this.rays[i][0].x,
        this.rays[i][0].y,
      );
      ctx.lineTo(
        end.x,
        end.y
      );
      ctx.stroke();
      // Draw "blocked" part of the ray
      ctx.beginPath();
      ctx.lineWidth = 2;
      ctx.strokeStyle = "black";
      ctx.moveTo(
        this.rays[i][1].x,
        this.rays[i][1].y,
      );
      ctx.lineTo(
        end.x,
        end.y
      );
      ctx.stroke();
    }
  }

  /**
   * Checks if the ray comes into contact with the road border
   * @param {Array<number>} ray - Array with beginning and end coordinates 
   * @param {Array<Array<number>>} roadBorders - Each roadBorder has a beginning and end coordinate
   * @param {Array<Car>} traffic 
   * @returns {null|Intersection} Returns null if the ray doesn't touch any road border, 
   *  otherwise returns the nearest intersection
   */
  #getReading(ray, roadBorders, traffic) {
    var touches = [];

    for (var i = 0; i < roadBorders.length; i++) {
      const touch = getIntersection(
        ray[0],
        ray[1],
        roadBorders[i][0],
        roadBorders[i][1]
      );

      if (touch) {
        touches.push(touch);
      }
    }

    for (var i = 0; i < traffic.length; i++) {
      const poly = traffic[i].polygon;
      for (var j = 0; j < poly.length; j++) {
        const value = getIntersection(
          ray[0],
          ray[1],
          poly[j],
          poly[(j+1) % poly.length]
        );
        if (value) {
          touches.push(value);
        }
      }
    }

    if (touches.length == 0) {
      return null;
    } else {
      const offsets = touches.map(e => e.offset);
      // A single ray can touch many things. We care only for the nearest object
      const minOffset = Math.min(...offsets);
      return touches.find(e => e.offset == minOffset);
    }
  }

  #castRays() {
    this.rays = [];

    for (var i = 0; i < this.rayCount; i++) {
      // Calculate the angle of each individual ray. Imagine a line that goes
      // forward from the car (90 degrees) => the left extreme is going to be 
      // raySpread/2º degrees anti-clockwise and for the right extreme the same 
      // but clockwise (so negative degrees) 
      const rayAngle = lerp(
        this.raySpread/2,
        -this.raySpread/2,
        this.rayCount == 1 ? 0.5 : i / (this.rayCount - 1)
      ) + this.car.angle;
      // Points of the ray
      const start = {x: this.car.x, y: this.car.y};
      const end = {
        x: this.car.x - Math.sin(rayAngle) * this.rayLength,
        y: this.car.y - Math.cos(rayAngle) * this.rayLength
      };

      this.rays.push([start, end]);
    }
  }
}