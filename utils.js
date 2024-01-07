/**
 * Performs a linear interpolation between two values.
 *
 * @param {number} A
 * @param {number} B
 * @param {number} t - The interpolation factor, typically between 0 (returns A) and 1 (returns B).
 * @returns {number} - The interpolated value between A and B.
 */
function lerp(A, B, t) {
  return A + (B - A) * t;
}

 /**
   * @typedef {Object} Intersection
   * @property {number} offset - The distance from the origin
   * @property {Array<number>} coordinate - The coordinate of the touch
   */
 
/**
 * Find the intersection point between to line segments on a 2D plane.
 * @param {2dCoordinate} A - Beginning of first line
 * @param {2dCoordinate} B - Ending of first line
 * @param {2dCoordinate} C - Beginning of second line
 * @param {2dCoordinate} D - Ending of second line
 * @returns {Intersection} - Coordinate of intersecion with offset from point A
 */
function getIntersection(A, B, C, D) { 
  // Calculate t and u using parametric form of line equations.
  // Parameter t = a/b = tTop/bottom. Parameter u = c/b = uTop/bottom. 
  const tTop   = (D.x-C.x)*(A.y-C.y) - (D.y-C.y)*(A.x-C.x);
  const uTop   = (C.y-A.y)*(A.x-B.x) - (C.x-A.x)*(A.y-B.y);
  const bottom = (D.y-C.y)*(B.x-A.x) - (D.x-C.x)*(B.y-A.y);
  
  if(bottom != 0) {
    const t = tTop/bottom;
    const u = uTop/bottom;
    // If parameters are not in [0, 1], then the segments do not intersect
    if(t >= 0 && t <= 1 && u >= 0 && u <= 1) {
      return {
        x:lerp(A.x,B.x,t),
        y:lerp(A.y,B.y,t),
        offset:t
        }
    }
  }
  // If bottom = 0, then the lines are parallel
  return null;
}