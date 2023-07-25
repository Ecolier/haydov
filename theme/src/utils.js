const getTokenNameFromProperty = function (prop) {
  if (prop.attributes.category === 'size' && prop.attributes.type === 'font') {
    return prop.path.slice(2, prop.path.length).join(' ').concat(' size');
  } else {
    return prop.path.slice(1, prop.path.length).join(' ');
  }
};

module.exports = {
  getTokenNameFromProperty,
};