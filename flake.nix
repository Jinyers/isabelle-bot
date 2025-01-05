{
  description = "Isabelle Discord Bot";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
  	system = "x86_64-darwin";
    pkgs = nixpkgs.legacyPackages.${system};
  in
	{
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
				rustup
			];
		};
	};
}
