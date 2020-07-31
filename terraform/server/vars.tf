variable "ami_id" {
  type = string
}

variable "iam_instance_profile" {
  default = ""
  type    = string
}

variable "instance_type" {
  default = "t2.micro"
  type    = string
}

variable "name" {
  type = string
}

variable "key_pair" {
  type = string
}

variable "private_ip" {
  default = ""
  type    = string
}

variable "subnet_id" {
  default = ""
  type    = string
}

variable "vpc_security_group_ids" {
  default = []
  type = list(string)
}