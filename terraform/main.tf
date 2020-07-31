module "patchup_server" {
  source = "./server"
  ami_id = "ami-0ac80df6eff0e70b5"
  key_pair = aws_key_pair.patchup_key.key_name
  name = "patchup-server"
}