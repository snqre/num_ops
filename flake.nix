{
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";
	inputs.crane.url = "github:ipetkov/crane";

	outputs = inputs @ { flake-parts, ... }:
	flake-parts.lib.mkFlake {
		inherit inputs;
	} {
		systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

		perSystem = { self', pkgs, system, ... }: 
		let
			crane_lib = inputs.crane.mkLib pkgs;
			crane_src = crane_lib.cleanCargoSource (crane_lib.path ./.);
		in {
			packages.default =
			let
				pname = "num_ops";
				version = "0.1.0";
				src = crane_src;
				strictDeps = true;
				doNotPostBuildInstallCargoBinaries = false;
				doCheck = true;
				
				nativeBuildInputs = [
					pkgs.pkg-config
				];
				
				cargoArtifacts = crane_lib.buildDepsOnly {
					inherit src;
					inherit strictDeps;
					inherit nativeBuildInputs;
				};
			in crane_lib.buildPackage {
				inherit pname;
				inherit version;
				inherit src;
				inherit strictDeps;
				inherit doNotPostBuildInstallCargoBinaries;
				inherit doCheck;
				inherit nativeBuildInputs;
				inherit cargoArtifacts;
			};
			
			packages.release = pkgs.writeShellApplication {
				name = "release";
				
				runtimeInputs = [
					pkgs.nushell
					pkgs.cargo
					pkgs.rustc
					pkgs.pkg-config
					pkgs.openssl
					pkgs.gh
				];
				
				text = 
				let
					src_template = builtins.readFile ./script/release.nu;
					
					src = builtins.replaceStrings [ 
						"@bin_path@" 
					] [ 
						"${self'.packages.default}/bin" 
					] src_template;
				in ''
					# shellcheck disable=SC2016
					nu -c '${src}'
				'';
			};
			
			devShells.default = pkgs.mkShell {
				nativeBuildInputs = [
					pkgs.nushell
					pkgs.nixd
					pkgs.nixpkgs-fmt
					pkgs.clippy
					pkgs.cargo
					pkgs.rustc
					pkgs.rust-analyzer
					pkgs.pkg-config
					
					self'.packages.release
				];
			};
		};
	};
}
 