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
            version = "1.0.0";
            src = ./.;
            cargoHash = "sha256-QEEqkcsJulZtMpVZXqy5D187nA+ksRya8ggPB9YWILU=";
            buildInputs = with pkgs;
              [ libiconv ] ++ lib.optional stdenv.hostPlatform.isDarwin
              darwin.apple_sdk.frameworks.SystemConfiguration;
            meta = {
              description =
                "A command-line translator powered by Google Translate";
              longDescription = ''
                This command-line tool leverages Google Translate to translate text quickly between languages.
                It can be used to translate command-line output, allowing users to work with any language from the terminal.
              '';
              homepage = "https://github.com/NewDawn0/tl";
              license = pkgs.lib.licenses.mit;
              maintainers = with pkgs.lib.maintainers; [ NewDawn0 ];
              platforms = pkgs.lib.platforms.all;
            };
          };
        });
    };
}
