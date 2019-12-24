with import  <nixpkgs> {};
stdenv.mkDerivation {
  name = "dev-env";
  buildInputs = [ yarn pkg-config openssl wasm-pack ];
  shellHook = ''
  '';
}

