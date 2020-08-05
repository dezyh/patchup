provider "aws" {
  region = "us-east-1"
  access_key = "patchup"
  secret_key = "quilt"
  version = "~> 2.0"  

  skip_credentials_validation = true
  skip_requesting_account_id = true
  skip_metadata_api_check = true
  s3_force_path_style = true
  
  endpoints {
    s3 = "http://localhost:4572"
  }
}
