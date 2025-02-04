{
  description = "MCP Guardian";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixgl.url = "github:nix-community/nixGL";
  };

  outputs = inputs:
    (inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            inputs.rust-overlay.overlays.default
            inputs.nixgl.overlay
          ];
        };

        inherit (pkgs) callPackage;

        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        # rustfmt from rust-nightly used for advanced options in rustfmt
        rustfmt-nightly = pkgs.rust-bin.nightly.latest.rustfmt;

        shellPkgs = [
          rustfmt-nightly # must come before `rust` to so this version of rustfmt is first in PATH
          rust
        ] ++ (with pkgs; [
          cargo-tauri
          just
          nixpkgs-fmt
          openssl
          neovim
          nodejs_22
          pkg-config
          present-cli
          webkitgtk_4_1
          (yarn.override { nodejs = nodejs_22; })
        ]);

      in
      {
        devShells = {
          default = pkgs.mkShell {
            shellHook = ''
              nix run --impure .#nixGL -- nix develop .#dev
            '';
          };
          dev = pkgs.mkShell {
            buildInputs = shellPkgs;
            shellHook = ''
              export PATH="$(pwd)/_build/bin:$PATH"
              # optionally specify a shell to launch, otherwise it remains in bash
              if [ -f .devshell ]; then
                export SHELL=$(cat .devshell)
                exec $SHELL
              fi
            '';
            WEBKIT_DISABLE_COMPOSITING_MODE = 1;
          };
          fhs = (pkgs.buildFHSEnv {
            name = "fhs-shell";
            targetPkgs = _: shellPkgs;
          }).env;
        };

        packages = {
          codium = callPackage ./nix/vscode.nix { vscode = pkgs.vscodium; };
          nixGL = pkgs.nixgl.auto.nixGLDefault;
        };
      }));
}
