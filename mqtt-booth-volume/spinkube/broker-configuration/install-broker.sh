#!/bin/bash
DIR="$( dirname "$(realpath "$0")")"

# FROM https://docs.emqx.com/en/emqx-operator/latest/getting-started/getting-started.html
install-emqx-cluster() {
    # 1) Make sure cert manager is installed

    # 2) Install EMQX Operator
    helm repo add emqx https://repos.emqx.io/charts
    helm repo update
    helm upgrade --install emqx-operator emqx/emqx-operator \
    --namespace emqx-operator-system \
    --create-namespace

    # 3) Wait till ready
    kubectl wait --for=condition=Ready pods -l "control-plane=controller-manager" -n emqx-operator-system

    # 4) Install EMQX Cluster CRD (Azure specific from https://docs.emqx.com/en/emqx-operator/latest/deployment/on-azure-aks.html#apps.emqx.io/v2beta1)
    kubectl apply -f ${DIR}/emqx-cluster-crd.yaml

    # 5) Get external IP
    external_ip=$(kubectl get svc emqx-listeners -o json | jq '.status.loadBalancer.ingress[0].ip')
    echo "MQTT Broker is available at mqtt://${external_ip}:1883"
    echo "Start publishing messages: mqttx pub -t 'hello' -h ${external_ip} -p 1883   -m "hello world""
}

install-emqx-pod() {
    kubectl apply -f ${DIR}/emqx-pod.yaml
    echo "MQTT Broker is available to cluster services at mqtt://emqx.default.svc.cluster.local:1883"
}

case "$1" in
    -internal)
        install-emqx-pod
        ;;
    -external)
        install-emqx-cluster
        ;;
    *)
        echo "Specify whether to install the MQTT broker for external or internal only access"
        echo "Usage: $0 {-internal|-external}"
        exit 1
        ;;
esac