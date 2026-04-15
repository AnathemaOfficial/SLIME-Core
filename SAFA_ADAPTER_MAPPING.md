# SAFA Adapter Mapping — SLIME-Core to SAFA Implementation

**Status:** Reference (non-canon)
**Purpose:** Map SLIME-Core canonical concepts to SAFA's actual API surface.
**Added:** 2026-04-15 (post-tordeur audit alignment)

---

## Why This Document Exists

SLIME-Core defines the formal specification. SAFA (SLIME Adapter for Agents)
is the production implementation. SAFA extends the SLIME model with
multi-agent containment (P3), HMAC authentication, and a richer HTTP API.

Developers building against SAFA need to know how SLIME-Core concepts map
to SAFA's actual endpoints, headers, and request/response formats.

**This document does NOT modify canon.** It is a translation guide.

---

## 1. Endpoint Mapping

| SLIME-Core Canon | SAFA Implementation |
|---|---|
| `POST /action` | `POST /ama/action` |
| `GET /health` | `GET /ama/status` |
| *(not in canon)* | `GET /ama/manifest/{agent_id}` |
| *(not in canon)* | `GET /ama/proof/{request_id}` |
| *(not in canon)* | `GET /version` |

The `/ama/` prefix is a historical artifact — SAFA was originally named
AMA (Agent Machine Armor). The prefix is retained for backward compatibility.

---

## 2. Request Format Mapping

### SLIME-Core Canon Request

```json
{
  "domain": "payments",
  "magnitude": 250,
  "payload": "eyJ0cmFuc2FjdGlvbiI6ICJURVNUIn0="
}
```

### SAFA ActionRequest

```json
{
  "action": "file_write",
  "target": "/workspace/output.txt",
  "magnitude": 1,
  "payload": "SGVsbG8gd29ybGQ=",
  "method": "POST",
  "args": ["--flag", "value"]
}
```

| SLIME Field | SAFA Field | Notes |
|---|---|---|
| `domain` | `action` | SAFA uses action type names (`file_write`, `shell_exec`, `http_request`) instead of abstract domain names |
| `magnitude` | `magnitude` | Same semantics |
| `payload` | `payload` | Same (base64, opaque) |
| *(n/a)* | `target` | Actuator-specific target path/URL |
| *(n/a)* | `method` | HTTP method for `http_request` actions |
| *(n/a)* | `args` | Arguments for `shell_exec` actions |

---

## 3. Authentication (SAFA-specific, not in canon)

SAFA requires HMAC-SHA256 request authentication on all `/ama/action`
requests:

```
X-Agent-Id: agent-001
X-Agent-Timestamp: 2026-04-15T12:00:00Z
X-Agent-Signature: <HMAC-SHA256(secret, agent_id|timestamp|body)>
```

- **Replay window:** 300 seconds
- **Replay cache:** Per-signature, prevents duplicate processing
- **Missing/invalid headers:** Returns `401 Unauthorized`

SLIME-Core canon states "No authentication mechanism" (§Security
Considerations). SAFA extends this for multi-agent environments where
agent identity is a security boundary.

---

## 4. Response Format Mapping

| SLIME-Core Canon | SAFA Implementation |
|---|---|
| `{"status":"AUTHORIZED","effect_id":"<UUID>"}` | `{"status":"authorized","action_id":"<UUID>","result":{...}}` |
| `{"status":"IMPOSSIBLE"}` | `{"status":"denied","error_class":"...","message":"..."}` |
| `{"error":"invalid_request","message":"..."}` | `{"status":"error","error_class":"validation","message":"..."}` |

**Key differences:**
- SAFA uses lowercase status values
- SAFA includes structured error information (audit trail, not feedback)
- SAFA returns action results in the `result` field
- SAFA adds `X-Safa-Policy-Hash` header on every response

---

## 5. Additional SAFA Concepts (not in SLIME-Core canon)

### 5.1 Agent Containment (P3)

Each agent operates in an isolated workspace subdirectory:
- `file_read`/`file_write`: bounded by `WorkspacePath` to `workspace_root/{agent_id}/`
- `shell_exec`: bounded by `{{agent_workspace}}` intent template
- Cross-agent filesystem access is structurally impossible

### 5.2 Idempotency Key

SAFA requires a UUID v4 `Idempotency-Key` header on all `POST /ama/action`
requests. Missing it returns `400 Bad Request`.

### 5.3 Concurrency Cap

SAFA applies a `concurrency_limit(8)` middleware layer. Requests beyond 8
simultaneous connections receive `503 Service Unavailable`.

### 5.4 Policy Bundle Hash

Every SAFA response includes `X-Safa-Policy-Hash` — a SHA-256 hash of the
effective policy surface (`domains.toml`, `intents.toml`, `allowlist.toml`,
per-agent capabilities). Deployment metadata (`config.toml`) is excluded.

---

## 6. Lineage

```
SYF-Core (mathematical invariants)
    ↓
SYF-Gate (admission decision)
    ↓
SYF-Shield (irreversible capacity consumption)
    ↓ ← these three layers are compiled into:
SLIME-Core (formal specification, this repo)
    ↓
SLIME-Enterprise (enterprise deployment layer)
    ↓
SAFA (production adapter for multi-agent containment)
```

The SYF-Gate / SYF-Shield / Anathema-Breaker architecture described in
`ARCHITECTURE_SECURITY_MODEL.md` is the **logical** decomposition. In SAFA's
deployment, these layers are embedded via `safa-core/src/slime.rs`
(`P0Authorizer`) — there is no separate SYF-Gate or SYF-Shield process.

---

## Cross-references

- **SAFA repo:** [github.com/AnathemaOfficial/SAFA](https://github.com/AnathemaOfficial/SAFA)
- **SLIME-Core canon:** `specs/` directory in this repo
- **SAFA P3 scope:** `SAFA/docs/P3_SCOPE.md`
- **SAFA threat model:** `SAFA/docs/THREAT_MODEL.md`

---

**END — SAFA ADAPTER MAPPING**
