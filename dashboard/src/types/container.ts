export interface ContainerPort {
  private_port: number;
  public_port: number | null;
  ip: string;
}

export interface Container {
  id: string;
  container_id: string;
  name: string;
  image: string;
  status: string;
  node_id: string;
  ports?: string[] | ContainerPort[];
  created_at?: string;
}
