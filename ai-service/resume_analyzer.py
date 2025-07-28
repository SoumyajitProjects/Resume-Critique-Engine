from langchain_openai import ChatOpenAI
from langchain.prompts import ChatPromptTemplate
from langchain.output_parsers import PydanticOutputParser
from langchain.schema import HumanMessage
from pydantic import BaseModel, Field
from typing import Dict, Any, List
import json
import re

class ResumeScores(BaseModel):
    overall_score: float = Field(description="Overall resume score (0-5)")
    structure_score: float = Field(description="Resume structure and organization score (0-5)")
    keywords_score: float = Field(description="Industry keywords usage score (0-5)")
    action_verbs_score: float = Field(description="Action verbs usage score (0-5)")
    quantified_impact_score: float = Field(description="Quantified achievements score (0-5)")
    readability_score: float = Field(description="Readability and clarity score (0-5)")
    detailed_feedback: Dict[str, str] = Field(description="Detailed feedback for each category")
    improvement_suggestions: Dict[str, List[str]] = Field(description="Specific improvement suggestions")

class ResumeAnalyzer:
    def __init__(self, api_key: str, model_name: str = "gpt-4", max_tokens: int = 2000, temperature: float = 0.3):
        self.llm = ChatOpenAI(
            api_key=api_key,
            model=model_name,
            max_tokens=max_tokens,
            temperature=temperature
        )
        self.parser = PydanticOutputParser(pydantic_object=ResumeScores)

    async def analyze_resume(self, resume_text: str, filename: str) -> Dict[str, Any]:
        """Analyze resume using LangChain and OpenAI GPT-4"""
        
        # Create structured prompt
        prompt_template = ChatPromptTemplate.from_messages([
            ("system", self._get_system_prompt()),
            ("human", "Resume filename: {filename}\n\nResume content:\n{resume_text}\n\n{format_instructions}")
        ])
        
        # Format the prompt
        formatted_prompt = prompt_template.format_messages(
            filename=filename,
            resume_text=resume_text,
            format_instructions=self.parser.get_format_instructions()
        )
        
        try:
            # Get AI response
            response = await self.llm.ainvoke(formatted_prompt)
            
            # Parse the structured response
            parsed_result = self.parser.parse(response.content)
            
            return {
                "overall_score": parsed_result.overall_score,
                "structure_score": parsed_result.structure_score,
                "keywords_score": parsed_result.keywords_score,
                "action_verbs_score": parsed_result.action_verbs_score,
                "quantified_impact_score": parsed_result.quantified_impact_score,
                "readability_score": parsed_result.readability_score,
                "detailed_feedback": parsed_result.detailed_feedback,
                "improvement_suggestions": parsed_result.improvement_suggestions
            }
        
        except Exception as e:
            # Fallback to basic analysis if structured parsing fails
            print(f"Structured parsing failed: {e}, falling back to basic analysis")
            return await self._fallback_analysis(resume_text, filename)

    def _get_system_prompt(self) -> str:
        return """
You are an expert resume reviewer and career coach with over 15 years of experience in talent acquisition and career development. Your task is to provide comprehensive, actionable feedback on resumes.

Analyze the following resume across these specific criteria:

1. **Structure & Organization (0-5 scale)**:
   - Clear sections (Contact, Summary, Experience, Education, Skills)
   - Logical flow and hierarchy
   - Consistent formatting
   - Appropriate length (1-2 pages)

2. **Keywords & Industry Relevance (0-5 scale)**:
   - Industry-specific terminology
   - Role-relevant keywords
   - Technical skills mentioned
   - ATS-friendly language

3. **Action Verbs Usage (0-5 scale)**:
   - Strong, specific action verbs
   - Variety in verb usage
   - Present/past tense consistency
   - Impact-focused language

4. **Quantified Impact (0-5 scale)**:
   - Measurable achievements
   - Specific numbers, percentages, or metrics
   - Results-oriented statements
   - Business impact demonstration

5. **Readability & Clarity (0-5 scale)**:
   - Clear, concise language
   - Appropriate grammar and spelling
   - Professional tone
   - Easy to scan and read

Provide scores for each category and an overall score. Include detailed feedback explaining your scores and specific, actionable improvement suggestions.
"""

    async def _fallback_analysis(self, resume_text: str, filename: str) -> Dict[str, Any]:
        """Fallback analysis if structured parsing fails"""
        
        # Basic analysis using simple heuristics
        word_count = len(resume_text.split())
        
        # Count action verbs (simplified)
        action_verbs = ['managed', 'led', 'developed', 'created', 'implemented', 'improved', 'increased', 'achieved']
        verb_count = sum(1 for verb in action_verbs if verb.lower() in resume_text.lower())
        
        # Count quantified statements (simplified)
        numbers = re.findall(r'\b\d+[%]?\b', resume_text)
        quantified_count = len(numbers)
        
        # Basic scoring
        structure_score = min(5.0, max(1.0, word_count / 100))  # Rough estimate
        keywords_score = 3.5  # Default
        action_verbs_score = min(5.0, verb_count * 0.5 + 2.0)
        quantified_impact_score = min(5.0, quantified_count * 0.3 + 2.0)
        readability_score = 4.0  # Default
        overall_score = (structure_score + keywords_score + action_verbs_score + quantified_impact_score + readability_score) / 5
        
        return {
            "overall_score": round(overall_score, 1),
            "structure_score": round(structure_score, 1),
            "keywords_score": round(keywords_score, 1),
            "action_verbs_score": round(action_verbs_score, 1),
            "quantified_impact_score": round(quantified_impact_score, 1),
            "readability_score": round(readability_score, 1),
            "detailed_feedback": {
                "structure": "Resume structure appears adequate based on length analysis.",
                "keywords": "Consider adding more industry-specific keywords.",
                "action_verbs": f"Detected {verb_count} strong action verbs. Consider adding more variety.",
                "quantified_impact": f"Found {quantified_count} quantified achievements. Add more specific metrics.",
                "readability": "Resume appears to have good readability."
            },
            "improvement_suggestions": {
                "structure": ["Ensure clear section headers", "Maintain consistent formatting"],
                "keywords": ["Research job descriptions for relevant keywords", "Include technical skills section"],
                "action_verbs": ["Use more diverse action verbs", "Start bullet points with strong verbs"],
                "quantified_impact": ["Add specific numbers and percentages", "Quantify achievements where possible"],
                "readability": ["Keep bullet points concise", "Use professional language throughout"]
            }
        }
