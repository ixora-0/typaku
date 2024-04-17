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
Instead of installing Node.js, you can also install [bun](https://bun.sh/) instead.

## Running
 - First run
```sh
$ bun install
```
if you are using bun (comes with the nix shell) or
```sh
$ npm install
```
if you are using Node.js.

 - Then run the following command to start a development build of the app:
```sh
$ cargo tauri dev
```

