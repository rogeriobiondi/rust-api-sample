# Rust API Sample

API REST desenvolvida em Rust com Axum, SQLx e PostgreSQL (Supabase), demonstrando operações CRUD com busca, ordenação e paginação.

## Tecnologias

- **Rust** - Linguagem de programação
- **Axum** - Framework web assíncrono
- **SQLx** - Cliente SQL assíncrono com verificação em tempo de compilação
- **PostgreSQL** - Banco de dados (Supabase)
- **Utoipa** - Documentação OpenAPI/Swagger automática
- **Tower HTTP** - Middleware CORS

## Funcionalidades

- ✅ CRUD completo de itens (Create, Read, Update, Delete)
- ✅ Busca por ID ou nome do produto
- ✅ Ordenação por ID, nome ou preço (ascendente/descendente)
- ✅ Paginação com controle de itens por página
- ✅ Documentação Swagger UI interativa
- ✅ CORS habilitado para integração com frontends

## Pré-requisitos

- [Rust](https://rustup.rs/) (1.70+)
- Conta no [Supabase](https://supabase.com/) ou PostgreSQL local

## Instalação

1. Clone o repositório:
```bash
git clone git@github.com:rogeriobiondi/rust-api-sample.git
cd rust-api-sample
```

2. Configure as variáveis de ambiente:
```bash
cp .env.example .env
```

3. Edite o arquivo `.env` com suas credenciais do banco:
```
DATABASE_URL=postgresql://postgres:SUA_SENHA@db.SEU_PROJETO.supabase.co:5432/postgres
```

4. Crie a tabela no banco de dados executando o SQL em `sql/itens.sql`

5. (Opcional) Popule com dados de teste executando `sql/seed.sql`

## Executando

### Modo desenvolvimento
```bash
cargo run
```

A API estará disponível em `http://localhost:3000`

### Modo release (otimizado)
```bash
cargo run --release
```

## Gerando o binário

### Build de desenvolvimento
```bash
cargo build
```
Binário gerado em: `target/debug/rust-api-sample`

### Build de produção (otimizado)
```bash
cargo build --release
```
Binário gerado em: `target/release/rust-api-sample`

## Endpoints

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/` | Mensagem de boas-vindas |
| GET | `/itens` | Lista itens com busca, ordenação e paginação |
| GET | `/itens/{id}` | Busca item por ID |
| POST | `/itens` | Cria novo item |
| PUT | `/itens/{id}` | Atualiza item existente |
| DELETE | `/itens/{id}` | Remove item |

### Parâmetros de listagem (GET /itens)

| Parâmetro | Tipo | Descrição |
|-----------|------|-----------|
| `busca` | string | Busca por ID ou nome (case-insensitive) |
| `ordenar_por` | string | Campo: `id`, `nome` ou `preco` |
| `ordem` | string | Direção: `asc` ou `desc` |
| `pagina` | number | Número da página (começa em 1) |
| `por_pagina` | number | Itens por página (padrão: 10, máx: 100) |

### Exemplo de requisição
```bash
curl "http://localhost:3000/itens?busca=notebook&ordenar_por=preco&ordem=desc&pagina=1&por_pagina=5"
```

### Exemplo de resposta
```json
{
  "itens": [
    { "id": 1, "nome": "Notebook Dell", "preco": 3499.90 }
  ],
  "total": 1,
  "pagina": 1,
  "por_pagina": 5,
  "total_paginas": 1
}
```

## Documentação Swagger

Acesse a documentação interativa em:
```
http://localhost:3000/swagger-ui/
```

## Estrutura do projeto

```
rust-api-sample/
├── Cargo.toml          # Dependências e configuração
├── .env.example        # Template de variáveis de ambiente
├── .gitignore          # Arquivos ignorados pelo Git
├── sql/
│   ├── itens.sql       # Script de criação da tabela
│   └── seed.sql        # Dados de teste (30 produtos)
└── src/
    ├── main.rs         # Código principal da API
    └── swagger-ui.html # Template do Swagger UI
```

## Licença

MIT
