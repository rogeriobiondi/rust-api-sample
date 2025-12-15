# Exemplo de arquivo de variáveis para Terraform
# Copie este arquivo para terraform.tfvars e preencha com seus valores

# Obrigatório: ID do seu projeto Google Cloud
project_id = "tamago-research"

# Opcional: Região de deployment (padrão: us-central1)
region = "us-east1"

# Opcional: Nome do serviço Cloud Run (padrão: rust-api-sample)
service_name = "rust-api-sample"

# Opcional: Configurações do banco de dados
database_instance = "rust-api-db"
database_name = "postgres"
database_username = "postgres"
