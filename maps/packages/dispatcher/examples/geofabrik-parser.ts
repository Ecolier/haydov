// URL item representing a download target
export class UrlItem {
  name: string
  url: string
  region: string
  fullPath: string
  
  constructor(name: string, url: string, region: string, fullPath: string) {
    this.name = name
    this.url = url
    this.region = region
    this.fullPath = fullPath
  }
}

// Schema structure matching your YAML
export class Schema {
  baseUrl: string
  filenameTemplate: string
  regions: RegionItem[]
  
  constructor(baseUrl: string, filenameTemplate: string, regions: RegionItem[]) {
    this.baseUrl = baseUrl
    this.filenameTemplate = filenameTemplate
    this.regions = regions
  }
}

// Region can be either a string (leaf) or a map (branch)
export abstract class RegionItem {
  abstract isLeaf(): bool
}

export class LeafRegion extends RegionItem {
  name: string
  
  constructor(name: string) {
    super()
    this.name = name
  }
  
  isLeaf(): bool {
    return true
  }
}

export class BranchRegion extends RegionItem {
  name: string
  children: RegionItem[]
  
  constructor(name: string, children: RegionItem[]) {
    super()
    this.name = name
    this.children = children
  }
  
  isLeaf(): bool {
    return false
  }
}

// Helper function to build URL path from region hierarchy
function buildPath(regionPath: string[]): string {
  let path = ""
  for (let i = 0; i < regionPath.length; i++) {
    if (i > 0) {
      path += "/"
    }
    path += regionPath[i]
  }
  return path
}

// Helper function to get the leaf region name (last in the path)
function getLeafName(regionPath: string[]): string {
  return regionPath[regionPath.length - 1]
}

// Helper function to copy an array
function copyArray(arr: string[]): string[] {
  const newArr: string[] = []
  for (let i = 0; i < arr.length; i++) {
    newArr.push(arr[i])
  }
  return newArr
}

// Recursive function to process regions and build URLs
function processRegions(
  regions: RegionItem[],
  currentPath: string[],
  urls: UrlItem[],
  schema: Schema
): void {
  for (let i = 0; i < regions.length; i++) {
    const region = regions[i]
    
    if (region.isLeaf()) {
      // Leaf node - create URL
      const leaf = region as LeafRegion
      const fullPath = copyArray(currentPath)
      fullPath.push(leaf.name)
      
      const urlPath = buildPath(fullPath)
      const leafName = getLeafName(fullPath)
      const filename = schema.filenameTemplate.replace("{region}", leafName)
      const fullUrl = schema.baseUrl + "/" + urlPath + "/" + filename
      
      const urlItem = new UrlItem(filename, fullUrl, leaf.name, urlPath)
      urls.push(urlItem)
      
    } else {
      // Branch node - recurse into children
      const branch = region as BranchRegion
      const newPath = copyArray(currentPath)
      newPath.push(branch.name)
      
      processRegions(branch.children, newPath, urls, schema)
    }
  }
}

// Main function to generate URLs from schema
export function generateUrls(schema: Schema): UrlItem[] {
  const urls: UrlItem[] = []
  const initialPath: string[] = []
  
  processRegions(schema.regions, initialPath, urls, schema)
  
  return urls
}