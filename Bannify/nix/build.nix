{ pkgs }:

pkgs.rustPlatform.buildRustPackage {
  pname = "bannify";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [ ];

  buildInputs = with pkgs; [
    openssl
    zlib
  ];

  postInstall = ''
    mkdir -p $out/share/bannify
    if [ ! -f "$HOME/.config/bannify_config.toml" ]; then
      mkdir -p "$HOME/.config"
      cp $src/src/default_languages.toml "$HOME/.config/bannify_config.toml"
      echo "Installed default bannify config to ~/.config/bannify_config.toml"
    fi
  '';

  meta = with pkgs.lib; {
    description = "being a Comment Banner Tool";
    homepage = "https://github.com/AaronF86/bannify";
    license = licenses.mit;
    maintainers = with maintainers; [ AaronF86 ];
    platforms = platforms.all;
  };
}
