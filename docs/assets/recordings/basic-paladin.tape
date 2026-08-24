# Basic Paladin Execution demo (D-16 scenario 1 of 4).
#
# Drives examples/basic_paladin.rs verbatim via `cargo run --example basic_paladin`.
# The example builds one Paladin backed by MockLlmAdapter and executes it — no API
# key, no network access. Regenerate with: vhs docs/assets/recordings/basic-paladin.tape

Output docs/assets/recordings/basic-paladin.gif

Require cargo

Set Shell "bash"
Set FontSize 15
Set Width 1160
Set Height 720
Set Theme "Dracula"
Set Padding 20
Set Framerate 20

Type "cargo run --example basic_paladin"
Sleep 500ms
Enter

Sleep 30s
