{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.RUST_BACKTRACE = 1;
  env.AOC_DAY = 7;

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    tokei
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.test.exec = ''
    echo "Running tests"
    for i in $(seq 1 $AOC_DAY); do
      cargo test --bin $i
    done
  '';

  scripts.run.exec = ''
    echo "Running tests"
    for i in $(seq 1 $AOC_DAY); do
      cargo run --bin $i
    done
  '';

  enterShell = ''
    git --version
    tokei
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    for i in $(seq 1 $AOC_DAY); do
      cargo test --bin $i
    done
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
