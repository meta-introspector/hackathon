# Deployment Guide - Hackathon Games

**Deploy to both Cloudflare Workers and HuggingFace Spaces**

---

## 🚀 Quick Deploy

### HuggingFace Spaces (Recommended First)

```bash
# 1. Create Space at https://huggingface.co/spaces
#    - Name: hackathon-games
#    - SDK: Gradio
#    - Visibility: Public

# 2. Clone and deploy
git clone https://huggingface.co/spaces/YOUR_USERNAME/hackathon-games
cd hackathon-games

# 3. Copy files
cp /mnt/data1/time2/time/2023/07/30/hackathon/app.py .
cp /mnt/data1/time2/time/2023/07/30/hackathon/requirements.txt .
cp /mnt/data1/time2/time/2023/07/30/hackathon/README.md .
cp -r /mnt/data1/time2/time/2023/07/30/hackathon/game .

# 4. Push
git add .
git commit -m "Deploy Project Provenance games"
git push
```

### Cloudflare Workers

```bash
# 1. Install wrangler
npm install -g wrangler

# 2. Login
wrangler login

# 3. Deploy
cd /mnt/data1/time2/time/2023/07/30/hackathon
wrangler deploy
```

---

## 📋 Pre-Deployment Checklist

### HuggingFace
- [ ] HuggingFace account created
- [ ] Space created with Gradio SDK
- [ ] `app.py` tested locally
- [ ] `requirements.txt` complete
- [ ] `game/` directory copied
- [ ] README.md configured

### Cloudflare
- [ ] Cloudflare account with Workers access
- [ ] `wrangler.toml` configured
- [ ] Worker name matches config
- [ ] API token set (if using GitHub Actions)
- [ ] `game/` directory in place

---

## 🔧 Configuration

### HuggingFace Space Settings

**README.md frontmatter:**
```yaml
---
title: Project Provenance - Hackathon
emoji: 🎮
sdk: gradio
sdk_version: 4.0.0
app_file: app.py
---
```

### Cloudflare Worker Settings

**wrangler.toml:**
```toml
name = "tradewars-hackathon"
main = "game/api.js"
compatibility_date = "2024-01-01"

[site]
bucket = "./game"
```

---

## 🌐 Access URLs

### After Deployment

**HuggingFace:**
```
https://huggingface.co/spaces/YOUR_USERNAME/hackathon-games
```

**Cloudflare:**
```
https://tradewars-hackathon.YOUR_SUBDOMAIN.workers.dev
```

---

## 🐛 Troubleshooting

### HuggingFace Issues

**Build fails:**
```bash
# Check logs in Space → Logs tab
# Common fixes:
pip install -r requirements.txt  # Test locally
python app.py  # Test app locally
```

**App doesn't load:**
- Verify `demo.launch()` is called
- Check for syntax errors in `app.py`
- Review error logs in Space

### Cloudflare Issues

**Deployment fails:**
```bash
# Check wrangler config
wrangler whoami
wrangler deploy --dry-run

# Common fixes:
# - Verify worker name matches wrangler.toml
# - Check API token permissions
# - Ensure game/ directory exists
```

---

## 📊 Monitoring

### HuggingFace
- View logs: Space → Logs tab
- Check usage: Space → Analytics
- Monitor uptime: Space status indicator

### Cloudflare
- View logs: Dashboard → Workers → Your Worker → Logs
- Check analytics: Dashboard → Analytics
- Monitor requests: Real-time metrics

---

## 🔄 Updates

### Update HuggingFace Space
```bash
cd hackathon-games
# Make changes
git add .
git commit -m "Update"
git push
```

### Update Cloudflare Worker
```bash
cd /mnt/data1/time2/time/2023/07/30/hackathon
# Make changes
wrangler deploy
```

---

## ✅ Post-Deployment Verification

### Test Checklist
- [ ] HuggingFace Space loads
- [ ] All 3 game tabs visible
- [ ] 8D game iframe loads
- [ ] Documentation links work
- [ ] Cloudflare Worker responds
- [ ] Game assets load correctly
- [ ] No console errors

---

## 📞 Support

- **HuggingFace:** https://huggingface.co/docs/hub/spaces
- **Cloudflare:** https://developers.cloudflare.com/workers/
- **GitHub Issues:** https://github.com/meta-introspector/hackathon/issues

---

**Status:** Ready for deployment  
**Estimated Time:** 15 minutes  
**Difficulty:** Beginner-friendly
