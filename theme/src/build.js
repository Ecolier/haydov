const { kebabCase, camelCase, snakeCase, upperFirst } = require('lodash');
const {getTokenNameFromProperty} = require('./utils');
const StyleDictionary = require('style-dictionary');

StyleDictionary.registerTransform({
  name: 'name/dsp/kebab',
  type: 'name',
  matcher: function (prop) {
    return true;
  },
  transformer: function (prop) {
    return kebabCase(getTokenNameFromProperty(prop));
  },
});

StyleDictionary.registerTransform({
  name: 'name/dsp/camel',
  type: 'name',
  matcher: function (prop) {
    return true;
  },
  transformer: function (prop) {
    return camelCase(getTokenNameFromProperty(prop));
  },
});

StyleDictionary.registerTransform({
  name: 'name/dsp/snake',
  type: 'name',
  matcher: function (prop) {
    return true;
  },
  transformer: function (prop) {
    return snakeCase(getTokenNameFromProperty(prop));
  },
});

StyleDictionary.registerTransform({
  name: 'name/dsp/pascal',
  type: 'name',
  matcher: function (prop) {
    return true;
  },
  transformer: function (prop) {
    return upperFirst(camelCase(getTokenNameFromProperty(prop)));
  },
});

StyleDictionary.registerTransformGroup({
  name: 'css',
  transforms: [
    'attribute/cti',
    'name/dsp/kebab', 
    'time/seconds',
    'content/icon',
    'size/rem',
    'color/css',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'scss',
  transforms: [
    'attribute/cti',
    'name/dsp/kebab',
    'time/seconds',
    'content/icon',
    'size/rem',
    'color/css',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'android',
  transforms: [
    'attribute/cti',
    'name/dsp/snake', 
    'color/hex8android',
    'size/remToSp',
    'size/remToDp',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'ios',
  transforms: [
    'attribute/cti',
    'name/dsp/pascal',
    'color/UIColor',
    'content/objC/literal',
    'asset/objC/literal',
    'size/remToPt',
    'font/objC/literal',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'ios-swift',
  transforms: [
    'attribute/cti',
    'name/dsp/camel',
    'color/UIColorSwift',
    'content/swift/literal',
    'asset/swift/literal',
    'size/swift/remToCGFloat',
    'font/swift/literal',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'js',
  transforms: [
    'attribute/cti',
    'name/dsp/pascal',
    'size/rem',
    'color/hex',
  ],
});

StyleDictionary.registerTransformGroup({
  name: 'flutter',
  transforms: [
    'attribute/cti',
    'name/dsp/camel',
    'color/hex8flutter',
    'size/flutter/remToDouble',
    'content/flutter/literal',
    'asset/flutter/literal',
    'font/flutter/literal',
  ],
});

module.exports = ({config, platforms}) => {
  const styleDictionary = StyleDictionary.extend(config);
  platforms
    ? platforms.forEach(platform => styleDictionary.buildPlatform(platform))
    : styleDictionary.buildAllPlatforms();
};
