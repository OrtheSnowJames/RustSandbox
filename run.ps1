#use script dir
$originalPath = Get-Location
$scriptPath = $MyInvocation.MyCommand.Path
$scriptDir = Split-Path -Parent $scriptPath
Set-Location $scriptDir


cargo run
Set-Location $originalPath