{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "distrohoop";
  version = "1.0.1";

  src = fetchFromGitHub {
    owner = "br0sinski";
    repo = "distrohoop";
    rev = version;
    hash = "sha256-I+veO961puqd6aJ789Jlnn/7KJm9YIiSnsaqx0E3nPE=";
  };
  sourceRoot = "source/distrohoop";
  cargoLock = {
    lockFile = ./distrohoop/Cargo.lock;
  };
  postPatch = ''
    ln -s ${./distrohoop/Cargo.lock} Cargo.lock
  '';

  useFetchCargoVendor = true;
  cargoHash = "";

  meta = {
    description = "A CLI tool that determines your next distro written in Rust 🦀 - presumably the last program you will download on your linux installation";
    homepage = "https://github.com/br0sinski/distrohoop";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ "witttccchhher" ];
    mainProgram = "distrohoop";
    platforms = lib.platforms.all;
  };
}
