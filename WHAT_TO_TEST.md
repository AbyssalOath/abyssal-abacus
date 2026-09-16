# What to Test

Manual checklist for verifying a build before tagging a release. Automated
coverage for the shared math lives in `abyssal_abacus_core`'s test suite
(`cargo test -p abyssal_abacus_core`); this list is for things that need a
human (or at least a real terminal/window) to check.

## CLI - REPL mode

Run `abyssal-abacus-cli` with no arguments.

- [ ] Banner and prompt (`>`) are shown on start.
- [ ] Each operator works: `3+4` -> `7`, `10-3` -> `7`, `6*7` -> `42`,
      `20/4` -> `5`, `10%3` -> `1`.
- [ ] Decimal numbers: `10.5+2.5` -> `13`.
- [ ] Negative numbers: `-5+3` -> `-2`, `5*-3` -> `-15`.
- [ ] Division by zero: `10/0` -> `0` (no error).
- [ ] Modulo by zero: `10%0` -> `0` (no error).
- [ ] Invalid expression (e.g. `hello`) prints a friendly
      `Error: Invalid expression: ...` message and the REPL keeps running.
- [ ] Invalid number on one side (e.g. `3+abc`) prints
      `Error: Invalid number: 'abc'` and the REPL keeps running.
- [ ] Blank line input is ignored (no crash, prompt reappears).
- [ ] `exit` and `EXIT` (case-insensitive) print `Goodbye!` and quit.
- [ ] `quit` and `QUIT` (case-insensitive) print `Goodbye!` and quit.
- [ ] Ctrl-D (EOF) exits cleanly without printing an error.

## CLI - one-shot mode

- [ ] `abyssal-abacus-cli "3 + 4"` prints `7` and exits 0, no prompt shown.
- [ ] `abyssal-abacus-cli "10/0"` prints `0` and exits 0.
- [ ] `abyssal-abacus-cli "bogus"` prints an `Error: ...` message to stderr
      and exits non-zero (check with `echo $?`).
- [ ] Multi-argument input is joined with spaces, e.g.
      `abyssal-abacus-cli 3 + 4` behaves the same as `"3 + 4"`.

## GUI - mouse

Launch the GUI binary.

- [ ] Retro black/red theme renders correctly (title bar, screen, keypad).
- [ ] Every digit key (`0`-`9`) and `.` enter the expected digits on screen.
- [ ] `+ - × /` and `%` (percent divides the current value by 100) behave
      correctly, including chained operations without precedence
      (`5 + 2 × 3` should give `21`, not `11`).
- [ ] `=` evaluates the pending operation.
- [ ] `C` clears the display back to `0` and resets any pending operator.
- [ ] `±` toggles the sign of the current value (no-op on `0`).
- [ ] Backspace-equivalent key removes the last digit; deleting the last
      digit or a lone `-` resets the display to `0`.
- [ ] Division/modulo by zero shows `0`, not an error or crash.
- [ ] Dragging the title bar moves the window.
- [ ] Double-clicking the title bar toggles maximize/restore.
- [ ] The minimize, maximize, and close buttons in the title bar work.
- [ ] Dragging the bottom-right grip resizes the window, and the layout
      (screen + keypad) scales and stays centered as one piece.
- [ ] Window can be resized down to its minimum size and up to a large size
      without the layout breaking or clipping.

## GUI - keyboard

- [ ] Number keys and `.` type into the display same as clicking.
- [ ] `+ - * / %` trigger the matching operator.
- [ ] `Enter` triggers `=`.
- [ ] `Escape` triggers `C` (clear).
- [ ] `c`/`C` also triggers clear.
- [ ] `Backspace` deletes the last digit.
- [ ] Keyboard and mouse input can be mixed mid-calculation without the
      state getting confused (e.g. type a number, click an operator, type
      the next number, press `=`).

## Cross-platform packaging (before a tagged release)

- [ ] Linux CLI and GUI tarballs extract and run (`tar -xJf ...`).
- [ ] Windows CLI and GUI zips extract and run (`.exe` launches, no missing
      DLL errors).
- [ ] macOS CLI tarball runs; `.dmg` mounts, the app copies to
      `/Applications`, and (since it's unsigned) the right-click-Open
      workaround for the "unidentified developer" warning works.
- [ ] Installing the CLI to `/usr/local/bin` and invoking
      `abyssal-abacus-calc` from an arbitrary directory works.

## Regression watch-list

These are easy to silently break when touching `abyssal_abacus_core`:

- [ ] Unary minus is still distinguished from subtraction in
      `evaluate_expr` (`-5+3`, `5*-3`, and a plain subtraction like `5-3`
      must all still parse correctly).
- [ ] Whole-number results still display without a trailing `.0`
      (`format_number`).
- [ ] Chained GUI operations still evaluate left-to-right with no operator
      precedence.
