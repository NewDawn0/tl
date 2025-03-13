{
  description = "A cli translator using google translate";

  inputs.utils.url = "github:NewDawn0/nixUtils";

  outputs = { self, utils, ... }: {
    overlays.default = final: prev: {
      translate = self.packages.${prev.system}.default;
    };
    packages = utils.lib.eachSystem { } (pkgs: {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "tl";
        version = "1.0.0";
        src = ./.;
        useFetchCargoVendor = true;
        cargoHash = "sha256-lWiwoBV97/i6mlqmgQbahui3t+u3Uml/MHNXzo2ytZQ=";
        buildInputs = with pkgs;
          [ libiconv ] ++ lib.optional stdenv.hostPlatform.isDarwin
          darwin.apple_sdk.frameworks.SystemConfiguration;
        meta = {
          description = "A command-line translator powered by Google Translate";
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
