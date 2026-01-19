# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

cc-glm:
    claude --settings ./.claude/settings.glm.json

cc-volc:
    claude --settings ./.claude/settings.volc.json

cc-nati:
    claude --settings ./.claude/settings.native.json
