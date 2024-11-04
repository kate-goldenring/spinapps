# Spin MQTT Booth Volume App

Companies go to conferences to increase the visibility of their products. A big driver for that is getting conference attendees to engage with the company booth. One way engagement is often tracked is through scanning all booth visitors' badges. This is not only a manual process but also fails to indicate the level of engagement of a visitor. Nor does it track the times of the day when visitors are engaged in the booth which could be useful to understand for better booth staffing.

This Spin app aims to simplify measuring booth engagement at a conference by using the volume of sound around the booth as a proxy for booth engagement at any given point.

A prerequisite of using this app in real life would be having a sound sensor that publishes sound over MQTT. See the [example sound sensor section](##Example-Sound-Sensor) to reference a device option.

To run the MQTT broker and Spin app, simply run `make all`.

## Running an MQTT Broker

Eclipse provides a free publically available [MQTT broker](https://test.mosquitto.org/) that can be used for demos. Use `test.mosquitto.org` as the broker address with port 1883. Connect and subscribe to a topic on the broker as follows:

```sh
mqttx sub  -h test.mosquitto.org -t "/booth/20" -p 1883
```

Alternatively, run your own broker with [EMQX](https://github.com/emqx/emqx), which is an open source MQTT broker that can easily be [run as a Docker container](https://mqttx.app/docs/get-started). Run it locally on port 1883 in the background as follows:

```sh
docker run -d --name emqx -p 1883:1883 emqx/emqx
```

Connect and subscribe to a topic on the locally running EMQX broker:
```sh
mqttx sub  -h '127.0.0.1' -t "/booth/20" -p 1883
```

## Running the spin app to listen to a specific topic

Now, that our broker is running, we can start our Spin app, setting the broker URI to be our locally running broker. We will also listen for all messages posted to a topic that matches `booth/+`. The `+` sign is a single-level wildcard that will match any string in place of the wildcard, i.e. `booth/20` but not `booth/20/b`.

```sh
SPIN_VARIABLE_MQTT_BROKER_URI="mqtt://localhost:1883" SPIN_VARIABLE_MQTT_TOPIC="booth/+" spin build --up --sqlite @mqtt-message-persister/migration.up.sql
```

## Publishing Messages from a Fake Device

First, download [MQTTX CLI](https://github.com/emqx/MQTTX/tree/main/cli) which facilitates connecting to a broker and then publishing or subscribing to topics.

Now, publish to the topic for booth 20. Set the broker host address to that of your broker, we are using Mosquitto's public broker here:

```sh
mqttx pub -h test.mosquitto.org -t 'booth/20' -p 1883 -m '{"volume": 350}'
```

## Example Sound Sensor

To use a real device with this Spin app, the device must meet the following criteria:

- Has a sound sensor
- Can publish messages over MQTT (requires WiFi access)

The [`sound-sensor`](./sound-sensor/mqttsound) folder contains an Anduino program that can be loaded on a device with a sound sensor. It has been tested with a a device comprised of the following components, which all can be purchased from the Arduino store:

- An [Arduino UNO R4 WiFi Board](https://store-usa.arduino.cc/products/uno-r4-wifi?variant=42871580917967)
- A [Grove Sound Sensor](https://store-usa.arduino.cc/products/grove-sound-sensor?variant=39277290488015) (plugged into A2)
- A [Grove Base Shield V2.0](https://store-usa.arduino.cc/products/grove-base-shield-v2-0-for-arduino?variant=39557870682319) for Arduino

## Running on SpinKube with Local SQLite Database

The Spin runtime supports persisting data to SQLite databases. A local one can be used by default.
It cannot be pre-configured in SpinKube since it will be created in the container filesystem when
the `SpinApp` Pod is started. Instead, you can configure the database after the app has started
using the SQLite explorer component. Be sure to uncomment the SQLite explorer component from the
[`spin.toml`](./spin.toml) and push your app to a registry first.

```sh
export REGISTRY=ghcr.io/username/mqtt-booth-volume
export TAG=v0.1.0
spin registry push $REGISTRY:$TAG
make emqx-apply
SPIN_VARIABLE_MQTT_BROKER_URI="mqtt://emqx.default.svc.cluster.local:1883" make app-apply
# port-forward the app service
kubectl port-forward svc/mqtt-booth-volume 3000:80
```

> Note: the local SQLite database should only be used for testing purposes and only on `SpinApp`
> deployments with 1 replica. For more complex scenarios, see the [next
> section](#persisting-to-a-turso-database-with-runtime-config) on how to use runtime configuration
> with an external database.

Navigate to localhost:3000/internal/sqlite. Log in with credentials "admin" / "password" and perform
the initial database migration by entering the contents of
[`migration.up.sql`](./mqtt-message-persister/migration.up.sql). Now that the database is
initialized, you can deploy the fake device:

```sh
make mock-device-apply
```

## Persisting to a Turso Database with Runtime Config

[Download the Turso CLI](https://github.com/tursodatabase/turso-cli). Then, login to your account, create a database, and generate a token for it.

```sh
turso auth login
turso db create mqtt-booth-volume
turso db shell mqtt-booth-volume < mqtt-message-persister/migration.up.sql
turso db tokens create mqtt-booth-volume
```

Update the [runtime-config.toml](./spinkube/runtime-config.toml) file to contain your database URL and token. Now, you can instruct the `spin` and `spin kube` CLI's to use the runtime configuration using the `--runtime-config-file` flag.

## Running EMQX Inside Your Cluster

Use the [`install-broker.sh`](./spinkube/broker-configuration/install-broker.sh) script to install an EMQX broker in your cluster. Provide the `-external` flag to deploy [EMQX cluster](https://docs.emqx.com/en/emqx-operator/latest/deployment/on-azure-aks.html#apps.emqx.io/v2beta1) and configure it to be available outside of the cluster.
