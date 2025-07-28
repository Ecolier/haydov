class Region {
  type: string;
  name: string;
  path: string;
  file: string;
  regions: Region[];

  constructor(type: string, name: string, path: string = "", file: string = "") {
    this.type = type;
    this.name = name;
    this.path = path;
    this.file = file;
    this.regions = [];
  }
}

class DownloadItem {
  file: string;
  url: string;
  success: boolean;

  constructor(file: string, url: string) {
    this.file = file;
    this.url = url;
    this.success = true;
  }
}

// Helper functions
function createNode(name: string, path: string, regions: Region[]): Region {
  const node = new Region("Node", name, path);
  node.regions = regions;
  return node;
}

function createLeaf(name: string, file: string): Region {
  return new Region("Leaf", name, "", file);
}

function joinUrl(baseUrl: string, path: string): string {
  if (baseUrl.endsWith("/")) {
    return baseUrl + path;
  } else {
    return baseUrl + "/" + path;
  }
}

// Recursive function to collect URLs
function collectUrls(regions: Region[], basePath: string, outList: DownloadItem[]): void {
  for (let i = 0; i < regions.length; i++) {
    const region = regions[i];
    if (region.type == "Node") {
      const newBase = joinUrl(basePath, region.path);
      collectUrls(region.regions, newBase, outList);
    } else if (region.type == "Leaf") {
      const url = joinUrl(basePath, region.file);
      outList.push(new DownloadItem(region.file, url));
    }
  }
}

// Main function
export function createDownloadList(baseUrl: string): DownloadItem[] {
  const regions = [
    createNode("Europe", "europe", [
      createLeaf("Montenegro", "montenegro-latest.osm.pbf"),
      createLeaf("Malta", "malta-latest.osm.pbf"),
      createLeaf("Macedonia", "macedonia-latest.osm.pbf")
    ]),
    createNode("Asia", "asia", [
      createLeaf("Japan", "japan-latest.osm.pbf")
    ])
  ];

  const downloadList: DownloadItem[] = [];
  collectUrls(regions, baseUrl, downloadList);
  return downloadList;
}

// Memory management helpers for Wasmer
export function allocateString(str: string): i32 {
  const buffer = String.UTF8.encode(str);
  const ptr = heap.alloc(buffer.byteLength);
  memory.copy(ptr, changetype<usize>(buffer), buffer.byteLength);
  return ptr;
}

export function readString(ptr: i32, len: i32): string {
  return String.UTF8.decode(memory.slice(ptr, ptr + len));
}