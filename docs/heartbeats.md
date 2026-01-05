# Heartbeats sent by Anytime

```json
POST /api/v1/users/current/heartbeats.bulk
accept: application/json
content-type: application/json
user-agent: wakatime/unset (linux-none-none) anytime/0.1.0 anytime-wakatime/0.1.0 (github.com/iamawatermelo/Anytime, f=hfl;hbl;hpl;noam;nsr;npn;tir)

[{
  "entity": "Anytime/docs/heartbeats.md",
  "type": "file",
  "time": 1767569198.0,
  "category": "coding",
  "project": "Anytime",
  "branch": "witch",
  "language": "Markdown",
  "lines": 27,
  "lineno": 18,
  "cursorpos": 18,
  "is_write": true,
  "machine": "playground",
  "operating_system": "linux",
  "editor": "anytime",
  "plugin": "anytime",
  "x-anytime-metadata": {
    "flags": ["hfl", "hbl", "hpl", "noam", "nsr", "npn", "tir"],
    "sti": 396032,
    "rti": 28800
    "skf": 36,
    "xcs": 101.82
    "mcs": 98.82
    "lcs": [101.82, 98.82, 100.17, 99.37, 102.01]
  }
}]
```

## Anytime metadata

- `flags`: also present in UA. See [Flags]
- `sti`: system time in seconds Anytime has been running
- `rti`: monotonic time in seconds Anytime has been running
- `skf`: forward time irregularities (instances where time skipped forward, i.e because the device went to sleep)
- `xcs`: max byt/sec detected by Anytime (may be negative)
- `mcs`: min byt/sec detected by Anytime (may be negative)
- `acs`: avg byt/sec detected by Anytime (may be negative)

## Flags

- `hfl`: filename obfuscation enabled
- `hbl`: branch name obfuscation enabled
- `hpl`: project name obfuscation enabled
- `noam`: machine obfuscation enabled
- `nsr`: non-standard heartbeat interval used
- `npn`: project name was overridden by a file
- `tir`: time went backwards while Anytime was open