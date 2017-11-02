#Injektilo
A proof of concept simple command runner designed to parse a json file hosted anywhere for commands to execute on the host. Can be paired with powershell for maximum effect.

example json to define a powershell command:
```
{"cmd":"powershell","args":["-NoExit", "-ExecutionPolicy ByPass", "-Command Get-ChildItem"]}
```
example json to define a bash command:
```
{"cmd":"sh","args":["-c", "ls -laf"]}
```

The url to query for json can be added to the slice where you find the pastebin link(the current pastebin link contains the example for bash)

Always remember to compile with the release version and use gnu strip afterward to acheive a 539kb binary(compared to the unstripped 3MB).
