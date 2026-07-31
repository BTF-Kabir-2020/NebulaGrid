output "control_plane_ips" {
  description = "IP addresses of control plane nodes"
  value       = digitalocean_droplet.control_plane[*].ipv4_address
}

output "agent_ips" {
  description = "IP addresses of agent nodes"
  value       = digitalocean_droplet.agent_node[*].ipv4_address
}

output "gateway_url" {
  description = "Gateway URL"
  value       = "https://${digitalocean_droplet.control_plane[0].ipv4_address}"
}
