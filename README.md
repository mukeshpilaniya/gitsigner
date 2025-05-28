
# gitsigner

A terminal-based Git wrapper that helps you **select your Git email address** at commit time, using a terminal UI. Useful when you switch between personal and work email addresses.

Built with [Ratatui](https://github.com/ratatui/ratatui), [`crossterm`](https://crates.io/crates/crossterm), and Rust.

---

##  Features

- Automatically reads `~/.gitconfig` to find all email addresses, including commented-out ones.
- Interactive terminal UI lets you pick the email address.
- Sets `GIT_AUTHOR_EMAIL` and `GIT_COMMITTER_EMAIL` automatically.
- Only activates when the `-s` / `--signoff` flag is passed.
- Transparently passes all other `git commit` arguments.

---

## Installation

```bash
git clone https://github.com/yourname/gitsigner.git
cd gitsigner
cargo build --release
```

Then add it to your path or alias it:

```bash
alias git-commit="path/to/gitsigner"
```

Or use it directly:

```bash
./target/release/gitsigner -s
```

---

## How It Works

If you run:

```bash
gitsigner -s -v
```

It:

1. Parses your `~/.gitconfig` and finds any `email = ...` lines, even commented ones.
2. Shows a terminal UI to pick the email.
3. Sets:
   - `GIT_AUTHOR_EMAIL`
   - `GIT_COMMITTER_EMAIL`
4. Runs:
   ```bash
   git commit -s -v
   ```

If you don’t pass `-s` or `--signoff`, it just runs `git commit` normally with your arguments.

---

##  Supported Commands

| Command                     | Shows Email Picker? |
|-----------------------------|---------------------|
| `gitsigner -s`              | ✅ Yes              |
| `gitsigner commit -s -v`    | ✅ Yes              |
| `gitsigner --amend -s`      | ✅ Yes              |
| `gitsigner`                 | ❌ No               |
| `gitsigner commit --amend` | ❌ No               |

---

## Configuration

Make sure your `~/.gitconfig` has your emails:

```ini
[user]
    name = Your Name
    email = work@example.com
#   email = personal@example.com
```

Both emails (even the commented one) will appear in the picker.

---

## 🧪 Example

```bash
gitsigner -s -v
```

Shows:

```
┌────────────────────────────┐
│     Select Git Email       │
├────────────────────────────┤
│ work@example.com           │
│ personal@example.com       │
└────────────────────────────┘
```

Press ↑/↓ and Enter to confirm.

---

## Tip

You can also use this as a Git alias:

```bash
git config --global alias.sign '!gitsigner'
```

Then commit with:

```bash
git sign -s
```

---

##  License

MIT
````

