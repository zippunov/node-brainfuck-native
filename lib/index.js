'use strict';
const bfck = require('../native');

module.exports = function (bfCode, inputString) {
    return bfck.runCode(bfCode, inputString)
};