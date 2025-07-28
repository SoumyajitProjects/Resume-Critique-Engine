from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi.middleware.cors import CORSMiddleware
import sqlite3
import json
import os
from datetime import datetime
import hashlib
from typing import Dict, Any

app = FastAPI(title="Resume Critique Simple Backend", version="1.0.0")

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3001", "http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize SQLite database
def init_db():
    conn = sqlite3.connect('resume_critique.db')
    cursor = conn.cursor()
    
    # Create tables
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS resumes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            content TEXT NOT NULL,
            uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    ''')
    
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS critiques (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            resume_id INTEGER,
            overall_score REAL,
            structure_score REAL,
            keywords_score REAL,
            action_verbs_score REAL,
            quantified_impact_score REAL,
            readability_score REAL,
            detailed_feedback TEXT,
            improvement_suggestions TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (resume_id) REFERENCES resumes (id)
        )
    ''')
    
    conn.commit()
    conn.close()

# Initialize database on startup
init_db()

# Mock AI analysis function (replace with actual AI service call)
def analyze_resume_mock(content: str, filename: str) -> Dict[str, Any]:
    """Mock analysis - replace with actual AI service call"""
    import random
    
    # Generate random but realistic scores
    structure_score = round(random.uniform(3.0, 5.0), 1)
    keywords_score = round(random.uniform(2.5, 4.5), 1)
    action_verbs_score = round(random.uniform(3.5, 4.8), 1)
    quantified_impact_score = round(random.uniform(2.8, 4.3), 1)
    readability_score = round(random.uniform(3.2, 4.6), 1)
    
    overall_score = round((structure_score + keywords_score + action_verbs_score + 
                          quantified_impact_score + readability_score) / 5, 1)
    
    return {
        "overall_score": overall_score,
        "structure_score": structure_score,
        "keywords_score": keywords_score,
        "action_verbs_score": action_verbs_score,
        "quantified_impact_score": quantified_impact_score,
        "readability_score": readability_score,
        "detailed_feedback": {
            "structure": f"Resume structure shows good organization with clear sections.",
            "keywords": f"Consider adding more industry-specific keywords relevant to your field.",
            "action_verbs": f"Good use of action verbs, but could benefit from more variety.",
            "quantified_impact": f"Some quantified achievements present, but more specific metrics would strengthen impact.",
            "readability": f"Resume is generally readable with professional formatting."
        },
        "improvement_suggestions": {
            "structure": ["Use consistent formatting throughout", "Add clear section headers"],
            "keywords": ["Research job descriptions for relevant keywords", "Include technical skills section"],
            "action_verbs": ["Use more diverse action verbs", "Start each bullet point with a strong verb"],
            "quantified_impact": ["Add specific numbers and percentages", "Quantify achievements where possible"],
            "readability": ["Keep bullet points concise", "Use professional language throughout"]
        }
    }

@app.get("/")
async def root():
    return {"message": "Resume Critique Simple Backend", "status": "running"}

@app.post("/upload-resume")
async def upload_resume(resume: UploadFile = File(...)):
    try:
        # Read file content
        content = await resume.read()
        content_str = content.decode('utf-8') if resume.content_type == 'text/plain' else f"Binary content from {resume.filename}"
        
        # Store resume in database
        conn = sqlite3.connect('resume_critique.db')
        cursor = conn.cursor()
        
        cursor.execute(
            "INSERT INTO resumes (filename, content) VALUES (?, ?)",
            (resume.filename, content_str)
        )
        resume_id = cursor.lastrowid
        
        # Analyze resume (mock for now)
        analysis = analyze_resume_mock(content_str, resume.filename)
        
        # Store critique
        cursor.execute(
            """INSERT INTO critiques 
               (resume_id, overall_score, structure_score, keywords_score, 
                action_verbs_score, quantified_impact_score, readability_score,
                detailed_feedback, improvement_suggestions)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
            (resume_id, analysis["overall_score"], analysis["structure_score"],
             analysis["keywords_score"], analysis["action_verbs_score"],
             analysis["quantified_impact_score"], analysis["readability_score"],
             json.dumps(analysis["detailed_feedback"]), 
             json.dumps(analysis["improvement_suggestions"]))
        )
        critique_id = cursor.lastrowid
        
        conn.commit()
        conn.close()
        
        return {
            "message": "Resume uploaded and analyzed successfully",
            "critique_id": critique_id,
            "resume_filename": resume.filename,
            "overall_score": analysis["overall_score"],
            "scores": {
                "structure": analysis["structure_score"],
                "keywords": analysis["keywords_score"],
                "action_verbs": analysis["action_verbs_score"],
                "quantified_impact": analysis["quantified_impact_score"],
                "readability": analysis["readability_score"]
            },
            "detailed_feedback": analysis["detailed_feedback"],
            "improvement_suggestions": analysis["improvement_suggestions"]
        }
        
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error processing resume: {str(e)}")

@app.get("/get-critique/{critique_id}")
async def get_critique(critique_id: int):
    try:
        conn = sqlite3.connect('resume_critique.db')
        cursor = conn.cursor()
        
        cursor.execute("""
            SELECT c.*, r.filename 
            FROM critiques c
            JOIN resumes r ON c.resume_id = r.id
            WHERE c.id = ?
        """, (critique_id,))
        
        result = cursor.fetchone()
        conn.close()
        
        if not result:
            raise HTTPException(status_code=404, detail="Critique not found")
        
        return {
            "id": result[0],
            "resume_filename": result[11],
            "overall_score": result[2],
            "scores": {
                "structure": result[3],
                "keywords": result[4],
                "action_verbs": result[5],
                "quantified_impact": result[6],
                "readability": result[7]
            },
            "detailed_feedback": json.loads(result[8]),
            "improvement_suggestions": json.loads(result[9]),
            "created_at": result[10]
        }
        
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error retrieving critique: {str(e)}")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=3000)
