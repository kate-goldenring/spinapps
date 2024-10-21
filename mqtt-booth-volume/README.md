# Spin MQTT Booth Volume App

Companies go to conferences to increase the visibility of their products. A big driver for that is getting conference attendees to engage with the company booth. One way engagement is often tracked is through scanning all booth visitors' badges. This is not only a manual process but also fails to indicate the level of engagement of a visitor. Nor does it track the times of the day when visitors are engaged in the booth which could be useful to understand for better booth staffing.

This Spin app aims to simplify measuring booth engagement at a conference by using the volume of sound around the booth as a proxy for booth engagement at any given point.

A prerequisite of using this app in real life would be having a sound sensor that publishes sound over MQTT. See the [example sound sensor section](##Example-Sound-Sensor) to reference a device option.

To run the MQTT broker and Spin app, simply run `make all`.

## Running an MQTT Broker

[EMQX](https://github.com/emqx/emqx) is an open source MQTT broker that can easily be [run as a Docker container](https://mqttx.app/docs/get-started). Run it locally on port 1883 in the background as follows:

```sh
docker run -d --name emqx -p 1883:1883 emqx/emqx
```

## Running the spin app to listen to a specific topic

Now, that our broker is running, we can start our Spin app, setting the broker URI to be our locally running broker. We will also listen for all messages posted to a topic that matches `booth/+`. The `+` sign is a single-level wildcard that will match any string in place of the wildcard, i.e. `booth/20` but not `booth/20/b`.

```sh
SPIN_VARIABLE_BROKER_URI="mqtt://localhost:1883" SPIN_VARIABLE_TOPIC="booth/+" spin build --up --sqlite @handle-mqtt/migration.up.sql
```

## Publishing Messages from a Fake Device

First, download [MQTTX CLI](https://github.com/emqx/MQTTX/tree/main/cli) which facilitates connecting to a broker and then publishing or subscribing to topics.

Now, publish to the topic for booth 20.

```sh
mqttx pub -t 'booth/20' -h '127.0.0.1' -p 1883 -m '{"volume": 350}'
```

## Example Sound Sensor

To use a real device with this Spin app, the device must meet the following criteria:

- Has a sound sensor
- Can publish messages over MQTT (requires WiFi access)

The [`sound-sensor`](./sound-sensor) folder contains an Anduino program that can be loaded on a device with a sound sensor. It has been tested with a a device comprised of the following components, which all can be purchased from the Arduino store:

- An [Arduino UNO R4 WiFi Board](https://store-usa.arduino.cc/products/uno-r4-wifi?variant=42871580917967)
- A [Grove Sound Sensor](https://store-usa.arduino.cc/products/grove-sound-sensor?variant=39277290488015) (plugged into A2)
- A [Grove Base Shield V2.0](https://store-usa.arduino.cc/products/grove-base-shield-v2-0-for-arduino?variant=39557870682319) for Arduino