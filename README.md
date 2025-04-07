# Xouse

## Dev

A little workaround because I write code in WSL but have to run the dev server in windows

Copy over files in WSL to windows
```sh
watchexec -d 1s 'rsync -av --exclude=".git" --exclude="target" --exclude-from=".gitignore" ./ <wsl_style_dev_path>'
```

Run windows dev server
```sh
cmd.exe '/C' 'cd <windows_style_dev_path> && npm i && cargo tauri dev'
```

## TODO:
- [ ] Shortcut for toggling between 'xouse' mode and controller mode.
