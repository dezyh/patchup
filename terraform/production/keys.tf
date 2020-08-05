resource "aws_key_pair" "patchup_key" {
  key_name   = "patchup-key"
  public_key = file("./patchup.pem")
}