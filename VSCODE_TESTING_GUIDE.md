# 🚀 VSCode Step-by-Step Testing Guide

## Overview
This guide will help you run and test the Resume Critique Engine project using VSCode terminals and extensions.

## 📁 Project Structure
```
resume-critique-engine/
├── simple-backend/     # Python FastAPI backend (simplified)
├── ai-service/         # AI analysis service (optional for now)
├── frontend/           # Next.js frontend
└── docs/              # Documentation
```

---

## 🔧 Step 1: Open Project in VSCode

1. **Open VSCode**
2. **File → Open Folder**
3. **Navigate to**: `~/personal-projects/resume-critique-engine`
4. **Click "Open"**

---

## ⚡ Step 2: Setup Backend (Python FastAPI)

### Terminal 1: Backend Setup
1. **Open Terminal**: `Terminal → New Terminal` (or Ctrl/Cmd + Shift + `)
2. **Navigate to backend**:
   ```bash
   cd simple-backend
   ```
3. **Create virtual environment**:
   ```bash
   python3 -m venv venv
   ```
4. **Install dependencies**:
   ```bash
   source venv/bin/activate && pip install -r requirements.txt
   ```
5. **Start backend server**:
   ```bash
   source venv/bin/activate && python3 main.py
   ```

✅ **Expected Output**:
```
INFO:     Started server process [xxxxx]
INFO:     Waiting for application startup.
INFO:     Application startup complete.
INFO:     Uvicorn running on http://0.0.0.0:3000
```

🔗 **Test Backend**: Open http://localhost:3000 in browser
- Should see: `{"message": "Resume Critique Simple Backend", "status": "running"}`

---

## 🎨 Step 3: Setup Frontend (Next.js)

### Terminal 2: Frontend Setup
1. **Open New Terminal**: `Terminal → New Terminal`
2. **Navigate to frontend**:
   ```bash
   cd frontend
   ```
3. **Install dependencies** (may take a few minutes):
   ```bash
   npm install --legacy-peer-deps
   ```
4. **Start frontend server**:
   ```bash
   npm run dev
   ```

✅ **Expected Output**:
```
- ready started server on 0.0.0.0:3000, url: http://localhost:3000
- info Loaded env from .env.local
```

🔗 **Test Frontend**: Open http://localhost:3000 in browser
- Should see the Resume Critique Engine homepage

---

## 🧪 Step 4: Test the Complete Flow

### Create Test Resume File
1. **Create new file**: `test-resume.txt`
2. **Add sample content**:
```txt
John Doe
Software Engineer

Experience:
- Developed web applications using React and Node.js
- Managed team of 5 developers
- Increased system efficiency by 25%
- Led project that reduced costs by $50,000

Skills:
- JavaScript, Python, React
- Team Leadership
- Project Management
```

### Test Upload Process
1. **Go to Frontend**: http://localhost:3000
2. **Upload test resume**:
   - Click "Click to upload" or drag file
   - Select `test-resume.txt`
   - Click "Analyze Resume"
3. **Watch terminals** for processing logs
4. **View results** with scores and suggestions

---

## 🐛 Step 5: Debug Common Issues

### Issue 1: Port Already in Use
```bash
# Kill process on port 3000
lsof -ti:3000 | xargs kill -9
```

### Issue 2: Frontend Won't Start
```bash
# Clear npm cache and reinstall
npm cache clean --force
rm -rf node_modules
npm install --legacy-peer-deps
```

### Issue 3: Python Dependencies
```bash
# Install missing packages
pip3 install fastapi uvicorn python-multipart
```

---

## 📊 Step 6: Monitor and Test

### VSCode Extensions (Recommended)
1. **Python** (Microsoft)
2. **REST Client** (for API testing)
3. **Thunder Client** (alternative to Postman)

### Manual API Testing
Create `test-api.http` file:
```http
### Test Backend Health
GET http://localhost:3000/

### Test File Upload (use Thunder Client for file upload)
POST http://localhost:3000/upload-resume
Content-Type: multipart/form-data
```

---

## 🎯 Step 7: Development Workflow

### Making Changes
1. **Backend changes**: Server auto-reloads (FastAPI with `--reload`)
2. **Frontend changes**: Hot reload (Next.js development mode)

### Viewing Logs
- **Backend Terminal**: API requests, errors, processing
- **Frontend Terminal**: Build info, React errors
- **Browser Console**: Frontend JavaScript errors

### File Structure for Development
```
VSCode Explorer:
├── simple-backend/
│   ├── main.py          ← Edit backend logic
│   └── resume_critique.db ← SQLite database (auto-created)
├── frontend/
│   ├── src/
│   │   ├── app/page.tsx ← Main app page
│   │   └── components/  ← React components
│   └── .env.local       ← Frontend config
└── test-resume.txt      ← Your test file
```

---

## ✅ Success Checklist

- [ ] Backend running on port 3000
- [ ] Frontend running on port 3000 (different from backend)
- [ ] Can upload resume file
- [ ] See analysis results with scores
- [ ] Can view improvement suggestions
- [ ] No errors in terminals

---

## 🚀 Next Steps After Testing

1. **Add Real AI**: Replace mock analysis with OpenAI integration
2. **Enhanced UI**: Customize styling and add features
3. **User Authentication**: Add login/signup
4. **File Storage**: Save uploaded files
5. **History**: View previous analyses

---

## 🆘 Need Help?

### Check Logs
- Backend errors: Simple-backend terminal
- Frontend errors: Frontend terminal + browser console
- Network errors: Browser Network tab

### Common Solutions
- Restart servers if something breaks
- Check port conflicts (3000 is used by both)
- Verify file permissions
- Clear browser cache if UI issues

Happy coding! 🎉
