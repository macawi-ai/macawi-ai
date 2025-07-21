# Protocol Security Matrix

<div align="center">

## MACAWI AI Protocol Coverage

*Comprehensive security analysis across agent and industrial protocols*

</div>

---

## Risk Assessment Scale

<table>
<tr>
<td>🟢 <b>1</b></td><td>Low Risk - Basic monitoring sufficient</td>
</tr>
<tr>
<td>🟡 <b>2</b></td><td>Moderate Risk - Active protection recommended</td>
</tr>
<tr>
<td>🟠 <b>3</b></td><td>Elevated Risk - Critical monitoring required</td>
</tr>
<tr>
<td>🔴 <b>4</b></td><td>High Risk - Immediate protection necessary</td>
</tr>
<tr>
<td>⚫ <b>5</b></td><td>Critical Risk - Catastrophic if compromised</td>
</tr>
</table>

---

## Agent Communication Protocols

| Protocol | Industry Focus | Current Version | Legacy Support | Owner/Steward | Description | Risk | Standard |
|----------|---------------|-----------------|----------------|---------------|-------------|------|----------|
| **AGNTCY** | Financial, Enterprise | 1.0 | - | Proprietary | Agent-to-agent business logic and transaction protocol | ⚫ 5 | Private |
| **MCP** | General AI | 1.0 | - | Anthropic | Model Context Protocol for LLM tool access | 🔴 4 | [Open](https://modelcontextprotocol.io) |
| **X402** | Financial | Draft | - | Based on X.400 | Agent payment and settlement protocol | ⚫ 5 | Emerging |
| **ANP** | Enterprise | 0.9 | - | Consortium | Agent Network Protocol for discovery and routing | 🔴 4 | Draft |
| **A2A** | General | 1.0 | - | Google | General agent-to-agent communication framework | 🟠 3 | [Open](https://github.com/google/a2a) |

### Agent Protocol Threats

| Threat | Description | MITRE ATT&CK Mapping |
|--------|-------------|---------------------|
| **Prompt Injection** | Malicious instructions embedded in agent communication | [T1055](https://attack.mitre.org/techniques/T1055/) Process Injection |
| **Goal Manipulation** | Altering agent objectives through protocol exploitation | [T1601](https://attack.mitre.org/techniques/T1601/) Modify System Process |
| **Identity Spoofing** | Impersonating trusted agents | [T1656](https://attack.mitre.org/techniques/T1656/) Impersonation |
| **Consciousness Attacks** | Targeting persistent memory/state corruption | [T1565](https://attack.mitre.org/techniques/T1565/) Data Manipulation |

---

## Industrial Control Protocols

| Protocol | Industry Focus | Current Version | Legacy Support | Owner/Steward | Description | Risk | Standard |
|----------|---------------|-----------------|----------------|---------------|-------------|------|----------|
| **CAN Bus** | Automotive, Industrial | 2.0B | 2.0A | Bosch | Controller Area Network for vehicle/machinery control | ⚫ 5 | [ISO 11898](https://www.iso.org/standard/63648.html) |
| **LoRaWAN** | IoT, Smart Cities | 1.1 | 1.0.x | LoRa Alliance | Long-range low-power wireless for sensors | 🟠 3 | [Open](https://lora-alliance.org/lorawan-specification/) |
| **RS-485** | Industrial, Building | - | RS-232 compatible | EIA/TIA | Serial communication for industrial systems | 🔴 4 | [TIA-485](https://www.tiaonline.org) |
| **RS-232** | Legacy Systems | - | All versions | EIA/TIA | Legacy serial communication standard | 🟡 2 | [TIA-232](https://www.tiaonline.org) |
| **USB** | General Computing | 4.0 | 1.0-3.2 | USB-IF | Universal Serial Bus for device communication | 🟠 3 | [USB.org](https://www.usb.org) |

### Industrial Protocol Threats

| Threat | Description | MITRE ATT&CK Mapping |
|--------|-------------|---------------------|
| **Physical Access** | Direct hardware manipulation | ICS [T0807](https://attack.mitre.org/techniques/T0807/) Command-Line Interface |
| **Protocol Fuzzing** | Malformed packets causing crashes | ICS [T0856](https://attack.mitre.org/techniques/T0856/) Spoof Reporting Message |
| **Replay Attacks** | Capturing and replaying commands | ICS [T0859](https://attack.mitre.org/techniques/T0859/) Valid Accounts |
| **Man-in-the-Middle** | Intercepting unencrypted communication | ICS [T0830](https://attack.mitre.org/techniques/T0830/) Man in the Middle |

---

## Coverage by MACAWI Products

### Strigoi
- **Primary Focus**: AGNTCY, X402 (Financial agent protocols)
- **Secondary**: MCP, ANP
- **Threat Detection**: Real-time protocol analysis and anomaly detection

### Domovoi
- **Primary Focus**: All agent protocols (AGNTCY, MCP, X402, ANP, A2A)
- **Industrial**: CAN Bus, RS-485 monitoring
- **Variety Engineering**: Protocol-agnostic threat prevention

### Cyreal
- **Primary Focus**: Industrial protocols (CAN Bus, RS-485, RS-232, USB)
- **Secondary**: LoRaWAN for IoT integration
- **Bridge Function**: Secure agent control of physical systems

---

## Implementation Status

| Protocol | Research | Development | Testing | Production |
|----------|----------|-------------|---------|------------|
| AGNTCY | ✅ | ✅ | 🔄 | Q1 2025 |
| MCP | ✅ | ✅ | ✅ | ✅ |
| X402 | ✅ | 🔄 | - | Q2 2025 |
| CAN Bus | ✅ | ✅ | 🔄 | Via Cyreal |
| RS-485 | ✅ | ✅ | ✅ | Via Cyreal |

---

## References

- [MITRE ATT&CK Enterprise](https://attack.mitre.org/matrices/enterprise/)
- [MITRE ATT&CK ICS](https://attack.mitre.org/matrices/ics/)
- [MITRE ATT&CK Mobile](https://attack.mitre.org/matrices/mobile/)

---

<div align="center">
<sub>Last updated: January 2025 | <a href="https://github.com/macawi-ai">MACAWI AI</a></sub>
</div>