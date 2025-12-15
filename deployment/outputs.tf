output "service_url" {
  description = "URL do serviço Cloud Run"
  value       = google_cloud_run_service.rust_api_service.status[0].url
}

output "database_instance_name" {
  description = "Nome da instância Cloud SQL"
  value       = google_sql_database_instance.postgres_instance.name
}

output "database_connection_name" {
  description = "Nome de conexão da instância Cloud SQL"
  value       = google_sql_database_instance.postgres_instance.connection_name
}

output "database_public_ip" {
  description = "IP público da instância Cloud SQL"
  value       = google_sql_database_instance.postgres_instance.public_ip_address
}

output "artifact_registry_url" {
  description = "URL do Artifact Registry"
  value       = google_artifact_registry_repository.docker_repo.name
}
