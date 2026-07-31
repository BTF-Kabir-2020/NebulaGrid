variable "do_token" {
  description = "DigitalOcean API token"
  type        = string
  sensitive   = true
}

variable "region" {
  description = "DigitalOcean region"
  type        = string
  default     = "fra1"
}

variable "control_plane_count" {
  description = "Number of control plane nodes"
  type        = number
  default     = 1
}

variable "control_plane_size" {
  description = "Droplet size for control plane"
  type        = string
  default     = "s-2vcpu-4gb"
}

variable "agent_count" {
  description = "Number of agent nodes"
  type        = number
  default     = 3
}

variable "agent_size" {
  description = "Droplet size for agent nodes"
  type        = string
  default     = "s-1vcpu-2gb"
}

variable "ssh_keys" {
  description = "SSH key IDs or fingerprints"
  type        = list(string)
}

variable "vpc_cidr_block" {
  description = "VPC CIDR block for internal traffic"
  type        = list(string)
  default     = ["10.0.0.0/16"]
}
