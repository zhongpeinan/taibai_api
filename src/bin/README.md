# Harness CLI

Test harness for verifying Kubernetes type implementations (defaults, conversion, validation).

## Build

```bash
cd taibai_api
cargo build --features harness --bin harness
```

## Commands

### list - List registered GVKs

```bash
cargo run --features harness --bin harness -- list
```

Output:
```json
[
  "core/v1/Pod",
  "core/v1/Service",
  "apps/v1/Deployment",
  ...
]
```

### default - Apply defaults

Apply Kubernetes defaulting logic to a resource.

```bash
echo '{}' | cargo run --features harness --bin harness -- default core/v1/Pod
```

Output:
```json
{
  "gvk": "core/v1/Pod",
  "result": {
    "apiVersion": "v1",
    "kind": "Pod",
    "spec": {
      "dnsPolicy": "ClusterFirst",
      "restartPolicy": "Always",
      ...
    }
  },
  "defaults_applied": true
}
```

### convert - Conversion roundtrip

Perform V1 -> Internal -> V1 conversion roundtrip.

```bash
echo '{"spec":{"containers":[{"name":"nginx","image":"nginx"}]}}' | \
  cargo run --features harness --bin harness -- convert core/v1/Pod
```

Output:
```json
{
  "gvk": "core/v1/Pod",
  "original": { ... },
  "converted": { ... },
  "roundtrip": { ... },
  "success": true
}
```

Fields:
- `original`: Input JSON parsed as V1 type
- `converted`: Internal representation (for debugging conversion logic)
- `roundtrip`: Result of V1 -> Internal -> V1 conversion
- `success`: Whether conversion succeeded

### validate - Validate resource

Validate a resource against Kubernetes validation rules.

```bash
echo '{"metadata":{"name":"test"}}' | \
  cargo run --features harness --bin harness -- validate core/v1/Pod
```

Output:
```json
{
  "gvk": "core/v1/Pod",
  "valid": false,
  "errors": [
    {
      "field": "spec.containers",
      "message": "Required value",
      "type": "FieldValueRequired"
    }
  ]
}
```

### pipeline - Full pipeline

Run the complete pipeline: default -> convert -> validate.

```bash
echo '{"metadata":{"name":"test"},"spec":{"containers":[{"name":"c","image":"nginx"}]}}' | \
  cargo run --features harness --bin harness -- pipeline core/v1/Pod
```

Output:
```json
{
  "gvk": "core/v1/Pod",
  "defaults": { ... },
  "conversion": { ... },
  "validation": { ... },
  "success": true
}
```

## Usage with Go fixture tools

The harness is designed for Go fixture tools to verify Rust implementations:

```bash
# From Go test
result=$(echo "$json" | ./harness convert core/v1/Pod)
# Compare $result with Go implementation output
```

## GVK format

GVK (Group/Version/Kind) format: `{group}/{version}/{kind}`

Examples:
- `core/v1/Pod`
- `core/v1/Service`
- `core/v1/ConfigMap`
- `apps/v1/Deployment`
- `apps/v1/StatefulSet`
- `batch/v1/Job`
- `batch/v1/CronJob`

Use `list` command to see all registered GVKs.
