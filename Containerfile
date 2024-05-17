# Use uma imagem base mínima do Rust
FROM rust:alpine AS builder

# Instale dependências necessárias para compilar o projeto
RUN apk add --no-cache musl-dev

# Crie o diretório de trabalho e copie o código-fonte para o contêiner
WORKDIR /app
COPY . .

# Compile o código
RUN cargo build --release

# Etapa de construção concluída. Agora, crie uma imagem menor para a execução
FROM alpine:latest

# Copie o binário compilado para o contêiner final
COPY --from=builder /app/target/release/http_server /usr/local/bin/http_server

# Copie os arquivos estáticos para o contêiner final
COPY --chmod=755 response_files /response_files

# Exponha a porta em que o servidor HTTP está ouvindo
EXPOSE 8080

# Comando para iniciar o servidor HTTP quando o contêiner for iniciado
CMD ["http_server"]
