# Load Helm extension
load('ext://helm_resource', 'helm_resource', 'helm_repo')

# Configuration
config.define_string("context", args=False, usage="Kubernetes context to use")
cfg = config.parse()

# Set Kubernetes context if provided
if cfg.get("context"):
    k8s_context(cfg.get("context"))

# Allow k8s contexts (equivalent to Skaffold's cluster detection)
allow_k8s_contexts(['docker-desktop', 'minikube', 'kind-haydov'])

# Build workspace root images
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

# Build geography-dispatcher with dependency
docker_build(
    'geography-dispatcher',
    context='./services/geography/dispatcher',
    dockerfile='./services/geography/dispatcher/Dockerfile',
    target='dev',
    build_args={'workspace_root': 'cargo-workspace'},
    # Live sync for development
    live_update=[
        sync('./services/geography/dispatcher/src', '/app/src'),
        run('cargo check', trigger=['./services/geography/dispatcher/src/**/*.rs'])
    ]
)

# Build geography-importer with dependency  
docker_build(
    'geography-importer',
    context='./services/geography/importer', 
    dockerfile='./services/geography/importer/Dockerfile',
    target='dev',
    build_args={'workspace_root': 'pnpm-workspace'},
    # Live sync for development
    live_update=[
        sync('./services/geography/importer/src', '/app/src'),
        run('pnpm nx dev geography-importer', trigger=['./services/geography/importer/src/**/*.ts'])
    ]
)

# Add Helm repositories
helm_repo('bitnami', 'https://charts.bitnami.com/bitnami')

# Deploy message-broker via Helm
helm_resource(
    'message-broker',
    'bitnami/rabbitmq',
    namespace='haydov',
    flags=[
        '--version=16.0.10',
        '--values=services/message-broker/helm/values.yaml'
    ],
    port_forwards=[
        port_forward(15672, 15672, name='rabbitmq-management')
    ]
)

# Deploy geography-storage via Helm  
helm_resource(
    'geography-storage',
    'bitnami/minio',
    namespace='haydov', 
    flags=[
        '--version=17.0.12',
        '--values=services/geography/storage/helm/values.yaml'
    ],
    port_forwards=[
        port_forward(9090, 9090, name='minio-console')
    ]
)

# Deploy Kustomize manifests
k8s_yaml(kustomize('./services/geography'))
k8s_yaml(kustomize('./services/geography/storage'))
k8s_yaml(kustomize('./services/message-broker'))

# Deploy raw YAML manifests
k8s_yaml([
    './services/geography/dispatcher/deployment.yaml',
    './services/geography/dispatcher/service.yaml', 
    './services/geography/importer/deployment.yaml',
    './services/geography/importer/service.yaml'
])

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