# GitHub Pages Deployment Guide

## Automatic Deployment (Recommended)

1. **Fork this repository to your GitHub account**

2. **Enable GitHub Actions**
   - Go to your repository settings (Settings)
   - Click "Actions" in the left menu
   - Select "Allow all actions and reusable workflows"

3. **Configure GitHub Pages**
   - Find "Pages" option in repository settings
   - Set Source to "Deploy from a branch"
   - Set Branch to "gh-pages"
   - Click Save

4. **Trigger Build**
   - Push any commit to the `main` branch
   - GitHub Actions will automatically build and deploy to `gh-pages` branch
   - Your website will be live at `https://your-username.github.io/visual_novel_recommendation_engine` in a few minutes

## Manual Deployment

If you want to deploy manually:

1. **Local Build**
   ```bash
   ./build_web.sh
   ```

2. **Create gh-pages branch**
   ```bash
   git checkout --orphan gh-pages
   git rm -rf .
   cp -r docs/* .
   git add .
   git commit -m "Deploy to GitHub Pages"
   git push origin gh-pages
   ```

3. **Configure GitHub Pages**
   - In repository settings, select "Deploy from a branch"
   - Set Branch to "gh-pages" / (root)

## Custom Domain

If you have your own domain:

1. Create a `CNAME` file in the `docs/` directory
2. File content should be your domain, e.g.: `vnrec.example.com`
3. Add a CNAME record in your DNS settings pointing to `your-username.github.io`

## Troubleshooting

### Build Failed
- Check GitHub Actions logs
- Ensure wasm-pack is properly installed
- Verify Rust code compiles

### Page Shows Blank
- Check browser console for errors
- Ensure WASM files load correctly
- Verify path settings are correct

### WASM Loading Error
- Ensure server supports `.wasm` file type
- GitHub Pages supports it by default, but some CDNs may need configuration

## Updating the Website

After modifying code:

1. Push to main branch
2. GitHub Actions will automatically rebuild
3. Updates will take effect in a few minutes

Or manually:

```bash
./build_web.sh
# Then push changes in docs/ directory to gh-pages branch
```
