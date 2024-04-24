# Developing
## Getting development shell
### With nix shell
You can use the default bash shell:
```sh
$ nix develop
```

Or specify your preffered shell:
```sh
$ nix develop -c zsh
```

You can also install [nix-direnv](https://github.com/nix-community/nix-direnv#installation) to automatically switch shells when you `cd` into the project.

### Without nix shell
Follow the appropriate [prerequisites for tauri](https://tauri.app/v1/guides/getting-started/prerequisites).
Install [bun](https://bun.sh/) instead of [Node.js](https://nodejs.org/).

## Running
 - First run
```sh
$ bun install
```
 - Then run the following command to start a development build of the app:
```sh
$ cargo tauri dev
```
