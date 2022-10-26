echo LIBRARY APPHELP > apphelp.def
echo EXPORTS >> apphelp.def
for /f "skip=19 tokens=4" %%A in ('dumpbin /exports C:\Windows\system32\apphelp.dll') do echo %%A >> apphelp.def
lib /def:apphelp.def /out:apphelp.lib /machine:x64
