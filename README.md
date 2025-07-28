# AI-Powered Resume Critique Engine

A comprehensive full-stack application that analyzes resumes using AI and provides detailed feedback with scoring across multiple criteria.

## Tech Stack

- **Backend**: Rust + Warp (API server)
- **AI Service**: Python + FastAPI + LangChain + OpenAI
- **Database**: PostgreSQL (via Docker)
- **Frontend**: Next.js + Tailwind CSS
- **File Processing**: PyMuPDF for PDF text extraction

## Architecture

```
Frontend (Next.js) → Backend (Rust/Warp) → AI Service (Python/FastAPI) → OpenAI GPT-4
                            ↓
                    PostgreSQL Database
```

## Features

- Resume upload (PDF/DOCX support)
- AI-powered critique with scoring on:
  - Structure & Organization
  - Keyword Optimization
  - Action Verbs Usage
  - Quantified Impact
  - Readability
- User authentication
- Feedback history
- Iterative improvement tracking

## Quick Start

1. **Prerequisites**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Node.js (v18+)
   # Install Python 3.9+
   # Install Docker
   ```

2. **Database Setup**
   ```bash
   cd database
   docker-compose up -d
   ```

3. **Backend Setup**
   ```bash
   cd backend
   cargo run
   ```

4. **AI Service Setup**
   ```bash
   cd ai-service
   pip install -r requirements.txt
   python -m uvicorn main:app --reload --port 8001
   ```

5. **Frontend Setup**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

## Environment Variables

Create `.env` files in each service directory:

- `backend/.env`
- `ai-service/.env`
- `frontend/.env.local`

See individual service READMEs for specific variables needed.

## API Endpoints

- `POST /upload-resume` - Upload resume file
- `GET /get-critique/:id` - Get critique results
- `POST /auth/login` - User authentication
- `GET /auth/me` - Get current user
- `GET /history` - Get user's critique history

## Development

Each service runs independently:
- Backend: http://localhost:3000
- AI Service: http://localhost:8001
- Frontend: http://localhost:3001
- Database: localhost:5432

## License

MIT
