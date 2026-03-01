#!/usr/bin/env just --justfile
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]
bin_name := "upnpc"
dist_dir := "target/dist"


build:
    cargo build --release
    just _package

[private]
[windows]
_package:
        if (!(Test-Path "{{ dist_dir }}")) { New-Item -ItemType Directory -Path "{{ dist_dir }}" | Out-Null }
        $staging = "{{ dist_dir }}/_staging"; \
        if (Test-Path $staging) { Remove-Item $staging -Recurse -Force }; \
        New-Item -ItemType Directory -Path $staging | Out-Null; \
        Copy-Item "target/release/{{ bin_name }}.exe" $staging; \
        $zip = "{{ dist_dir }}/upnpc-{{ os() }}-{{ arch() }}.zip"; \
        if (Test-Path $zip) { Remove-Item $zip -Force }; \
        Compress-Archive -Path "$staging/*" -DestinationPath $zip; \
        Remove-Item $staging -Recurse -Force; \
        Write-Host "Packaged: $zip"



[private]
[unix]
_package:
    mkdir -p "{{ dist_dir }}/_staging"
    cp "target/release/{{ bin_name }}" "{{ dist_dir }}/_staging/"
    cd "{{ dist_dir }}/_staging" && zip -r "../upnpc-{{ os() }}-{{ arch() }}.zip" .
    rm -rf "{{ dist_dir }}/_staging"
    @echo "Packaged: {{ dist_dir }}/upnpc-{{ os() }}-{{ arch() }}.zip"