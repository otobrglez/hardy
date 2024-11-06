{ pkgs, lib, config, inputs, ... }:
let
  pkgs-unstable = import inputs.nixpkgs { system = pkgs.stdenv.system; config.allowUnfree = true;  };
in
{
  name = "hardy";
  env.HARDY_ENV = "development";
  env.RUST_LOG = "info";

  packages = [
    pkgs-unstable.ngrok
  ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
    frameworks.CoreServices
    frameworks.SystemConfiguration
    frameworks.Security
    frameworks.Cocoa
  ]) ++ [
    pkgs.libiconv
    pkgs.just
	pkgs.websocat
	pkgs.flyctl
  ];

  languages.rust = {
  	enable = true;
    channel = "stable";
    components = [
      "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"
    ];
  };

  processes = {
    ngrok-8884.exec = "ngrok http 8884 --log ./tmp/ngrok.log";
    ngrok-log.exec = "rm -rf ./tmp/ngrok.log && touch ./tmp/ngrok.log && tail -f ./tmp/ngrok.log";
	# lt.exec = "yarn exec lt -- --port 8884 -s hardy --print-requests";
  };

  #scripts.install.exec = ''
  #    cargo install cargo-watch
  #'';


  tasks = {
    "project:setup" = {
      before = ["devenv:enterShell"];
      exec = "cargo install cargo-watch";
      # status = "[ -d \"node_modules/\" ]";
    };
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  enterShell = ''
  	echo "~~~ hardy ~~~"
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  enterTest = ''
    cargo test
  '';
}
