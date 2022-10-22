{
  inputs.easy.url = "github:jooooscha/easy-flake";
  outputs = { easy, ... }:
    easy.rust.env { };
}
