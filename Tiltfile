load('ext://helm_resource', 'helm_resource', 'helm_repo')
load('./common/Tiltfile', 'install_common')
load('./maps/Tiltfile', 'install_maps')

helm_repo('bitnami', 'https://charts.bitnami.com/bitnami')

config.define_string("context", args=False, usage="Kubernetes context to use")
cfg = config.parse()

if cfg.get("context"):
    k8s_context(cfg.get("context"))

allow_k8s_contexts(['kind-haydov'])

docker_build(
    'rust-manifests', 
    context='.',
    dockerfile='./Dockerfile',
    target='rust-manifests',
)

docker_build(
    'utils-sources', 
    context='.',
    dockerfile='./Dockerfile',
    target='rust-sources',
    build_args={
        'workspace': './maps/packages/utils'
    }
)

install_common()
install_maps()