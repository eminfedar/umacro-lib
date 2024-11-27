# umacro-lib
Simulate Keyboard &amp; Mouse events in Linux (X11 &amp; Wayland) with uinput.

# Example:
Mouse
```rust
// Create a virtual device:
let mut device = create_virtual_device()?;

// Wait for device initialization:
device.wait(200);

// Absolute mouse movement
device.mouse_move(250, 200)?;
device.wait(500);

// Mouse Down
device.mouse_down(Mouse::Left)?;
device.wait(1000);

// Relative Mouse Movement
device.mouse_move_relative(150, 150)?;
device.wait(500);

// Mouse Up
device.mouse_up(Mouse::Left)?;
device.wait(500);

// Mouse Up & Down (Click)
device.mouse_click(Mouse::Right)?;
```

Keyboard
```rust
let mut device = create_virtual_device()?;

device.wait(3000); // Wait 3 seconds to initialize keyboard

// This is case insensitive, it presses real keyboard buttons, not sending chars.
// So you can't write emojis and special characters here.
// 10 is wait milliseconds between key presses
device.key_write("hello world", 10)?;

// Key Press & Release (Click)
device.key(Key::Space)?;

//If you want to write BIG chars, press down shift before, then release.
device.key_down(Key::LeftShift)?;
device.key_write("big hello world", 200)?; // result: "BIG HELLO WORLD"
device.key_up(Key::LeftShift)?;
```