$port = 8080

$connections = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue

if ($connections) {
    Write-Host "🔁 Restarting existing session..."

    $connections |
    Where-Object { $_.OwningProcess -ne 0 } |   # 🔥 ignore system process
    ForEach-Object {
        try {
            Stop-Process -Id $_.OwningProcess -Force -ErrorAction Stop
        }
        catch {
            Write-Host "⚠️ Could not stop PID $($_.OwningProcess)"
        }
    }

    Start-Sleep -Seconds 1

    trunk serve --port $port
}
else {
    Write-Host "🆕 First launch..."

    trunk serve --open --port $port
}
