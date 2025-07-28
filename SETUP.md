# Resume Critique Engine - Setup Guide

## Quick Start

### Prerequisites
- Rust (latest stable version)
- Node.js 18+
- Python 3.9+
- Docker (for PostgreSQL)

### 1. Database Setup
```bash
cd database
docker-compose up -d
```

### 2. Backend Setup (Rust)
```bash
cd backend
cp .env.example .env
# Edit .env with your JWT secret
cargo run
```
Backend will run on `http://localhost:3000`

### 3. AI Service Setup (Python)
```bash
cd ai-service
cp .env.example .env
# Add your OpenAI API key to .env
pip install -r requirements.txt
python -m uvicorn main:app --reload --port 8001
```
AI Service will run on `http://localhost:8001`

### 4. Frontend Setup (Next.js)
```bash
cd frontend
npm install
npm run dev
```
Frontend will run on `http://localhost:3001`

## Environment Variables

### Backend (.env)
```
DATABASE_URL=postgresql://postgres:password@localhost:5432/resume_critique
JWT_SECRET=your-super-secret-jwt-key-here
AI_SERVICE_URL=http://localhost:8001
UPLOAD_DIR=./uploads
MAX_FILE_SIZE=10485760
RUST_LOG=info
```

### AI Service (.env)
```
OPENAI_API_KEY=your-openai-api-key-here
MODEL_NAME=gpt-4
MAX_TOKENS=2000
TEMPERATURE=0.3
```

## Development Commands

### Backend
- `cargo run` - Start server
- `cargo test` - Run tests
- `cargo build --release` - Build for production

### AI Service
- `python -m uvicorn main:app --reload --port 8001` - Start with hot reload
- `python -m pytest` - Run tests

### Frontend
- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server

## API Endpoints

- `POST /upload-resume` - Upload and analyze resume
- `GET /get-critique/:id` - Get critique by ID
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `GET /auth/me` - Get current user
- `GET /history` - Get user's critique history

## Project Structure

```
resume-critique-engine/
├── backend/           # Rust + Warp API server
├── ai-service/        # Python + FastAPI + LangChain
├── frontend/          # Next.js + Tailwind CSS
├── database/          # PostgreSQL setup
└── docs/             # Documentation
```

## Features

✅ Resume upload (PDF/TXT support)
✅ AI-powered analysis with 5 criteria scoring
✅ User authentication with JWT
✅ Critique history tracking
✅ Clean, responsive UI
✅ Async processing
✅ Error handling

## Scoring Criteria

1. **Structure & Organization** (0-5)
2. **Keywords & Industry Relevance** (0-5)  
3. **Action Verbs Usage** (0-5)
4. **Quantified Impact** (0-5)
5. **Readability & Clarity** (0-5)

## Next Steps

- [ ] Add PDF text extraction with proper libraries
- [ ] Implement user authentication in frontend
- [ ] Add resume version comparison
- [ ] Implement feedback loop for improvement tracking
- [ ] Add more file format support
- [ ] Deploy to cloud platforms
