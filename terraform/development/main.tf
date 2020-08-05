module "patchup_bucket" {
  source = "../bucket"

  acl    = "public-read"
  bucket = "test-bucket"
}
