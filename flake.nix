{
  description = "A cli translator using google translate";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nix-systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      eachSystem = nixpkgs.lib.genAttrs (import inputs.nix-systems);
      mkPkgs = system:
        import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
    in {
      overlays.default =
        (final: prev: { tl = self.packages.${prev.system}.default; });
      packages = eachSystem (system:
        let pkgs = mkPkgs system;
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "tl";
            version = "0.0.1";
            buildInputs = with pkgs;
              if pkgs.stdenv.isDarwin then
                with darwin; [
                  libiconv
                  apple_sdk.frameworks.SystemConfiguration
                ]
              else
                [ libiconv ];

            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            meta = with pkgs.lib; {
              description = "A cli translator using google translate";
              homepage = "https://github.com/NewDawn0/tl";
              maintainers = with maintainers; [ "NewDawn0" ];
              license = licenses.mit;
            };
          };
        });
    };
}
