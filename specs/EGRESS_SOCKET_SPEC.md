# SLIME v0 — Egress Socket Specification

**Status:** CANON (v0)  
**Scope:** Linux-first, fixed wiring, zero configuration

---

## Overview

SLIME v0 delivers authorized effects to the environment via a **fixed Unix domain socket**.

The socket is the **only egress interface**.  
No network ports, no configuration, no alternatives.

---

## Fixed Endpoint

**Socket path:** `/run/slime/egress.sock`

**Properties:**
- Type: Unix domain socket (SOCK_STREAM)
- Owner: `slime` user
- Group: `slime` group  
- Permissions: `0660` (read/write owner and group)
- Created by SLIME on startup
- Removed on SLIME shutdown

**The socket path is hardcoded.**  
No environment variables, flags, or runtime parameters are allowed.

---

## Message Contract

### Payload Structure

SLIME writes binary `AuthorizedEffect` structures to the socket.

**C struct representation:**
```c
typedef struct {
    uint64_t domain_id;
    uint64_t magnitude;
    __uint128_t actuation_token;
} __attribute__((packed)) AuthorizedEffect;
```

**Size:** 24 bytes  
**Alignment:** Packed (no padding)  
**Byte order:** Little-endian (x86_64 native)

**Fields:**
- `domain_id` (8 bytes): Domain identifier
- `magnitude` (8 bytes): Action magnitude
- `actuation_token` (16 bytes): Cryptographic authorization token

**Note:** The `AuthorizedEffect` structure contains authorization metadata only.  
Payload data (if needed for actuation) must be managed by the actuator bridge separately.  
AB-S authorizes based on domain and magnitude, not payload content.

---

## Write Semantics

### Authorized Effects

When AB-S authorizes an action:
1. SLIME constructs `AuthorizedEffect` structure
2. SLIME writes 24 bytes to socket (single write call)
3. SLIME does not wait for acknowledgment
4. SLIME continues to next action

**Ordering:** Preserved (FIFO)  
**Buffering:** Kernel socket buffer (typically 64KB)  
**Framing:** None (fixed 32-byte messages)

### Impossibilities

When AB-S blocks an action (`Err(Impossibility)`):
- **No socket write occurs**
- No error is emitted
- No signal is sent  
- No observable side effect

Impossibility is a **terminal non-event**.

---

## Failure Modes

### Socket Unavailable at Startup

If `/run/slime/egress.sock` cannot be created:
- SLIME fails to start
- Error logged to stdout/stderr
- Process exits with non-zero code

This is **fail-closed by construction**.

### Socket Disconnected During Operation

If the actuator bridge disconnects:
- SLIME continues accepting actions
- Authorized effects buffer in kernel socket buffer
- When buffer fills, writes block (backpressure)
- While blocked, ingress continues but effects queue
- No retry mechanism exists

**Critical:** SLIME does not bypass a failed socket.

### Write Failure

If `write()` system call fails:
- Effect is **dropped** (not retried)
- No error propagation to ingress
- No fallback mechanism
- Dashboard may log write failure (observation only)

This is **fail-closed**.

---

## Environment Responsibilities

The environment must:

1. **Create actuator bridge** that connects to `/run/slime/egress.sock`
2. **Read 32-byte `AuthorizedEffect` messages** from socket
3. **Map effect to actuation** based on domain_id and magnitude
4. **Perform mechanical actuation** in the world
5. **Handle disconnection** (reconnect logic is environment's responsibility)

**Payload handling:**
- `AuthorizedEffect` contains only authorization metadata (domain, magnitude, token)
- If actuation requires payload data, the actuator bridge must:
  - Maintain correlation between ingress actions and egress effects (e.g., via domain_id)
  - Store payload separately if needed
  - Or reconstruct actuation from domain_id + magnitude alone

SLIME guarantees:
- Socket exists while SLIME is running
- Every write is exactly 24 bytes
- Effects are written in order of authorization
- No writes occur for impossibilities
- Authorization token is cryptographically valid

---

## Implementation Examples

### Python Actuator Bridge

```python
import socket
import struct

SOCKET_PATH = '/run/slime/egress.sock'

def actuator_bridge():
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(SOCKET_PATH)
    
    try:
        while True:
            # Read exactly 24 bytes
            data = sock.recv(24)
            if len(data) != 24:
                break
            
            # Unpack AuthorizedEffect
            domain_id, magnitude, token_low = struct.unpack('<QQQ', data)
            # Note: token is 128-bit, only lower 64 bits shown
            
            # Perform actuation
            actuate(domain_id, magnitude)
    finally:
        sock.close()

def actuate(domain_id, magnitude):
    # Mechanical actuation logic
    print(f"Actuating domain {domain_id} with magnitude {magnitude}")

if __name__ == '__main__':
    actuator_bridge()
```

### C Actuator Bridge

```c
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

#define SOCKET_PATH "/run/slime/egress.sock"

typedef struct __attribute__((packed)) {
    uint64_t domain_id;
    uint64_t magnitude;
    __uint128_t actuation_token;
} AuthorizedEffect;

void actuate(AuthorizedEffect *effect) {
    printf("Actuating domain %lu with magnitude %lu\n",
           effect->domain_id, effect->magnitude);
    // Mechanical actuation implementation
}

int main() {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) {
        perror("socket");
        return 1;
    }
    
    struct sockaddr_un addr = {
        .sun_family = AF_UNIX,
        .sun_path = SOCKET_PATH
    };
    
    if (connect(fd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("connect");
        close(fd);
        return 1;
    }
    
    AuthorizedEffect effect;
    ssize_t n;
    
    while ((n = read(fd, &effect, sizeof(effect))) == sizeof(effect)) {
        actuate(&effect);
    }
    
    if (n < 0) {
        perror("read");
    }
    
    close(fd);
    return 0;
}
```

### Rust Actuator Bridge

```rust
use std::os::unix::net::UnixStream;
use std::io::Read;

const SOCKET_PATH: &str = "/run/slime/egress.sock";

#[repr(C, packed)]
struct AuthorizedEffect {
    domain_id: u64,
    magnitude: u64,
    actuation_token: u128,
}

fn actuate(effect: &AuthorizedEffect) {
    println!("Actuating domain {} with magnitude {}",
             effect.domain_id, effect.magnitude);
    // Mechanical actuation implementation
}

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let mut buffer = [0u8; 24];
    
    loop {
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let effect = unsafe {
                    std::ptr::read(buffer.as_ptr() as *const AuthorizedEffect)
                };
                actuate(&effect);
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}
```

---

## Systemd Integration

SLIME and actuator bridge can run as separate systemd services:

**SLIME service:** `/etc/systemd/system/slime.service`
```ini
[Unit]
Description=SLIME v0
After=network.target

[Service]
Type=simple
User=slime
ExecStart=/usr/local/bin/slime
RuntimeDirectory=slime
RuntimeDirectoryMode=0755
UMask=0007
Restart=always

[Install]
WantedBy=multi-user.target
```

**Actuator bridge service:** `/etc/systemd/system/actuator-bridge.service`
```ini
[Unit]
Description=Actuator Bridge for SLIME
After=slime.service
Requires=slime.service

[Service]
Type=simple
User=actuator
Group=slime
SupplementaryGroups=slime
ExecStart=/usr/local/bin/actuator-bridge
Restart=always

[Install]
WantedBy=multi-user.target
```

**Note:** 
- `RuntimeDirectory=slime` in SLIME service ensures `/run/slime` is created automatically
- `User=actuator` with `SupplementaryGroups=slime` allows socket access via group membership
- Process isolation: actuator bridge runs as different user than SLIME

---

## Security Considerations

### Permissions

The socket has `0660` permissions (owner and group read/write).

**Access control:**
- User `slime` (owner) can read/write
- Members of group `slime` can read/write
- All others have no access

**Recommended deployment:**

Run actuator bridge as separate user, add to `slime` group:
```bash
# Create actuator user
sudo useradd --system --no-create-home actuator

# Add to slime group
sudo usermod -a -G slime actuator

# Run bridge as actuator user
sudo -u actuator /usr/local/bin/actuator-bridge
```

This provides **process isolation** while allowing socket access via group membership.

### No Authentication

The socket itself provides **no authentication** beyond Unix permissions.

Authorization is cryptographic (via `actuation_token`), not socket-based.

**Critical:** The actuator bridge must verify the authenticity of effects using the `actuation_token` if operating in adversarial environments.

### Socket Exhaustion

If no actuator connects, the kernel socket buffer (typically 64KB) fills up.

Once full, SLIME's writes block, creating backpressure to ingress.

This is **fail-closed** behavior - no effects are dropped silently.

---

## Monitoring

### Socket Status

Check if socket exists:
```bash
ls -l /run/slime/egress.sock
```

Expected output:
```
srw-rw---- 1 slime slime 0 Feb  7 10:30 /run/slime/egress.sock
```

### Connection Status

Check if actuator is connected:
```bash
lsof /run/slime/egress.sock
```

Should show both SLIME (listening) and actuator bridge (connected).

### Write Failures

SLIME may log write failures to stdout/stderr (observation only):
```json
{"ts":"2026-02-07T10:23:45Z","level":"WARN","event":"egress_write_failed","errno":32}
```

**Note:** Logging is for observation, not operational control.

---

## Prohibitions (Non-Negotiable)

The following are **structurally impossible** in SLIME v0:

- No alternative socket paths
- No dynamic endpoint configuration
- No network egress (TCP/UDP)
- No HTTP webhook fallback
- No acknowledgment protocol
- No backpressure signals from actuator to SLIME
- No retry logic
- No error channels
- No configuration files

Any request to add these features **violates canon**.

---

## Canonical Statement

> **Egress is fixed.**  
> **Authorization produces a write.**  
> **Impossibility produces silence.**

---

**END — SLIME v0 EGRESS SOCKET SPECIFICATION**

This specification is **non-negotiable** and forms part of the SLIME v0 canonical interface.
