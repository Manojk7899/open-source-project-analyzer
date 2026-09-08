# Open Source Project Analyzer

A comprehensive tool to analyze GitHub repositories and generate detailed insights using repository metrics, deterministic scoring, and AI-powered analysis.

## Overview

This project provides an automated solution for analyzing open-source GitHub repositories. It fetches repository metadata, calculates quality scores, and generates AI-powered insights to help developers understand project health and characteristics.

## Features

- 🔍 **GitHub Repository Analysis** - Fetch and analyze repository metadata, README content, and commit history
- 📊 **Deterministic Scoring** - Calculate consistent repository quality scores based on multiple metrics
- 🤖 **AI-Powered Insights** - Generate detailed analysis using Gemini AI
- 💾 **PostgreSQL Storage** - Persistent storage for repositories, scores, and analyses
- 🎨 **Web Frontend** - Next.js-based UI for easy interaction and visualization
- 🔌 **RESTful API** - Complete backend API for programmatic access
- 📈 **Comprehensive Reports** - Combine data, scores, and analysis into detailed reports

## Tech Stack

- **Backend:** Rust with Actix-web framework
- **Frontend:** Next.js with React
- **Database:** PostgreSQL
- **AI:** Google Gemini API
- **Containerization:** Docker & Docker Compose

## Prerequisites

Before getting started, ensure you have:

- **Rust** and **Cargo** (latest stable version)
- **Docker** and **Docker Compose**
- **Node.js** (v16 or higher) and **npm**
- **GitHub Account** (for repository access)
- **Gemini API Key** (get one from [Google AI Studio](https://aistudio.google.com/apikey))

## Installation & Setup

### 1. Clone the Repository

```bash
git clone https://github.com/Manojk7899/open-source-project-analyzer.git
cd open-source-project-analyzer
```

### 2. Environment Configuration

Create a `.env` file in the project root with your API keys and database configuration:

```env
DATABASE_URL=postgres://analyzer:analyzer@localhost:5432/open_source_analyzer
GEMINI_API_KEY=your-gemini-api-key-here
GEMINI_MODEL=gemini-3.6-flash
RUST_LOG=info
```

⚠️ **Important:** Never commit `.env` or API keys to version control. Add `.env` to `.gitignore`.

### 3. Start PostgreSQL Database

```bash
# Start PostgreSQL container
docker compose up -d

# Verify container is running
docker compose ps
```

**Database Details:**
- Host: `localhost`
- Port: `5432`
- User: `analyzer`
- Password: `analyzer`
- Database: `open_source_analyzer`

### 4. Run Database Migrations

Initialize the database schema:

```bash
cargo run --bin migrate
```

### 5. Start the Rust Backend

In your main terminal:

```bash
cargo run
```

The API server will start at: `http://127.0.0.1:8081`

**Health Check:**
```bash
curl http://127.0.0.1:8081/health
```

### 6. Start the Next.js Frontend

In a second terminal:

```bash
cd frontend
npm install
npm run dev
```

The frontend will be available at: `http://localhost:3000` (or the URL shown in terminal)

## API Usage

All API endpoints follow a specific workflow. A repository must be fetched first before scores and analyses can be generated.

### Complete Workflow Example

**Step 1: Fetch Repository**
```http
POST http://127.0.0.1:8081/api/repository/fetch
Content-Type: application/json

{
  "url": "https://github.com/tokio-rs/axum"
}
```

**Step 2: Calculate Score** (replace `1` with the returned repository ID)
```http
POST http://127.0.0.1:8081/api/repository/1/score
```

**Step 3: Generate AI Analysis**
```http
POST http://127.0.0.1:8081/api/repository/1/ai-analysis
```

**Step 4: Get Complete Report**
```http
GET http://127.0.0.1:8081/api/repository/1/report
```

### Additional Endpoints

```http
# Preview repository (without storing)
POST http://127.0.0.1:8081/api/repository/preview

# List all repositories
GET http://127.0.0.1:8081/api/repository/

# Get specific repository
GET http://127.0.0.1:8081/api/repository/1

# Get repository score
GET http://127.0.0.1:8081/api/repository/1/score

# Get AI analysis
GET http://127.0.0.1:8081/api/repository/1/ai-analysis
```

**Note:** All requests are available in the `requests.http` file for testing with REST clients.

## Database Verification

To verify stored data in PostgreSQL:

```bash
PGPASSWORD=analyzer psql -h localhost -p 5432 -U analyzer -d open_source_analyzer
```

Then run SQL queries:

```sql
-- View all analyzed repositories
SELECT * FROM repositories;

-- View repository scores
SELECT * FROM repository_scores;

-- View AI analyses
SELECT * FROM repository_ai_analyses;
```

## Project Structure

```
.
├── src/                    # Rust backend source code
├── frontend/               # Next.js frontend application
├── migrations/             # Database migration files
├── docker-compose.yml      # Docker services configuration
├── requests.http           # HTTP client requests for testing
├── .env.example            # Example environment variables
└── README.md              # This file
```

## Development

### Backend Development

Check Rust code for errors:
```bash
cargo check
```

Run tests:
```bash
cargo test
```

Run with debug logging:
```bash
RUST_LOG=debug cargo run
```

### Frontend Development

Check for linting issues:
```bash
cd frontend
npm run lint
```

Build for production:
```bash
npm run build
```

Start production build:
```bash
npm start
```

## Troubleshooting

### Database Connection Issues

- Ensure Docker is running: `docker compose ps`
- Check database credentials in `.env`
- Verify PostgreSQL is accessible: `psql -h localhost -U analyzer`

### API Not Responding

- Confirm Rust backend is running on `http://127.0.0.1:8081`
- Check terminal for error logs
- Verify environment variables are loaded

### Frontend Can't Connect to API

- Ensure both backend and frontend are running
- Check that backend is on port `8081` and accessible
- Look for CORS errors in browser console

### Gemini API Errors

- Verify API key is correct in `.env`
- Ensure API is enabled in Google Cloud Console
- Check rate limits and quota usage

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support & Issues

Found a bug or have a feature request? Please open an issue on [GitHub Issues](https://github.com/Manojk7899/open-source-project-analyzer/issues).

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Next.js](https://nextjs.org/)
- Powered by [Google Gemini AI](https://ai.google.dev/)
- Database powered by [PostgreSQL](https://www.postgresql.org/)

---

**Happy analyzing! 🚀**
