# Internal G1-G9 PR queue (WhatsFast). Run from repo root.
$ErrorActionPreference = 'Continue'
$repo = Split-Path $PSScriptRoot -Parent
Set-Location $repo

$env:GIT_AUTHOR_NAME = 'LisandroNahuelH'
$env:GIT_AUTHOR_EMAIL = '226882487+LisandroNahuelH@users.noreply.github.com'
$env:GIT_COMMITTER_NAME = $env:GIT_AUTHOR_NAME
$env:GIT_COMMITTER_EMAIL = $env:GIT_AUTHOR_EMAIL

$groups = @(
    @{ n = 2; branch = 'feat/g2-poll-toggle'; title = 'feat(settings): optional create poll button'; commits = @('aeb0cf5'); bullet = 'Settings can hide the create poll button beside the attachment menu.' }
    @{ n = 3; branch = 'feat/g3-forward-order'; title = 'feat(settings): forward messages in order'; commits = @('e16497f', '43e042d'); bullet = 'Forwarded batches can send one message at a time to preserve order, with a Settings toggle.' }
    @{ n = 4; branch = 'feat/g4-settings-toggle'; title = 'feat(ui): toggle settings panel closed'; commits = @('c25ae59'); bullet = 'Click the active settings section again to close the panel.' }
    @{ n = 5; branch = 'feat/g5-starred'; title = 'feat(starred): starred messages list'; commits = @('ef545b7', '3029901', '01b065e', 'f1a48ef', '41e6f91'); bullet = 'Star messages from the chat and browse them in a sidebar list with bubble previews.' }
    @{ n = 6; branch = 'feat/g6-schedule'; title = 'feat(chat): scheduled messages'; commits = @('f828c91', '469a41e', '2c79740', 'afd3051'); bullet = 'Schedule messages once or on a repeat rule from the composer clock control.' }
    @{ n = 7; branch = 'feat/g7-compact-rail'; title = 'feat(ui): compact sidebar rail'; commits = @('55d3c3b', '3f94455', '8c3d01e', 'd825e04'); bullet = 'A narrow sidebar rail keeps navigation visible in every view.' }
    @{ n = 8; branch = 'feat/g8-pin-drag'; title = 'feat(chats): reorder pinned chats by drag'; commits = @('c09b108', 'd9f69e0', 'fde9f92', '7a58eda'); bullet = 'Hold and drag pinned chats to reorder them on the chat list.' }
    @{ n = 9; branch = 'feat/g9-select-messages'; title = 'feat(chat): multi-select messages'; commits = @('d887a80', '07febcb', 'e8699f2', '6a7f7b9', 'ca353b9'); bullet = 'Select multiple messages from the bubble menu or row, then forward, download, or star the batch.' }
)

function Add-ReadmeBullet($bullet) {
    $path = Join-Path $repo 'README.md'
    $text = Get-Content $path -Raw
    if ($text -match [regex]::Escape($bullet)) { return }
    $needle = "## WhatsFast improvements`r`n`r`n"
    if ($text -notmatch '## WhatsFast improvements') { throw 'README missing improvements section' }
    $insert = "## WhatsFast improvements`r`n`r`n- $bullet`r`n"
    if ($text -match '- ') {
        $text = $text -replace '(## WhatsFast improvements\r?\n\r?\n)', "`$1- $bullet`r`n"
    } else {
        $text = $text -replace '(## WhatsFast improvements\r?\n\r?\n)', $insert
    }
    Set-Content -Path $path -Value $text -NoNewline -Encoding utf8
}

git fetch origin main
git fetch local-zapfast local/select-messages

function Invoke-CherryPick($hash) {
    git cherry-pick $hash
    if ($LASTEXITCODE -eq 0) { return }
    $unmerged = git diff --name-only --diff-filter=U
    if ($unmerged) {
        foreach ($f in $unmerged) {
            git checkout --theirs -- $f
            git add -- $f
        }
        git cherry-pick --continue --no-edit
    }
    if ($LASTEXITCODE -ne 0) {
        git cherry-pick --skip
    }
    if ($LASTEXITCODE -ne 0) { throw "cherry-pick failed for $hash" }
}

foreach ($g in $groups) {
    Write-Host "=== G$($g.n) $($g.branch) ==="
    git checkout main 2>&1 | Out-Null
    git pull origin main
    git checkout -B $g.branch origin/main
    foreach ($c in $g.commits) {
        Invoke-CherryPick $c
    }
    Add-ReadmeBullet $g.bullet
    git add README.md
    git commit -m "docs(readme): note G$($g.n) improvement" -m $g.bullet
    git push -u origin $g.branch -f
    $bodyFile = Join-Path $repo "AGENTS/pr-bodies/g$($g.n).md"
    if (-not (Test-Path (Split-Path $bodyFile))) { New-Item -ItemType Directory -Force -Path (Split-Path $bodyFile) | Out-Null }
    Set-Content $bodyFile "- $($g.bullet)" -Encoding utf8
    gh pr create --repo LisandroNahuelH/whatsfast --base main --head $g.branch --title $g.title --body-file $bodyFile
    $num = gh pr list --repo LisandroNahuelH/whatsfast --head $g.branch --json number --jq '.[0].number'
    if (-not $num) { throw "PR not created for $($g.branch)" }
    gh pr merge $num --repo LisandroNahuelH/whatsfast --merge
    git checkout main
    git pull origin main
}

Write-Host 'G1-G9 queue done.'
