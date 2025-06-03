function py2_round(value: number) {
    // Google's polyline algorithm uses the same rounding strategy as Python 2, which is different from JS for negative values
    return Math.floor(Math.abs(value) + 0.5) * (value >= 0 ? 1 : -1);
}

function _encode(current: number, previous: number, factor: number) {
    current = py2_round(current * factor);
    previous = py2_round(previous * factor);
    let coordinate = (current - previous) * 2;
    if (coordinate < 0) {
        coordinate = -coordinate - 1
    }
    let output = '';
    while (coordinate >= 0x20) {
        output += String.fromCharCode((0x20 | (coordinate & 0x1f)) + 63);
        coordinate /= 32;
    }
    output += String.fromCharCode((coordinate | 0) + 63);
    return output;
}

export function decode(str: string, precision: number) {
    let index = 0,
        lat = 0,
        lng = 0,
        coordinates = [],
        shift = 0,
        result = 0,
        byte = null,
        latitude_change,
        longitude_change,
        factor = Math.pow(10, Number.isInteger(precision) ? precision : 5);

    // Coordinates have variable length when encoded, so just keep
    // track of whether we've hit the end of the string. In each
    // loop iteration, a single coordinate is decoded.
    while (index < str.length) {

        // Reset shift, result, and byte
        byte = null;
        shift = 1;
        result = 0;

        do {
            byte = str.charCodeAt(index++) - 63;
            result += (byte & 0x1f) * shift;
            shift *= 32;
        } while (byte >= 0x20);

        latitude_change = (result & 1) ? ((-result - 1) / 2) : (result / 2);

        shift = 1;
        result = 0;

        do {
            byte = str.charCodeAt(index++) - 63;
            result += (byte & 0x1f) * shift;
            shift *= 32;
        } while (byte >= 0x20);

        longitude_change = (result & 1) ? ((-result - 1) / 2) : (result / 2);

        lat += latitude_change;
        lng += longitude_change;

        coordinates.push([lat / factor, lng / factor]);
    }

    return coordinates;
};

export function encode(coordinates: number[][], precision: number) {
    if (!coordinates.length) { return ''; }

    let factor = Math.pow(10, Number.isInteger(precision) ? precision : 5),
        output = _encode(coordinates[0][0], 0, factor) + _encode(coordinates[0][1], 0, factor);

    for (let i = 1; i < coordinates.length; i++) {
        let a = coordinates[i], b = coordinates[i - 1];
        output += _encode(a[0], b[0], factor);
        output += _encode(a[1], b[1], factor);
    }

    return output;
};

function flipped(coords: number[][]) {
    let flipped = [];
    for (let i = 0; i < coords.length; i++) {
        let coord = coords[i].slice();
        flipped.push([coord[1], coord[0]]);
    }
    return flipped;
}

export function fromGeoJSON(geojson: any, precision: number) {
    if (geojson && geojson.type === 'Feature') {
        geojson = geojson.geometry;
    }
    if (!geojson || geojson.type !== 'LineString') {
        throw new Error('Input must be a GeoJSON LineString');
    }
    return encode(flipped(geojson.coordinates), precision);
}

export function toGeoJSON(str: string, precision: number) {
    let coords = decode(str, precision);
    return {
        type: 'LineString',
        coordinates: flipped(coords)
    };
};