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
                tmp="$file.FORMATTED"
                ${dx} fmt --file - <"$file" > "$tmp"
                if ! cmp -s "$file" "$tmp"; then
                  mv -f "$tmp" "$file"
                else
                  rm "$tmp"
                fi
              done

              exit "''${rc:-0}"
            '';
        };

        includes = [ "*.rs" ];
      };
    };
  };
}
