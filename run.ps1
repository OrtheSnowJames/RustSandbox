#use script dir
$originalPath = Get-Location
$scriptPath = $MyInvocation.MyCommand.Path
$scriptDir = Split-Path -Parent $scriptPath
Set-Location $scriptDir

notepad envvars.ps1
. .\envvars.ps1

cargo run
Set-Location $originalPath