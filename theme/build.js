module.exports = ({config}) => {
  const StyleDictionary = require('style-dictionary').extend(config);
  StyleDictionary.buildAllPlatforms();
}