terraform {
  required_version = ">= 1.5"
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

provider "digitalocean" {
  token = var.do_token
}

resource "digitalocean_droplet" "control_plane" {
  count    = var.control_plane_count
  name     = "nebula-cp-${count.index + 1}"
  region   = var.region
  size     = var.control_plane_size
  image    = "ubuntu-24-04-x64"
  ssh_keys = var.ssh_keys

  tags = ["nebula", "control-plane"]
}

resource "digitalocean_droplet" "agent_node" {
  count    = var.agent_count
  name     = "nebula-agent-${count.index + 1}"
  region   = var.region
  size     = var.agent_size
  image    = "ubuntu-24-04-x64"
  ssh_keys = var.ssh_keys

  tags = ["nebula", "agent"]
}

resource "digitalocean_firewall" "nebula" {
  name = "nebula-firewall"

  droplet_ids = flatten([
    digitalocean_droplet.control_plane[*].id,
    digitalocean_droplet.agent_node[*].id,
  ])

  inbound_rule {
    protocol         = "tcp"
    port_range       = "22"
    source_addresses = ["0.0.0.0/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "443"
    source_addresses = ["0.0.0.0/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "9000-9100"
    source_addresses = var.vpc_cidr_block
  }

  outbound_rule {
    protocol              = "tcp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0"]
  }

  outbound_rule {
    protocol              = "udp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0"]
  }
}
