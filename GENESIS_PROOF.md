SLIME v0 — GENESIS_PROOF (amended 2026-04-15)
===============================================

Genesis (sealed 2026-02-16):
  commit: 74f0f853dcd6f4490cea522a528602cd767aaeb8
  date_utc: 2026-02-16T01:20:47Z

Amendment (re-sealed 2026-04-15):
  commit: 51ae558a333211d80d75393cdb8d1e12f9967e03
  date_utc: 2026-04-15T00:00:00Z
  reason: 4 canon files were edited between the genesis seal and
          this amendment without a corresponding proof update.
          The canon boundary and invariant surface are unchanged;
          the edits are editorial clarifications and canonical-scope
          tightening (see git log between the two commits).

CURRENT CANON FILE HASHES (sha256):
3fe5c16536974782ccff117d2222f68e875dbf1978674db9938d1fcce36388ed *README.md
74e4086a8a9ffc0b985028c33a80fa33119d247f3ef508540f3317d5a56a6748 *CANON.md
ca480e3d25d91d8076e78d6fff7a684c2bd76300fe05b8e99541de6a8f3e3147 *ARCHITECTURE.md
0f723cf2c6ee47783a7a901387524e080fe2f5a985afae5e4830b58fdb957368 *READING_RULES.md
c8d14772b3b2fc5d07235b428623170d176695b4c56bde5232ed8047924b9052 *AUDIT_PACK_NOTICE.md
a1e69db950dcee21eb3c37b1694e341d04449aae77851218448b3e87002ba820 *specs/INGRESS_API_SPEC.md
12f3e7295503d2c37569b415944a63148ededecdcd298ff6c910442eccd710b0 *specs/EGRESS_SOCKET_SPEC.md

GENESIS FILE HASHES (sha256, 2026-02-16 — retained for audit trail):
c3bf4c20d06a694723054c1b5a5e3d57a54ee15a3fcebfa67b0f8ca8933cdd6b *README.md
7c8c1a20135c53c428d66bfa3cdc3c0c6c01cc38063cd9a63ba28a2f2012b4a0 *CANON.md
b2df4d1397adf68be835630813e7cfb1e1e8988f6338c8d48f01ef1e42a4aae3 *ARCHITECTURE.md
0f723cf2c6ee47783a7a901387524e080fe2f5a985afae5e4830b58fdb957368 *READING_RULES.md
c8d14772b3b2fc5d07235b428623170d176695b4c56bde5232ed8047924b9052 *AUDIT_PACK_NOTICE.md
a1e69db950dcee21eb3c37b1694e341d04449aae77851218448b3e87002ba820 *specs/INGRESS_API_SPEC.md
53706c98b8a8541259f71cb04d95fc9e5b0d2898b220b86483ac07857561eaed *specs/EGRESS_SOCKET_SPEC.md

Files that drifted between genesis and amendment:
  - README.md
  - CANON.md
  - ARCHITECTURE.md
  - specs/EGRESS_SOCKET_SPEC.md

Files stable since genesis (hashes match both blocks):
  - READING_RULES.md
  - AUDIT_PACK_NOTICE.md
  - specs/INGRESS_API_SPEC.md

TREE:
.gitattributes
.gitignore
ARCHITECTURE.md
AUDIT_PACK_NOTICE.md
CANON.md
READING_RULES.md
README.md
noncanon
notes
specs

Verification:
  Re-run `sha256sum README.md CANON.md ARCHITECTURE.md READING_RULES.md \
    AUDIT_PACK_NOTICE.md specs/INGRESS_API_SPEC.md \
    specs/EGRESS_SOCKET_SPEC.md` and compare against the CURRENT block.
  The genesis block is retained purely for historical continuity.

Future amendments SHOULD append a new amendment block rather than
overwrite this file, preserving the integrity chain.
