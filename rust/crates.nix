{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    # TODO: change this to your crate's name
    crateName = "rocket-jump";
  in {
    # declare projects
    # TODO: change this to your crate's path
    nci.projects.${crateName}.path = ./.;
    # configure crates
    nci.crates.${crateName} = {};
  };
}
