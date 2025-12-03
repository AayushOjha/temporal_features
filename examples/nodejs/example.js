/**
 * Example usage of temporal_features in Node.js
 */

const { TemporalFeatures } = require('temporal_features');

function main() {
    // Create an instance
    const tf = new TemporalFeatures();

    // Use the add function (placeholder implementation)
    const result = tf.add(5, 3);
    console.log(`5 + 3 = ${result}`);

    // More examples
    const result2 = tf.add(100, 200);
    console.log(`100 + 200 = ${result2}`);
}

main();
