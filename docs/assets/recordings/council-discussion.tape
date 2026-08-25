# Council Discussion demo (D-16 scenario 3 of 4).
#
# Drives examples/council_discussion.rs verbatim via
# `cargo run --example council_discussion`. The example uses a MockLlmAdapter —
# no API key, no network access. Regenerate with:
# vhs docs/assets/recordings/council-discussion.tape

Output docs/assets/recordings/council-discussion.gif

Require cargo

Set Shell "bash"
Set FontSize 13
Set Width 1160
Set Height 820
Set Theme "Dracula"
Set Padding 20
Set Framerate 20

Type "cargo run --example council_discussion"
Sleep 500ms
Enter

Sleep 85s
