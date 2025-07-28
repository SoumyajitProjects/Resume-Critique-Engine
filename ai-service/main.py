from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Dict, Any
import os
from dotenv import load_dotenv
import json

from resume_analyzer import ResumeAnalyzer

load_dotenv()

app = FastAPI(title="Resume Critique AI Service", version="1.0.0")

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize the resume analyzer
analyzer = ResumeAnalyzer(
    api_key=os.getenv("OPENAI_API_KEY"),
    model_name=os.getenv("MODEL_NAME", "gpt-4"),
    max_tokens=int(os.getenv("MAX_TOKENS", "2000")),
    temperature=float(os.getenv("TEMPERATURE", "0.3"))
)

class CritiqueRequest(BaseModel):
    resume_text: str
    filename: str

class CritiqueResponse(BaseModel):
    overall_score: float
    structure_score: float
    keywords_score: float
    action_verbs_score: float
    quantified_impact_score: float
    readability_score: float
    detailed_feedback: Dict[str, Any]
    improvement_suggestions: Dict[str, Any]

@app.get("/health")
async def health_check():
    return {"status": "healthy", "service": "resume-critique-ai"}

@app.post("/critique", response_model=CritiqueResponse)
async def critique_resume(request: CritiqueRequest):
    try:
        # Analyze the resume using LangChain and OpenAI
        analysis_result = await analyzer.analyze_resume(
            resume_text=request.resume_text,
            filename=request.filename
        )
        
        return CritiqueResponse(**analysis_result)
    
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Analysis failed: {str(e)}")

@app.get("/")
async def root():
    return {
        "message": "Resume Critique AI Service",
        "version": "1.0.0",
        "endpoints": {
            "health": "/health",
            "critique": "/critique"
        }
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8001)
