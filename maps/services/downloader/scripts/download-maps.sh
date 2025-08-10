#!/usr/bin/env bash
set -euo pipefail

echo "📥 Downloading map files..."

WATCH_MODE=false
PROVIDER=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --watch|-w)
      WATCH_MODE=true
      shift
      ;;
    --provider|-p)
      PROVIDER="$2"
      shift 2
      ;;
    --help|-h)
      echo "Usage: download-maps [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --watch, -w        Enable watch mode with bacon"
      echo "  --provider, -p     Run specific provider (osm|wof)"
      echo "  --help, -h         Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

export CONFIG_PATH=./maps/services/downloader/config.yaml
export STORAGE_USERNAME=${MAPS_STORAGE_USERNAME:-}
export STORAGE_PASSWORD=${MAPS_STORAGE_PASSWORD:-}

# Build the WASM target
echo "🔨 Building WASM target..."
cargo build --manifest-path "./maps/services/downloader/Cargo.toml" --target=wasm32-wasip2

if [ "$WATCH_MODE" = true ]; then
  echo "👀 Starting in watch mode with bacon..."
  if [ -n "$PROVIDER" ]; then
    echo "📍 Watching provider: $PROVIDER"
    PROVIDER_CONFIG_PATH=./maps/services/downloader/providers/$PROVIDER/config.yaml \
      bacon run -- --package=maps-downloader
  else
    echo "📍 Watching all providers..."
    bacon run
  fi
else
  if [ -n "$PROVIDER" ]; then
    echo "📍 Running provider: $PROVIDER"
    PROVIDER_CONFIG_PATH=./maps/services/downloader/providers/$PROVIDER/config.yaml \
      cargo run --package=maps-downloader
  else
    echo "📍 Running all providers..."
    PROVIDER_CONFIG_PATH=./maps/services/downloader/providers/osm/config.yaml \
      cargo run --package=maps-downloader
    PROVIDER_CONFIG_PATH=./maps/services/downloader/providers/wof/config.yaml \
      cargo run --package=maps-downloader
  fi
fi