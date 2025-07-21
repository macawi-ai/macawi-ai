# External Dependencies Management

<div align="center">

## MACAWI AI Dependency Strategy

*Professional management of external repositories and dependencies*

</div>

---

## Directory Structure

```
macawi-ai/
├── external/           # Git submodules (version controlled)
├── references/         # Documentation and links only
└── vendor/            # Frozen snapshots for critical deps
    └── critical/      # Mission-critical vendored code
```

---

## Dependency Categories

### 🔐 Security Protocols
External protocol implementations and references

| Repository | Type | Location | Purpose |
|------------|------|----------|----------|
| metaspt | Submodule | `/external/metaspt` | Meta-SPT protocol reference implementation |
| protocol-standards | Reference | `/references/PROTOCOL_REFS.md` | Links to official standards |

### 🛠️ Development Tools
Tools that enhance our development workflow

| Repository | Type | Location | Purpose |
|------------|------|----------|----------|
| duckdb-tools | Submodule | `/external/duckdb-tools` | DuckDB utilities and extensions |
| mcp-servers | Reference | `/references/MCP_SERVERS.md` | Available MCP server implementations |

### 📚 Core Libraries
Fundamental libraries our systems depend on

| Repository | Type | Location | Purpose |
|------------|------|----------|----------|
| cybernetic-core | Vendor | `/vendor/critical/cybernetic-core` | Frozen cybernetic primitives |

---

## Management Commands

### Adding a Git Submodule
```bash
# Add new external dependency
git submodule add https://github.com/org/repo external/repo-name
git submodule update --init --recursive

# Pin to specific version
cd external/repo-name
git checkout v1.2.3
cd ../..
git add external/repo-name
git commit -m "Pin repo-name to v1.2.3"
```

### Updating Submodules
```bash
# Update all submodules
git submodule update --remote

# Update specific submodule
git submodule update --remote external/repo-name
```

### Vendoring Critical Dependencies
```bash
# Only vendor if absolutely necessary
cp -r external/critical-dep vendor/critical/
echo "Vendored from commit: $(cd external/critical-dep && git rev-parse HEAD)" \
  > vendor/critical/critical-dep/VENDOR_INFO.txt
```

---

## Decision Matrix

When to use each approach:

| Approach | When to Use | Example |
|----------|-------------|---------|
| **Submodule** | Active development dependency | Protocol implementations |
| **Reference** | Documentation or optional tools | Standards documents |
| **Vendor** | Critical frozen dependency | Core security libraries |

---

## Policies

### 1. Version Pinning
- All submodules MUST be pinned to specific tags/commits
- Document the reason for each version pin
- Review updates quarterly

### 2. Security Review
- All external code undergoes security review before inclusion
- Vendored code is audited line-by-line
- Submodules are reviewed at integration points

### 3. License Compliance
- Verify license compatibility before adding dependencies
- Maintain LICENSE file in vendor directory
- Prefer MIT, Apache 2.0, or BSD licenses

### 4. Minimal Dependencies
- Evaluate if functionality can be implemented internally
- Remove unused dependencies quarterly
- Document why each dependency is necessary

---

## Current External Dependencies

### Active Submodules
```bash
# Initialize after cloning
git submodule update --init --recursive
```

*None yet - to be added as needed*

### Reference Documentation
See `/references/` directory for:
- Protocol specifications
- Tool recommendations  
- Integration guides

### Vendored Code
See `/vendor/critical/README.md` for:
- Vendored dependencies list
- Reasons for vendoring
- Update procedures

---

## Adding New Dependencies

1. **Evaluate Need**
   - Can we implement this internally?
   - Is this truly necessary?
   - What's the maintenance burden?

2. **Security Review**
   - Check for known vulnerabilities
   - Review recent commit history
   - Verify maintainer reputation

3. **Choose Approach**
   - Submodule for active dependencies
   - Reference for documentation
   - Vendor only if critical and stable

4. **Document Decision**
   - Add to this file
   - Include in relevant README
   - Note in ARCHITECTURE.md if architectural

---

## Maintenance Schedule

- **Weekly**: Check for security updates
- **Monthly**: Review unused dependencies
- **Quarterly**: Update pinned versions
- **Annually**: Full dependency audit

---

<div align="center">
<sub>Clean dependencies, clean architecture</sub>
</div>