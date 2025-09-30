{ pkgs, rustPkgs, ... }:
{
  projectRootFile = "flake.nix";

  programs = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    taplo.enable = true;
    yamlfmt.enable = true;
  };

  settings = {
    formatter = {
      dxfmt = {
        # XXX: copy-pasted this approach from the treefmt impl for packer
        # CONTRIB: add this to treefmt-nix, maybe make it better
        command = pkgs.writeShellApplication {
          name = "treefmt-nix-dx-fmt-wrapper";
          runtimeInputs = [ rustPkgs ];
          text =
            let
              dx = "${pkgs.dioxus-cli}/bin/dx";
            in
            ''
              set -eu

              for file in "$@"; do
                # first we check whether the file needs changes, because apparently,
                # formatting it always updates the mtime, appears as modified to treefmt,
                # and treefmt on check mode (for pre-commit) will always fail
                if ! ${dx} fmt --check --file "$file" > /dev/null; then
                  ${dx} fmt --file "$file" || rc="$?"
                fi
              done

              exit "''${rc:-0}"
            '';
        };

        includes = [ "*.lol" ];
      };
    };
  };
}
