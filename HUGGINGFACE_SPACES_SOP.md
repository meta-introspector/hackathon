# Standard Operating Procedure: HuggingFace Spaces Deployment

**Version:** 1.0  
**Last Updated:** 2026-01-26  
**Purpose:** Deploy interactive applications to HuggingFace Spaces

---

## 1. Prerequisites

### 1.1 Required Access
- HuggingFace account (free tier available)
- Git/GitHub repository access
- HuggingFace API token (optional for CLI)

### 1.2 Required Files
- `app.py` - Gradio/Streamlit application
- `requirements.txt` - Python dependencies
- `README.md` - Space documentation

---

## 2. Initial Setup

### 2.1 Create New Space

**Steps:**
1. Navigate to [HuggingFace Spaces](https://huggingface.co/spaces)
2. Click **Create new Space**
3. Choose **Space name** (e.g., `provenance-game`)
4. Select **SDK**: Gradio or Streamlit
5. Choose **License**: MIT recommended
6. Select **Visibility**: Public or Private
7. Click **Create Space**

### 2.2 Connect Git Repository

**Option A: Direct Push**
```bash
# Clone the Space
git clone https://huggingface.co/spaces/USERNAME/SPACE_NAME
cd SPACE_NAME

# Add your files
cp /path/to/app.py .
cp /path/to/requirements.txt .

# Commit and push
git add .
git commit -m "Initial deployment"
git push
```

**Option B: GitHub Sync**
1. Go to Space **Settings** → **Repository**
2. Enable **GitHub Sync**
3. Connect GitHub repository
4. Select branch to sync
5. Auto-deploys on push

---

## 3. Application Configuration

### 3.1 Gradio App Structure

**Minimal app.py:**
```python
import gradio as gr

def game_interface():
    with gr.Blocks() as demo:
        gr.Markdown("# Project Provenance")
        # Add your game UI here
    return demo

if __name__ == "__main__":
    demo = game_interface()
    demo.launch()
```

**requirements.txt:**
```
gradio>=4.0.0
polars
```

### 3.2 Streamlit App Structure

**Minimal app.py:**
```python
import streamlit as st

st.title("Project Provenance")
# Add your game UI here
```

**requirements.txt:**
```
streamlit>=1.30.0
polars
```

### 3.3 Space Configuration (README.md)

```yaml
---
title: Project Provenance
emoji: 🎮
colorFrom: blue
colorTo: purple
sdk: gradio
sdk_version: 4.0.0
app_file: app.py
pinned: false
---

# Project Provenance

Educational game teaching data provenance through interactive gameplay.
```

---

## 4. Deployment Methods

### 4.1 Web Interface Deployment

1. Go to your Space
2. Click **Files** tab
3. Click **Add file** → **Upload files**
4. Upload `app.py`, `requirements.txt`, `README.md`
5. Commit changes
6. Space auto-builds and deploys

### 4.2 Git CLI Deployment

```bash
# Clone Space
git clone https://huggingface.co/spaces/USERNAME/SPACE_NAME
cd SPACE_NAME

# Copy files
cp /path/to/hackathon/app.py .
cp /path/to/hackathon/requirements.txt .

# Commit and push
git add .
git commit -m "Deploy Project Provenance"
git push
```

### 4.3 HuggingFace CLI Deployment

```bash
# Install CLI
pip install huggingface_hub

# Login
huggingface-cli login

# Upload files
huggingface-cli upload USERNAME/SPACE_NAME ./app.py
huggingface-cli upload USERNAME/SPACE_NAME ./requirements.txt
```

---

## 5. Environment Variables & Secrets

### 5.1 Add Secrets

1. Go to Space **Settings** → **Repository secrets**
2. Click **New secret**
3. Add name and value
4. Click **Add secret**

**Access in code:**
```python
import os
secret = os.environ.get("SECRET_NAME")
```

### 5.2 Common Secrets
- API keys
- Database credentials
- Authentication tokens

---

## 6. Monitoring and Troubleshooting

### 6.1 View Build Logs

1. Go to your Space
2. Click **Logs** tab
3. View real-time build output
4. Check for errors

### 6.2 Common Issues

**Build fails:**
- Check `requirements.txt` syntax
- Verify Python version compatibility
- Check for missing dependencies

**App doesn't load:**
- Verify `app.py` has `demo.launch()` (Gradio)
- Check port configuration
- Review error logs

**Slow performance:**
- Optimize data loading
- Use caching (`@st.cache_data`)
- Reduce model size

---

## 7. Best Practices

### 7.1 File Structure
```
space/
├── app.py              # Main application
├── requirements.txt    # Dependencies
├── README.md          # Space config
├── assets/            # Images, data
└── utils/             # Helper modules
```

### 7.2 Performance
- Cache expensive operations
- Lazy load large datasets
- Use efficient data formats (Parquet)

### 7.3 Security
- Never commit secrets to git
- Use environment variables
- Validate user inputs

---

## 8. Quick Reference

### 8.1 Common Commands

```bash
# Clone Space
git clone https://huggingface.co/spaces/USER/SPACE

# Push changes
git add .
git commit -m "Update"
git push

# View logs
huggingface-cli logs USER/SPACE
```

### 8.2 URLs

- **Spaces Dashboard:** https://huggingface.co/spaces
- **Documentation:** https://huggingface.co/docs/hub/spaces
- **Gradio Docs:** https://gradio.app/docs
- **Streamlit Docs:** https://docs.streamlit.io

---

## 9. Deployment Checklist

### Pre-Deployment
- [ ] `app.py` tested locally
- [ ] `requirements.txt` complete
- [ ] README.md configured
- [ ] Secrets added (if needed)
- [ ] Assets uploaded

### Post-Deployment
- [ ] Space builds successfully
- [ ] App loads without errors
- [ ] All features functional
- [ ] Performance acceptable
- [ ] Share URL with users

---

**Document Control:**
- **Owner:** DevOps Team
- **Review Cycle:** Quarterly
- **Next Review:** 2026-04-26
