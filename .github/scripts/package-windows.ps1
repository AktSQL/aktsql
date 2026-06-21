$ErrorActionPreference = "Stop"

$version = $env:GITHUB_REF_NAME
if ([string]::IsNullOrWhiteSpace($version)) {
    $version = "0.0.0"
}
$version = $version.TrimStart("v")

$root = (Resolve-Path ".").Path
$sourceDir = Join-Path $root "target\release"
$dist = Join-Path $root "dist\windows"
$exe = Join-Path $sourceDir "aktsql.exe"
$wxs = Join-Path $root "packaging\windows\AktSQL.wxs"
$wixObj = Join-Path $dist "AktSQL.wixobj"
$msi = Join-Path $dist "AktSQL-$version-x64.msi"

New-Item -ItemType Directory -Force -Path $dist | Out-Null
Copy-Item $exe (Join-Path $dist "aktsql.exe") -Force

$wixBin = "${env:ProgramFiles(x86)}\WiX Toolset v3.14\bin"
if (Test-Path $wixBin) {
    $env:PATH = "$env:PATH;$wixBin"
}

candle.exe -dSourceDir="$sourceDir" -dProductVersion="$version" -out "$wixObj" "$wxs"
light.exe -ext WixUIExtension -out "$msi" "$wixObj"

Remove-Item "$dist\*.wixobj" -Force
Remove-Item "$dist\*.wixpdb" -Force -ErrorAction SilentlyContinue
