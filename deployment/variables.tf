variable "project_id" {
  description = "ID do projeto Google Cloud"
  type        = string
}

variable "region" {
  description = "Região para deployment dos recursos"
  type        = string
  default     = "us-central1"
}

variable "service_name" {
  description = "Nome do serviço Cloud Run"
  type        = string
  default     = "rust-api-sample"
}

variable "database_instance" {
  description = "Nome da instância Cloud SQL"
  type        = string
  default     = "rust-api-db"
}

variable "database_name" {
  description = "Nome do banco de dados PostgreSQL"
  type        = string
  default     = "postgres"
}

variable "database_username" {
  description = "Usuário do banco de dados PostgreSQL"
  type        = string
  default     = "postgres"
}
