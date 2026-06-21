$ErrorActionPreference = "Stop"

$version = $env:GITHUB_REF_NAME
if ([string]::IsNullOrWhiteSpace($version)) {
    $version = "0.0.0"
}
$version = $version.TrimStart("v")
if ($version -notmatch '^\d+\.\d+\.\d+(\.\d+)?$') {
    $manifest = Get-Content "crates\aktsql_app\Cargo.toml"
    $versionLine = $manifest | Where-Object { $_ -match '^version\s*=' } | Select-Object -First 1
    $version = ($versionLine -replace 'version\s*=\s*"', '') -replace '"', ''
}

$root = (Resolve-Path ".").Path
$sourceDir = Join-Path $root "target\release"
$dist = Join-Path $root "dist\windows"
$exe = Join-Path $sourceDir "aktsql.exe"
$wxs = Join-Path $root "packaging\windows\AktSQL.wxs"
$wixObj = Join-Path $dist "AktSQL.wixobj"
$stableExe = Join-Path $dist "AktSQL-windows-x64.exe"
$stableMsi = Join-Path $dist "AktSQL-windows-x64.msi"
$versionedExe = Join-Path $dist "AktSQL-$version-windows-x64.exe"
$versionedMsi = Join-Path $dist "AktSQL-$version-windows-x64.msi"

New-Item -ItemType Directory -Force -Path $dist | Out-Null
Copy-Item $exe $stableExe -Force
Copy-Item $exe $versionedExe -Force

$wixBin = "${env:ProgramFiles(x86)}\WiX Toolset v3.14\bin"
if (Test-Path $wixBin) {
    $env:PATH = "$env:PATH;$wixBin"
}

candle.exe -dSourceDir="$sourceDir" -dProductVersion="$version" -out "$wixObj" "$wxs"
light.exe -ext WixUIExtension -out "$stableMsi" "$wixObj"
Copy-Item $stableMsi $versionedMsi -Force

Remove-Item "$dist\*.wixobj" -Force
Remove-Item "$dist\*.wixpdb" -Force -ErrorAction SilentlyContinue
