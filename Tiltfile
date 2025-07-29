load('ext://helm_resource', 'helm_resource', 'helm_repo')

config.define_string("context", args=False, usage="Kubernetes context to use")
cfg = config.parse()

if cfg.get("context"):
    k8s_context(cfg.get("context"))

allow_k8s_contexts(['docker-desktop', 'minikube', 'kind-haydov'])

# Build workspace root images (shared across all services)
docker_build(
    'pnpm-workspace',
    context='.',
    dockerfile='./Dockerfile',
    target='pnpm-workspace',
    only=[
        './package.json',
        './pnpm-workspace.yaml',
        './pnpm-lock.yaml',
        './nx.json'
    ]
)

docker_build(
    'cargo-workspace', 
    context='.',
    dockerfile='./Dockerfile',
    target='cargo-workspace',
    only=[
        './Cargo.toml',
        './Cargo.lock'
    ]
)

load('./maps/Tiltfile', 'build_maps')
build_maps();

helm_repo('bitnami', 'https://charts.bitnami.com/bitnami')

load('./common/services/message-broker/Tiltfile', 'install_message_broker')
install_message_broker()

load('./maps/services/storage/Tiltfile', 'install_maps_storage')
install_maps_storage()

k8s_yaml(kustomize('./services/geography'))
k8s_yaml(kustomize('./services/geography/storage'))
k8s_yaml(kustomize('./services/message-broker'))

k8s_yaml([
    './maps/services/osm-downloader/k8s/job.yaml',
])

k8s_resource('geography-storage-bootstrap', resource_deps=['geography-storage'])

# Create k8s resources for your services
k8s_resource('geography-dispatcher', 
             port_forwards=[
                 port_forward(8080, 8080, name='dispatcher-api')
             ])

k8s_resource('geography-importer',
             port_forwards=[
                 port_forward(3000, 3000, name='importer-api')
             ])

# Resource dependencies (ensure infrastructure is ready first)
k8s_resource('geography-dispatcher', resource_deps=['message-broker', 'geography-storage'])
k8s_resource('geography-importer', resource_deps=['message-broker', 'geography-storage'])

# Local development helpers
local_resource(
    'install-deps',
    cmd='pnpm install --frozen-lockfile',
    deps=['./package.json', './pnpm-lock.yaml'],
    labels=['setup']
)

local_resource(
    'rust-check',
    cmd='cargo check --workspace', 
    deps=['./Cargo.toml', './services/geography/dispatcher/src'],
    labels=['validation']
)

# Watch for changes and trigger rebuilds
watch_file('./pnpm-workspace.yaml')
watch_file('./nx.json')
watch_file('./Cargo.toml')

print("""
🚀 Haydov Development Environment

Services will be available at:
  📨 RabbitMQ Management: http://localhost:15672
  🗄️  MinIO Console: http://localhost:9090  
  🚀 Geography Dispatcher: http://localhost:8080
  🌍 Geography Importer: http://localhost:3000

Use 'tilt up' to start all services
Use 'tilt down' to stop everything
""")