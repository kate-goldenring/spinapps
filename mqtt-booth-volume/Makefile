SPIN_VARIABLE_BROKER_URI ?= "mqtt://localhost:1883"
SPIN_VARIABLE_TOPIC ?= "booth/+"
VOLUME ?= 350
BOOTH ?= 20

all: emqx spin-build-up

emqx:
	docker run -d --name emqx -p 1883:1883 emqx/emqx

spin-build-up:
	SPIN_VARIABLE_BROKER_URI=${SPIN_VARIABLE_BROKER_URI} SPIN_VARIABLE_TOPIC=${SPIN_VARIABLE_TOPIC} SPIN_VARIABLE_SQLITE_USERNAME="admin" SPIN_VARIABLE_SQLITE_PASSWORD="password"  spin build --up --sqlite @mqtt-message-persister/migration.up.sql

spin-up:
	SPIN_VARIABLE_BROKER_URI=${SPIN_VARIABLE_BROKER_URI} SPIN_VARIABLE_TOPIC=${SPIN_VARIABLE_TOPIC}  SPIN_VARIABLE_SQLITE_USERNAME="admin" SPIN_VARIABLE_SQLITE_PASSWORD="password" spin up --sqlite @mqtt-message-persister/migration.up.sql

pub:
	mqttx pub -t 'booth/${BOOTH}' -h '127.0.0.1' -p 1883 -m '{"volume": ${VOLUME}}'

clean:
	docker rm -f emqx

app-apply:
	spin kube scaffold --from ghcr.io/kate-goldenring/spin-apps/mqtt-booth-volume:v0.2.0 --variable broker_uri="mqtt://emqx.default.svc.cluster.local:1883" --variable topic="booth/+" --variable sqlite_username="admin" --variable sqlite_password=password --replicas 1 | kubectl apply -f -

emqx-apply:
	kubectl apply -f spinkube/broker.yaml

mock-device-apply:
	kubectl apply -f spinkube/sound-device.yaml

k8s-clean:
	kubectl delete -f spinkube/broker.yaml
	kubectl delete -f spinkube/sound-device.yaml
	spin kube scaffold --from ghcr.io/kate-goldenring/spin-apps/mqtt-booth-volume:v0.2.0 | kubectl delete -f -
