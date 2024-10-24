# Example Sound Sensor

This is a collection of programs to load on microcontrollers with sound sensors that configures them to publish sound levels to an MQTT broker on a specific topic.

All devices must meet the following requirements:
- Has a sound sensor
- Can publish messages over MQTT (requires WiFi access)

## Arduino Sketch

The [`mqttsound`](./sound-sensor/mqttsound) folder contains an Anduino program that can be loaded on a device with a sound sensor. It has been tested with a a device comprised of the following components, which all can be purchased from the Arduino store:

- An [Arduino UNO R4 WiFi Board](https://store-usa.arduino.cc/products/uno-r4-wifi?variant=42871580917967)
- A [Grove Sound Sensor](https://store-usa.arduino.cc/products/grove-sound-sensor?variant=39277290488015) (plugged into A2)
- A [Grove Base Shield V2.0](https://store-usa.arduino.cc/products/grove-base-shield-v2-0-for-arduino?variant=39557870682319) for Arduino

Before uploading the program to your device, be sure to do the following:
1. Add the netword SSID (name) and password to [`arduino_secrets.h`](./mqttsound/arduino_secrets.h)
1. Configure constants in the [arduino sketch](./mqttsound/mqttsound.ino):
    1. Set the address and port for your MQTT broker. Defaults to the [public Mosquitto broker](https://test.mosquitto.org/) at host `test.mosquitto.org` and port `1883`.
    1. Configure the MQTT topic for the device. Defaults to `booth/demo`.
    1. Configure the sound threshold for the device. Any volume above this threshold is published. Defaults to 200.
1. If using a broker that requires authentication, be sure to set a username and password in the sketch and uncomment the line that does authentication.

### Troubleshooting Arduino Uno

#### New firmware not seeming to work?
Solution summarized from [Arduino forum](https://forum.arduino.cc/t/no-device-found-on-ttyacm0-both-with-arduino-ide-2-0-3-and-arduino-iot-cloud/1062050/2):
1. Be sure updates are saved in Arduino studio
2. Double click white reset button to reset bootloader -- it is now ready for a new flash
3. Upload

#### Restart powered device?
Single click the white reset button