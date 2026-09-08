# Open Source Project Analyzer

A Rust backend and Next.js frontend that analyze GitHub repositories using repository metrics, deterministic scoring, PostgreSQL, and Gemini AI analysis.

## Features

- Fetch GitHub repository metadata and README content
- Store repositories in PostgreSQL
- Calculate deterministic repository scores
- Generate and store Gemini AI analysis
- Combine repository data, scores, and AI analysis into a report
- Use the web frontend or `requests.http` to test the API

## Requirements

- Rust and Cargo
- Docker and Docker Compose
- Node.js and npm
- A Gemini API key

## Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://analyzer:analyzer@localhost:5432/open_source_analyzer
GEMINI_API_KEY=your-gemini-api-key
GEMINI_MODEL=gemini-3.6-flash
```

Do not commit `.env` or API keys to GitHub.

## Start PostgreSQL

From the project root:

```bash
docker compose up -d
docker compose ps
```

The database runs on `localhost:5432` with these default values:

- User: `analyzer`
- Password: `analyzer`
- Database: `open_source_analyzer`

## Run Database Migrations

```bash
cargo run --bin migrate
```

## Run the Rust API

```bash
cargo run
```

The API runs at:

```text
http://127.0.0.1:8081
```

Health check:

```text
GET http://127.0.0.1:8081/health
```

## API Workflow

A repository must be fetched before scores and reports can be generated.

### 1. Fetch and store a repository

```http
POST http://127.0.0.1:8081/api/repository/fetch
Content-Type: application/json

{
  "url": "https://github.com/tokio-rs/axum"
}
```

### 2. Calculate and store the score

Replace `1` with the returned database ID:

```http
POST http://127.0.0.1:8081/api/repository/1/score
```

### 3. Generate and store Gemini analysis

```http
POST http://127.0.0.1:8081/api/repository/1/ai-analysis
```

### 4. Get the complete report

```http
GET http://127.0.0.1:8081/api/repository/1/report
```

### Other endpoints

```http
POST http://127.0.0.1:8081/api/repository/preview
GET http://127.0.0.1:8081/api/repository/
GET http://127.0.0.1:8081/api/repository/1
GET http://127.0.0.1:8081/api/repository/1/score
GET http://127.0.0.1:8081/api/repository/1/ai-analysis
```

The same requests are available in [requests.http](requests.http).

## Run the Frontend

In a second terminal:

```bash
cd frontend
npm install
npm run dev
```

The frontend runs at the URL printed by Next.js, usually:

```text
http://localhost:3000
```

The frontend expects the Rust API at `http://127.0.0.1:8081`.

## Verify Stored Data

```bash
PGPASSWORD=analyzer psql \
  -h localhost \
  -p 5432 \
  -U analyzer \
  -d open_source_analyzer
```

Then run:

```sql
SELECT * FROM repositories;
SELECT * FROM repository_scores;
SELECT * FROM repository_ai_analyses;
```

## Checks

Backend:

```bash
cargo check
```

Frontend:

```bash
cd frontend
npm run lint
```
