{
  pkgs,
  treefmt,
  rustPkgs,
  ...
}:

{
  check-merge-conflicts.enable = true;
  check-added-large-files.enable = true;
  commitizen = {
    enable = true;
    package = pkgs.commitizen;
  };

  gitleaks = {
    name = "gitleaks";
    enable = true;
    entry = "${pkgs.gitleaks}/bin/gitleaks detect";
    stages = [ "pre-commit" ];
  };

  treefmt = {
    enable = true;
    package = treefmt;
    # only way I got it to run
    pass_filenames = false;
  };

  statix.enable = true;
  deadnix.enable = true;
  nil.enable = true;
  flake-checker.enable = true;

  actionlint.enable = true;

  clippy = {
    enable = true;
    # override from rust-overlay, which is more up-to-date
    packageOverrides = {
      clippy = rustPkgs;
      cargo = rustPkgs;
    };
    settings = {
      allFeatures = true;
    };
  };

}
