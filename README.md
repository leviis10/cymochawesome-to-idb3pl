# 🌟 Cypress Mochawesome to InfluxDB 3 Protocol Line

A utility tool to convert Cypress Mochawesome reports into InfluxDB 3 Protocol Line format.

## 🚀 Features
- Convert Mochawesome JSON reports to InfluxDB line protocol
- Direct insertion to InfluxDB 3.0
- Lightweight and fast processing
- Flexible output options

## 🛠 Usage

### Basic Command
```bash
cymochawesome-to-idb3pl [OPTIONS]
```

### 🔧 Options
| Option | Description | Required |
|--------|-------------|----------|
| `--file <path>` | Path to Mochawesome JSON file | ✅ Yes |
| `--text-only` | Generate protocol lines without DB insertion | ❌ No |
| `--host <url>` | InfluxDB host URL | ✅ Yes (for DB insertion) |
| `--port <number>` | InfluxDB port | ❌ No (defaults to 8181) |
| `--database <name>` | InfluxDB database name | ✅ Yes (for DB insertion) |
| `--token <string>` | InfluxDB authentication token | ✅ Yes (for DB insertion) |

## 💡 Examples

### Generate protocol lines only
```bash
cymochawesome-to-idb3pl --file ./mochawesome-report.json --text-only
```

### Insert results directly to InfluxDB
```bash
cymochawesome-to-idb3pl \
  --file ./mochawesome-report.json \
  --host influxdb.example.com \
  --database cypress_metrics \
  --token your-auth-token-here
```

## 📝 Notes
- All data is written to the `cypress_test_results` table
- Timestamps are automatically generated from test execution times

## 🛣 Roadmap
- [ ] Add unit tests
- [ ] Fix option-as-value bug
- [ ] Optimize dependecies features included

## 📄 License
MIT © Levi Indrajaya Sutantio