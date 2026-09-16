# Architecture

Abyssal Abacus is a Cargo workspace with three crates. All the math lives in
one place; the CLI and GUI are thin front ends on top of it.

```
abyssal-abacus/
├── Cargo.toml                  <- workspace root, lists the three members
├── abyssal_abacus_core/        <- shared math + calculator state, no UI code
│   └── src/lib.rs
├── abyssal_abacus_cli/         <- terminal front end
│   └── src/main.rs
└── abyssal_abacus_gui/         <- retro desktop front end (egui/eframe)
    └── src/main.rs
```

## `abyssal_abacus_core`

Pure logic, no I/O, no UI dependencies. This is what makes it possible for
the CLI and GUI to share one source of truth for arithmetic while looking
and behaving completely differently.

It exposes two independent APIs because the CLI and GUI have different
input models:

- **`evaluate_expr(&str) -> Result<f64, String>`** - a one-shot expression
  parser used by the CLI. It scans a string like `"3+4"` or `"5*-3"` for
  the first "real" operator (`find_operator`), being careful to skip a
  leading `-` or a `-` immediately following another operator, since those
  are sign markers rather than subtraction. It then parses the two operand
  strings as `f64` and applies the operator (`apply_op`).

- **`CalcEngine`** - a small stateful "four-function" calculator engine
  used by the GUI, modeled the way a physical calculator works: it holds
  the string currently on the display, an accumulator, and a pending
  operator. Button/keyboard events (`input_digit`, `input_dot`,
  `input_operator`, `equals`, `backspace`, `toggle_sign`, `percent`,
  `clear`) mutate this state one at a time. Operators are evaluated
  left-to-right with no precedence (`5 + 2 * 3` = `(5+2)*3` = `21`), which
  matches both the reference design and how a physical calculator behaves.

Both APIs share the same operator semantics: **division and modulo by zero
resolve to `0` instead of returning an error.** This is a deliberate product
decision (see the README), not an oversight, and it's implemented once in
each of `apply_op` and `Op::apply` so the CLI and GUI can't drift apart.

`format_number` is the shared display formatter: whole-number results print
without a decimal point (`7` not `7.0`), fractional results are rounded to
10 decimal places and trimmed of trailing zeros, and non-finite results
(`NaN`/`inf` - not reachable via the current operators since division by
zero is special-cased) print as `"Error"`.

## `abyssal_abacus_cli`

Two modes, both calling `evaluate_expr`:

- **Interactive REPL** (no args): prints a prompt, reads a line, evaluates
  it, prints the result or an error, and loops. `exit`/`quit`
  (case-insensitive) or EOF (Ctrl-D) end the loop.
- **One-shot mode** (`abyssal-abacus-cli "3 + 4"`): joins all CLI args into
  one string, evaluates it, prints the result, and exits - a non-zero exit
  code on error so it's script-friendly.

## `abyssal_abacus_gui`

Built on [egui/eframe](https://github.com/emilk/egui), which renders
everything itself (no system GTK/Qt dependency), so the same code produces
the same look on Linux, macOS, and Windows. It wraps a `CalcEngine` and
never touches arithmetic directly - every button press and keystroke turns
into a call into `abyssal_abacus_core`.

Notable implementation details:

- **Custom-drawn window chrome.** Decorations are turned off
  (`with_decorations(false)`) and the app paints its own title bar, so it
  can match the retro black/red theme. Dragging the bar moves the window;
  double-clicking toggles maximize; a hand-drawn grip in the bottom-right
  corner starts an interactive resize for window managers that don't give
  undecorated windows a resize border.
- **Scale-and-center layout.** The whole calculator (screen + keypad) is
  laid out once at a fixed logical size (`dims::CONTENT_W/H`) and then
  scaled and centered as one piece to fit whatever window size the user
  picks, instead of individual widgets repositioning independently. This
  keeps the screen and keypad from drifting apart as the window is resized.
- **Keyboard and mouse feed the same engine.** `handle_keyboard` translates
  key/text events into the same `CalcEngine` calls that button clicks use
  (`press`), so there is exactly one code path per operation regardless of
  input method.

## Why a workspace instead of one crate

Keeping `abyssal_abacus_core` dependency-free means:

- Its test suite (`cargo test -p abyssal_abacus_core`) compiles in a few
  seconds even though the GUI's dependency tree (egui/eframe and
  everything windowing/graphics-related they pull in) is large.
- A future third front end (e.g. a web/WASM build) could reuse
  `CalcEngine` without pulling in either the terminal or desktop UI code.

## Release pipeline

`.github/workflows/release.yml` builds and packages Linux, Windows, and
macOS binaries whenever a `vX.Y.Z` tag is pushed, and uploads them to a
GitHub Release. `release.sh` does the equivalent build+package step
locally (Linux + Windows-via-cross-compile) for testing packaging changes
without needing to push a tag.
