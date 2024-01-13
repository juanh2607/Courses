class NeuralNetwork {
  /**
   * @param {Array<number>} neuronCounts - Number of neurons on each layer
   */
  constructor(neuronCounts) {
    this.levels = [];
    for (var i = 0; i < neuronCounts.length-1; i++) {
      this.levels.push(new Level(
        neuronCounts[i], neuronCounts[i+1]
      ));
    }
  }

  /**
   * 
   * @param {*} givenInputs 
   * @param {NeuralNetwork} network 
   */
  static feedForward(givenInputs, network) {
    var outputs = Level.feedForward(
      givenInputs, network.levels[0]
    );
    for (var i = 1; i < network.levels.length; i++) {
      outputs = Level.feedForward(
        outputs, network.levels[i]
      );
    }
    
    return outputs;
  }
}

/**
 * Level has a layer of input neurons("floor") and a layer of output neurons 
 * ("ceiling")
 */
class Level {
  /**
   * @param {number} inputCount 
   * @param {number} outPutCount 
   */
  constructor(inputCount, outPutCount) {
    this.inputs  = new Array(inputCount);
    this.outPuts = new Array(outPutCount);
    // Each output neuron has a bias: it's the value above which it will fire
    this.biases  = new Array(outPutCount); 
    // Weight for each input - output connection. It is a value between -1 and 1
    // We use positive and negative values to express "turn or don't turn to right"
    // where "don't turn" means "turn left"
    this.weights = [];
    // For each input node we have an output amount of connections
    for (var i = 0; i < inputCount; i++) {
      this.weights[i] = new Array(outPutCount);
    }
    // For now, we will begin with a random brain
    Level.#randomize(this);
  }

  // Set to static in order to serialize this (methods don't serialize)
  /**
   * Used to randomize the values of the weights and biases
   * @param {Level} level 
   */
  static #randomize(level) {
    for (var i = 0; i < level.inputs.length; i++) {
      for (var j = 0; j < level.outPuts.length; j++) {
        level.weights[i][j] = Math.random() * 2 - 1; // [-1, 1]
      }
    }

    for (var i = 0; i < level.biases.length; i++) {
      level.biases[i] = Math.random() * 2 - 1;
    }
  }

  /**
   * Algorithm used to compute the output values
   * @param {*} givenInputs - Values that come from the car Sensor
   * @param {Level} level 
   */
  static feedForward(givenInputs, level) {
    for (var i = 0; i < level.inputs.length; i++) {
      level.inputs[i] = givenInputs[i];
    }

    // Calculate a sum between the value of the inputs and the weight
    for (var i = 0; i < level.outPuts.length; i++) {
      var sum = 0;
      for (var j = 0; j < level.inputs.length; j++) {
        sum += level.inputs[j] * level.weights[j][i];
      }

      if (sum > level.biases[i]) { // Turn on the output
        level.outPuts[i] = 1; 
      } else { // Turn off the output
        level.outPuts[i] = 0;
      }
    }

    return level.outPuts;
  }
}