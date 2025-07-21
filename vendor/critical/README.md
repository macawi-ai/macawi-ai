# Critical Vendored Dependencies

## Purpose

This directory contains frozen snapshots of mission-critical dependencies that:
1. Must remain stable regardless of upstream changes
2. Have undergone thorough security audit
3. Are essential for core functionality

---

## Vendoring Policy

Dependencies are vendored here ONLY when:
- ✅ They are absolutely critical to core security functions
- ✅ We need guaranteed stability (no surprise updates)
- ✅ Full source audit has been completed
- ✅ License permits redistribution
- ✅ Upstream reliability is questionable

---

## Current Vendored Dependencies

*None yet - this directory is reserved for future critical dependencies*

---

## Vendoring Process

1. **Justify the vendoring** in writing (create issue)
2. **Complete security audit** of source code
3. **Freeze at specific version** with clear documentation
4. **Create VENDOR_INFO.txt** with:
   - Original source URL
   - Commit hash/version
   - Date vendored
   - Reason for vendoring
   - Security audit results
5. **Update this README** with dependency info

---

## Update Procedure

Vendored code should rarely be updated. When necessary:

1. Create issue documenting need for update
2. Perform full security audit of changes
3. Test extensively in isolated environment
4. Update with full commit history
5. Document all changes

---

## Example Structure

```
vendor/critical/
├── README.md (this file)
├── example-lib/
│   ├── VENDOR_INFO.txt
│   ├── LICENSE
│   ├── src/
│   └── README.md
└── another-lib/
    ├── VENDOR_INFO.txt
    ├── LICENSE
    └── ...
```

---

<div align="center">
<sub>Stability through careful curation</sub>
</div>