/**
 * This class provides methods to visualize a neural network.
 */
class Visualizer {
  /**
   * @param {CanvasRenderingContext2D} ctx 
   * @param {NeuralNetwork} network 
   */
  static drawNetwork(ctx, network) {
    const margin = 50;
    const left   = margin; 
    const top    = margin;
    const width  = ctx.canvas.width - margin*2;
    const height = ctx.canvas.height - margin*2; 
  
    const levelHeight = height / network.levels.length;

    // Draw each level of the network
    for(let i = network.levels.length-1; i >= 0; i--) {
      const ratio = network.levels.length === 1 ? 0.5 : i / (network.levels.length - 1);
      const levelTop = top + lerp(height - levelHeight, 0, ratio);

      ctx.setLineDash([7, 3]);
      Visualizer.drawLevel(ctx, network.levels[i],
        left, levelTop, width, levelHeight,
        i == network.levels.length-1
            ? ['🠉','🠈','🠊','🠋']
            : []
      );
    }
  }

  /**
   * Draws a level of the neural network
   * @param {CanvasRenderingContext2D} ctx 
   * @param {Level} level 
   * @param {number} left - Left position of the level
   * @param {number} top - Top position of the level
   * @param {number} width - Width of the level
   * @param {number} height - Height of the level
   * @param {Array<string>} outputLabels - Labels for the output nodes
   */
  static drawLevel(ctx,level,left,top,width,height,outputLabels){
    const right  = left + width;
    const bottom = top + height;
    const {inputs, outputs, weights, biases} = level;

    // Draw the connections between nodes
    for(let i = 0; i < inputs.length; i++) {
      for(let j = 0; j < outputs.length; j++) {
        ctx.beginPath();
        ctx.moveTo(Visualizer.#getNodeX(inputs, i, left, right), bottom);
        ctx.lineTo(Visualizer.#getNodeX(outputs, j, left, right), top);
        ctx.lineWidth = 2;
        ctx.strokeStyle = getRGBA(weights[i][j]);
        ctx.stroke();
      }
    }

    const nodeRadius = 18;
    // Draw the input nodes
    for(let i = 0; i < inputs.length; i++) { 
      const x = Visualizer.#getNodeX(inputs, i, left, right);
      ctx.beginPath();
      ctx.arc(x, bottom, nodeRadius, 0, Math.PI*2);
      ctx.fillStyle = "black";
      ctx.fill();
      ctx.beginPath();
      ctx.arc(x, bottom, nodeRadius*0.6, 0, Math.PI*2);
      ctx.fillStyle = getRGBA(inputs[i]);
      ctx.fill();
    }
    // Draw the output nodes
    for(let i = 0; i < outputs.length; i++) {
      const x = Visualizer.#getNodeX(outputs, i, left, right);
      ctx.beginPath();
      ctx.arc(x, top, nodeRadius, 0, Math.PI*2);
      ctx.fillStyle = "black";
      ctx.fill();
      ctx.beginPath();
      ctx.arc(x, top, nodeRadius*0.6, 0, Math.PI*2);
      ctx.fillStyle = getRGBA(outputs[i]);
      ctx.fill();
      // Draw the bias indicator (pointed line surrounding node)
      ctx.beginPath();
      ctx.lineWidth = 2;
      ctx.arc(x, top, nodeRadius*0.8, 0, Math.PI*2);
      ctx.strokeStyle = getRGBA(biases[i]);
      ctx.setLineDash([3,3]);
      ctx.stroke();
      ctx.setLineDash([]);

      // Draw the output labels
      if(outputLabels[i]) {
        ctx.beginPath();
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillStyle = "black";
        ctx.strokeStyle = "white";
        ctx.font = (nodeRadius*1.5) + "px Arial";
        ctx.fillText(outputLabels[i], x, top+nodeRadius*0.1);
        ctx.lineWidth = 0.5;
        ctx.strokeText(outputLabels[i], x, top+nodeRadius*0.1);
      }
    }
  }

  /**
   * Calculates the x position of a node.
   * @param {Array<number>} nodes 
   * @param {number} index - Index of the node 
   * @param {number} left - Left position of the level 
   * @param {number} right - Right position of the level
   * @returns {number}
   */
  static #getNodeX(nodes, index, left, right){
    return lerp(
      left,
      right,
      nodes.length == 1
      ? 0.5
      : index / (nodes.length-1)
    );
  }
}