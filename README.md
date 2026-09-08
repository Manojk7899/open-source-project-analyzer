# Open Source Project Analyzer

A powerful full-stack application for analyzing and understanding open-source projects. Built with **Rust** backend and **Next.js** frontend, this tool provides insights into project structure, dependencies, language composition, and other metrics.

## Features

- 📊 **Project Analysis** - Analyze repository structure and composition
- 🔍 **Language Detection** - Identify programming languages used in projects
- 📈 **Metrics & Insights** - Generate comprehensive project statistics
- 🎨 **Modern UI** - Interactive Next.js-based frontend
- ⚡ **High Performance** - Rust-based backend for fast processing
- 🐳 **Docker Support** - Easy deployment with Docker Compose

## Tech Stack

- **Backend**: Rust with Actix-web/Rocket framework
- **Frontend**: Next.js with TypeScript
- **Database**: PostgreSQL (migrations included)
- **DevOps**: Docker & Docker Compose
- **API**: RESTful HTTP endpoints

## Project Structure

```
.
├── src/                    # Rust backend source code
├── frontend/               # Next.js frontend application
├── migrations/             # Database migrations
├── Cargo.toml             # Rust dependencies
├── Cargo.lock             # Rust dependency lock file
├── compose.yml            # Docker Compose configuration
├── requests.http          # HTTP request examples
└── .gitignore             # Git ignore rules
```

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Docker](https://www.docker.com/products/docker-desktop) & Docker Compose
- [PostgreSQL](https://www.postgresql.org/) (or use Docker)

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/Manojk7899/open-source-project-analyzer.git
cd open-source-project-analyzer

# Start all services
docker-compose up -d

# Access the application
# Frontend: http://localhost:3000
# Backend API: http://localhost:8080
```

### Manual Setup

#### Backend (Rust)

```bash
# Install dependencies and build
cargo build --release

# Run the server
cargo run --release

# Run tests
cargo test
```

The backend server will start on `http://localhost:8080` by default.

#### Frontend (Next.js)

```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
npm start
```

The frontend will be available at `http://localhost:3000`.

#### Database Setup

```bash
# Run migrations
sqlx migrate run

# Or using Cargo
cargo sqlx migrate run
```

## API Endpoints

Refer to `requests.http` for example API calls. Key endpoints include:

- `GET /api/projects` - List all analyzed projects
- `POST /api/projects/analyze` - Analyze a new project
- `GET /api/projects/{id}` - Get project details
- `GET /api/projects/{id}/metrics` - Get project metrics

For detailed endpoint documentation, see the backend source code in `src/`.

## Configuration

### Environment Variables

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgresql://user:password@localhost/open_source_analyzer
RUST_LOG=info
API_HOST=0.0.0.0
API_PORT=8080
```

### Database

Update the connection string in `Cargo.toml` or environment variables to match your PostgreSQL setup.

## Development

### Backend Development

```bash
# Watch mode
cargo watch -x run

# Format code
cargo fmt

# Lint with Clippy
cargo clippy
```

### Frontend Development

```bash
cd frontend

# Development with hot reload
npm run dev

# Linting
npm run lint
```

## Deployment

### Deploy with Docker

```bash
# Build images
docker-compose build

# Start services
docker-compose up -d

# View logs
docker-compose logs -f
```

### Deploy to Production

1. Update environment variables in `.env.production`
2. Use your preferred hosting platform (Vercel for frontend, AWS/DigitalOcean for backend)
3. Configure database backups and monitoring

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Support

For issues, questions, or suggestions:
- Open an [Issue](https://github.com/Manojk7899/open-source-project-analyzer/issues)
- Check existing [Discussions](https://github.com/Manojk7899/open-source-project-analyzer/discussions)
- Read the [Wiki](https://github.com/Manojk7899/open-source-project-analyzer/wiki)

## Roadmap

- [ ] Advanced filtering and search capabilities
- [ ] Real-time project monitoring
- [ ] Export reports (PDF, CSV)
- [ ] GitHub integration
- [ ] Performance benchmarking
- [ ] Machine learning-based insights

## Author

**Manojk7899** - [GitHub Profile](https://github.com/Manojk7899)

---

**Happy Analyzing! 🚀**
