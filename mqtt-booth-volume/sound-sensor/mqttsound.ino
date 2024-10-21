#include "WiFiS3.h"
#include "arduino_secrets.h"
#include <ArduinoMqttClient.h>

int sound_sensor = A2; // Sound sensor should be plugged into pin A2

char ssid[] = SECRET_SSID;        // your network SSID (name)
char pass[] = SECRET_PASS;        // your network password (use for WPA, or use as key for WEP)

char mqtt_user[] = "arduino";
char mqtt_pass[] = "pass";


WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

const char broker[] = "10.1.2.3"; // IP address of the EMQX broker.
int        port     = 1883;
const char subscribe_topic[]  = "/hello";
const char publish_topic[]  = "/hello/world";

void setup() {
  Serial.begin(9600);
  // // Wait for serial to initialize.
  // while (!Serial) {
  //   ;
  // }

  // Connect to WiFi
  Serial.print("Attempting to connect to WPA SSID: ");
  Serial.println(ssid);
  while (WiFi.begin(ssid, pass) != WL_CONNECTED) {
    // failed, retry
    Serial.print(".");
    delay(5000);
  }

  Serial.println("You're connected to the network");
  Serial.println();

  mqttClient.setUsernamePassword(mqtt_user, mqtt_pass);

  Serial.println("Attempting to connect to the MQTT broker.");

  if (!mqttClient.connect(broker, port)) {
    Serial.println("MQTT connection failed! Error code = ");
    Serial.println(mqttClient.connectError());

    while (1);
  }

  Serial.println("You're connected to the MQTT broker!");
}

void loop() {
  int soundValue = 0; 
  for (int i = 0; i < 32; i++) { 
    // Read the sound sensor
    soundValue += analogRead(sound_sensor);  
  } 

  soundValue >>= 5; // Bitshift operation
  Serial.println(soundValue);

  // If a value higher than 300 is registered, we will publish the sound level to the MQTT broker
  if (soundValue > 300) {
    // Send message, the Print interface can be used to set the message contents
    mqttClient.beginMessage(publish_topic);
    mqttClient.print(soundValue);
    mqttClient.endMessage();
  }

  delay(100); / delay for 100ms
}