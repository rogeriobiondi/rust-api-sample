terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.1"
    }
  }
  required_version = ">= 1.0"
}

# Habilitar APIs necessárias
resource "google_project_service" "vpcaccess" {
  project = var.project_id
  service = "vpcaccess.googleapis.com"
}

# Configure o Google Provider
provider "google" {
  project = var.project_id
  region  = var.region
}

# Gerar senha aleatória para o banco de dados
resource "random_password" "db_password" {
  length  = 16
  special = true
}

# Criar Artifact Registry para armazenar imagens Docker
resource "google_artifact_registry_repository" "docker_repo" {
  location      = var.region
  repository_id = "rust-api-registry"
  description   = "Docker repository for Rust API"
  format        = "DOCKER"

  docker_config {
    immutable_tags = false
  }
}

# Criar conta de serviço para Cloud Build
resource "google_service_account" "cloudbuild_sa" {
  account_id   = "cloudbuild-sa"
  display_name = "Cloud Build Service Account"
}

# Conceder permissões para a conta de serviço do Cloud Build
resource "google_project_iam_member" "cloudbuild_artifact_registry" {
  project = var.project_id
  role    = "roles/artifactregistry.writer"
  member  = "serviceAccount:${google_service_account.cloudbuild_sa.email}"
}

resource "google_project_iam_member" "cloudbuild_run_admin" {
  project = var.project_id
  role    = "roles/run.admin"
  member  = "serviceAccount:${google_service_account.cloudbuild_sa.email}"
}

# Criar instância Cloud SQL PostgreSQL com PSC
resource "google_sql_database_instance" "postgres_instance" {
  name             = var.database_instance
  database_version = "POSTGRES_14"
  region           = var.region

  settings {
    tier = "db-f1-micro"
    
    ip_configuration {
      ipv4_enabled = false
      enable_private_path_for_google_cloud_services = true
      
      # Habilitar PSC
      psc_config {
        psc_enabled = true
        allowed_consumer_projects = [var.project_id]
      }
      
      # Configuração SSL
      ssl_mode = "ALLOW_UNENCRYPTED_AND_ENCRYPTED"
    }

    backup_configuration {
      enabled            = false
      binary_log_enabled = false
    }
  }

  deletion_protection = false
}

# O banco postgres já é criado automaticamente pelo Cloud SQL
# resource "google_sql_database" "postgres_db" {
#   name     = var.database_name
#   instance = google_sql_database_instance.postgres_instance.name
# }

# Criar usuário do banco de dados
resource "google_sql_user" "postgres_user" {
  name     = var.database_username
  instance = google_sql_database_instance.postgres_instance.name
  password = random_password.db_password.result
}

# Cloud Build trigger removido temporariamente - focar na infraestrutura básica primeiro
# resource "google_cloudbuild_trigger" "docker_build" { ... }

# Criar bucket para armazenar arquivos de build
resource "google_storage_bucket" "cloudbuild_files" {
  name     = "${var.project_id}-cloudbuild-files"
  location = var.region

  uniform_bucket_level_access = true

  lifecycle_rule {
    condition {
      age = 1
    }
    action {
      type = "Delete"
    }
  }
}

# Arquivo de build do Cloud Build
resource "google_storage_bucket_object" "cloudbuild_yaml" {
  name   = "cloudbuild.yaml"
  bucket = google_storage_bucket.cloudbuild_files.name
  content = templatefile("${path.module}/cloudbuild.yaml.tftpl", {
    project_id       = var.project_id
    region           = var.region
    _REGISTRY_REGION = var.region
    _PROJECT_ID      = var.project_id
    _SERVICE_NAME    = var.service_name
  })
}

# Criar serviço Cloud Run com imagem placeholder
resource "google_cloud_run_service" "rust_api_service" {
  name     = var.service_name
  location = var.region

  template {
    spec {
      containers {
        # Imagem placeholder pública que sempre existe
        image = "gcr.io/google-samples/hello-app:1.0"
        
        ports {
          container_port = 8080
        }

        env {
          name  = "DATABASE_URL"
          value = "postgresql://${var.database_username}:${random_password.db_password.result}@${google_sql_database_instance.postgres_instance.connection_name}/postgres?host=/cloudsql/${google_sql_database_instance.postgres_instance.connection_name}"
        }

        resources {
          limits = {
            cpu    = "1000m"
            memory = "512Mi"
          }
        }
      }

      container_concurrency = 10
      timeout_seconds       = 300
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "10"
        "autoscaling.knative.dev/minScale" = "0"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  depends_on = [
    google_sql_database_instance.postgres_instance
  ]
}

# Acesso público bloqueado pela política da organização
# resource "google_cloud_run_service_iam_member" "public_access" {
#   location = google_cloud_run_service.rust_api_service.location
#   project  = google_cloud_run_service.rust_api_service.project
#   service  = google_cloud_run_service.rust_api_service.name
#   role     = "roles/run.invoker"
#   member   = "allUsers"
# }

# Conceder permissão para Cloud Run acessar Cloud SQL
resource "google_project_iam_member" "cloudrun_sql_client" {
  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.cloudbuild_sa.email}"
}
