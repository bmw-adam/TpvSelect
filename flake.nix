{
  description = "Nix flake to install trunk with wasm support using rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    tpv-frontend-vendor = {
      url = "github:bmw-adam/tpvselect-frontend-vendor";
      flake = false;
    };
    tpv-backend-vendor = {
      url = "github:bmw-adam/tpvselect-backend-vendor";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, tpv-frontend-vendor, tpv-backend-vendor, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        buildTargetWasm = "wasm32-unknown-unknown";

        rustWasmToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ buildTargetWasm ];
        };
  
        rustWasmPlatform = pkgs.makeRustPlatform {
          cargo = rustWasmToolchain;
          rustc = rustWasmToolchain;
        };

        rustServerPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };

        tpv-vyber-wasm = rustWasmPlatform.buildRustPackage {
          name = "frontend";
          version = "0.1.0";

          src = pkgs.runCommand "wrapped-frontend-src" {
            # nativeBuildInputs = [ pkgs.git ]; # optional
          } ''
            mkdir -p $out
            cp -r ${./TpvVyber/frontend}/* $out/
            cp -r ${tpv-frontend-vendor}/vendor $out/vendor
          '';
          cargoVendorDir = "vendor";
          nativeBuildInputs = [ pkgs.trunk rustWasmToolchain pkgs.rustc pkgs.llvmPackages_20.lld pkgs.wasm-bindgen-cli_0_2_100 ];
          buildInputs = [ pkgs.wasm-bindgen-cli_0_2_104 ];
          cargoLock.lockFile = ./TpvVyber/frontend/Cargo.lock;
          
          buildPhase = ''
            echo "_________________-BUILD-_________________"
            export HOME=$(mktemp -d)
            export CARGO_HOME=t$(mktemp -d)
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"

            export WASM_BINDGEN_PATH=${pkgs.wasm-bindgen-cli_0_2_100}/bin/wasm-bindgen
            echo "WASM_BINDGEN_PATH set to $WASM_BINDGEN_PATH"

            # find . | grep .dist

            echo "Building frontend..."
            # cargo metadata --format-version 1
            # ls
            trunk build --release
          '';
  
          installPhase = ''
            echo "_________________-INSTALL-_________________"
            mkdir -p $out/lib
            cp -r dist $out/lib/
          '';
  
          # Disable checks if they only work for WASM
          doCheck = false;
        };

        tpv-vyber-server = rustServerPlatform.buildRustPackage {
          name = "backend";
          version = "0.1.0";
          cargoVendorDir = "vendor";

          buildPhase = ''
            echo "_________________-BUILD-_________________"
            export HOME=$(mktemp -d)
            export CARGO_HOME=t$(mktemp -d)
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"

            echo "Building backend..."
            # cargo metadata --format-version 1
            ls
            find . -name diesel\*
            # ls ./vendor
            cargo build --release
          '';

          installPhase = ''
            echo "_________________-INSTALL-_________________"
            mkdir -p $out/bin
            cp -r ./target/release/* $out/bin

            cp -r ${tpv-vyber-wasm}/lib/dist $out/bin            

            # [ -f .env ] && rm .env
            # touch .env
          '';

          src = pkgs.runCommand "backend" {
          } ''
            mkdir -p $out
            cp -rf ${./TpvVyber/backend}/* $out/
            cp -rf ${tpv-backend-vendor}/vendor $out/vendor
          '';

          cargoLock.lockFile = ./TpvVyber/backend/Cargo.lock;
          buildInputs = [ tpv-vyber-wasm pkgs.sqlite ];
          nativeBuildInputs = [ pkgs.postgresql ];
        };

      in {
        packages.default = tpv-vyber-server;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.watchexec
            rustWasmToolchain
            pkgs.openssl
            pkgs.postgresql

            pkgs.diesel-cli

            tpv-vyber-server
          ];

          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

          shellHook = ''
            echo "🦀 Rust + Trunk + WASM dev shell ready"
          '';
        };
      });
}


  #      	apps.default = {
	#   type = "app";
  #         #program = pkgs.lib.getExe (pkgs.writeShellScriptBin "TpvSelect");
	#   # program = packages.default;
	#   #program = pkgs.lib.getExe (pkgs.writeShellScriptBin "TpvSelect" "${pkgs.dotnet-sdk_8}/bin/dotnet ${self.packages.${system}.default}/lib/TpvSelect/TpvSelect.dll");
	# };

          # packages.default = pkgs.dockerTools.buildImage {
        #   name = "tpv-vyber-server";
        #   tag = "latest";
        #   contents = [ 
        #     tpv-vyber-server
        #     tpv-vyber-wasm
        #     tpv-vyber-certs
        #     pkgs.postgresql
        #     pkgs.coreutils
        #     pkgs.bash
        #   ];
        #   config = {
        #     Cmd = [ "bash" "-c" ''
        #       set -euo pipefail

        #       export PGDATA=/var/lib/postgresql/data
        #       mkdir -p "$PGDATA"
        #       initdb -D "$PGDATA"
        #       pg_ctl -D "$PGDATA" -l logfile start

        #       export DATABASE_URL="postgres://localhost:5432/postgres"

        #       # Wait until DB is up
        #       until pg_isready; do sleep 0.2; done

        #       echo "Starting app..."
        #       ${tpv-vyber-server}/bin/backend
        #     '' ];

        #     WorkingDir = "/app/bin";
        #     Env = [
        #       "KEY=${tpv-vyber-certs}/key.pem"
        #       "CERT=${tpv-vyber-certs}/cert.pem"
        #       "DIST=${tpv-vyber-wasm}/lib/dist"
        #     ];
        #   };
        # };

