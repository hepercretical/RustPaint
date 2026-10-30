param([switch]$SkipBuild,[string]$PackageName='RustPaint')
$ErrorActionPreference='Stop'
if($PackageName -notmatch '^RustPaint(?:-[A-Za-z0-9._-]+)?$'){throw 'PackageName must be RustPaint or RustPaint followed by a simple version suffix'}
$projectRoot=Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectRoot
if(Test-Path -LiteralPath (Join-Path $projectRoot '.tools\cargo\bin\cargo.exe')){
    $env:CARGO_HOME=Join-Path $projectRoot '.tools\cargo'
    $env:RUSTUP_HOME=Join-Path $projectRoot '.tools\rustup'
    $env:PATH="$env:CARGO_HOME\bin;$env:PATH"
}
$artifactRoot=Join-Path $projectRoot 'target\standalone'
if(-not $SkipBuild){
    $savedErrorAction=$ErrorActionPreference
    $ErrorActionPreference='Continue'
    & cargo build --release --locked --target-dir $artifactRoot
    $buildExit=$LASTEXITCODE
    $ErrorActionPreference=$savedErrorAction
    if($buildExit -ne 0){throw "Release build failed ($buildExit)"}
}
$packageRoot=Join-Path $projectRoot "dist\$PackageName"
$null=New-Item -ItemType Directory -Path $packageRoot -Force
$binary=Join-Path $packageRoot 'RustPaint.exe'
Copy-Item -LiteralPath (Join-Path $artifactRoot 'release\mspaint-port.exe') -Destination $binary -Force
$smoke=Start-Process -FilePath $binary -ArgumentList '--smoke-test' -WindowStyle Normal -Wait -PassThru
if($smoke.ExitCode -ne 0){throw "Packaged desktop smoke test failed ($($smoke.ExitCode))"}
@'
Rust Paint - standalone Windows x64 editor

Open RustPaint.exe. Rust, Cargo and the original mspaint.exe are not required.
Uses Windows' built-in USER32/GDI/GDI+ libraries. No Copilot or AI features.

Pencil, round brush, eraser, fill, color picker, lines, rectangles and ellipses.
Text: select Text and click the canvas; enter text and font size.
Selection: drag a rectangle, then drag inside it to move its pixels.
Copy/Cut/Paste: Ctrl+C/X/V. Select All: Ctrl+A. Delete selection: Delete.
Crop: Image > Crop. Resize: Ctrl+W. Canvas size: Ctrl+E.
Image menu: rotate, flip, invert colors and custom colors.
Undo/Redo: Ctrl+Z/Y. New/Open/Save: Ctrl+N/O/S. Save As: Ctrl+Shift+S.
Printing: Ctrl+P. View menu: Fit, 25%-800%. Ctrl+mouse wheel zooms;
mouse wheel pans vertically, Shift+wheel pans horizontally.
View > Light/Dark/Use Windows theme changes the editor appearance.
Hide/Show collapses the grouped toolbar; the canvas stays centered.
The bottom zoom slider and +/- buttons zoom around the viewport center.
Ctrl+wheel anchors zoom to the pointer. Ctrl+G toggles the 400%+ pixel grid.
Hold Shift for straight horizontal/vertical/diagonal lines or square/circle shapes.
Right-click a palette color for the secondary color; right-drag to draw with it.
Drag a picture file onto the editor, or run RustPaint.exe "picture.png".

Open and save PNG, JPEG, BMP, GIF and TIFF. Transparency is flattened to white;
GIF/TIFF input uses the first frame. Save to a new name to preserve originals.
The raster model is limited to 16 million pixels with bounded undo history.

This is a Rust Paint-style editor, not a byte-for-byte recreation of Microsoft's
application. It does not yet reproduce the original ribbon, layers, transparent
editing, free-form selection, every brush/style, scanner support or page setup.
'@ | Set-Content -LiteralPath (Join-Path $packageRoot 'README.txt') -Encoding UTF8
$zip=Join-Path $projectRoot "dist\$PackageName-windows-x64.zip"
Compress-Archive -Path (Join-Path $packageRoot '*') -DestinationPath $zip -Force
Get-Item -LiteralPath $binary,$zip | Select-Object FullName,Length
