# Spin Application Examples

A collection of Spin application examples that use external resources via runtime configuration. All
applications can be deployed to run on Kubernetes using [SpinKube](https://www.spinkube.dev/).

## Examples

- [Conference Booth Engagement Application](./mqtt-booth-volume/) - measure booth engagement by ingesting sound level data over MQTT from a sensor.
- [Simple Key Value Spin Application](./simple-kv/) - baseline for using external key value stores (Redis) from a Spin application, whether locally or on SpinKube.
- [Key Value Spin Application](./kv/) - showcases the variety of key value store operations that can be performed from a Spin application.
- [Outbound HTTP Example](./outbound-http/) - demonstrates how to make outbound HTTP requests from a Spin application, with the target host configured via an application variable.
- [Application Variables Example](./application-vars/) - demonstrates how to use multiple application variable providers in a single Spin application, and how to use the `spin variables` CLI command to manage application variables.
- [A/B Testing Single/Split Screen](./screen-mode/) - demonstrates the ability to do A/B testing with application variables and key value.