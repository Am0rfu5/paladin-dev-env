# Grove Routing demo (D-16 scenario 4 of 4).
#
# Drives examples/grove_routing.rs verbatim via `cargo run --example grove_routing`.
# The example wires an inline mock PaladinPort implementation and only exercises
# keyword-match routing — no API key, no network access. Regenerate with:
# vhs docs/assets/recordings/grove-routing.tape

Output docs/assets/recordings/grove-routing.gif

Require cargo

Set Shell "bash"
Set FontSize 14
Set Width 1160
Set Height 780
Set Theme "Dracula"
Set Padding 20
Set Framerate 20

Type "cargo run --example grove_routing"
Sleep 500ms
Enter

Sleep 60s
