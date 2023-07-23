# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

test:
	cargo test --features "std"
