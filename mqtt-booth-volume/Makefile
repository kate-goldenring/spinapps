SPIN_VARIABLE_BROKER_URI ?= "mqtt://localhost:1883"
SPIN_VARIABLE_TOPIC ?= "booth/+"
VOLUME ?= 350
BOOTH ?= 20

all: emqx spin-up

emqx:
	docker run -d --name emqx -p 1883:1883 emqx/emqx

spin-up:
	SPIN_VARIABLE_BROKER_URI=${SPIN_VARIABLE_BROKER_URI} SPIN_VARIABLE_TOPIC=${SPIN_VARIABLE_TOPIC} spin build --up --sqlite @handle-mqtt/migration.up.sql

pub:
	mqttx pub -t 'booth/${BOOTH}' -h '127.0.0.1' -p 1883 -m '{"volume": ${VOLUME}}'

clean:
	docker rm -f emqx

k8s-apply:
	spin kube scaffold --from ghcr.io/kate-goldenring/spin-apps/mqtt-booth-volume:v0.1.0 --variable broker_uri="mqtt://emqx.default.svc.cluster.local:1883" --variable topic="booth/+" --replicas 1 | kubectl apply -f -