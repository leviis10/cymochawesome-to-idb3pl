# Cypress Mochawesome to InfluxDB 3 Protocol Line

## Basic Command
cymochawesome-to-idb3pl [OPTIONS]

## List of options
--file <path-to-file>: specify file path
--text-only: to generate only the protocol lines
--host: specify host
--port: specify port
--database: specify influxdb3 database
--token: specidy influxdb3 token

## Example commands
### print the protocol line only
cymochawesome-to-idb3pl --text-only

### Create the protocol line and insert to influxdb3
cymochawesome-to-idb3pl --file <file-path> --host <host> --database <database> --token <token>

## Notes
This command will write into `cypress_test_results` table