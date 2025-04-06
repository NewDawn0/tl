{
  description = "A cli translator using google translate";

  inputs.utils.url = "github:NewDawn0/nixUtils";

  outputs = { self, utils, ... }: {
    overlays.default = final: prev: {
      translate = self.packages.${prev.system}.default;
    };
    packages = utils.lib.eachSystem { } (pkgs: {
      default = pkgs.rustPlatform.buildRustPackage {
        name = "translate";
        version = "2.0.0";
        src = ./.;
        useFetchCargoVendor = true;
        cargoHash = "sha256-xMA2bus+ElNQhc9E7aK7XC1Zfuu8p0TW9fkWfv5z2AA=";
        buildInputs = with pkgs;
          [ libiconv ] ++ lib.optional stdenv.hostPlatform.isDarwin
          darwin.apple_sdk.frameworks.SystemConfiguration;
        installPhase = ''
          runHook preInstall
          mkdir -p $out/bin $out/include $out/lib
          # Dynamically find header bin and lib files
          cp $(find target -type f \( -name 'translate' -o -name 'translate.exe' \) | head -n1) $out/bin
          cp $(find target -name translate.h | head -n1) $out/include
          cp $(find target -name 'libtranslate\.*' ! -name 'libtranslate.d' | head -n1) $out/lib
          runHook postInstall
        '';
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
