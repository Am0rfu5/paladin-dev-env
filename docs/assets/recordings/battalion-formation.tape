# Battalion Formation demo (D-16 scenario 2 of 4).
#
# Drives examples/formation_sequential.rs verbatim via
# `cargo run --example formation_sequential`. The example wires an inline mock
# PaladinPort implementation — no API key, no network access. Regenerate with:
# vhs docs/assets/recordings/battalion-formation.tape

Output docs/assets/recordings/battalion-formation.gif

Require cargo

Set Shell "bash"
Set FontSize 14
Set Width 1160
Set Height 780
Set Theme "Dracula"
Set Padding 20
Set Framerate 20

Type "cargo run --example formation_sequential"
Sleep 500ms
Enter

Sleep 60s
