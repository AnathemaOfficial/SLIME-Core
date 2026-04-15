# SLIME v0 — Ingress API Specification

**Status:** CANON (v0)  
**Scope:** HTTP interface for action submission

---

## Overview

SLIME v0 accepts action requests via a **fixed HTTP API**.

The API is **declarative only** - no logic, no interpretation, no branching.

**Canonical principle:**

> **Ingress accepts declarative ActionRequests only.**  
> **SLIME does not validate intent, correctness, or feasibility.**  
> **Authorization is decided exclusively by AB-S.**

SLIME performs only strict format normalization. All semantic decisions belong to AB-S.

---

## Fixed Endpoint

**HTTP Server:** Binds to `127.0.0.1:8080` (localhost only)  
**Endpoint:** `POST /action`  
**Content-Type:** `application/json`

**The bind address and port are hardcoded.**  
No environment variables, flags, or runtime parameters.

**Remote access:**  
SLIME does not provide remote access. If needed, use external infrastructure:
- Reverse proxy (nginx, HAProxy, Caddy)
- SSH tunnel
- VPN
- API gateway with authentication

---

## Request Format

### JSON Schema

```json
{
  "domain": "string",
  "magnitude": number,
  "payload": "base64_string"
}
```

### Fields

**`domain` (string, required)**
- Symbolic domain identifier (e.g., "payments", "actuation", "control")
- Mapped to `domain_id` (64-bit hash)
- Maximum length: 256 characters
- Allowed characters: `[a-zA-Z0-9_-]`

**`magnitude` (number, required)**
- Action magnitude (energy/capacity consumption)
- Positive integer (> 0)
- Range: `1` to `2^64-1`
- Floating point values are rounded down

**`payload` (string, optional)**
- Base64-encoded opaque action data
- Maximum decoded size: 65536 bytes (64KB)
- Part of the ActionRequest submitted to AB-S
- May be reflected in AuthorizedEffect if action is authorized
- **SLIME does not interpret payload content**
- If omitted, empty payload assumed

### Example Request

```bash
curl -X POST http://localhost:8080/action \
  -H "Content-Type: application/json" \
  -d '{
    "domain": "payments",
    "magnitude": 250,
    "payload": "eyJ0cmFuc2FjdGlvbiI6ICJURVNUIn0="
  }'
```

Decoded payload: `{"transaction": "TEST"}`

---

## Response Format

### Success (200 OK)

**Action Authorized:**
```json
{
  "status": "AUTHORIZED",
  "effect_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Action Blocked (Impossibility):**
```json
{
  "status": "IMPOSSIBLE"
}
```

**Critical:**
- Both responses return HTTP 200
- No error codes distinguish authorization from impossibility
- No retry-after headers
- No explanation fields
- No reason codes exposed
- **No confidence scores**
- **No hints or suggestions**
- **No semantic metadata**

The response is **binary**: authorized or impossible. Nothing else.

### Error (4xx/5xx)

**Malformed Request (400 Bad Request):**
```json
{
  "error": "invalid_request",
  "message": "Missing required field: domain"
}
```

**Payload Too Large (413 Payload Too Large):**
```json
{
  "error": "payload_too_large",
  "message": "Decoded payload exceeds 65536 bytes"
}
```

**Server Error (500 Internal Server Error):**
```json
{
  "error": "internal_error",
  "message": "SLIME runtime failure"
}
```

**Note:** 4xx/5xx errors indicate **malformed requests**, not impossibilities.  
Impossibilities return 200 with `"status": "IMPOSSIBLE"`.

---

## Processing Rules

### Normalization

SLIME performs **strict normalization** on ingress:

1. `domain` string → `domain_id` (`u64` via stable hash function or compile-time table)
2. `magnitude` float → truncate to `u64`
3. `payload` base64 → binary buffer

**Domain ID width:** Domain IDs are `u64` end-to-end per `FULL_STACK_CONFORMANCE.md` R-7.
No truncation to `u32`/`u16` is permitted anywhere in the pipeline.

> **Errata (2026-04-15):** Prior versions of this spec included a 32-bit mask
> (`domain_id & 0xFFFFFFFF`). This contradicted R-7 and has been removed.
> Implementations using compile-time domain tables (e.g., slime-runner) map
> domain strings directly to `u64` values without hashing.

**No semantic validation** - SLIME does not interpret domain names or payload content.  
**No business logic** - SLIME does not validate intent, correctness, or feasibility.  
**No inference** - SLIME does not infer missing fields or "helpful" defaults.

Normalization is **purely mechanical** - transform format, preserve meaning exactly.

**Payload flow:**
- Payload is part of the `ActionRequest` structure submitted to AB-S
- AB-S may include payload reference in `AuthorizedEffect` if action is authorized
- SLIME never interprets, validates, or transforms payload content
- Payload semantic validation (if any) is the actuator bridge's responsibility

### Rejection Criteria

Requests are rejected (4xx) **only for format violations**:
- Missing required fields (`domain`, `magnitude`)
- Invalid JSON syntax
- `domain` contains illegal characters
- `magnitude` is negative
- `payload` is not valid base64
- Decoded payload exceeds 64KB

**These are not semantic judgments** - they are format enforcement only.

**Impossibilities are not rejections** - they return 200 with `IMPOSSIBLE`.

---

## Concurrency

### Thread Safety

SLIME accepts **concurrent requests**.

**Guarantees:**
- Each request processed independently
- Effects delivered to egress in order of authorization
- No request blocking on others
- No global locking

**No guarantee on request ordering** when concurrent - AB-S may authorize in different order than arrival.

### Rate Limiting

**SLIME v0 has no rate limiting.**

If ingress overwhelms AB-S capacity:
- More actions return `IMPOSSIBLE`
- AB-S state may transition to `SATURATED`
- Eventually AB-S may reach `SEALED` (terminal)

**No throttling, no backpressure signals to client.**

---

## Failure Modes

### Ingress Unavailable

If SLIME process is not running:
- Port 8080 is not listening
- Connection refused
- Client should handle as **service unavailable**

This is **fail-closed** - no actions accepted.

### AB-S Failure

If AB-S core panics or becomes unresponsive:
- SLIME returns 500 Internal Server Error
- No actions are authorized
- Process may exit (restart policy handles recovery)

This is **fail-closed** - no effects egress.

### Egress Blocked

If egress socket fills (no actuator consuming effects):
- SLIME continues accepting actions
- Authorized effects queue in kernel buffer
- Once buffer full, egress writes block
- Ingress continues normally (effects buffered)

**No HTTP-level backpressure** - clients see normal latency.

---

## Integration Examples

### Python Client

```python
import requests
import base64

def submit_action(domain, magnitude, payload_dict=None):
    payload_b64 = ""
    if payload_dict:
        import json
        payload_json = json.dumps(payload_dict)
        payload_b64 = base64.b64encode(payload_json.encode()).decode()
    
    response = requests.post(
        "http://localhost:8080/action",
        json={
            "domain": domain,
            "magnitude": magnitude,
            "payload": payload_b64
        }
    )
    
    if response.status_code == 200:
        result = response.json()
        if result["status"] == "AUTHORIZED":
            print(f"Authorized: {result['effect_id']}")
        else:
            print("Impossible (blocked by AB-S)")
    else:
        print(f"Error: {response.status_code}")

# Example usage
submit_action("payments", 250, {"amount": 250, "currency": "USD"})
```

### JavaScript Client

```javascript
async function submitAction(domain, magnitude, payloadObj = null) {
  let payload = "";
  if (payloadObj) {
    const payloadJson = JSON.stringify(payloadObj);
    payload = btoa(payloadJson);
  }
  
  const response = await fetch("http://localhost:8080/action", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ domain, magnitude, payload })
  });
  
  if (response.ok) {
    const result = await response.json();
    if (result.status === "AUTHORIZED") {
      console.log(`Authorized: ${result.effect_id}`);
    } else {
      console.log("Impossible (blocked)");
    }
  } else {
    console.error(`Error: ${response.status}`);
  }
}

// Example usage
submitAction("actuation", 100, { type: "move", direction: "forward" });
```

### Rust Client

```rust
use reqwest;
use serde::{Deserialize, Serialize};
use base64;

#[derive(Serialize)]
struct ActionRequest {
    domain: String,
    magnitude: u64,
    payload: String,
}

#[derive(Deserialize)]
struct ActionResponse {
    status: String,
    effect_id: Option<String>,
}

async fn submit_action(domain: &str, magnitude: u64, payload: Option<&str>) 
    -> Result<ActionResponse, Box<dyn std::error::Error>> 
{
    let payload_b64 = match payload {
        Some(p) => base64::encode(p),
        None => String::new(),
    };
    
    let request = ActionRequest {
        domain: domain.to_string(),
        magnitude,
        payload: payload_b64,
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/action")
        .json(&request)
        .send()
        .await?;
    
    let result: ActionResponse = response.json().await?;
    Ok(result)
}

#[tokio::main]
async fn main() {
    match submit_action("control", 50, Some(r#"{"cmd":"start"}"#)).await {
        Ok(resp) if resp.status == "AUTHORIZED" => {
            println!("Authorized: {}", resp.effect_id.unwrap());
        }
        Ok(_) => println!("Impossible"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Health Check Endpoint

**Endpoint:** `GET /health`  
**Response (200 OK):**
```json
{
  "status": "ok"
}
```

**Purpose:** Binary liveness check for load balancers and monitoring systems.

**Does not indicate:**
- AB-S internal state
- Capacity levels
- Action counts
- Any operational metrics

**This is the only observability endpoint on ingress.**  
All other observation must be done via the dashboard (port 8081).

---

## Security Considerations

### Network Binding

**SLIME binds to localhost (127.0.0.1) only.**

Ingress and dashboard are **not accessible** from other hosts by default.

**This is intentional and non-configurable.**

If remote access is required:
- Use reverse proxy with authentication (nginx, Caddy, etc.)
- Use SSH port forwarding: `ssh -L 8080:localhost:8080 server`
- Use VPN to access localhost
- Use API gateway

**Do not modify SLIME to bind to 0.0.0.0** - this violates canon.

### No Authentication

SLIME v0 ingress has **no authentication mechanism**.

**Security model:**
- Localhost binding provides network isolation
- Authentication (if needed) must be provided by external reverse proxy
- Authorization is handled by AB-S (opaque, sealed)

### Payload Validation

SLIME does **not validate payload content**.

**Flow:**
1. SLIME normalizes payload from base64 to binary
2. Payload becomes part of ActionRequest to AB-S
3. If authorized, payload reference may be included in AuthorizedEffect
4. Actuator bridge receives AuthorizedEffect (which may reference payload)
5. **Actuator bridge is responsible for payload validation and sanitization**

SLIME guarantees only:
- Payload is valid base64
- Decoded payload does not exceed 64KB
- Payload bytes are preserved exactly (no transformation)

**No semantic guarantees** - payload could contain:
- Invalid data structures
- Malicious code
- Incorrect parameters
- Garbage data

The actuator bridge **must** validate payload content before actuation.

### Denial of Service

SLIME has **no DoS protection** at the application layer.

**Network-level protection:**
- Localhost binding limits attack surface to local processes only
- Remote attackers cannot reach SLIME directly

**If exposed via reverse proxy:**
- Rate limiting must be implemented at proxy layer
- Connection limits must be enforced externally
- SLIME itself provides no throttling

If local processes overwhelm SLIME:
- AB-S capacity exhausts (SATURATED state)
- Eventually reaches SEALED (terminal)
- No recovery mechanism exists

---

## Prohibitions (Non-Negotiable)

The following are **structurally impossible** in SLIME v0:

- No alternative ports or paths
- No configuration of endpoints
- No authentication/authorization (beyond network security)
- No retry hints or backoff headers
- **No explanation of why action was blocked**
- **No confidence scores or probabilities**
- **No semantic validation of payloads**
- **No business logic validation**
- **No intent inference**
- **No "helpful" transformations**
- No rate limiting
- No request prioritization
- No request cancellation

**SLIME does not interpret, infer, or judge actions semantically.**  
**All semantic decisions are made by AB-S, which is opaque and sealed.**

Any request to add these features **violates canon**.

---

## Canonical Statement

> **Ingress accepts declarative ActionRequests only.**  
> **SLIME does not validate intent, correctness, or feasibility.**  
> **Authorization is decided exclusively by AB-S.**  
> **Normalization is mechanical, interpretation is zero.**  
> **Response is binary: AUTHORIZED or IMPOSSIBLE.**

---

**END — SLIME v0 INGRESS API SPECIFICATION**

This specification is **non-negotiable** and forms part of the SLIME v0 canonical interface.
