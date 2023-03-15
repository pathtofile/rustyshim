# RustyShim
Research-quality Shim Database builder and hooking using Rust.

This is all very early, designed more as a project to learn to use Rust than
a pratical implementation.

# Build
```bash
cargo build
```
This will build 3 components:
- `sdbbuilder.exe` -> A program to compile JSON configuration into an SDB database (see below)
- `shim.dll` -> A DLL that can be used as an SDB Shim to inject into programs
- `shomee.exe` -> An example program to be shimmed


# Confugure
Use [example.json](example.json) as a Guide, which will hook all executions of programs named `shimee.exe`:
```json
{
  "name": "example.sdb",
  "data": [
    {"tag": "TAG_DATABASE", "type": "LIST", "value": [
      {"tag": "TAG_OS_PLATFORM", "type": "DWORD", "value": 4 },
      {"tag": "TAG_NAME", "type": "STRING", "value": "rsshim" },
      {"tag": "TAG_DATABASE_ID", "type": "GUID", "value": "{5b368b98-e1fb-4259-bb71-562c10120830}" },
      {"tag": "TAG_LIBRARY", "type": "LIST", "value": [
        {"tag": "TAG_SHIM", "type": "LIST", "value": [
          {"tag": "TAG_NAME", "type": "STRING", "value": "rsshimdll" },
          {"tag": "TAG_DLLFILE", "type": "STRING", "value": "AcRes.dll" },
          {"tag": "TAG_FIX_ID", "type": "STRING", "value": "shim_func" },
          {"tag": "TAG_RUNTIME_PLATFORM", "type": "DWORD", "value": 39 },
          {"type": "NULL", "tag": "TAG_GENERAL"}
        ]}
      ]},
      {"tag": "TAG_EXE", "type": "LIST", "value": [
        {"tag": "TAG_NAME", "type": "STRING", "value": "shimee.exe" },
        {"tag": "TAG_WILDCARD_NAME", "type": "STRING", "value": "shimee.exe" },
        {"tag": "TAG_RUNTIME_PLATFORM", "type": "DWORD", "value": 39 },
        {"tag": "TAG_EXE_ID", "type": "GUID", "value": "{5b1f2e34-ba7d-4af6-84a0-560938841e84}" },
        {"tag": "TAG_SHIM_REF", "type": "LIST", "value": [
          {"tag": "TAG_NAME", "type": "STRING", "value": "rsshimdll" }
        ]}
      ]}
    ]}
  ]
}
```

Unlike other SDB Builders, this one directly maps to the [SDB API](https://learn.microsoft.com/en-us/windows/win32/devnotes/sdbcreatedatabase).

# Running
If using example config, first, copy `./target/debug/shim.dll` to `C:\Windows\System32\AcRes.dll`. Then:
```bash
# Build database
./target/debug/sdbbuilder.exe -c path/to/config.json

# Install database
sdbinst example.sdb

# Run shimee binary and observe hook using DBGView
./target/debug/shimee.exe
```

# Uninstall
```bash
sdbinst -u example.sdb
```

# Refrences
 * https://gist.github.com/ariscop/f24ffc95a7a1767f8f83
 * https://github.com/pathtofile/rustyshim
 * https://learn.microsoft.com/en-us/windows/win32/devnotes/sdbcreatedatabase
 * https://github.com/microsoft/windows-rs
 * https://docs.rs/winapi/0.3.7/winapi/index.html
 * https://learn.microsoft.com/en-us/windows-hardware/get-started/adk-install
 * https://fleexlab.blogspot.com/2020/12/turning-compatibility-administrator.html
 * https://stackoverflow.com/questions/1711665/windows-how-to-create-custom-appcompat-shims-application-fixes#1713385
 * https://github.com/Fleex255/CustomShim
 * https://github.com/Fleex255/CustomShim/blob/master/sdb.sprint
 * https://learn.microsoft.com/en-au/windows/win32/winprog/windows-data-types?redirectedfrom=MSDN#PCWSTR
 * https://github.com/microsoft/vscode-remote-release/issues/7272
 * https://www.geoffchappell.com/studies/windows/km/ntoskrnl/api/kshim/ksecore/index.htm
