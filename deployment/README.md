# Deploy Rust API no Google Cloud Run com Terraform

Este guia demonstra como fazer o deployment da aplicação Rust API no Google Cloud Run usando Terraform.

## Pré-requisitos

- [Google Cloud SDK (gcloud)](https://cloud.google.com/sdk/docs/install)
- [Terraform](https://developer.hashicorp.com/terraform/downloads)
- Conta no Google Cloud com permissões de administrador
- Docker instalado localmente

## Instalação do Terraform

### Linux (Ubuntu/Debian)
```bash
# Instalar dependências
sudo apt-get update && sudo apt-get install -y gnupg software-properties-common curl

# Adicionar chave GPG do HashiCorp
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -

# Adicionar repositório HashiCorp
sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"

# Instalar Terraform
sudo apt-get update && sudo apt-get install terraform

# Verificar instalação
terraform --version
```

### Linux (Outras distribuições)
```bash
# Baixar Terraform
wget https://releases.hashicorp.com/terraform/1.8.0/terraform_1.8.0_linux_amd64.zip

# Descompactar
unzip terraform_1.8.0_linux_amd64.zip

# Mover para diretório binário
sudo mv terraform /usr/local/bin/

# Verificar instalação
terraform --version
```

### macOS
```bash
# Usar Homebrew
brew install terraform

# Ou baixar manualmente
curl -LO https://releases.hashicorp.com/terraform/1.8.0/terraform_1.8.0_darwin_amd64.zip
unzip terraform_1.8.0_darwin_amd64.zip
sudo mv terraform /usr/local/bin/
```

## Configuração do Google Cloud Provider

### 1. Autenticar com Google Cloud
```bash
# Login na conta Google
gcloud auth login

# Configurar credenciais para Application Default Credentials
gcloud auth application-default login

# Configurar projeto
gcloud config set project SEU_PROJECT_ID
```

### 2. Habilitar APIs necessárias
```bash
# Habilitar APIs para Cloud Run, Artifact Registry e Cloud Build
gcloud services enable run.googleapis.com
gcloud services enable artifactregistry.googleapis.com
gcloud services enable cloudbuild.googleapis.com
gcloud services enable sql-component.googleapis.com
gcloud services enable sqladmin.googleapis.com
```

### 3. Configurar variáveis de ambiente
```bash
# Opcional: exportar credenciais (se não usar gcloud auth)
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-key.json"
```

## Estrutura do Projeto

```
deployment/
├── README.md              # Este arquivo
├── main.tf               # Configuração principal
├── variables.tf          # Variáveis de entrada
├── outputs.tf            # Saídas do deployment
├── docker-compose.yml    # Para testes locais
└── rust-api-sample/      # Arquivos da aplicação
    ├── Dockerfile        # Configuração do container
    └── .dockerignore     # Arquivos ignorados pelo Docker
```

## Passos para Deployment

### 1. Preparar a aplicação
A aplicação já está configurada para rodar na porta 3000 e aceitar conexões externas (`0.0.0.0:3000`).

### 2. Configurar variáveis do Terraform
Copie e edite o arquivo `terraform.tfvars`:
```bash
cp terraform.tfvars.example terraform.tfvars
# Edite o arquivo com suas configurações
```

### 3. Inicializar Terraform
```bash
terraform init
```

### 4. Planejar deployment
```bash
terraform plan
```

### 5. Aplicar configuração
```bash
terraform apply
```

## Variáveis de Configuração

| Variável | Descrição | Valor Padrão |
|----------|-----------|--------------|
| `project_id` | ID do projeto Google Cloud | Obrigatório |
| `region` | Região de deployment | `us-central1` |
| `service_name` | Nome do serviço Cloud Run | `rust-api-sample` |
| `database_instance` | Nome da instância Cloud SQL | `rust-api-db` |
| `database_name` | Nome do banco de dados | `postgres` |
| `database_username` | Usuário do banco | `postgres` |
| `database_password` | Senha do banco | Gerado automaticamente |

## Serviços Criados

O Terraform criará os seguintes recursos:

1. **Artifact Registry** - Repositório para imagens Docker
2. **Cloud Build** - Pipeline para build da imagem
3. **Cloud SQL** - Banco de dados PostgreSQL
4. **Cloud Run** - Serviço para executar a API
5. **IAM** - Permissões necessárias para os serviços

## Acesso à Aplicação

Após o deployment, a API estará disponível na URL mostrada no output do Terraform, similar a:
```
https://rust-api-sample-abcdef1234-uc.a.run.app
```

Endpoints disponíveis:
- `GET /` - Mensagem de boas-vindas
- `GET /itens` - Listar itens
- `POST /itens` - Criar item
- `GET /itens/{id}` - Buscar item
- `PUT /itens/{id}` - Atualizar item
- `DELETE /itens/{id}` - Deletar item
- `GET /swagger-ui` - Documentação interativa

## Monitoramento e Logs

### Verificar status do serviço
```bash
gcloud run services describe rust-api-sample --region=us-central1
```

### Verificar logs
```bash
gcloud run services logs read rust-api-sample --region=us-central1
```

### Verificar build
```bash
gcloud builds list --limit=10
```

## Limpeza

Para remover todos os recursos criados:
```bash
terraform destroy
```

## Troubleshooting

### Problemas comuns

1. **Permissões negadas**: Verifique se sua conta tem as permissões necessárias
2. **Build falha**: Verifique o Dockerfile e dependências da aplicação
3. **Conexão com banco**: Verifique se as regras de firewall permitem conexão do Cloud Run

### Comandos úteis
```bash
# Verificar status dos serviços
gcloud run services list

# Verificar imagens no registry
gcloud artifacts docker images list us-central1-docker.pkg.dev/SEU_PROJECT_ID/rust-api-registry

# Acessar shell do Cloud Run (para debug)
gcloud run services rust-api-sample --region=us-central1
```

## Custos

Os serviços criados podem gerar custos:
- Cloud Run: Pago por uso (requisições e tempo de CPU)
- Cloud SQL: Pago por instância e storage
- Artifact Registry: Pago por storage de imagens
- Cloud Build: Gratuitos até 120 minutos/dia

Consulte a [calculadora de preços Google Cloud](https://cloud.google.com/products/calculator) para estimativas.

## Segurança

- A aplicação usa variáveis de ambiente para credenciais do banco
- O Cloud Run oferece criptografia em trânsito por padrão
- Considere usar VPC Connector para conexão privada com o banco
- Habilite IAM para controle de acesso granular
