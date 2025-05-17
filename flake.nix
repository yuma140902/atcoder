{
	description = "AtCoder";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/24.11";
		utils.url = "github:numtide/flake-utils";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, fenix, nixpkgs, ... }@inputs: inputs.utils.lib.eachSystem [
		"x86_64-linux"
	] (system: let
			rust-toolchain = fenix.packages.${system}.fromToolchainFile {
				file = ./rust-toolchain.toml;
				sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
			};
			pkgs = import nixpkgs {
				inherit system;
				config.allowUnfree = true;
			}; 
		in {
			devShells.default = pkgs.mkShell {
				name = "atcoder";
				packages = with pkgs; [
					llvmPackages_16.libstdcxxClang
					gcc12
					ac-library
					bear
					rust-toolchain
				];
				hardeningDisable = [ "fortify" ];
				pure=true;
			};
		});
}
