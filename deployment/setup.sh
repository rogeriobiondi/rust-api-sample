#!/bin/bash

# Script de setup para deployment da Rust API no Google Cloud Run

set -e

echo "🚀 Configurando deployment da Rust API no Google Cloud Run"

# Verificar se o gcloud está instalado
if ! command -v gcloud &> /dev/null; then
    echo "❌ Google Cloud SDK (gcloud) não encontrado. Instale-o primeiro:"
    echo "https://cloud.google.com/sdk/docs/install"
    exit 1
fi

# Verificar se o Terraform está instalado
if ! command -v terraform &> /dev/null; then
    echo "❌ Terraform não encontrado. Instale-o primeiro:"
    echo "Siga as instruções no README.md"
    exit 1
fi

# Verificar se o Docker está instalado
if ! command -v docker &> /dev/null; then
    echo "❌ Docker não encontrado. Instale-o primeiro:"
    exit 1
fi

echo "✅ Dependências encontradas"

# Login no Google Cloud
echo "🔐 Fazendo login no Google Cloud..."
gcloud auth login
gcloud auth application-default login

# Configurar projeto
if [ -z "$GOOGLE_CLOUD_PROJECT" ]; then
    echo "📋 Informe o ID do seu projeto Google Cloud:"
    read -r PROJECT_ID
else
    PROJECT_ID=$GOOGLE_CLOUD_PROJECT
fi

gcloud config set project "$PROJECT_ID"

# Habilitar APIs necessárias
echo "🔧 Habilitando APIs do Google Cloud..."
gcloud services enable run.googleapis.com
gcloud services enable artifactregistry.googleapis.com
gcloud services enable cloudbuild.googleapis.com
gcloud services enable sql-component.googleapis.com
gcloud services enable sqladmin.googleapis.com

echo "✅ APIs habilitadas"

# Criar arquivo terraform.tfvars se não existir
if [ ! -f "terraform.tfvars" ]; then
    echo "📝 Criando arquivo terraform.tfvars..."
    cp terraform.tfvars.example terraform.tfvars
    
    # Substituir o project_id no arquivo
    sed -i "s/seu-project-id-aqui/$PROJECT_ID/" terraform.tfvars
    
    echo "✅ Arquivo terraform.tfvars criado com project_id: $PROJECT_ID"
fi

# Inicializar Terraform
echo "🔧 Inicializando Terraform..."
terraform init

echo "🎉 Setup concluído!"
echo ""
echo "Próximos passos:"
echo "1. Revise o arquivo terraform.tfvars se necessário"
echo "2. Execute: terraform plan"
echo "3. Execute: terraform apply"
echo ""
echo "Para testar localmente com Docker Compose:"
echo "1. Copie o código da aplicação para esta pasta"
echo "2. Execute: docker-compose up --build"
