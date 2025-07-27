# 🌎 Haydov

## A map-based travel platform for digital nomads and long-term travelers

Haydov is a comprehensive travel platform that helps independent travelers plan their journey across regions or continents. The name "Haydov" comes from the Uzbek word "haydovchi", meaning "driver" or "one who guides" - reflecting our mission to guide travelers through meaningful and spontaneous routes.

## ✨ What Haydov Does

🛏️ **Accommodation Discovery** - Find hosts and homestays along your planned path  
🧭 **Route Planning** - Generate walking, cycling, or driving routes using self-hosted Valhalla  
🎉 **Event Discovery** - Discover local events happening during your predicted stopover dates  
🤝 **Community Connection** - Connect with other travelers for meetups and shared journey segments  

## 🏗️ Architecture

Haydov is built as a modern, cloud-native microservices platform:

### Core Services

- **Geography Importer** (Node.js/TypeScript) - Processes and imports geographical data
- **Geography Dispatcher** (Rust) - High-performance routing and geographical computations
- **Geography Storage** (MinIO) - S3-compatible object storage for maps and geographical data
- **Message Broker** (RabbitMQ) - Event-driven communication between services

### Technology Stack

- **Languages**: Rust, TypeScript/Node.js
- **Container Orchestration**: Kubernetes with Helm
- **Development Tools**: Tilt, Skaffold, Nix
- **Data Processing**: Pelias/OpenStreetMap integration
- **Build System**: Multi-workspace setup with pnpm + Cargo
- **Infrastructure**: Self-hosted Valhalla routing engine

## 🚀 Quick Start

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- [Docker Daemon](https://docs.docker.com/get-docker/)

### Development Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/your-org/haydov.git
   cd haydov
   ```

2. **Enter the development environment**

   ```bash
   nix develop
   ```

   This automatically provisions a Kubernetes cluster and sets up all development tools.

3. **Deploy services**

   ```bash
   nix run .#deploy
   ```

4. **Access services**
   - 📨 RabbitMQ Management: <http://localhost:15672>
   - 🗄️ MinIO Console: <http://localhost:9090>
   - 🚀 Geography Dispatcher: <http://localhost:8080>
   - 🌍 Geography Importer: <http://localhost:3000>

## 🛠️ Development Workflow

### Using Tilt (Recommended)

Tilt provides the best development experience with live reloading and integrated service management:

```bash
# Start all services with live reloading
tilt up

# View the Tilt dashboard
open http://localhost:10350

# Stop all services
tilt down
```

### Using Nix Scripts

```bash
# Start infrastructure services (RabbitMQ, MinIO)
nix run .#start-infrastructure

# Build Docker images
nix run .#build-images

# Start containerized applications
nix run .#start-containers

# Full development setup
nix run .#full-dev

# Stop everything
nix run .#stop-all
```

### Manual Development

```bash
# Install dependencies
pnpm install --frozen-lockfile

# Run services individually
cd services/geography/importer && pnpm nx dev geography-importer
cd services/geography/dispatcher && cargo run

# Run tests
pnpm nx test
cargo test --workspace
```

## 📁 Project Structure

``` txt
haydov/
├── services/
│   ├── geography/
│   │   ├── dispatcher/          # Rust routing service
│   │   ├── importer/            # Node.js data processing
│   │   ├── storage/             # MinIO Helm charts
│   │   └── openstreetmap/       # Pelias integration
│   └── message-broker/          # RabbitMQ Helm charts
├── Dockerfile                   # Multi-stage workspace builds
├── Tiltfile                     # Development orchestration
├── skaffold.yaml               # Alternative dev workflow
├── flake.nix                   # Nix development environment
└── README.md
```

## 🔧 Service Architecture

### Geography Importer (Node.js)

Handles data ingestion and processing:

- Processes OpenStreetMap data via Pelias
- Manages geographical data imports
- Communicates via RabbitMQ events
- Exposes gRPC interface for real-time imports

### Geography Dispatcher (Rust)

High-performance geographical computations:

- Route calculation and optimization
- Real-time geographical queries
- Integration with Valhalla routing engine
- High-throughput data processing

### Event-Driven Communication

Services communicate through RabbitMQ events:

```typescript
// Trigger geographical import
await channel.sendToQueue('geography.import.requests', Buffer.from(JSON.stringify({
  command: 'import',
  args: ['--admin-lookup']
})));
```

## 🐳 Container Strategy

### Multi-Stage Builds

Each service uses sophisticated multi-stage Dockerfiles:

- **Development stage**: Hot-reloading with volume mounts
- **Build stage**: Optimized compilation
- **Production stage**: Minimal runtime containers

### Workspace Optimization

- **pnpm workspaces**: Shared Node.js dependencies
- **Cargo workspaces**: Shared Rust dependencies  
- **Cached builds**: BuildKit cache mounts for faster iterations

## ☸️ Deployment

### Local Kubernetes

Development uses local Kubernetes with kind:

```bash
kind create cluster --name haydov --image kindest/node:v1.33.2
```

### Production

Services deploy to production Kubernetes using:

- **Helm charts** for infrastructure (RabbitMQ, MinIO)
- **Kustomize** for application configuration
- **GitOps** workflows (coming soon)

## 🧪 Testing

```bash
# Run all tests
pnpm nx test
cargo test --workspace

# Integration tests
pnpm nx e2e geography-importer

# Load testing
k6 run tests/performance/dispatcher-load.js
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Enter the development environment: `nix develop`
4. Make your changes and test: `tilt up`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.
