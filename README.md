# RustyShim
Research-quality Shim Database builder and hooking using Rust.

This is all very early, designed more as a project to learn to use Rust than
a pratical implementation.

# Build
```bash
cargo build
```

# Confugure
Use [example.json](example.json) as a Guide. Unlike other SDB Builders,
this one directly maps to the [SDB API](https://learn.microsoft.com/en-us/windows/win32/devnotes/sdbcreatedatabase).

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
https://github.com/pathtofile/rustyshim

https://learn.microsoft.com/en-us/windows/win32/devnotes/sdbcreatedatabase

https://github.com/microsoft/windows-rs
https://docs.rs/winapi/0.3.7/winapi/index.html

https://learn.microsoft.com/en-us/windows-hardware/get-started/adk-install


https://fleexlab.blogspot.com/2020/12/turning-compatibility-administrator.html

https://stackoverflow.com/questions/1711665/windows-how-to-create-custom-appcompat-shims-application-fixes#1713385
https://github.com/Fleex255/CustomShim
https://github.com/Fleex255/CustomShim/blob/master/sdb.sprint


https://learn.microsoft.com/en-au/windows/win32/winprog/windows-data-types?redirectedfrom=MSDN#PCWSTR

https://github.com/microsoft/vscode-remote-release/issues/7272
https://www.geoffchappell.com/studies/windows/km/ntoskrnl/api/kshim/ksecore/index.htm

python .\py\sdb.py C:\Windows\apppatch\sysmain.sdb
base64.b64encode(uuid.uuid4().bytes)