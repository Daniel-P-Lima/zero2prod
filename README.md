# Zero to Production - Newsletter API

Uma API de newsletter construída em Rust seguindo o livro "Zero to Production".

## Configuração

### 1. Clonagem e Dependências

```bash
git clone <seu-repositorio>
cd zero2prod
cargo build
```

### 2. Configuração do Banco de Dados

#### Para Desenvolvimento Local:
Certifique-se de ter PostgreSQL rodando localmente e execute:
```bash
./scripts/init_db.sh
```

#### Para Produção (DigitalOcean):
1. Copie o arquivo de exemplo:
```bash
cp .env.example .env
```

2. Preencha as variáveis de ambiente no arquivo `.env`:
```bash
# Database Configuration
DATABASE_HOST=app-c4ab7ff9-91ec-4912-bcb4-06bf2e9078c5-do-user-35339565-0.d.db.ondigitalocean.com
DATABASE_PORT=25060
DATABASE_USERNAME=newsletter
DATABASE_PASSWORD=sua-senha-aqui
DATABASE_NAME=newsletter

# Application Environment
APP_ENVIRONMENT=production
```

### 3. Executar a Aplicação

#### Desenvolvimento:
```bash
cargo run
```

#### Produção:
```bash
APP_ENVIRONMENT=production cargo run
```

## Deploy no DigitalOcean App Platform

1. Configure as variáveis de ambiente no painel do DigitalOcean App Platform
2. Use o `spec.yaml` para deploy via CLI ou faça upload do código diretamente

## Segurança

- **Nunca commite credenciais** no código
- Use variáveis de ambiente para informações sensíveis
- O arquivo `.env` está no `.gitignore` para evitar acidentes