resource "aws_s3_bucket" "patchup_bucket_s3" {
  acl    = var.acl
  bucket = var.bucket
}
