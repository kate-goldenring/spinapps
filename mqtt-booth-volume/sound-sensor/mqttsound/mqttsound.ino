#include "WiFiS3.h"
#include "arduino_secrets.h"
#include <ArduinoMqttClient.h>
#include <ArduinoJson.h>

int sound_sensor = A2; // Sound sensor should be plugged into pin A2

char ssid[] = SECRET_SSID;        // your network SSID (name)
char pass[] = SECRET_PASS;        // your network password (use for WPA, or use as key for WEP)

// char mqtt_user[] = "";
// char mqtt_pass[] = "";

const char broker[] = "test.mosquitto.org"; // IP address of the MQTT broker.
int        port     = 1883;
const char publish_topic[]  = "booth/demo"; // Unique topic for the "booth" this sensor is in
int threshold = 200;
int delay_ms = 100;

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

void setup() {
  Serial.begin(9600);
  // Wait for serial to initialize.
  while (!Serial) {
    ;
  }
  connectWiFi();     // Establish Wi-Fi connection
  connectMQTT();     // Connect to MQTT broker
}

void loop() {
  if (!mqttClient.connected()) {
    reconnectMQTT();  // Reconnect if the connection is lost
  }
  int soundValue = 0; 
  for (int i = 0; i < 32; i++) { 
    // Read the sound sensor
    soundValue += analogRead(sound_sensor);  
  } 

  soundValue >>= 5; // Bitshift operation
  Serial.println(soundValue);

  // If a value higher than 300 is registered, we will publish the sound level to the MQTT broker
  if (soundValue > threshold) {
    // Send data
    sendVolumeData(soundValue);
  }

  delay(delay_ms); // Delay between sound readings
}


void connectWiFi() {
  Serial.print("Attempting to connect to WPA SSID: ");
  Serial.println(ssid);
  while (WiFi.begin(ssid, pass) != WL_CONNECTED) {
    // failed, retry
    Serial.print(".");
    delay(1000);
  }

  Serial.println("You're connected to the network");
}

void connectMQTT() {
  Serial.println("Attempting to connect to the MQTT broker.");
    // mqttClient.setUsernamePassword(mqtt_user, mqtt_pass);
  while (!mqttClient.connect(broker, port)) {
    Serial.print(".");
    delay(1000);
  }

  Serial.println("You're connected to the MQTT broker!");
}

void reconnectMQTT() {
  Serial.println("MQTT connection lost. Reconnecting...");
  while (!mqttClient.connect(broker, port)) {
    Serial.print(".");
    delay(1000);
  }
  Serial.println(" Reconnected!");
}

void sendVolumeData(int soundValue) {
    // Create a JSON object
    StaticJsonDocument<64> doc;
    doc["volume"] = soundValue;

    // Serialize the JSON object to a string
    char jsonBuffer[64];
    serializeJson(doc, jsonBuffer, sizeof(jsonBuffer));

    // Send the serialized JSON string over MQTT
    mqttClient.beginMessage(publish_topic);
    // The print interface can be used to set the message contents
    mqttClient.print(jsonBuffer);
    mqttClient.endMessage();
}