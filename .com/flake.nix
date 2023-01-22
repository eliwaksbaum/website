{
  description = "Rocket Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , crane
    , ...
    } @ inputs:
    flake-utils.lib.eachSystem [ "aarch64-linux" "x86_64-linux" ] (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      inherit (pkgs) lib;

      rust = rust-overlay.packages.${system}.default.override {
        extensions = [ "rust-src" ];
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain rust;

      rocket = craneLib.buildPackage {
        src = self;
      };
    in
    {
      packages.rocket = rocket;

      devShell = pkgs.mkShell {
        name = "rocket-shell";

        buildInputs = with pkgs; [
          rust
          openssl
          pkg-config
        ];
      };

      defaultPackage = rocket;
    });
}