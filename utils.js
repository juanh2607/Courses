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